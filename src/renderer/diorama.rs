use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use crate::engine::state::GameState;
use crate::engine::eras::EraId;
use super::draw_ui_text;

pub struct DioramaRenderer {
    pub animation_frame: f32,
}

impl DioramaRenderer {
    pub fn new() -> Self {
        Self {
            animation_frame: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation_frame += dt * 6.0; // 6 frames por segundo para animación de marcha
    }

    pub unsafe fn render(
        &self,
        hdc: HDC,
        rect: &RECT,
        state: &GameState,
        is_floating: bool,
    ) {
        unsafe {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // 1. Dibujar fondo de marco según la era (Material Adaptativo)
            let (bg_color, border_color, text_color, era_tag_color) = match state.current_era {
                EraId::Paleolithic | EraId::Neolithic | EraId::Chalcolithic => (
                    COLORREF(0x001C1917), // Carbón basalto
                    COLORREF(0x00065F46), // Musgo/Ámbar
                    COLORREF(0x00E2E8F0), // Hueso claro
                    COLORREF(0x000684D9), // Ocre dorado
                ),
                EraId::BronzeAge | EraId::IronAge | EraId::LateAntiquity => (
                    COLORREF(0x000F172A), // Pizarra imperial
                    COLORREF(0x000E53B4), // Bronce bruñido
                    COLORREF(0x00FFFFFF), // Blanco mármol
                    COLORREF(0x000B9EF5), // Oro clásico
                ),
                EraId::EarlyMiddleAges | EraId::LateMiddleAges => (
                    COLORREF(0x0017191C), // Roble oscuro
                    COLORREF(0x001B1B99), // Carmesí heráldico
                    COLORREF(0x00F8FAFC), // Pergamino
                    COLORREF(0x002626DC), // Estandarte rojo
                ),
                EraId::Renaissance | EraId::Enlightenment | EraId::Industrial => (
                    COLORREF(0x001E293B), // Hierro forjado
                    COLORREF(0x00C78402), // Azul vapor / Latón
                    COLORREF(0x00FFFFFF), // Blanco puro
                    COLORREF(0x00E9A50E), // Brillo industrial
                ),
                _ => (
                    COLORREF(0x00080D16), // Obsidiana estelar
                    COLORREF(0x00F0F000), // Cian cuántico
                    COLORREF(0x00FFFFFF), // Blanco neón
                    COLORREF(0x00EF46D9), // Púrpura cuántico
                ),
            };

            let bg_brush = CreateSolidBrush(bg_color);
            FillRect(hdc, rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            // Borde superior iluminado
            let border_pen = CreatePen(PS_SOLID, 2, border_color);
            let old_pen = SelectObject(hdc, border_pen);
            let _ = MoveToEx(hdc, 0, 0, None);
            let _ = LineTo(hdc, width, 0);

            if is_floating {
                // Marco perimetral completo si es widget flotante
                let _ = LineTo(hdc, width, height);
                let _ = LineTo(hdc, 0, height);
                let _ = LineTo(hdc, 0, 0);
            }

            SelectObject(hdc, old_pen);
            let _ = DeleteObject(border_pen);

            SetBkMode(hdc, TRANSPARENT);

            // 2. Zona Izquierda: Orbe y Era
            let font_title = CreateFontW(
                13, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Outfit"),
            );
            let font_small = CreateFontW(
                10, 0, 0, 0, FW_NORMAL.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );

            let old_font = SelectObject(hdc, font_title);
            SetTextColor(hdc, era_tag_color);

            let era_title = format!("🏛️ {}", state.current_era.short_name());
            let mut title_rect = RECT { left: 12, top: 6, right: 180, bottom: 24 };
            draw_ui_text(hdc, &era_title, &mut title_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_small);
            SetTextColor(hdc, COLORREF(0x0094A3B8));
            let year_str = format!("Año {} | Pop: {}", state.year, state.population);
            let mut year_rect = RECT { left: 12, top: 25, right: 180, bottom: 44 };
            draw_ui_text(hdc, &year_str, &mut year_rect, DT_LEFT | DT_SINGLELINE);

            // 3. Zona Central: Diorama Viviente Animado (Micro-sprites procedurales)
            let diorama_start_x = 190;
            let diorama_end_x = width - 260;
            let diorama_width = (diorama_end_x - diorama_start_x).max(100);

            // Línea de suelo o riel de logística
            let floor_pen = CreatePen(PS_DOT, 1, COLORREF(0x00475569));
            let old_floor = SelectObject(hdc, floor_pen);
            let _ = MoveToEx(hdc, diorama_start_x, height - 14, None);
            let _ = LineTo(hdc, diorama_end_x, height - 14);
            SelectObject(hdc, old_floor);
            let _ = DeleteObject(floor_pen);

            // Dibujar figuras animadas según la época
            let walk_cycle = (self.animation_frame as i32) % 4;
            let bobbing_y = if walk_cycle % 2 == 0 { 0 } else { -2 };

            // Posición del viajero en tránsito
            let travel_t = (state.epoch_time * 0.1) % 1.0;
            let figure_x = diorama_start_x + (travel_t * (diorama_width - 40) as f32) as i32;
            let figure_y = height - 28 + bobbing_y;

            SetTextColor(hdc, text_color);
            let figure_sprite = match state.current_era {
                EraId::Paleolithic | EraId::Neolithic => "🏃🌾 [Cazador]",
                EraId::Chalcolithic | EraId::BronzeAge => "🛞🏺 [Carreta]",
                EraId::IronAge | EraId::LateAntiquity => "🛡️🐎 [Legión]",
                EraId::EarlyMiddleAges | EraId::LateMiddleAges => "⚔️🐴 [Caballero]",
                EraId::Renaissance => "⛵📜 [Carabela]",
                EraId::Enlightenment => "🎩🔭 [Ilustrado]",
                EraId::Industrial => "🚂💨 [Locomotora]",
                EraId::Atomic => "✈️⚛️ [Reactor]",
                EraId::SolarExpansion | EraId::Interstellar => "🚀🌌 [Carguero]",
                EraId::Singularity => "⚛️✨ [Trascendencia]",
            };

            let mut sprite_rect = RECT { left: figure_x, top: figure_y, right: figure_x + 120, bottom: figure_y + 18 };
            draw_ui_text(hdc, figure_sprite, &mut sprite_rect, DT_LEFT | DT_SINGLELINE);

            // 4. Zona Derecha: Recursos Consolidados y Botón Pantalla Completa
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let res_str = format!(
                "🌾{:.0} 🪵{:.0} 🪙{:.0} 🔬{:.0}",
                state.food, state.materials, state.gold, state.science
            );
            let mut res_rect = RECT { left: width - 250, top: 14, right: width - 70, bottom: 34 };
            draw_ui_text(hdc, &res_str, &mut res_rect, DT_RIGHT | DT_SINGLELINE);

            // Botón Pantalla Completa (F11)
            let btn_rect = RECT { left: width - 58, top: 8, right: width - 10, bottom: 40 };
            let btn_brush = CreateSolidBrush(COLORREF(0x00334155));
            FillRect(hdc, &btn_rect, btn_brush);
            let _ = DeleteObject(btn_brush);

            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut btn_text_rect = RECT { left: width - 58, top: 15, right: width - 10, bottom: 35 };
            draw_ui_text(hdc, "⛶ F11", &mut btn_text_rect, DT_CENTER | DT_SINGLELINE);

            SelectObject(hdc, old_font);
            let _ = DeleteObject(font_title);
            let _ = DeleteObject(font_small);
        }
    }
}
