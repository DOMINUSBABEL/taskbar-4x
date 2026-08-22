use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use crate::engine::state::GameState;
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
        self.animation_frame += dt * 8.0;
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

            // Fondo de consola de mando metálica oscura con remaches (Demise of Nations style)
            let bg_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            // Marco superior e inferior de oro y acero bruñido
            let gold_pen = CreatePen(PS_SOLID, 2, COLORREF(0x000B9EF5));
            let old_pen = SelectObject(hdc, gold_pen);
            let _ = MoveToEx(hdc, 0, 0, None);
            let _ = LineTo(hdc, width, 0);
            let _ = MoveToEx(hdc, 0, height - 1, None);
            let _ = LineTo(hdc, width, height - 1);

            if is_floating {
                let _ = MoveToEx(hdc, 0, 0, None);
                let _ = LineTo(hdc, 0, height);
                let _ = MoveToEx(hdc, width - 1, 0, None);
                let _ = LineTo(hdc, width - 1, height);
            }
            SelectObject(hdc, old_pen);
            let _ = DeleteObject(gold_pen);

            SetBkMode(hdc, TRANSPARENT);

            let font_bold = CreateFontW(
                12, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );
            let font_res = CreateFontW(
                14, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Outfit"),
            );

            // 1. ZONA IZQUIERDA: Insignia de Civilización & Oro Imperial
            let crest_rect = RECT { left: 10, top: 6, right: 42, bottom: height - 6 };
            let crest_brush = CreateSolidBrush(COLORREF(0x00059669));
            FillRect(hdc, &crest_rect, crest_brush);
            let _ = DeleteObject(crest_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut cr_t = RECT { left: 10, top: 12, right: 42, bottom: height - 6 };
            draw_ui_text(hdc, "🗡️", &mut cr_t, DT_CENTER | DT_SINGLELINE);

            // Oro en cápsula dorada
            let gold_pod = RECT { left: 50, top: 8, right: 170, bottom: height - 8 };
            let g_brush = CreateSolidBrush(COLORREF(0x0017191C));
            FillRect(hdc, &gold_pod, g_brush);
            let _ = DeleteObject(g_brush);

            SelectObject(hdc, font_res);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let gold_str = format!("🪙 {:.0}", state.gold);
            let mut gt = RECT { left: 50, top: 12, right: 170, bottom: height - 8 };
            draw_ui_text(hdc, &gold_str, &mut gt, DT_CENTER | DT_SINGLELINE);

            // 2. ZONA CENTRAL: Micro-Diorama Animado de 32 Bits & Recursos
            let res_pod = RECT { left: 180, top: 8, right: 400, bottom: height - 8 };
            let r_brush = CreateSolidBrush(COLORREF(0x001E293B));
            FillRect(hdc, &res_pod, r_brush);
            let _ = DeleteObject(r_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let res_str = format!("🪵{:.0} ⛏️{:.0} 🌾{:.0}", state.materials, state.materials_rate * 50.0, state.food);
            let mut rt = RECT { left: 185, top: 13, right: 395, bottom: height - 8 };
            draw_ui_text(hdc, &res_str, &mut rt, DT_CENTER | DT_SINGLELINE);

            // Micro-regimiento marchando en vivo en la barra
            let march_x = 420 + ((self.animation_frame as i32 * 4) % (width.saturating_sub(700).max(100)));
            let march_y = 12;

            SetTextColor(hdc, COLORREF(0x0022C55E));
            let march_sprite = "🚩 💂‍♂️💂‍♂️💂‍♂️ [1.ª Legión en Marcha]";
            let mut mt = RECT { left: march_x, top: march_y, right: march_x + 240, bottom: march_y + 24 };
            draw_ui_text(hdc, march_sprite, &mut mt, DT_LEFT | DT_SINGLELINE);

            // 3. ZONA DERECHA: Botones de Acción Rápida RTS
            let btn_attack = RECT { left: width - 260, top: 8, right: width - 180, bottom: height - 8 };
            let ba_brush = CreateSolidBrush(COLORREF(0x00DC2626));
            FillRect(hdc, &btn_attack, ba_brush);
            let _ = DeleteObject(ba_brush);

            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut bat = RECT { left: width - 260, top: 13, right: width - 180, bottom: height - 8 };
            draw_ui_text(hdc, "⚔️ Marcha", &mut bat, DT_CENTER | DT_SINGLELINE);

            let btn_speed = RECT { left: width - 170, top: 8, right: width - 100, bottom: height - 8 };
            let bs_brush = CreateSolidBrush(COLORREF(0x00059669));
            FillRect(hdc, &btn_speed, bs_brush);
            let _ = DeleteObject(bs_brush);

            let mut bst = RECT { left: width - 170, top: 13, right: width - 100, bottom: height - 8 };
            draw_ui_text(hdc, "⚡ Blitz", &mut bst, DT_CENTER | DT_SINGLELINE);

            let btn_max = RECT { left: width - 90, top: 8, right: width - 10, bottom: height - 8 };
            let bm_brush = CreateSolidBrush(COLORREF(0x000284C7));
            FillRect(hdc, &btn_max, bm_brush);
            let _ = DeleteObject(bm_brush);

            let mut bmt = RECT { left: width - 90, top: 13, right: width - 10, bottom: height - 8 };
            draw_ui_text(hdc, "⛶ F11", &mut bmt, DT_CENTER | DT_SINGLELINE);

            let _ = DeleteObject(font_bold);
            let _ = DeleteObject(font_res);
        }
    }
}
