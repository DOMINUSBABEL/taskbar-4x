use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use crate::engine::state::GameState;
use crate::engine::buildings::{BuildingType, get_building_definition};
use crate::engine::military::get_unit_definition;
use super::draw_ui_text;

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
}

impl TacticalRenderer {
    pub fn new() -> Self {
        Self {
            active_tab: TacticalTab::CampaignMap,
        }
    }

    pub unsafe fn render(&self, hdc: HDC, rect: &RECT, state: &GameState) {
        unsafe {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // Fondo oscuro de comando táctico
            let bg_brush = CreateSolidBrush(COLORREF(0x000F172A));
            FillRect(hdc, rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            SetBkMode(hdc, TRANSPARENT);

            // 1. HEADER SUPERIOR IMPERIAL (48px)
            let header_rect = RECT { left: 0, top: 0, right: width, bottom: 48 };
            let header_brush = CreateSolidBrush(COLORREF(0x001E293B));
            FillRect(hdc, &header_rect, header_brush);
            let _ = DeleteObject(header_brush);

            let font_title = CreateFontW(
                15, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
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
            SetTextColor(hdc, COLORREF(0x0000F0FF)); // Cian neón

            let title_text = format!("🏛️ TASK BAR 4X | {}", state.current_era.name());
            let mut t_rect = RECT { left: 16, top: 12, right: 400, bottom: 38 };
            draw_ui_text(hdc, &title_text, &mut t_rect, DT_LEFT | DT_SINGLELINE);

            // Recursos en el Header
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));

            let res_line = format!(
                "🌾Comida: {:.0} (+{:.1}/s) | 🪵Mat: {:.0} (+{:.1}/s) | 🪙Oro: {:.0} (+{:.1}/s) | 🕯️Fe: {:.0} | 📜Fil: {:.0} | 🔬Cien: {:.0} | ⚔️Mil: {:.0}",
                state.food, state.food_rate, state.materials, state.materials_rate, state.gold, state.gold_rate, state.faith, state.philosophy, state.science, state.military_power
            );
            let mut r_rect = RECT { left: 410, top: 14, right: width - 180, bottom: 36 };
            draw_ui_text(hdc, &res_line, &mut r_rect, DT_LEFT | DT_SINGLELINE);

            // Botón de Salir a Barra (F11 / Esc)
            let exit_rect = RECT { left: width - 160, top: 8, right: width - 16, bottom: 40 };
            let exit_brush = CreateSolidBrush(COLORREF(0x00334155));
            FillRect(hdc, &exit_rect, exit_brush);
            let _ = DeleteObject(exit_brush);

            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut ex_rect = RECT { left: width - 160, top: 14, right: width - 16, bottom: 36 };
            draw_ui_text(hdc, "◀ MODO BARRA (F11)", &mut ex_rect, DT_CENTER | DT_SINGLELINE);

            // 2. BARRA DE PESTAÑAS DE NAVEGACIÓN (36px)
            let tab_bar_rect = RECT { left: 0, top: 48, right: width, bottom: 84 };
            let tab_brush = CreateSolidBrush(COLORREF(0x0017191C));
            FillRect(hdc, &tab_bar_rect, tab_brush);
            let _ = DeleteObject(tab_brush);

            let tabs = [
                (TacticalTab::CampaignMap, "🗺️ 1. Mapa de Campaña 2.5D"),
                (TacticalTab::CityManager, "🏙️ 2. Ciudades y Edificios"),
                (TacticalTab::TechTree, "📜 3. Árbol de 8 Ramas de Desarrollo"),
                (TacticalTab::MilitaryCabinet, "⚔️ 4. Gabinete Militar y Frentes"),
                (TacticalTab::WondersAndLog, "🏛️ 5. Maravillas y Crónica Histórica"),
            ];

            let tab_width = width / tabs.len() as i32;
            for (i, (tab_type, tab_name)) in tabs.iter().enumerate() {
                let t_x = (i as i32) * tab_width;
                let current_tab_rect = RECT { left: t_x, top: 48, right: t_x + tab_width, bottom: 84 };

                if *tab_type == self.active_tab {
                    let active_brush = CreateSolidBrush(COLORREF(0x000284C7));
                    FillRect(hdc, &current_tab_rect, active_brush);
                    let _ = DeleteObject(active_brush);
                    SetTextColor(hdc, COLORREF(0x00FFFFFF));
                    SelectObject(hdc, font_bold);
                } else {
                    SetTextColor(hdc, COLORREF(0x0094A3B8));
                    SelectObject(hdc, font_body);
                }

                let mut text_rect = RECT { left: t_x, top: 56, right: t_x + tab_width, bottom: 80 };
                draw_ui_text(hdc, tab_name, &mut text_rect, DT_CENTER | DT_SINGLELINE);
            }

            // 3. CONTENIDO DE LA PESTAÑA ACTIVA
            let content_rect = RECT { left: 20, top: 100, right: width - 20, bottom: height - 20 };

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

    unsafe fn render_campaign_map(&self, hdc: HDC, rect: &RECT, state: &GameState, _font_title: HFONT, _font_body: HFONT, font_bold: HFONT) {
        unsafe {
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // Panel del Mapa 2.5D (Lienzo Izquierdo)
            let map_width = (width * 65) / 100;
            let map_rect = RECT { left: rect.left, top: rect.top, right: rect.left + map_width, bottom: rect.bottom };
            let map_brush = CreateSolidBrush(COLORREF(0x00091E13)); // Verde oscuro de mapa
            FillRect(hdc, &map_rect, map_brush);
            let _ = DeleteObject(map_brush);

            // Marco del mapa
            let border_pen = CreatePen(PS_SOLID, 2, COLORREF(0x00059669));
            let old_pen = SelectObject(hdc, border_pen);
            let _ = MoveToEx(hdc, map_rect.left, map_rect.top, None);
            let _ = LineTo(hdc, map_rect.right, map_rect.top);
            let _ = LineTo(hdc, map_rect.right, map_rect.bottom);
            let _ = LineTo(hdc, map_rect.left, map_rect.bottom);
            let _ = LineTo(hdc, map_rect.left, map_rect.top);
            SelectObject(hdc, old_pen);
            let _ = DeleteObject(border_pen);

            // Dibujar Provincias en el mapa
            for (i, prov) in state.provinces.iter().enumerate() {
                let px = map_rect.left + (prov.x * map_width as f32) as i32;
                let py = map_rect.top + (prov.y * height as f32) as i32;

                let (node_color, node_size) = if prov.is_colonized {
                    (COLORREF(0x000284C7), 28) // Azul aliado
                } else if prov.is_hostile {
                    (COLORREF(0x00DC2626), 24) // Rojo hostil
                } else {
                    (COLORREF(0x0064748B), 20) // Gris inexplorado
                };

                let node_rect = RECT { left: px - node_size, top: py - node_size, right: px + node_size, bottom: py + node_size };
                let node_brush = CreateSolidBrush(node_color);
                FillRect(hdc, &node_rect, node_brush);
                let _ = DeleteObject(node_brush);

                if i == state.selected_province {
                    let ring_pen = CreatePen(PS_SOLID, 2, COLORREF(0x00FBBF24));
                    let old_p = SelectObject(hdc, ring_pen);
                    let _ = MoveToEx(hdc, px - node_size - 4, py - node_size - 4, None);
                    let _ = LineTo(hdc, px + node_size + 4, py - node_size - 4);
                    let _ = LineTo(hdc, px + node_size + 4, py + node_size + 4);
                    let _ = LineTo(hdc, px - node_size - 4, py + node_size + 4);
                    let _ = LineTo(hdc, px - node_size - 4, py - node_size - 4);
                    SelectObject(hdc, old_p);
                    let _ = DeleteObject(ring_pen);
                }

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x00FFFFFF));
                let lbl = format!("{}. {}", prov.id + 1, prov.name);
                let mut lbl_rect = RECT { left: px - 80, top: py + node_size + 4, right: px + 80, bottom: py + node_size + 24 };
                draw_ui_text(hdc, &lbl, &mut lbl_rect, DT_CENTER | DT_SINGLELINE);
            }

            // Panel Contextual Derecho (Detalles de Provincia)
            let info_x = map_rect.right + 20;
            let info_rect = RECT { left: info_x, top: rect.top, right: rect.right, bottom: rect.bottom };
            let info_brush = CreateSolidBrush(COLORREF(0x001E293B));
            FillRect(hdc, &info_rect, info_brush);
            let _ = DeleteObject(info_brush);

            if let Some(prov) = state.provinces.get(state.selected_province) {
                SelectObject(hdc, _font_title);
                SetTextColor(hdc, COLORREF(0x0000F0FF));
                let p_title = format!("Provincia: {}", prov.name);
                let mut pt_rect = RECT { left: info_x + 16, top: rect.top + 16, right: rect.right - 16, bottom: rect.top + 40 };
                draw_ui_text(hdc, &p_title, &mut pt_rect, DT_LEFT | DT_SINGLELINE);

                SelectObject(hdc, _font_body);
                SetTextColor(hdc, COLORREF(0x00E2E8F0));
                let desc = format!(
                    "Bioma: {}\nEstado: {}\nNivel de Desarrollo: {}\nGuarnición: {} hombres\n\n[Haz clic en una provincia del mapa para seleccionarla]",
                    prov.biome.name(),
                    if prov.is_colonized { "Asentamiento Imperial" } else if prov.is_hostile { "Territorio Bárbaro Hostil" } else { "Tierra Virgen Inexplorada" },
                    prov.development_level,
                    prov.garrison_strength
                );
                let mut pd_rect = RECT { left: info_x + 16, top: rect.top + 50, right: rect.right - 16, bottom: rect.top + 200 };
                draw_ui_text(hdc, &desc, &mut pd_rect, DT_LEFT | DT_WORDBREAK);
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

                // Lista de edificios construidos
                let mut y_off = rect.top + 70;
                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x00FBBF24));
                let mut bl_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 20 };
                draw_ui_text(hdc, "Edificios Construidos:", &mut bl_rect, DT_LEFT | DT_SINGLELINE);

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

