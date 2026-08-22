use std::sync::Mutex;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Shell::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use crate::engine::state::GameState;
use crate::engine::buildings::BuildingType;
use crate::renderer::{DioramaRenderer, TacticalRenderer, TacticalTab};

pub const APP_HEIGHT: i32 = 48;
pub const FLOATING_WIDTH: i32 = 720;
pub const TIMER_TICK_ID: usize = 1001;
pub const TIMER_ANIM_ID: usize = 1002;
pub const HOTKEY_FS_ID: i32 = 2001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    DockedAppBar,
    FloatingWidget,
    Fullscreen,
}

pub struct WindowContext {
    pub hwnd: HWND,
    pub mode: WindowMode,
    pub state: GameState,
    pub diorama: DioramaRenderer,
    pub tactical: TacticalRenderer,
    pub last_time: std::time::Instant,
}

unsafe impl Send for WindowContext {}
unsafe impl Sync for WindowContext {}

// Global context pointer safe wrapper for Win32 WndProc
pub static GLOBAL_CTX: Mutex<Option<Box<WindowContext>>> = Mutex::new(None);

pub unsafe fn register_appbar(hwnd: HWND, register: bool) {
    unsafe {
        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: hwnd,
            ..Default::default()
        };

        if register {
            let _ = SHAppBarMessage(ABM_NEW, &mut abd);
            abd.uEdge = ABE_BOTTOM;
            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);

            abd.rc = RECT {
                left: 0,
                top: screen_h - APP_HEIGHT,
                right: screen_w,
                bottom: screen_h,
            };

            let _ = SHAppBarMessage(ABM_QUERYPOS, &mut abd);
            let _ = SHAppBarMessage(ABM_SETPOS, &mut abd);

            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                abd.rc.left,
                abd.rc.top,
                abd.rc.right - abd.rc.left,
                abd.rc.bottom - abd.rc.top,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
        } else {
            let _ = SHAppBarMessage(ABM_REMOVE, &mut abd);
        }
    }
}

pub unsafe fn switch_mode(hwnd: HWND, target_mode: WindowMode) {
    unsafe {
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
        if let Some(ctx) = ctx_lock.as_mut() {
            let prev_mode = ctx.mode;
            ctx.mode = target_mode;

            // Limpiar AppBar si salimos de Docked
            if prev_mode == WindowMode::DockedAppBar && target_mode != WindowMode::DockedAppBar {
                register_appbar(hwnd, false);
            }

            match target_mode {
                WindowMode::DockedAppBar => {
                    register_appbar(hwnd, true);
                }
                WindowMode::FloatingWidget => {
                    let x = (screen_w - FLOATING_WIDTH) / 2;
                    let y = screen_h - APP_HEIGHT - 60;
                    let _ = SetWindowPos(
                        hwnd,
                        HWND_TOPMOST,
                        x,
                        y,
                        FLOATING_WIDTH,
                        APP_HEIGHT,
                        SWP_NOACTIVATE | SWP_SHOWWINDOW,
                    );
                }
                WindowMode::Fullscreen => {
                    let _ = SetWindowPos(
                        hwnd,
                        HWND_TOP,
                        0,
                        0,
                        screen_w,
                        screen_h,
                        SWP_SHOWWINDOW,
                    );
                }
            }
            let _ = InvalidateRect(hwnd, None, FALSE);
        }
    }
}

