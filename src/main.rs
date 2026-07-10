#![windows_subsystem = "windows"]

use std::sync::OnceLock;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Shell::*,
    Win32::Graphics::Gdi::*,
    Win32::Graphics::Direct2D::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Dxgi::Common::*,
    Win32::System::LibraryLoader::*,
    Win32::System::Com::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

// Identificadores de Win32
const TIMER_ID: usize = 1001;
const HOTKEY_ID: i32 = 2001;
const APP_HEIGHT: i32 = 48;

// Estructuras seguras para almacenar manejadores Win32 en OnceLock (Send/Sync)
struct SafeHWND(HWND);
unsafe impl Send for SafeHWND {}
unsafe impl Sync for SafeHWND {}

struct SafeHHOOK(HHOOK);
unsafe impl Send for SafeHHOOK {}
unsafe impl Sync for SafeHHOOK {}

// Estado global estático para el hook y el control de la ventana
static HWND_MAIN: OnceLock<SafeHWND> = OnceLock::new();
static HHOOK_MOUSE: OnceLock<SafeHHOOK> = OnceLock::new();
static IS_FULLSCREEN: OnceLock<std::sync::atomic::AtomicBool> = OnceLock::new();

// Variables lógicas de la simulación del imperio
static SIM_ERA: OnceLock<std::sync::Mutex<String>> = OnceLock::new();
static SIM_YEARS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

// Punteros a Direct2D
static D2D_FACTORY: OnceLock<ID2D1Factory> = OnceLock::new();
static D2D_RENDER_TARGET: OnceLock<std::sync::Mutex<Option<ID2D1HwndRenderTarget>>> = OnceLock::new();

fn get_sim_era() -> String {
    SIM_ERA.get_or_init(|| std::sync::Mutex::new("Edad de Piedra".to_string()))
        .lock()
        .unwrap()
        .clone()
}

fn set_sim_era(new_era: &str) {
    let mut era = SIM_ERA.get_or_init(|| std::sync::Mutex::new("Edad de Piedra".to_string()))
        .lock()
        .unwrap();
    *era = new_era.to_string();
}

fn main() -> Result<()> {
    // Inicializar el modelo COM en modo de apartamento
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    }

    IS_FULLSCREEN.get_or_init(|| std::sync::atomic::AtomicBool::new(false));
    SIM_ERA.get_or_init(|| std::sync::Mutex::new("Edad de Piedra".to_string()));
    D2D_RENDER_TARGET.get_or_init(|| std::sync::Mutex::new(None));

    // Inicializar Direct2D Factory
    let factory = unsafe {
        D2D1CreateFactory::<ID2D1Factory>(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?
    };
    D2D_FACTORY.get_or_init(|| factory);

    unsafe {
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("TaskBar4XClass");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance.into(),
            lpszClassName: window_class,
            hbrBackground: HBRUSH(GetStockObject(BLACK_BRUSH).0),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        RegisterClassW(&wc);

        // Obtener tamaño de pantalla principal
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        // Crear la ventana con estilos no intrusivos
        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_LAYERED,
            window_class,
            w!("TASK BAR 4X"),
            WS_POPUP | WS_VISIBLE,
            0,
            screen_height - APP_HEIGHT,
            screen_width,
            APP_HEIGHT,
            None,
            None,
            instance,
            None,
        )?;

        HWND_MAIN.get_or_init(|| SafeHWND(hwnd));

        // Configurar transparencia de capa base (85% opacidad)
        SetLayeredWindowAttributes(hwnd, COLORREF(0), 220, LWA_ALPHA)?;

        // Registrar como AppBar
        register_appbar(hwnd, true);

        // Registrar atajo global Win + Alt + X
        RegisterHotKey(hwnd, HOTKEY_ID, MOD_ALT | MOD_WIN, 0x58)?; // 0x58 = Tecla 'X'

        // Configurar temporizador lógico (1 Hz)
        SetTimer(hwnd, TIMER_ID, 1000, None);

        // Instalar gancho de ratón de bajo nivel para clics passthrough
        let hook = SetWindowsHookExW(
            WH_MOUSE_LL,
            Some(mouse_hook_proc),
            instance,
            0,
        )?;
        HHOOK_MOUSE.get_or_init(|| SafeHHOOK(hook));

        // Bucle de mensajes reactivo con WaitMessage()
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
            
            // Hibernar el hilo hasta que llegue un nuevo mensaje o venza el temporizador
            WaitMessage();
        }

        // Limpieza final
        if let Some(safe_hook) = HHOOK_MOUSE.get() {
            UnhookWindowsHookEx(safe_hook.0)?;
        }
        UnregisterHotKey(hwnd, HOTKEY_ID)?;
        register_appbar(hwnd, false);
        CoUninitialize();
    }

    Ok(())
}

