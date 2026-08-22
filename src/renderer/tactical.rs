use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use crate::engine::state::GameState;
use crate::engine::buildings::{BuildingType, get_building_definition};
use crate::engine::military::get_unit_definition;
use super::{draw_ui_text, blit_buffer_to_hdc, IsoWorldRenderer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalTab {
    CampaignMap = 0,
    CityManager = 1,
    TechTree = 2,
    MilitaryCabinet = 3,
    WondersAndLog = 4,
}

pub struct TacticalRenderer {
    pub active_tab: TacticalTab,
    pub iso_world: IsoWorldRenderer,
}

impl TacticalRenderer {
    pub fn new() -> Self {
        Self {
            active_tab: TacticalTab::CampaignMap,
            iso_world: IsoWorldRenderer::new(1280, 720),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.iso_world.update(dt);
    }

    pub unsafe fn render(&mut self, hdc: HDC, rect: &RECT, state: &GameState) {
        unsafe {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // Fondo madera/granite oscuro
            let bg_brush = CreateSolidBrush(COLORREF(0x000F172A));
            FillRect(hdc, rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            SetBkMode(hdc, TRANSPARENT);

            // 1. HEADER SUPERIOR IMPERIAL (48px) - Madera noble con marco de oro
            let header_rect = RECT { left: 0, top: 0, right: width, bottom: 48 };
            let header_brush = CreateSolidBrush(COLORREF(0x0017191C));
            FillRect(hdc, &header_rect, header_brush);
            let _ = DeleteObject(header_brush);

            let border_gold = CreatePen(PS_SOLID, 2, COLORREF(0x000B9EF5));
            let old_p = SelectObject(hdc, border_gold);
            let _ = MoveToEx(hdc, 0, 48, None);
            let _ = LineTo(hdc, width, 48);
            SelectObject(hdc, old_p);
            let _ = DeleteObject(border_gold);

            let font_title = CreateFontW(
                16, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Outfit"),
            );
            let font_body = CreateFontW(
                11, 0, 0, 0, FW_NORMAL.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );
            let font_bold = CreateFontW(
                11, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );

            let old_font = SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF)); // Cian/Oro neón

            let title_text = format!("🏛️ {} | {}", state.config.civ.name(), state.current_era.name());
            let mut t_rect = RECT { left: 16, top: 12, right: 400, bottom: 38 };
            draw_ui_text(hdc, &title_text, &mut t_rect, DT_LEFT | DT_SINGLELINE);

            // Rendimiento de recursos detallado (Civilization style)
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));

            let res_line = format!(
                "🌾 {:.0} (+{:.1}/s) | 🪵 {:.0} (+{:.1}/s) | 🪙 {:.0} (+{:.1}/s) | 🕯️ {:.0} | 📜 {:.0} | 🔬 {:.0} | ⚔️ {:.0}",
                state.food, state.food_rate, state.materials, state.materials_rate, state.gold, state.gold_rate, state.faith, state.philosophy, state.science, state.military_power
            );
            let mut r_rect = RECT { left: 410, top: 14, right: width - 260, bottom: 36 };
            draw_ui_text(hdc, &res_line, &mut r_rect, DT_LEFT | DT_SINGLELINE);

            // Botón de Minimizar a Barra de Tareas (F11 / Esc)
            let exit_rect = RECT { left: width - 240, top: 8, right: width - 16, bottom: 40 };
            let exit_brush = CreateSolidBrush(COLORREF(0x000284C7));
            FillRect(hdc, &exit_rect, exit_brush);
            let _ = DeleteObject(exit_brush);

            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            SelectObject(hdc, font_bold);
            let mut ex_rect = RECT { left: width - 240, top: 14, right: width - 16, bottom: 36 };
            draw_ui_text(hdc, "🔽 MINIMIZAR A BARRA (F11)", &mut ex_rect, DT_CENTER | DT_SINGLELINE);

            // 2. BARRA DE PESTAÑAS DE NAVEGACIÓN (36px)
            let tab_bar_rect = RECT { left: 0, top: 48, right: width, bottom: 84 };
            let tab_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &tab_bar_rect, tab_brush);
            let _ = DeleteObject(tab_brush);

            let tabs = [
                (TacticalTab::CampaignMap, "🗺️ 1. Mapa Isométrico 32-Bit & Asentamiento"),
                (TacticalTab::CityManager, "🏙️ 2. Gestión Urbana & Edificios"),
                (TacticalTab::TechTree, "📜 3. Árbol de 8 Ramas de Desarrollo"),
                (TacticalTab::MilitaryCabinet, "⚔️ 4. Gabinete de Guerra & Regimientos"),
                (TacticalTab::WondersAndLog, "🏛️ 5. Maravillas & Crónica Imperial"),
            ];

            let tab_width = width / tabs.len() as i32;
            for (i, (tab_type, tab_name)) in tabs.iter().enumerate() {
                let t_x = (i as i32) * tab_width;
                let current_tab_rect = RECT { left: t_x, top: 48, right: t_x + tab_width, bottom: 84 };

                if *tab_type == self.active_tab {
                    let active_brush = CreateSolidBrush(COLORREF(0x001E293B));
                    FillRect(hdc, &current_tab_rect, active_brush);
                    let _ = DeleteObject(active_brush);

                    let bar_pen = CreatePen(PS_SOLID, 3, COLORREF(0x000B9EF5));
                    let op = SelectObject(hdc, bar_pen);
                    let _ = MoveToEx(hdc, t_x, 83, None);
                    let _ = LineTo(hdc, t_x + tab_width, 83);
                    SelectObject(hdc, op);
                    let _ = DeleteObject(bar_pen);

                    SetTextColor(hdc, COLORREF(0x00FBBF24));
                    SelectObject(hdc, font_bold);
                } else {
                    SetTextColor(hdc, COLORREF(0x0094A3B8));
                    SelectObject(hdc, font_body);
                }

                let mut text_rect = RECT { left: t_x, top: 56, right: t_x + tab_width, bottom: 80 };
                draw_ui_text(hdc, tab_name, &mut text_rect, DT_CENTER | DT_SINGLELINE);
            }

            // 3. CONTENIDO DE LA PESTAÑA ACTIVA
            let content_rect = RECT { left: 20, top: 96, right: width - 20, bottom: height - 20 };

            match self.active_tab {
                TacticalTab::CampaignMap => {
                    self.render_campaign_map(hdc, &content_rect, state, font_title, font_body, font_bold);
                }
                TacticalTab::CityManager => {
                    self.render_city_manager(hdc, &content_rect, state, font_title, font_body, font_bold);
                }
                TacticalTab::TechTree => {
                    self.render_tech_tree(hdc, &content_rect, state, font_title, font_body, font_bold);
                }
                TacticalTab::MilitaryCabinet => {
                    self.render_military_cabinet(hdc, &content_rect, state, font_title, font_body, font_bold);
                }
                TacticalTab::WondersAndLog => {
                    self.render_wonders_and_log(hdc, &content_rect, state, font_title, font_body, font_bold);
                }
            }

            SelectObject(hdc, old_font);
            let _ = DeleteObject(font_title);
            let _ = DeleteObject(font_body);
            let _ = DeleteObject(font_bold);
        }
    }

    unsafe fn render_campaign_map(&mut self, hdc: HDC, rect: &RECT, state: &GameState, font_title: HFONT, _font_body: HFONT, font_bold: HFONT) {
        unsafe {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // Lienzo Isométrico 32-Bit (72% del ancho de pantalla)
            let iso_view_w = ((width * 72) / 100) as usize;
            let iso_view_h = height as usize;

            self.iso_world.resize(iso_view_w, iso_view_h);
            self.iso_world.render_world(state);

            // Transferir el búfer isométrico 32-bit a la pantalla en (rect.left, rect.top)
            blit_buffer_to_hdc(hdc, rect.left, rect.top, &self.iso_world.backbuffer);

            // Marco ornamental de madera y oro alrededor del visor isométrico
            let frame_pen = CreatePen(PS_SOLID, 3, COLORREF(0x001B9EF5)); // Oro
            let op = SelectObject(hdc, frame_pen);
            let _ = MoveToEx(hdc, rect.left, rect.top, None);
            let _ = LineTo(hdc, rect.left + iso_view_w as i32, rect.top);
            let _ = LineTo(hdc, rect.left + iso_view_w as i32, rect.top + iso_view_h as i32);
            let _ = LineTo(hdc, rect.left, rect.top + iso_view_h as i32);
            let _ = LineTo(hdc, rect.left, rect.top);
            SelectObject(hdc, op);
            let _ = DeleteObject(frame_pen);

            // Mini-Placa de Localización en el Visor
            let badge_rect = RECT { left: rect.left + 16, top: rect.top + 16, right: rect.left + 320, bottom: rect.top + 55 };
            let b_brush = CreateSolidBrush(COLORREF(0x000A0F1D));
            FillRect(hdc, &badge_rect, b_brush);
            let _ = DeleteObject(b_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut bl_rect = RECT { left: rect.left + 24, top: rect.top + 22, right: rect.left + 310, bottom: rect.top + 48 };
            let loc_text = format!("📍 {} | Asentamiento Principal", state.cities[0].name);
            draw_ui_text(hdc, &loc_text, &mut bl_rect, DT_LEFT | DT_SINGLELINE);

            // Panel Contextual Derecho (Dossier Imperial & Órdenes)
            let panel_x = rect.left + iso_view_w as i32 + 15;
            let _panel_w = width - iso_view_w as i32 - 15;
            let panel_rect = RECT { left: panel_x, top: rect.top, right: rect.right, bottom: rect.bottom };
            let panel_brush = CreateSolidBrush(COLORREF(0x001E293B));
            FillRect(hdc, &panel_rect, panel_brush);
            let _ = DeleteObject(panel_brush);

            // Marco del panel
            let p_border = CreatePen(PS_SOLID, 1, COLORREF(0x00475569));
            let old_pb = SelectObject(hdc, p_border);
            let _ = MoveToEx(hdc, panel_x, rect.top, None);
            let _ = LineTo(hdc, rect.right, rect.top);
            let _ = LineTo(hdc, rect.right, rect.bottom);
            let _ = LineTo(hdc, panel_x, rect.bottom);
            let _ = LineTo(hdc, panel_x, rect.top);
            SelectObject(hdc, old_pb);
            let _ = DeleteObject(p_border);

            // Título de Provincia
            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let p_name = format!("🦅 Provincia: {}", state.provinces[state.selected_province].name);
            let mut pn_rect = RECT { left: panel_x + 16, top: rect.top + 16, right: rect.right - 16, bottom: rect.top + 42 };
            draw_ui_text(hdc, &p_name, &mut pn_rect, DT_LEFT | DT_SINGLELINE);

            // Información y Datos de Campaña
            let prov = &state.provinces[state.selected_province];
            let desc = format!(
                "Bioma: {}\nEstado: {}\nNivel de Asentamiento: {}\nGuarnición: {} hombres\n\n═══════════════════════\nORDEN DE BATALLA (Cossacks Style):\n • Regimientos Activos: {} divisiones\n • Formación: Rectangular de 36 mosqueteros\n • Estandarte: Águila Imperial\n\n[Haz clic para ordenar maniobras o construir distritos]",
                prov.biome.name(),
                if prov.is_colonized { "Territorio Imperial Colonizado" } else if prov.is_hostile { "Feudo Hostil Rebelde" } else { "Tierra Virgen Inexplorada" },
                prov.development_level,
                prov.garrison_strength,
                state.armies.len()
            );

            SelectObject(hdc, _font_body);
            SetTextColor(hdc, COLORREF(0x00E2E8F0));
            let mut desc_rect = RECT { left: panel_x + 16, top: rect.top + 52, right: rect.right - 16, bottom: rect.top + 320 };
            draw_ui_text(hdc, &desc, &mut desc_rect, DT_LEFT | DT_WORDBREAK);

            // Botones de acción rápida en el panel
            let actions = [
                "⚔️ Reclutar Regimiento de Línea (36 hombres)",
                "🌾 Expandir Campos de Trigo (+5.0 Comida/s)",
                "⛏️ Abrir Cantera de Piedra (+4.0 Mat/s)",
                "🏛️ Construir Monumento Cívico (+10 Fe/s)",
            ];

            let mut act_y = rect.top + 330;
            for act in actions {
                let act_rect = RECT { left: panel_x + 16, top: act_y, right: rect.right - 16, bottom: act_y + 32 };
                let act_brush = CreateSolidBrush(COLORREF(0x00334155));
                FillRect(hdc, &act_rect, act_brush);
                let _ = DeleteObject(act_brush);

                SetTextColor(hdc, COLORREF(0x00FFFFFF));
                SelectObject(hdc, font_bold);
                let mut at_rect = RECT { left: panel_x + 24, top: act_y + 8, right: rect.right - 20, bottom: act_y + 28 };
                draw_ui_text(hdc, act, &mut at_rect, DT_LEFT | DT_SINGLELINE);

                act_y += 40;
            }
        }
    }

    unsafe fn render_city_manager(&self, hdc: HDC, rect: &RECT, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut t_rect = RECT { left: rect.left, top: rect.top, right: rect.right, bottom: rect.top + 30 };
            draw_ui_text(hdc, "🏙️ Gestión Urbana y Edificios de la Civilización", &mut t_rect, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));

            if let Some(city) = state.cities.get(state.selected_city) {
                let c_info = format!("Ciudad: {} (Población: {} habitantes)", city.name, city.population);
                let mut ci_rect = RECT { left: rect.left, top: rect.top + 35, right: rect.right, bottom: rect.top + 60 };
                draw_ui_text(hdc, &c_info, &mut ci_rect, DT_LEFT | DT_SINGLELINE);

                let mut y_off = rect.top + 70;
                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x00FBBF24));
                let mut bl_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 20 };
                draw_ui_text(hdc, "Edificios Construidos en la Ciudad:", &mut bl_rect, DT_LEFT | DT_SINGLELINE);

                y_off += 25;
                SelectObject(hdc, font_body);
                SetTextColor(hdc, COLORREF(0x00E2E8F0));
                for b in &city.buildings {
                    let def = get_building_definition(*b);
                    let b_str = format!(" • {} — {}", def.name, def.production_bonus_desc);
                    let mut bs_rect = RECT { left: rect.left + 15, top: y_off, right: rect.right, bottom: y_off + 20 };
                    draw_ui_text(hdc, &b_str, &mut bs_rect, DT_LEFT | DT_SINGLELINE);
                    y_off += 22;
                }

                y_off += 15;
                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x0038BDF8));
                let mut al_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 20 };
                draw_ui_text(hdc, "Edificios Disponibles para Construir (Haz clic para iniciar obra):", &mut al_rect, DT_LEFT | DT_SINGLELINE);

                y_off += 25;
                let available_buildings = [
                    BuildingType::Hearth,
                    BuildingType::GrainPit,
                    BuildingType::StoneQuarry,
                    BuildingType::ShamanHut,
                    BuildingType::MegalithCircle,
                    BuildingType::BronzeForge,
                    BuildingType::Forum,
                    BuildingType::Watermill,
                ];

                SelectObject(hdc, font_body);
                for b_type in available_buildings {
                    let def = get_building_definition(b_type);
                    let b_btn_rect = RECT { left: rect.left + 15, top: y_off, right: rect.left + 550, bottom: y_off + 26 };
                    let btn_brush = CreateSolidBrush(COLORREF(0x00334155));
                    FillRect(hdc, &b_btn_rect, btn_brush);
                    let _ = DeleteObject(btn_brush);

                    SetTextColor(hdc, COLORREF(0x00FFFFFF));
                    let opt_str = format!(" 🔨 Construir {} [🪵{} Mat, 🌾{} Comida] — {}", def.name, def.material_cost, def.food_cost, def.production_bonus_desc);
                    let mut opt_rect = RECT { left: rect.left + 18, top: y_off + 4, right: rect.left + 545, bottom: y_off + 24 };
                    draw_ui_text(hdc, &opt_str, &mut opt_rect, DT_LEFT | DT_SINGLELINE);

                    y_off += 30;
                }
            }
        }
    }

    unsafe fn render_tech_tree(&self, hdc: HDC, rect: &RECT, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let title = format!("📜 Árbol de 8 Ramas de Desarrollo — {}", state.current_era.name());
            let mut t_rect = RECT { left: rect.left, top: rect.top, right: rect.right, bottom: rect.top + 30 };
            draw_ui_text(hdc, &title, &mut t_rect, DT_LEFT | DT_SINGLELINE);

            let y_off = rect.top + 40;
            let col_width = (rect.right - rect.left - 20) / 2;

            for (i, tech) in state.era_technologies.iter().enumerate() {
                let col = i % 2;
                let row = i / 2;
                let card_x = rect.left + (col as i32) * (col_width + 15);
                let card_y = y_off + (row as i32) * 95;

                let card_rect = RECT { left: card_x, top: card_y, right: card_x + col_width, bottom: card_y + 85 };
                let card_brush = CreateSolidBrush(COLORREF(0x001E293B));
                FillRect(hdc, &card_rect, card_brush);
                let _ = DeleteObject(card_brush);

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x00FBBF24));
                let header = format!("{} {}", tech.branch.name(), tech.title);
                let mut h_rect = RECT { left: card_x + 8, top: card_y + 6, right: card_x + col_width - 8, bottom: card_y + 24 };
                draw_ui_text(hdc, &header, &mut h_rect, DT_LEFT | DT_SINGLELINE);

                SelectObject(hdc, font_body);
                SetTextColor(hdc, COLORREF(0x00E2E8F0));
                let opt_a_str = format!("A: {} ({})", tech.choice_a.name, tech.choice_a.buff_desc);
                let mut oa_rect = RECT { left: card_x + 8, top: card_y + 28, right: card_x + col_width - 8, bottom: card_y + 48 };
                draw_ui_text(hdc, &opt_a_str, &mut oa_rect, DT_LEFT | DT_SINGLELINE);

                let opt_b_str = format!("B: {} ({})", tech.choice_b.name, tech.choice_b.buff_desc);
                let mut ob_rect = RECT { left: card_x + 8, top: card_y + 50, right: card_x + col_width - 8, bottom: card_y + 70 };
                draw_ui_text(hdc, &opt_b_str, &mut ob_rect, DT_LEFT | DT_SINGLELINE);
            }
        }
    }

    unsafe fn render_military_cabinet(&self, hdc: HDC, rect: &RECT, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut t_rect = RECT { left: rect.left, top: rect.top, right: rect.right, bottom: rect.top + 30 };
            draw_ui_text(hdc, "⚔️ Gabinete de Guerra y Despliegue de Ejércitos", &mut t_rect, DT_LEFT | DT_SINGLELINE);

            let mut y_off = rect.top + 45;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00F87171));
            let mil_info = format!("Poder Militar Total del Imperio: {:.0} | Divisiones Activas: {}", state.military_power, state.armies.len());
            let mut mi_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 25 };
            draw_ui_text(hdc, &mil_info, &mut mi_rect, DT_LEFT | DT_SINGLELINE);

            y_off += 30;
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00E2E8F0));
            for army in &state.armies {
                let def = get_unit_definition(army.unit_type);
                let prov_name = state.provinces.get(army.province_id).map(|p| p.name.as_str()).unwrap_or("En marcha");
                let army_str = format!(" 🛡️ {} — {} x{} (Poder de Combate: {}) en {}", army.name, def.name, army.count, army.combat_power(), prov_name);
                let mut as_rect = RECT { left: rect.left + 10, top: y_off, right: rect.right, bottom: y_off + 22 };
                draw_ui_text(hdc, &army_str, &mut as_rect, DT_LEFT | DT_SINGLELINE);
                y_off += 25;
            }
        }
    }

    unsafe fn render_wonders_and_log(&self, hdc: HDC, rect: &RECT, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut t_rect = RECT { left: rect.left, top: rect.top, right: rect.right, bottom: rect.top + 30 };
            draw_ui_text(hdc, "🏛️ Grandes Maravillas Históricas y Crónica del Imperio", &mut t_rect, DT_LEFT | DT_SINGLELINE);

            let mut y_off = rect.top + 45;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut wh_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 25 };
            draw_ui_text(hdc, "Maravillas Emblemáticas de las 15 Edades:", &mut wh_rect, DT_LEFT | DT_SINGLELINE);

            y_off += 25;
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));

            for wonder in &state.wonders {
                let status = if wonder.is_completed { "✨ COMPLETADA" } else { "🔨 EN CONSTRUCCIÓN" };
                let w_str = format!(" • [{}] {} — Progreso: {:.1}% ({})", wonder.era.short_name(), wonder.name, wonder.progress, status);
                let mut ws_rect = RECT { left: rect.left + 15, top: y_off, right: rect.right, bottom: y_off + 20 };
                draw_ui_text(hdc, &w_str, &mut ws_rect, DT_LEFT | DT_SINGLELINE);
                y_off += 20;
                if y_off > rect.bottom - 120 { break; }
            }

            y_off += 15;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x0038BDF8));
            let mut lh_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 25 };
            draw_ui_text(hdc, "Bitácora de Eventos Recientes:", &mut lh_rect, DT_LEFT | DT_SINGLELINE);

            y_off += 25;
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x0094A3B8));
            for msg in state.event_log.iter().rev() {
                let log_str = format!(" • {}", msg);
                let mut ls_rect = RECT { left: rect.left + 15, top: y_off, right: rect.right, bottom: y_off + 20 };
                draw_ui_text(hdc, &log_str, &mut ls_rect, DT_LEFT | DT_SINGLELINE);
                y_off += 20;
                if y_off > rect.bottom - 20 { break; }
            }
        }
    }
}
