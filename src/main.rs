#![windows_subsystem = "windows"]

pub mod engine;
pub mod renderer;
pub mod window;

use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
    Win32::System::LibraryLoader::*,
};

use engine::state::GameState;
use renderer::{DioramaRenderer, TacticalRenderer, MenuRenderer};
use window::*;

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("TaskBar4XAlphaClass");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance.into(),
            lpszClassName: window_class,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        RegisterClassW(&wc);

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        // Crear ventana inicial a Pantalla Completa para el Menú y Configuración
        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW,
            window_class,
            w!("TASK BAR 4X - Imperivm Saeculorum"),
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            screen_w,
            screen_h,
            None,
            None,
            instance,
            None,
        )?;

        // Inicializar contexto del juego en estado Menú
        let ctx = WindowContext {
            hwnd,
            app_state: AppState::InMenu,
            state: GameState::new(),
            menu: MenuRenderer::new(),
            diorama: DioramaRenderer::new(),
            tactical: TacticalRenderer::new(),
            last_time: std::time::Instant::now(),
        };

        {
            let mut global_lock = GLOBAL_CTX.lock().unwrap();
            *global_lock = Some(Box::new(ctx));
        }

        let _ = SetForegroundWindow(hwnd);
        let _ = SetFocus(hwnd);

        // Registrar HotKey global Win + Alt + X
        let _ = RegisterHotKey(hwnd, HOTKEY_FS_ID, MOD_ALT | MOD_WIN, 0x58);

        // Bucle de mensajes reactivo con WaitMessage()
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
            let _ = WaitMessage();
        }

        let _ = UnregisterHotKey(hwnd, HOTKEY_FS_ID);
        register_appbar(hwnd, false);
    }

    Ok(())
}
