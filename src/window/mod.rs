use std::sync::Mutex;
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Shell::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use crate::engine::state::GameState;
use crate::engine::setup::{CivilizationChoice, LeaderTrait, GameSpeed};
use crate::renderer::{DioramaRenderer, TacticalRenderer, TacticalTab, MenuRenderer, MenuScreen};

pub const APP_HEIGHT: i32 = 52;
pub const FLOATING_WIDTH: i32 = 740;
pub const TIMER_TICK_ID: usize = 1001;
pub const TIMER_ANIM_ID: usize = 1002;
pub const HOTKEY_FS_ID: i32 = 2001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    InMenu,
    PlayingTactical,
    PlayingBarWidget,
}

pub struct WindowContext {
    pub hwnd: HWND,
    pub app_state: AppState,
    pub state: GameState,
    pub menu: MenuRenderer,
    pub diorama: DioramaRenderer,
    pub tactical: TacticalRenderer,
    pub last_time: std::time::Instant,
}

unsafe impl Send for WindowContext {}
unsafe impl Sync for WindowContext {}

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

pub unsafe fn switch_app_state(hwnd: HWND, target_state: AppState) {
    unsafe {
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
        if let Some(ctx) = ctx_lock.as_mut() {
            let prev_state = ctx.app_state;
            ctx.app_state = target_state;

            if prev_state == AppState::PlayingBarWidget && target_state != AppState::PlayingBarWidget {
                register_appbar(hwnd, false);
            }

            match target_state {
                AppState::InMenu | AppState::PlayingTactical => {
                    let _ = SetWindowPos(
                        hwnd,
                        HWND_TOP,
                        0,
                        0,
                        screen_w,
                        screen_h,
                        SWP_SHOWWINDOW,
                    );
                    let _ = SetForegroundWindow(hwnd);
                    let _ = SetFocus(hwnd);
                }
                AppState::PlayingBarWidget => {
                    register_appbar(hwnd, true);
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
                let _ = SetTimer(hwnd, TIMER_TICK_ID, 1000, None);
                let _ = SetTimer(hwnd, TIMER_ANIM_ID, 16, None);
                LRESULT(0)
            }
            WM_TIMER => {
                if wparam.0 == TIMER_TICK_ID {
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        if ctx.app_state != AppState::InMenu {
                            let speed = ctx.state.config.speed.multiplier();
                            ctx.state.tick(speed);
                        }
                    }
                    let _ = InvalidateRect(hwnd, None, FALSE);
                } else if wparam.0 == TIMER_ANIM_ID {
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        let now = std::time::Instant::now();
                        let dt = now.duration_since(ctx.last_time).as_secs_f32();
                        ctx.last_time = now;
                        if ctx.app_state == AppState::PlayingBarWidget {
                            ctx.diorama.update(dt);
                        } else if ctx.app_state == AppState::PlayingTactical {
                            ctx.tactical.update(dt);
                        }
                    }
                    let _ = InvalidateRect(hwnd, None, FALSE);
                }
                LRESULT(0)
            }
            WM_NCHITTEST => {
                let ctx_lock = GLOBAL_CTX.lock().unwrap();
                if let Some(ctx) = ctx_lock.as_ref() {
                    if ctx.app_state != AppState::PlayingBarWidget {
                        return LRESULT(HTCLIENT as isize);
                    }

                    let x = (lparam.0 & 0xFFFF) as i16 as i32;
                    let mut win_rect = RECT::default();
                    let _ = GetWindowRect(hwnd, &mut win_rect);

                    if x >= win_rect.right - 90 && x <= win_rect.right {
                        return LRESULT(HTCLIENT as isize);
                    }
                    if x >= win_rect.left && x <= win_rect.left + 180 {
                        return LRESULT(HTCLIENT as isize);
                    }

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
                let height = win_rect.bottom - win_rect.top;

                let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                if let Some(ctx) = ctx_lock.as_mut() {
                    match ctx.app_state {
                        AppState::InMenu => {
                            match ctx.menu.current_screen {
                                MenuScreen::Main => {
                                    let btn_w = 460;
                                    let btn_h = 50;
                                    let sx = (width - btn_w) / 2;
                                    let mut sy = (height / 6) + 120;

                                    // Botón 1: NUEVA CAMPAÑA
                                    if x >= sx && x <= sx + btn_w && y >= sy && y <= sy + btn_h {
                                        ctx.menu.current_screen = MenuScreen::Setup;
                                    }
                                    sy += btn_h + 16;

                                    // Botón 2: CONTINUAR PARTIDA
                                    if x >= sx && x <= sx + btn_w && y >= sy && y <= sy + btn_h {
                                        drop(ctx_lock);
                                        switch_app_state(hwnd, AppState::PlayingTactical);
                                        return LRESULT(0);
                                    }
                                    sy += btn_h + 16;

                                    // Botón 3: ÁRBOL DE ASCENSIÓN
                                    if x >= sx && x <= sx + btn_w && y >= sy && y <= sy + btn_h {
                                        ctx.menu.current_screen = MenuScreen::AscensionTree;
                                    }
                                    sy += btn_h + 16;

                                    // Botón 4: CONFIGURACIÓN
                                    if x >= sx && x <= sx + btn_w && y >= sy && y <= sy + btn_h {
                                        ctx.menu.current_screen = MenuScreen::Settings;
                                    }
                                    sy += btn_h + 16;

                                    // Botón 5: SALIR
                                    if x >= sx && x <= sx + btn_w && y >= sy && y <= sy + btn_h {
                                        drop(ctx_lock);
                                        PostQuitMessage(0);
                                        return LRESULT(0);
                                    }
                                }
                                MenuScreen::Setup => {
                                    let card_w = (width - 120) / 3;
                                    let card_h = 75;

                                    // Civilizaciones
                                    for (i, civ) in CivilizationChoice::ALL.iter().enumerate() {
                                        let col = (i % 3) as i32;
                                        let row = (i / 3) as i32;
                                        let cx = 40 + col * (card_w + 20);
                                        let cy = 135 + row * (card_h + 12);
                                        if x >= cx && x <= cx + card_w && y >= cy && y <= cy + card_h {
                                            ctx.menu.config.civ = *civ;
                                        }
                                    }

                                    // Líderes
                                    let leader_y = 320;
                                    for (i, leader) in LeaderTrait::ALL.iter().enumerate() {
                                        let col = (i % 3) as i32;
                                        let row = (i / 3) as i32;
                                        let lx = 40 + col * (card_w + 20);
                                        let ly = leader_y + 30 + row * (card_h + 12);
                                        if x >= lx && x <= lx + card_w && y >= ly && y <= ly + card_h {
                                            ctx.menu.config.leader = *leader;
                                        }
                                    }

                                    // Velocidad
                                    let speed_y = 510;
                                    let speeds = [GameSpeed::Blitz, GameSpeed::Normal, GameSpeed::Epic];
                                    for (i, sp) in speeds.iter().enumerate() {
                                        let sx = 40 + (i as i32) * (card_w + 20);
                                        let sy = speed_y + 30;
                                        if x >= sx && x <= sx + card_w && y >= sy && y <= sy + 44 {
                                            ctx.menu.config.speed = *sp;
                                        }
                                    }

                                    // Iniciar Campaña
                                    if x >= width - 450 && x <= width - 40 && y >= height - 90 && y <= height - 35 {
                                        let chosen_cfg = ctx.menu.config.clone();
                                        ctx.state = GameState::new_with_config(chosen_cfg);
                                        drop(ctx_lock);
                                        switch_app_state(hwnd, AppState::PlayingTactical);
                                        return LRESULT(0);
                                    }

                                    // Volver
                                    if x >= 40 && x <= 280 && y >= height - 90 && y <= height - 35 {
                                        ctx.menu.current_screen = MenuScreen::Main;
                                    }
                                }
                                MenuScreen::AscensionTree | MenuScreen::Settings => {
                                    if x >= 40 && x <= 280 && y >= height - 90 && y <= height - 35 {
                                        ctx.menu.current_screen = MenuScreen::Main;
                                    }
                                }
                            }
                        }
                        AppState::PlayingTactical => {
                            // Clic en botón de minimizar a barra (esquina superior derecha)
                            if x >= width - 200 && y <= 54 {
                                drop(ctx_lock);
                                switch_app_state(hwnd, AppState::PlayingBarWidget);
                                return LRESULT(0);
                            }

                            // Clic en botones del Dock Inferior (Demise of Nations style)
                            let dock_w = 640;
                            let dock_x = (width - dock_w) / 2;
                            let dock_y = height - 76;
                            if y >= dock_y && y <= height - 8 && x >= dock_x && x <= dock_x + dock_w {
                                let btn_w = 90;
                                let rel_x = x - dock_x;
                                let tab_idx = rel_x / btn_w;
                                match tab_idx {
                                    0 => ctx.tactical.active_tab = TacticalTab::CampaignMap,
                                    1 => ctx.tactical.active_tab = TacticalTab::CityManager,
                                    2 => ctx.tactical.active_tab = TacticalTab::TechTree,
                                    3 => ctx.tactical.active_tab = TacticalTab::MilitaryCabinet,
                                    4 => ctx.tactical.active_tab = TacticalTab::WondersAndLog,
                                    _ => {
                                        // Clic en botón Pasar Turno / Velocidad
                                        if rel_x >= dock_w - 140 {
                                            let _ = ctx.state.advance_era();
                                        }
                                    }
                                }
                            }

                            // Interacciones dentro del mapa táctico
                            if ctx.tactical.active_tab == TacticalTab::CampaignMap {
                                // Clic en Botón "⚔️ DESPLEGAR" (Mover ejército en tiempo real hacia la provincia)
                                if x >= width - 365 && x <= width - 200 && y >= height - 150 && y <= height - 105 {
                                    let target_p = ctx.state.selected_province;
                                    ctx.state.order_army_to_province(1, target_p);
                                }
                                // Clic en Botón "🏗️ CONSTRUIR" (Abrir modal de ciudad)
                                else if x >= width - 190 && x <= width - 30 && y >= height - 150 && y <= height - 105 {
                                    ctx.tactical.active_tab = TacticalTab::CityManager;
                                }
                                // Clic en provincias sobre el mapa
                                else {
                                    let map_origin_x = (width / 2) + ctx.tactical.iso_world.camera_x;
                                    let map_origin_y = 70 + ctx.tactical.iso_world.camera_y;

                                    for (i, prov) in ctx.state.provinces.iter().enumerate() {
                                        let px = map_origin_x + ((prov.x - 0.5) * (width as f32 * 0.8)) as i32;
                                        let py = map_origin_y + ((prov.y - 0.4) * (height as f32 * 0.75)) as i32;
                                        if (x - px).abs() < 70 && (y - py).abs() < 50 {
                                            ctx.state.selected_province = i;
                                            break;
                                        }
                                    }
                                }
                            } else if ctx.tactical.active_tab == TacticalTab::CityManager {
                                // Construir distritos regionales D4X
                                if y >= 140 {
                                    let d_idx = ((y - 140) / 50) as usize;
                                    let districts = [
                                        crate::engine::state::RegionalDistrict::WaterCatchment,
                                        crate::engine::state::RegionalDistrict::PlastacreteMine,
                                        crate::engine::state::RegionalDistrict::TradingPost,
                                        crate::engine::state::RegionalDistrict::MilitaryPost,
                                        crate::engine::state::RegionalDistrict::ResearchOutpost,
                                    ];
                                    if let Some(d_type) = districts.get(d_idx) {
                                        let _ = ctx.state.build_district_in_province(ctx.state.selected_province, *d_type);
                                    }
                                }
                            } else if ctx.tactical.active_tab == TacticalTab::TechTree {
                                // Seleccionar doctrinas A vs B
                                if y >= 150 {
                                    let row = ((y - 150) / 82) as usize;
                                    let col = if x < (width / 2) { 0 } else { 1 };
                                    let tech_idx = row * 2 + col;
                                    if tech_idx < ctx.state.era_technologies.len() {
                                        let choice = if (y % 82) < 45 { 0 } else { 1 };
                                        ctx.state.select_technology_choice(tech_idx, choice);
                                    }
                                }
                            }
                        }
                        AppState::PlayingBarWidget => {
                            // Clic en botón "⚔️ Marcha" en la barra
                            if x >= width - 260 && x <= width - 180 {
                                let next_p = (ctx.state.selected_province + 1) % ctx.state.provinces.len();
                                ctx.state.selected_province = next_p;
                                ctx.state.order_army_to_province(1, next_p);
                            }
                            // Clic en botón "⚡ Blitz"
                            else if x >= width - 170 && x <= width - 100 {
                                ctx.state.config.speed = match ctx.state.config.speed {
                                    GameSpeed::Blitz => GameSpeed::Normal,
                                    GameSpeed::Normal => GameSpeed::Epic,
                                    GameSpeed::Epic => GameSpeed::Blitz,
                                };
                            }
                            // Clic en botón "⛶ F11" o en el orbe
                            else if x >= width - 90 || x <= 170 {
                                drop(ctx_lock);
                                switch_app_state(hwnd, AppState::PlayingTactical);
                                return LRESULT(0);
                            }
                        }
                    }
                }
                let _ = InvalidateRect(hwnd, None, FALSE);
                LRESULT(0)
            }
            WM_KEYDOWN => {
                if wparam.0 == VK_F11.0 as usize {
                    let cur_state = {
                        let ctx_lock = GLOBAL_CTX.lock().unwrap();
                        ctx_lock.as_ref().map(|c| c.app_state).unwrap_or(AppState::InMenu)
                    };
                    if cur_state == AppState::PlayingBarWidget {
                        switch_app_state(hwnd, AppState::PlayingTactical);
                    } else if cur_state == AppState::PlayingTactical {
                        switch_app_state(hwnd, AppState::PlayingBarWidget);
                    }
                } else if wparam.0 == VK_ESCAPE.0 as usize {
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        if ctx.app_state == AppState::InMenu && ctx.menu.current_screen != MenuScreen::Main {
                            ctx.menu.current_screen = MenuScreen::Main;
                        } else if ctx.app_state == AppState::PlayingTactical {
                            drop(ctx_lock);
                            switch_app_state(hwnd, AppState::PlayingBarWidget);
                            return LRESULT(0);
                        }
                    }
                } else if wparam.0 == 0x20 { // Barra espaciadora: Avanzar Era en partida
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        if ctx.app_state != AppState::InMenu {
                            let _ = ctx.state.advance_era();
                        }
                    }
                    let _ = InvalidateRect(hwnd, None, FALSE);
                }
                LRESULT(0)
            }
            WM_HOTKEY => {
                if wparam.0 == HOTKEY_FS_ID as usize {
                    let cur_state = {
                        let ctx_lock = GLOBAL_CTX.lock().unwrap();
                        ctx_lock.as_ref().map(|c| c.app_state).unwrap_or(AppState::InMenu)
                    };
                    if cur_state == AppState::PlayingBarWidget {
                        switch_app_state(hwnd, AppState::PlayingTactical);
                    } else if cur_state == AppState::PlayingTactical {
                        switch_app_state(hwnd, AppState::PlayingBarWidget);
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

                let mem_dc = CreateCompatibleDC(hdc);
                let mem_bitmap = CreateCompatibleBitmap(hdc, width, height);
                let old_bmp = SelectObject(mem_dc, mem_bitmap);

                {
                    let mut ctx_lock = GLOBAL_CTX.lock().unwrap();
                    if let Some(ctx) = ctx_lock.as_mut() {
                        match ctx.app_state {
                            AppState::InMenu => {
                                ctx.menu.render(mem_dc, &rect);
                            }
                            AppState::PlayingTactical => {
                                ctx.tactical.render(mem_dc, &rect, &ctx.state);
                            }
                            AppState::PlayingBarWidget => {
                                ctx.diorama.render(mem_dc, &rect, &ctx.state, false);
                            }
                        }
                    }
                }

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