// Registro y dimensionamiento del AppBar en Windows
unsafe fn register_appbar(hwnd: HWND, register: bool) {
    let mut abd = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        hWnd: hwnd,
        ..Default::default()
    };

    if register {
        // Registrar nueva barra
        SHAppBarMessage(ABM_NEW, &mut abd);

        // Configurar posición
        abd.uEdge = ABE_BOTTOM;
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        abd.rc = RECT {
            left: 0,
            top: screen_height - APP_HEIGHT,
            right: screen_width,
            bottom: screen_height,
        };

        // Negociar y aplicar coordenadas físicas
        SHAppBarMessage(ABM_QUERYPOS, &mut abd);
        SHAppBarMessage(ABM_SETPOS, &mut abd);
        
        // Redimensionar ventana
        SetWindowPos(
            hwnd,
            HWND_TOP,
            abd.rc.left,
            abd.rc.top,
            abd.rc.right - abd.rc.left,
            abd.rc.bottom - abd.rc.top,
            SWP_NOACTIVATE | SWP_SHOWWINDOW,
        ).unwrap();
    } else {
        // Remover barra
        SHAppBarMessage(ABM_REMOVE, &mut abd);
    }
}

// Bucle de procedimiento de ventana (WndProc)
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            // Inicializar render target de Direct2D
            if let Some(factory) = D2D_FACTORY.get() {
                let mut rect = RECT::default();
                GetClientRect(hwnd, &mut rect).unwrap();

                let render_properties = D2D1_RENDER_TARGET_PROPERTIES {
                    r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                    pixelFormat: D2D1_PIXEL_FORMAT {
                        format: DXGI_FORMAT_B8G8R8A8_UNORM,
                        alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                    },
                    ..Default::default()
                };

                let hwnd_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd,
                    pixelSize: D2D_SIZE_U {
                        width: (rect.right - rect.left) as u32,
                        height: (rect.bottom - rect.top) as u32,
                    },
                    presentOptions: D2D1_PRESENT_OPTIONS_NONE,
                };

                if let Ok(rt) = factory.CreateHwndRenderTarget(&render_properties, &hwnd_properties) {
                    let mut global_rt = D2D_RENDER_TARGET.get().unwrap().lock().unwrap();
                    *global_rt = Some(rt);
                }
            }
            LRESULT(0)
        }
        WM_TIMER => {
            if wparam.0 == TIMER_ID {
                // Actualizar simulación (1 tick = 1 año)
                let current_year = SIM_YEARS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                
                // Evolución de era automatizada para la demostración del Alpha
                if current_year == 10 {
                    set_sim_era("Edad del Neolítico");
                } else if current_year == 25 {
                    set_sim_era("Edad del Bronce");
                } else if current_year == 50 {
                    set_sim_era("Edad Espacial");
                }

                // Solicitar redibujado de la ventana
                InvalidateRect(hwnd, None, FALSE);
            }
            LRESULT(0)
        }
        WM_HOTKEY => {
            if wparam.0 == HOTKEY_ID as usize {
                // Alternar modo Pantalla Completa
                if let Some(is_fs) = IS_FULLSCREEN.get() {
                    let was_fs = is_fs.fetch_xor(true, std::sync::atomic::Ordering::Relaxed);
                    let is_now_fs = !was_fs;

                    if is_now_fs {
                        // Desacoplar de la barra de tareas para ir a pantalla completa
                        let mut abd = APPBARDATA {
                            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
                            hWnd: hwnd,
                            ..Default::default()
                        };
                        SHAppBarMessage(ABM_REMOVE, &mut abd);

                        // Ajustar tamaño completo
                        let w = GetSystemMetrics(SM_CXSCREEN);
                        let h = GetSystemMetrics(SM_CYSCREEN);
                        SetWindowPos(hwnd, HWND_TOP, 0, 0, w, h, SWP_SHOWWINDOW).unwrap();
                    } else {
                        // Volver a acoplar en la barra inferior
                        register_appbar(hwnd, true);
                    }
                    InvalidateRect(hwnd, None, FALSE);
                }
            }
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            // Intentar renderizar con Direct2D
            let mut rendered_d2d = false;
            if let Some(global_rt_mutex) = D2D_RENDER_TARGET.get() {
                let mut global_rt = global_rt_mutex.lock().unwrap();
                if let Some(ref rt) = *global_rt {
                    // Obtener la interfaz base ID2D1RenderTarget para poder llamar los métodos
                    if let Ok(rt_base) = rt.cast::<ID2D1RenderTarget>() {
                        rt_base.BeginDraw();
                        
                        // Fondo oscuro translúcido
                        rt_base.Clear(Some(&D2D1_COLOR_F { r: 0.02, g: 0.02, b: 0.02, a: 1.0 }));

                        // Línea superior de borde de neón azul
                        let stroke_brush = rt_base.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.22, g: 0.74, b: 0.97, a: 1.0 }, None).unwrap();
                        
                        let mut rect = RECT::default();
                        GetClientRect(hwnd, &mut rect).unwrap();
                        let width = (rect.right - rect.left) as f32;
                        let height = (rect.bottom - rect.top) as f32;

                        // Dibujar borde cian superior
                        rt_base.DrawLine(
                            D2D_POINT_2F { x: 0.0, y: 1.0 },
                            D2D_POINT_2F { x: width, y: 1.0 },
                            &stroke_brush,
                            2.0,
                            None,
                        );

                        // Si está en pantalla completa, dibujar panel táctico
                        if IS_FULLSCREEN.get().unwrap().load(std::sync::atomic::Ordering::Relaxed) {
                            let fill_brush = rt_base.CreateSolidColorBrush(&D2D1_COLOR_F { r: 0.1, g: 0.1, b: 0.15, a: 0.8 }, None).unwrap();
                            rt_base.FillRectangle(
                                &D2D_RECT_F { left: 50.0, top: 50.0, right: width - 50.0, bottom: height - 50.0 },
                                &fill_brush,
                            );
                        }

                        let _ = rt_base.EndDraw(None, None);
                        rendered_d2d = true;
                    }
                }
            }

            // GDI Fallback para textos si Direct2D falla o para compatibilidad de depuración
            if rendered_d2d {
                // Dibujar textos informativos usando GDI transparente sobre la capa Direct2D
                SetBkMode(hdc, TRANSPARENT);
                
                // Color de texto cian/blanco
                SetTextColor(hdc, COLORREF(0x00FFFFFF)); // Blanco
                
                let mut rect = RECT::default();
                GetClientRect(hwnd, &mut rect).unwrap();

                // Construir la cadena de información
                let years = SIM_YEARS.load(std::sync::atomic::Ordering::Relaxed);
                let era = get_sim_era();
                let is_fs = IS_FULLSCREEN.get().unwrap().load(std::sync::atomic::Ordering::Relaxed);
                
                let mode_str = if is_fs { "PANTALLA COMPLETA (Win+Alt+X para salir)" } else { "MODO BARRA DE TAREAS" };
                let info_text = format!(
                    " TASK BAR 4X | [{}] | Era: {} | Año: {} | ⚡ CPU: 50% | ⚛️ RAM: 12GB | 🛡️ E/S: 4.5T ",
                    mode_str, era, years
                );
                
                let mut wide_text: Vec<u16> = info_text.encode_utf16().chain(std::iter::once(0)).collect();
                
                // Configurar fuente usando los valores crudos (.0) de las constantes newtype de windows-rs con casts primitivos
                let font = CreateFontW(
                    14, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                    DEFAULT_CHARSET.0 as u32,
                    OUT_DEFAULT_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    CLEARTYPE_QUALITY.0 as u32,
                    DEFAULT_PITCH.0 as u32,
                    w!("Outfit"),
                );
                let old_font = SelectObject(hdc, font);

                // Dibujar el texto centrado verticalmente
                let mut text_rect = RECT {
                    left: 20,
                    top: 15,
                    right: rect.right - 20,
                    bottom: rect.bottom,
                };
                
                let len = wide_text.len();
                DrawTextW(
                    hdc,
                    &mut wide_text[..len - 1],
                    &mut text_rect,
                    DT_LEFT | DT_VCENTER | DT_SINGLELINE,
                );

                // Marca de agua roja de copyright en la esquina derecha
                SetTextColor(hdc, COLORREF(0x005555FF)); // Rojo suave
                let water_text = format!("PROPIEDAD DE BABYLON.IA S.A.S.  ");
                let mut water_wide: Vec<u16> = water_text.encode_utf16().chain(std::iter::once(0)).collect();
                
                let mut water_rect = RECT {
                    left: rect.right - 400,
                    top: 15,
                    right: rect.right - 20,
                    bottom: rect.bottom,
                };
                
                let water_len = water_wide.len();
                DrawTextW(
                    hdc,
                    &mut water_wide[..water_len - 1],
                    &mut water_rect,
                    DT_RIGHT | DT_VCENTER | DT_SINGLELINE,
                );

                SelectObject(hdc, old_font);
                DeleteObject(font);
            }

            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// Procedimiento de gancho (hook) de ratón global de bajo nivel (Rust 2024 requiere bloques unsafe explícitos)
unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if code >= 0 {
            let mouse_struct = *(lparam.0 as *const MSLLHOOKSTRUCT);
            
            if let Some(safe_hwnd) = HWND_MAIN.get() {
                let hwnd = safe_hwnd.0;
                // Verificar si el cursor está sobre la ventana del HUD
                let mut win_rect = RECT::default();
                GetWindowRect(hwnd, &mut win_rect).unwrap();

                let pt = mouse_struct.pt;
                let is_over_hud = pt.x >= win_rect.left 
                    && pt.x <= win_rect.right 
                    && pt.y >= win_rect.top 
                    && pt.y <= win_rect.bottom;

                if is_over_hud {
                    let is_fs = IS_FULLSCREEN.get().map(|fs| fs.load(std::sync::atomic::Ordering::Relaxed)).unwrap_or(false);
                    
                    if !is_fs {
                        // En modo HUD:
                        // Si el cursor está en el lado izquierdo del HUD (donde está el Orbe / botón de modo),
                        // removemos WS_EX_TRANSPARENT para permitir clics.
                        // Si está en el lado derecho vacío, agregamos WS_EX_TRANSPARENT para permitir passthrough.
                        let middle_x = win_rect.left + (win_rect.right - win_rect.left) / 2;
                        let current_ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;

                        if pt.x < middle_x {
                            // Área activa (izquierda): Remover passthrough
                            if (current_ex_style & WS_EX_TRANSPARENT.0) != 0 {
                                let new_style = current_ex_style & !WS_EX_TRANSPARENT.0;
                                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style as isize);
                                // Forzar actualización de estilos de ventana
                                SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED).unwrap();
                            }
                        } else {
                            // Área transparente (derecha): Aplicar passthrough
                            if (current_ex_style & WS_EX_TRANSPARENT.0) == 0 {
                                let new_style = current_ex_style | WS_EX_TRANSPARENT.0;
                                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style as isize);
                                SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED).unwrap();
                            }
                        }
                    } else {
                        // En modo pantalla completa: Siempre desactivar passthrough para poder interactuar libremente
                        let current_ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
                        if (current_ex_style & WS_EX_TRANSPARENT.0) != 0 {
                            let new_style = current_ex_style & !WS_EX_TRANSPARENT.0;
                            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style as isize);
                            SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED).unwrap();
                        }
                    }
                }
            }
        }
        CallNextHookEx(None, code, wparam, lparam)
    }
}
