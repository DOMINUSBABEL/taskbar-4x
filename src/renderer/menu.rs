use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use crate::engine::setup::{CivilizationChoice, LeaderTrait, GameSpeed, GameConfig};
use super::draw_ui_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuScreen {
    Main,
    Setup,
    AscensionTree,
    Settings,
}

pub struct MenuRenderer {
    pub current_screen: MenuScreen,
    pub config: GameConfig,
}

impl MenuRenderer {
    pub fn new() -> Self {
        Self {
            current_screen: MenuScreen::Main,
            config: GameConfig::default(),
        }
    }

    pub unsafe fn render(&self, hdc: HDC, rect: &RECT) {
        unsafe {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // Fondo Mármol / Pizarra Imperial Oscura
            let bg_brush = CreateSolidBrush(COLORREF(0x000A0F1D)); // Pizarra noche profunda
            FillRect(hdc, rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            SetBkMode(hdc, TRANSPARENT);

            // Borde ornamental dorado estilo Imperivm / Cossacks
            let gold_pen = CreatePen(PS_SOLID, 3, COLORREF(0x001B9EF5)); // Oro imperial
            let old_pen = SelectObject(hdc, gold_pen);
            let _ = MoveToEx(hdc, 16, 16, None);
            let _ = LineTo(hdc, width - 16, 16);
            let _ = LineTo(hdc, width - 16, height - 16);
            let _ = LineTo(hdc, 16, height - 16);
            let _ = LineTo(hdc, 16, 16);
            SelectObject(hdc, old_pen);
            let _ = DeleteObject(gold_pen);

            let font_epic = CreateFontW(
                32, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Outfit"),
            );
            let font_sub = CreateFontW(
                14, 0, 0, 0, FW_NORMAL.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );
            let font_btn = CreateFontW(
                16, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Outfit"),
            );
            let font_body = CreateFontW(
                12, 0, 0, 0, FW_NORMAL.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );
            let font_bold = CreateFontW(
                12, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );

            match self.current_screen {
                MenuScreen::Main => {
                    self.render_main_menu(hdc, width, height, font_epic, font_sub, font_btn);
                }
                MenuScreen::Setup => {
                    self.render_setup_menu(hdc, width, height, font_epic, font_sub, font_btn, font_body, font_bold);
                }
                MenuScreen::AscensionTree => {
                    self.render_ascension_menu(hdc, width, height, font_epic, font_sub, font_btn, font_body);
                }
                MenuScreen::Settings => {
                    self.render_settings_menu(hdc, width, height, font_epic, font_sub, font_btn, font_body);
                }
            }

            let _ = DeleteObject(font_epic);
            let _ = DeleteObject(font_sub);
            let _ = DeleteObject(font_btn);
            let _ = DeleteObject(font_body);
            let _ = DeleteObject(font_bold);
        }
    }

    unsafe fn render_main_menu(&self, hdc: HDC, width: i32, height: i32, font_epic: HFONT, font_sub: HFONT, font_btn: HFONT) {
        unsafe {
            // Título Monumental
            SelectObject(hdc, font_epic);
            SetTextColor(hdc, COLORREF(0x000B9EF5)); // Oro
            let mut title_rect = RECT { left: 0, top: (height / 6), right: width, bottom: (height / 6) + 50 };
            draw_ui_text(hdc, "🏛️  T A S K  B A R  4 X  🏛️", &mut title_rect, DT_CENTER | DT_SINGLELINE);

            SelectObject(hdc, font_sub);
            SetTextColor(hdc, COLORREF(0x00E2E8F0)); // Blanco pergamino
            let mut sub_rect = RECT { left: 0, top: (height / 6) + 55, right: width, bottom: (height / 6) + 85 };
            draw_ui_text(hdc, "La Gran Odisea Histórica a través de 15 Edades de la Civilización Humana (4X + Idle Híbrido)", &mut sub_rect, DT_CENTER | DT_SINGLELINE);

            // Botones Principales
            let btn_width = 440;
            let btn_height = 48;
            let start_x = (width - btn_width) / 2;
            let mut start_y = (height / 6) + 130;

            let buttons = [
                ("⚔️ NUEVA PARTIDA / FUNDAR IMPERIO", COLORREF(0x000284C7), COLORREF(0x00FFFFFF)),
                ("⏳ CONTINUAR CAMPAÑA HISTÓRICA", COLORREF(0x001E293B), COLORREF(0x0094A3B8)),
                ("🌌 ÁRBOL DE ASCENSIÓN & SINGULARIDAD", COLORREF(0x00311042), COLORREF(0x00E879F9)),
                ("⚙️ OPCIONES & CONFIGURACIÓN DE BARRA", COLORREF(0x001E293B), COLORREF(0x00CBD5E1)),
                ("🚪 SALIR AL ESCRITORIO", COLORREF(0x0017191C), COLORREF(0x00F87171)),
            ];

            SelectObject(hdc, font_btn);

            for (label, bg_c, text_c) in buttons {
                let btn_rect = RECT { left: start_x, top: start_y, right: start_x + btn_width, bottom: start_y + btn_height };
                let brush = CreateSolidBrush(bg_c);
                FillRect(hdc, &btn_rect, brush);
                let _ = DeleteObject(brush);

                // Marco fino del botón
                let border_p = CreatePen(PS_SOLID, 1, COLORREF(0x00FBBF24));
                let old_p = SelectObject(hdc, border_p);
                let _ = MoveToEx(hdc, start_x, start_y, None);
                let _ = LineTo(hdc, start_x + btn_width, start_y);
                let _ = LineTo(hdc, start_x + btn_width, start_y + btn_height);
                let _ = LineTo(hdc, start_x, start_y + btn_height);
                let _ = LineTo(hdc, start_x, start_y);
                SelectObject(hdc, old_p);
                let _ = DeleteObject(border_p);

                SetTextColor(hdc, text_c);
                let mut text_rect = RECT { left: start_x, top: start_y + 12, right: start_x + btn_width, bottom: start_y + btn_height };
                draw_ui_text(hdc, label, &mut text_rect, DT_CENTER | DT_SINGLELINE);

                start_y += btn_height + 16;
            }

            // Pie de página de referencias históricas
            SelectObject(hdc, font_sub);
            SetTextColor(hdc, COLORREF(0x0064748B));
            let mut foot_rect = RECT { left: 0, top: height - 60, right: width, bottom: height - 20 };
            draw_ui_text(hdc, "Inspirado en Cossacks 3, Imperivm, Sid Meier's Civilization y Empire Earth | V0.1.0 Alpha", &mut foot_rect, DT_CENTER | DT_SINGLELINE);
        }
    }

    unsafe fn render_setup_menu(&self, hdc: HDC, width: i32, height: i32, font_epic: HFONT, font_sub: HFONT, font_btn: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            // Título de Configuración
            SelectObject(hdc, font_epic);
            SetTextColor(hdc, COLORREF(0x000B9EF5));
            let mut t_rect = RECT { left: 40, top: 30, right: width - 40, bottom: 70 };
            draw_ui_text(hdc, "🏛️  CONFIGURACIÓN DEL IMPERIO HISTÓRICO", &mut t_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_sub);
            SetTextColor(hdc, COLORREF(0x0094A3B8));
            let mut s_rect = RECT { left: 40, top: 72, right: width - 40, bottom: 95 };
            draw_ui_text(hdc, "Selecciona tu Cultura de Origen, Arquetipo de Líder y Ritmo de Simulación", &mut s_rect, DT_LEFT | DT_SINGLELINE);

            // 1. SELECCIÓN DE CIVILIZACIÓN (6 tarjetas)
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut c_hdr = RECT { left: 40, top: 105, right: width - 40, bottom: 125 };
            draw_ui_text(hdc, "1. CULTURA & CIVILIZACIÓN INICIAL:", &mut c_hdr, DT_LEFT | DT_SINGLELINE);

            let card_w = (width - 120) / 3;
            let card_h = 75;

            for (i, civ) in CivilizationChoice::ALL.iter().enumerate() {
                let col = (i % 3) as i32;
                let row = (i / 3) as i32;
                let cx = 40 + col * (card_w + 20);
                let cy = 135 + row * (card_h + 12);

                let is_selected = self.config.civ == *civ;
                let card_rect = RECT { left: cx, top: cy, right: cx + card_w, bottom: cy + card_h };
                let bg_color = if is_selected { COLORREF(0x000284C7) } else { COLORREF(0x001E293B) };
                let brush = CreateSolidBrush(bg_color);
                FillRect(hdc, &card_rect, brush);
                let _ = DeleteObject(brush);

                if is_selected {
                    let p = CreatePen(PS_SOLID, 2, COLORREF(0x00FBBF24));
                    let old_p = SelectObject(hdc, p);
                    let _ = MoveToEx(hdc, cx, cy, None);
                    let _ = LineTo(hdc, cx + card_w, cy);
                    let _ = LineTo(hdc, cx + card_w, cy + card_h);
                    let _ = LineTo(hdc, cx, cy + card_h);
                    let _ = LineTo(hdc, cx, cy);
                    SelectObject(hdc, old_p);
                    let _ = DeleteObject(p);
                }

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, if is_selected { COLORREF(0x00FFFFFF) } else { COLORREF(0x0038BDF8) });
                let mut ct_rect = RECT { left: cx + 10, top: cy + 8, right: cx + card_w - 10, bottom: cy + 28 };
                draw_ui_text(hdc, civ.name(), &mut ct_rect, DT_LEFT | DT_SINGLELINE);

                SelectObject(hdc, font_body);
                SetTextColor(hdc, COLORREF(0x00E2E8F0));
                let mut cb_rect = RECT { left: cx + 10, top: cy + 30, right: cx + card_w - 10, bottom: cy + card_h - 4 };
                draw_ui_text(hdc, civ.bonus_desc(), &mut cb_rect, DT_LEFT | DT_WORDBREAK);
            }

            // 2. SELECCIÓN DE LÍDER (6 tarjetas)
            let leader_y = 320;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut l_hdr = RECT { left: 40, top: leader_y, right: width - 40, bottom: leader_y + 20 };
            draw_ui_text(hdc, "2. ARQUETIPO DE LÍDER & RASGO IMPERIAL:", &mut l_hdr, DT_LEFT | DT_SINGLELINE);

            for (i, leader) in LeaderTrait::ALL.iter().enumerate() {
                let col = (i % 3) as i32;
                let row = (i / 3) as i32;
                let lx = 40 + col * (card_w + 20);
                let ly = leader_y + 30 + row * (card_h + 12);

                let is_selected = self.config.leader == *leader;
                let card_rect = RECT { left: lx, top: ly, right: lx + card_w, bottom: ly + card_h };
                let bg_color = if is_selected { COLORREF(0x009333EA) } else { COLORREF(0x001E293B) };
                let brush = CreateSolidBrush(bg_color);
                FillRect(hdc, &card_rect, brush);
                let _ = DeleteObject(brush);

                if is_selected {
                    let p = CreatePen(PS_SOLID, 2, COLORREF(0x00FBBF24));
                    let old_p = SelectObject(hdc, p);
                    let _ = MoveToEx(hdc, lx, ly, None);
                    let _ = LineTo(hdc, lx + card_w, ly);
                    let _ = LineTo(hdc, lx + card_w, ly + card_h);
                    let _ = LineTo(hdc, lx, ly + card_h);
                    let _ = LineTo(hdc, lx, ly);
                    SelectObject(hdc, old_p);
                    let _ = DeleteObject(p);
                }

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, if is_selected { COLORREF(0x00FFFFFF) } else { COLORREF(0x00C084FC) });
                let mut lt_rect = RECT { left: lx + 10, top: ly + 8, right: lx + card_w - 10, bottom: ly + 28 };
                draw_ui_text(hdc, leader.name(), &mut lt_rect, DT_LEFT | DT_SINGLELINE);

                SelectObject(hdc, font_body);
                SetTextColor(hdc, COLORREF(0x00E2E8F0));
                let mut lb_rect = RECT { left: lx + 10, top: ly + 30, right: lx + card_w - 10, bottom: ly + card_h - 4 };
                draw_ui_text(hdc, leader.bonus_desc(), &mut lb_rect, DT_LEFT | DT_WORDBREAK);
            }