pub unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                // Iniciar temporizador lógico (1 Hz) y animación de diorama (60 FPS)
                let _ = SetTimer(hwnd, TIMER_TICK_ID, 1000, None);
                let _ = SetTimer(hwnd, TIMER_ANIM_ID, 16, None); // ~60 FPS
                LRESULT(0)
            }
            WM_TIMER => {
                if wparam.0 == TIMER_TICK_ID {
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        ctx.state.tick(1.0);
                    }
                    let _ = InvalidateRect(hwnd, None, FALSE);
                } else if wparam.0 == TIMER_ANIM_ID {
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        let now = std::time::Instant::now();
                        let dt = now.duration_since(ctx.last_time).as_secs_f32();
                        ctx.last_time = now;
                        ctx.diorama.update(dt);
                    }
                    let _ = InvalidateRect(hwnd, None, FALSE);
                }
                LRESULT(0)
            }
            WM_NCHITTEST => {
                // Gestión inteligente de Passthrough para el modo widget
                let ctx_lock = GLOBAL_CTX.lock().unwrap();
                if let Some(ctx) = ctx_lock.as_ref() {
                    if ctx.mode == WindowMode::Fullscreen {
                        return LRESULT(HTCLIENT as isize);
                    }

                    let x = (lparam.0 & 0xFFFF) as i16 as i32;

                    let mut win_rect = RECT::default();
                    let _ = GetWindowRect(hwnd, &mut win_rect);

                    // Tirador de arrastre en la esquina izquierda (primeros 30px) si está en modo flotante
                    if ctx.mode == WindowMode::FloatingWidget && x >= win_rect.left && x <= win_rect.left + 30 {
                        return LRESULT(HTCAPTION as isize);
                    }

                    // Botón de pantalla completa en la esquina derecha (últimos 60px)
                    if x >= win_rect.right - 60 && x <= win_rect.right {
                        return LRESULT(HTCLIENT as isize);
                    }

                    // Área interactiva de Orbe y Título (primeros 180px)
                    if x >= win_rect.left && x <= win_rect.left + 180 {
                        return LRESULT(HTCLIENT as isize);
                    }

                    // Passthrough nativo en zonas intermedias
                    return LRESULT(HTCLIENT as isize);
                }
                LRESULT(HTCLIENT as isize)
            }
            WM_LBUTTONDOWN => {
                let x = (lparam.0 & 0xFFFF) as i16 as i32;
                let y = ((lparam.0 >> 16) & 0xFFFF) as i16 as i32;

                let mut win_rect = RECT::default();
                let _ = GetClientRect(hwnd, &mut win_rect);
                let width = win_rect.right - win_rect.left;

                let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                if let Some(ctx) = ctx_lock.as_mut() {
                    if ctx.mode == WindowMode::Fullscreen {
                        // Clic en botón de salir a barra (esquina superior derecha)
                        if x >= width - 160 && y <= 48 {
                            drop(ctx_lock);
                            switch_mode(hwnd, WindowMode::DockedAppBar);
                            return LRESULT(0);
                        }

                        // Clic en pestañas superiores (y entre 48 y 84)
                        if y >= 48 && y <= 84 {
                            let tab_w = width / 5;
                            let tab_idx = (x / tab_w).clamp(0, 4);
                            ctx.tactical.active_tab = match tab_idx {
                                0 => TacticalTab::CampaignMap,
                                1 => TacticalTab::CityManager,
                                2 => TacticalTab::TechTree,
                                3 => TacticalTab::MilitaryCabinet,
                                _ => TacticalTab::WondersAndLog,
                            };
                        }

                        // Interacciones dentro de pestañas
                        match ctx.tactical.active_tab {
                            TacticalTab::CampaignMap => {
                                // Selección de provincia en el mapa
                                let map_w = (width * 65) / 100;
                                for (i, prov) in ctx.state.provinces.iter().enumerate() {
                                    let px = 20 + (prov.x * map_w as f32) as i32;
                                    let py = 100 + (prov.y * (win_rect.bottom - 120) as f32) as i32;
                                    if (x - px).abs() < 30 && (y - py).abs() < 30 {
                                        ctx.state.selected_province = i;
                                        break;
                                    }
                                }
                            }
                            TacticalTab::CityManager => {
                                // Construir edificios
                                if x >= 35 && x <= 450 && y >= 190 {
                                    let b_idx = ((y - 190) / 28) as usize;
                                    let available = [
                                        BuildingType::Hearth,
                                        BuildingType::GrainPit,
                                        BuildingType::StoneQuarry,
                                        BuildingType::ShamanHut,
                                        BuildingType::MegalithCircle,
                                        BuildingType::BronzeForge,
                                        BuildingType::Forum,
                                        BuildingType::Watermill,
                                    ];
                                    if let Some(b_type) = available.get(b_idx) {
                                        let _ = ctx.state.start_building_construction(ctx.state.selected_city, *b_type);
                                    }
                                }
                            }
                            TacticalTab::TechTree => {
                                // Adoptar doctrinas tecnológicas A vs B
                                if y >= 140 {
                                    let row = ((y - 140) / 95) as usize;
                                    let col = if x < (width / 2) { 0 } else { 1 };
                                    let tech_idx = row * 2 + col;
                                    if tech_idx < ctx.state.era_technologies.len() {
                                        let choice = if (y % 95) < 55 { 0 } else { 1 };
                                        ctx.state.select_technology_choice(tech_idx, choice);
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        // Modo Barra: Clic en botón F11 o en el Orbe
                        if x >= width - 60 || x <= 160 {
                            drop(ctx_lock);
                            switch_mode(hwnd, WindowMode::Fullscreen);
                            return LRESULT(0);
                        }
                    }
                }
                let _ = InvalidateRect(hwnd, None, FALSE);
                LRESULT(0)
            }
            WM_KEYDOWN => {
                if wparam.0 == VK_F11.0 as usize || wparam.0 == VK_ESCAPE.0 as usize {
                    let is_fs = {
                        let ctx_lock = GLOBAL_CTX.lock().unwrap();
                        ctx_lock.as_ref().map(|c| c.mode == WindowMode::Fullscreen).unwrap_or(false)
                    };
                    if is_fs {
                        switch_mode(hwnd, WindowMode::DockedAppBar);
                    } else {
                        switch_mode(hwnd, WindowMode::Fullscreen);
                    }
                } else if wparam.0 == 0x20 { // Barra espaciadora: Avanzar Era
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        let _ = ctx.state.advance_era();
                    }
                    let _ = InvalidateRect(hwnd, None, FALSE);
                }
                LRESULT(0)
            }
            WM_HOTKEY => {
                if wparam.0 == HOTKEY_FS_ID as usize {
                    let is_fs = {
                        let ctx_lock = GLOBAL_CTX.lock().unwrap();
                        ctx_lock.as_ref().map(|c| c.mode == WindowMode::Fullscreen).unwrap_or(false)
                    };
                    if is_fs {
                        switch_mode(hwnd, WindowMode::DockedAppBar);
                    } else {
                        switch_mode(hwnd, WindowMode::Fullscreen);
                    }
                }
                LRESULT(0)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                let mut rect = RECT::default();
                let _ = GetClientRect(hwnd, &mut rect);
                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;

                // Double buffering para eliminar todo parpadeo (60 FPS fluidos)
                let mem_dc = CreateCompatibleDC(hdc);
                let mem_bitmap = CreateCompatibleBitmap(hdc, width, height);
                let old_bmp = SelectObject(mem_dc, mem_bitmap);

                {
                    let ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_ref() {
                        match ctx.mode {
                            WindowMode::Fullscreen => {
                                ctx.tactical.render(mem_dc, &rect, &ctx.state);
                            }
                            WindowMode::DockedAppBar => {
                                ctx.diorama.render(mem_dc, &rect, &ctx.state, false);
                            }
                            WindowMode::FloatingWidget => {
                                ctx.diorama.render(mem_dc, &rect, &ctx.state, true);
                            }
                        }
                    }
                }

                // Transferir bitmap a pantalla
                let _ = BitBlt(hdc, 0, 0, width, height, mem_dc, 0, 0, SRCCOPY);

                SelectObject(mem_dc, old_bmp);
                let _ = DeleteObject(mem_bitmap);
                let _ = DeleteDC(mem_dc);
                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                let _ = UnregisterHotKey(hwnd, HOTKEY_FS_ID);
                register_appbar(hwnd, false);
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