                // Edificios disponibles para construir en la Era actual
                y_off += 15;
                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x0038BDF8));
                let mut al_rect = RECT { left: rect.left, top: y_off, right: rect.right, bottom: y_off + 20 };
                draw_ui_text(hdc, "Edificios Disponibles para Construir en esta Era (Haz clic para iniciar):", &mut al_rect, DT_LEFT | DT_SINGLELINE);

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
                    let b_btn_rect = RECT { left: rect.left + 15, top: y_off, right: rect.left + 450, bottom: y_off + 24 };
                    let btn_brush = CreateSolidBrush(COLORREF(0x00334155));
                    FillRect(hdc, &b_btn_rect, btn_brush);
                    let _ = DeleteObject(btn_brush);

                    SetTextColor(hdc, COLORREF(0x00FFFFFF));
                    let opt_str = format!(" 🔨 Construir {} [🪵{} Mat, 🌾{} Comida] — {}", def.name, def.material_cost, def.food_cost, def.production_bonus_desc);
                    let mut opt_rect = RECT { left: rect.left + 18, top: y_off + 3, right: rect.left + 445, bottom: y_off + 22 };
                    draw_ui_text(hdc, &opt_str, &mut opt_rect, DT_LEFT | DT_SINGLELINE);

                    y_off += 28;
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

            // Bitácora histórica de eventos recientes
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