            // 3. SELECCIÓN DE RITMO DE JUEGO (3 opciones)
            let speed_y = 510;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut sp_hdr = RECT { left: 40, top: speed_y, right: width - 40, bottom: speed_y + 20 };
            draw_ui_text(hdc, "3. RITMO DE SIMULACIÓN & VELOCIDAD IDLE:", &mut sp_hdr, DT_LEFT | DT_SINGLELINE);

            let speeds = [GameSpeed::Blitz, GameSpeed::Normal, GameSpeed::Epic];
            for (i, sp) in speeds.iter().enumerate() {
                let sx = 40 + (i as i32) * (card_w + 20);
                let sy = speed_y + 30;

                let is_selected = self.config.speed == *sp;
                let s_rect = RECT { left: sx, top: sy, right: sx + card_w, bottom: sy + 44 };
                let bg_color = if is_selected { COLORREF(0x00059669) } else { COLORREF(0x001E293B) };
                let brush = CreateSolidBrush(bg_color);
                FillRect(hdc, &s_rect, brush);
                let _ = DeleteObject(brush);

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x00FFFFFF));
                let mut st_rect = RECT { left: sx, top: sy + 12, right: sx + card_w, bottom: sy + 40 };
                draw_ui_text(hdc, sp.name(), &mut st_rect, DT_CENTER | DT_SINGLELINE);
            }

            // Botones Inferiores de Acción
            let btn_start_rect = RECT { left: width - 450, top: height - 90, right: width - 40, bottom: height - 35 };
            let start_brush = CreateSolidBrush(COLORREF(0x000284C7));
            FillRect(hdc, &btn_start_rect, start_brush);
            let _ = DeleteObject(start_brush);

            SelectObject(hdc, font_btn);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut st_lbl = RECT { left: width - 450, top: height - 76, right: width - 40, bottom: height - 40 };
            draw_ui_text(hdc, "🏛️ FUNDAR IMPERIO E INICIAR", &mut st_lbl, DT_CENTER | DT_SINGLELINE);

            let btn_back_rect = RECT { left: 40, top: height - 90, right: 280, bottom: height - 35 };
            let back_brush = CreateSolidBrush(COLORREF(0x00334155));
            FillRect(hdc, &btn_back_rect, back_brush);
            let _ = DeleteObject(back_brush);

            SetTextColor(hdc, COLORREF(0x00CBD5E1));
            let mut bk_lbl = RECT { left: 40, top: height - 76, right: 280, bottom: height - 40 };
            draw_ui_text(hdc, "◀ VOLVER AL MENÚ", &mut bk_lbl, DT_CENTER | DT_SINGLELINE);
        }
    }

    unsafe fn render_ascension_menu(&self, hdc: HDC, width: i32, height: i32, font_epic: HFONT, font_sub: HFONT, font_btn: HFONT, font_body: HFONT) {
        unsafe {
            SelectObject(hdc, font_epic);
            SetTextColor(hdc, COLORREF(0x00E879F9)); // Púrpura cósmico
            let mut t_rect = RECT { left: 40, top: 40, right: width - 40, bottom: 80 };
            draw_ui_text(hdc, "🌌  ÁRBOL DE ASCENSIÓN & POLVO DE SINGULARIDAD", &mut t_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_sub);
            SetTextColor(hdc, COLORREF(0x0094A3B8));
            let mut s_rect = RECT { left: 40, top: 85, right: width - 40, bottom: 110 };
            draw_ui_text(hdc, "Meta-progresión permanente: Desbloquea leyes universales y artefactos atemporales", &mut s_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00E2E8F0));
            let desc = "Al completar las 15 Edades y alcanzar la Singularidad Cuántica, la civilización trasciende y genera Polvo de Singularidad. Con este recurso obtienes:\n\n • 🌟 Memoria Genética Ancestral (+15% Tasa de recursos permanente por ciclo)\n • ⚡ Impulso de Fusión Primordial (Inicio directo en Edad del Cobre)\n • 🏛️ Arquetipo Arquitectónico Divino (-20% Coste de Maravillas en todas las eras)\n • 🔮 Visión Astrológica Perenne (+50% Puntos de Ciencia)";
            let mut d_rect = RECT { left: 40, top: 140, right: width - 80, bottom: 350 };
            draw_ui_text(hdc, desc, &mut d_rect, DT_LEFT | DT_WORDBREAK);

            let btn_back_rect = RECT { left: 40, top: height - 90, right: 280, bottom: height - 35 };
            let back_brush = CreateSolidBrush(COLORREF(0x00334155));
            FillRect(hdc, &btn_back_rect, back_brush);
            let _ = DeleteObject(back_brush);

            SelectObject(hdc, font_btn);
            SetTextColor(hdc, COLORREF(0x00CBD5E1));
            let mut bk_lbl = RECT { left: 40, top: height - 76, right: 280, bottom: height - 40 };
            draw_ui_text(hdc, "◀ VOLVER AL MENÚ", &mut bk_lbl, DT_CENTER | DT_SINGLELINE);
        }
    }

    unsafe fn render_settings_menu(&self, hdc: HDC, width: i32, height: i32, font_epic: HFONT, font_sub: HFONT, font_btn: HFONT, font_body: HFONT) {
        unsafe {
            SelectObject(hdc, font_epic);
            SetTextColor(hdc, COLORREF(0x0038BDF8));
            let mut t_rect = RECT { left: 40, top: 40, right: width - 40, bottom: 80 };
            draw_ui_text(hdc, "⚙️  OPCIONES & CONFIGURACIÓN DE BARRA", &mut t_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_sub);
            SetTextColor(hdc, COLORREF(0x0094A3B8));
            let mut s_rect = RECT { left: 40, top: 85, right: width - 40, bottom: 110 };
            draw_ui_text(hdc, "Configura el comportamiento del Widget en la Barra de Tareas y Atajos Globales", &mut s_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00E2E8F0));
            let desc = " • Altura de Barra Docked: 48 píxeles (Optimizada para ergonomía)\n • Atajo Global de Alternancia: Win + Alt + X o F11\n • Gestión de Passthrough: Habilitado con WM_NCHITTEST inteligente\n • Modo Widget Flotante: Tirador de arrastre en esquina izquierda\n • Tasa de Refresco: 60 FPS fijos en diorama con doble búfer sin parpadeo\n • Consumo en Reposo: 0.0% CPU garantizado con WaitMessage() reactivo";
            let mut d_rect = RECT { left: 40, top: 140, right: width - 80, bottom: 350 };
            draw_ui_text(hdc, desc, &mut d_rect, DT_LEFT | DT_WORDBREAK);

            let btn_back_rect = RECT { left: 40, top: height - 90, right: 280, bottom: height - 35 };
            let back_brush = CreateSolidBrush(COLORREF(0x00334155));
            FillRect(hdc, &btn_back_rect, back_brush);
            let _ = DeleteObject(back_brush);

            SelectObject(hdc, font_btn);
            SetTextColor(hdc, COLORREF(0x00CBD5E1));
            let mut bk_lbl = RECT { left: 40, top: height - 76, right: 280, bottom: height - 40 };
            draw_ui_text(hdc, "◀ VOLVER AL MENÚ", &mut bk_lbl, DT_CENTER | DT_SINGLELINE);
        }
    }
}
