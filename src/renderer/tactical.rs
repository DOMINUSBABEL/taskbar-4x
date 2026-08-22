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

            SetBkMode(hdc, TRANSPARENT);

            let font_title = CreateFontW(
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
                13, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Segoe UI"),
            );
            let font_res = CreateFontW(
                18, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET.0 as u32, OUT_DEFAULT_PRECIS.0 as u32,
                CLIP_DEFAULT_PRECIS.0 as u32, CLEARTYPE_QUALITY.0 as u32,
                DEFAULT_PITCH.0 as u32, w!("Outfit"),
            );

            // 1. RENDERIZAR LIENZO ISOMÉTRICO 32-BIT EN TODO EL FONDO
            self.iso_world.resize(width as usize, height as usize);
            self.iso_world.render_world(state);
            blit_buffer_to_hdc(hdc, 0, 0, &self.iso_world.backbuffer);

            // 2. HEADER METÁLICO SUPERIOR (Demise of Nations Style)
            self.render_top_header(hdc, width, state, font_title, font_bold, font_res);

            // 3. MINIMAPA EN MARCO DE BRONCE REMACHADO (Esquina Superior Izquierda)
            if self.active_tab == TacticalTab::CampaignMap {
                self.render_minimap_panel(hdc, font_bold);
            }

            // 4. OVERLAYS SEGÚN PESTAÑA ACTIVA
            match self.active_tab {
                TacticalTab::CampaignMap => {
                    self.render_map_overlay_controls(hdc, width, height, state, font_bold, font_body);
                }
                TacticalTab::CityManager => {
                    self.render_city_modal(hdc, width, height, state, font_title, font_body, font_bold);
                }
                TacticalTab::TechTree => {
                    self.render_tech_modal(hdc, width, height, state, font_title, font_body, font_bold);
                }
                TacticalTab::MilitaryCabinet => {
                    self.render_military_modal(hdc, width, height, state, font_title, font_body, font_bold);
                }
                TacticalTab::WondersAndLog => {
                    self.render_wonders_modal(hdc, width, height, state, font_title, font_body, font_bold);
                }
            }

            // 5. DOCK DE COMANDOS INFERIOR CON BOTONES CIRCULARES METÁLICOS (Demise of Nations Style)
            self.render_bottom_dock(hdc, width, height, state, font_bold);

            let _ = DeleteObject(font_title);
            let _ = DeleteObject(font_body);
            let _ = DeleteObject(font_bold);
            let _ = DeleteObject(font_res);
        }
    }

    unsafe fn render_top_header(&self, hdc: HDC, width: i32, state: &GameState, _font_title: HFONT, font_bold: HFONT, font_res: HFONT) {
        unsafe {
            // Marco superior de bronce/oro remachado
            let header_rect = RECT { left: (width - 920) / 2, top: 8, right: (width + 920) / 2, bottom: 62 };
            let h_brush = CreateSolidBrush(COLORREF(0x00111827)); // Negro carbón metal
            FillRect(hdc, &header_rect, h_brush);
            let _ = DeleteObject(h_brush);

            let gold_pen = CreatePen(PS_SOLID, 3, COLORREF(0x000B9EF5)); // Borde Oro
            let op = SelectObject(hdc, gold_pen);
            let _ = MoveToEx(hdc, header_rect.left, header_rect.top, None);
            let _ = LineTo(hdc, header_rect.right, header_rect.top);
            let _ = LineTo(hdc, header_rect.right, header_rect.bottom);
            let _ = LineTo(hdc, header_rect.left, header_rect.bottom);
            let _ = LineTo(hdc, header_rect.left, header_rect.top);
            SelectObject(hdc, op);
            let _ = DeleteObject(gold_pen);

            let h_start = header_rect.left;

            // Insignia de Civilización con Laurel y Espada Verde
            let crest_rect = RECT { left: h_start + 12, top: 14, right: h_start + 50, bottom: 54 };
            let crest_brush = CreateSolidBrush(COLORREF(0x00059669)); // Verde imperial
            FillRect(hdc, &crest_rect, crest_brush);
            let _ = DeleteObject(crest_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut cr_text = RECT { left: h_start + 12, top: 22, right: h_start + 50, bottom: 50 };
            draw_ui_text(hdc, "🗡️", &mut cr_text, DT_CENTER | DT_SINGLELINE);

            // Nombre de Civilización y Jugador
            let mut name_rect = RECT { left: h_start + 58, top: 14, right: h_start + 230, bottom: 34 };
            let civ_title = format!("👑 {}", state.config.civ.name());
            draw_ui_text(hdc, &civ_title, &mut name_rect, DT_LEFT | DT_SINGLELINE);

            SetTextColor(hdc, COLORREF(0x0094A3B8));
            let mut sub_rect = RECT { left: h_start + 58, top: 35, right: h_start + 230, bottom: 55 };
            let sub_title = format!("Líder: {} | Era {}", state.config.leader.name(), state.current_era.index() + 1);
            draw_ui_text(hdc, &sub_title, &mut sub_rect, DT_LEFT | DT_SINGLELINE);

            // Cápsula de Oro Brillante (Demise of Nations Bullion style)
            let gold_pod = RECT { left: h_start + 240, top: 12, right: h_start + 420, bottom: 56 };
            let pod_brush = CreateSolidBrush(COLORREF(0x0017191C));
            FillRect(hdc, &gold_pod, pod_brush);
            let _ = DeleteObject(pod_brush);

            SelectObject(hdc, font_res);
            SetTextColor(hdc, COLORREF(0x0000F0FF)); // Oro brillante
            let mut gold_text = RECT { left: h_start + 245, top: 20, right: h_start + 415, bottom: 50 };
            let gold_str = format!("🪙 {:.0}", state.gold);
            draw_ui_text(hdc, &gold_str, &mut gold_text, DT_CENTER | DT_SINGLELINE);

            // Cápsulas de Madera, Materiales y Comida (Demise of Nations Pods)
            let res_pod = RECT { left: h_start + 430, top: 14, right: h_start + 730, bottom: 54 };
            let r_brush = CreateSolidBrush(COLORREF(0x001E293B));
            FillRect(hdc, &res_pod, r_brush);
            let _ = DeleteObject(r_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut r_text = RECT { left: h_start + 440, top: 22, right: h_start + 720, bottom: 48 };
            let res_str = format!("🪵 {:.0}   ⛏️ {:.0}   🌾 {:.0}   🔬 {:.0}", state.materials, state.materials_rate * 50.0, state.food, state.science);
            draw_ui_text(hdc, &res_str, &mut r_text, DT_CENTER | DT_SINGLELINE);

            // Medidor de Felicidad / Estabilidad (Felicidad: 81%)
            let mut hap_rect = RECT { left: h_start + 740, top: 14, right: h_start + 880, bottom: 34 };
            SetTextColor(hdc, COLORREF(0x0022C55E)); // Verde esmeralda
            let hap_str = format!("Felicidad: {:.0}%", state.stability);
            draw_ui_text(hdc, &hap_str, &mut hap_rect, DT_LEFT | DT_SINGLELINE);

            // Barra de progreso de felicidad
            let bar_rect = RECT { left: h_start + 740, top: 38, right: h_start + 880, bottom: 48 };
            let bar_bg = CreateSolidBrush(COLORREF(0x000F172A));
            FillRect(hdc, &bar_rect, bar_bg);
            let _ = DeleteObject(bar_bg);

            let fill_w = ((140.0 * (state.stability / 100.0)) as i32).clamp(0, 140);
            let fill_rect = RECT { left: h_start + 740, top: 38, right: h_start + 740 + fill_w, bottom: 48 };
            let fill_brush = CreateSolidBrush(COLORREF(0x0022C55E));
            FillRect(hdc, &fill_rect, fill_brush);
            let _ = DeleteObject(fill_brush);

            // Botón de Minimizar a Barra en la esquina derecha
            let btn_min_rect = RECT { left: width - 200, top: 12, right: width - 16, bottom: 54 };
            let min_brush = CreateSolidBrush(COLORREF(0x000284C7));
            FillRect(hdc, &btn_min_rect, min_brush);
            let _ = DeleteObject(min_brush);

            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut min_text = RECT { left: width - 200, top: 22, right: width - 16, bottom: 46 };
            draw_ui_text(hdc, "🔽 MODO BARRA (F11)", &mut min_text, DT_CENTER | DT_SINGLELINE);
        }
    }

    unsafe fn render_minimap_panel(&self, hdc: HDC, font_bold: HFONT) {
        unsafe {
            // Marco Remachado de Bronce para el Minimapa (Demise of Nations style)
            let m_rect = RECT { left: 16, top: 75, right: 240, bottom: 260 };
            let bg_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &m_rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            let border_p = CreatePen(PS_SOLID, 3, COLORREF(0x00FBBF24));
            let op = SelectObject(hdc, border_p);
            let _ = MoveToEx(hdc, m_rect.left, m_rect.top, None);
            let _ = LineTo(hdc, m_rect.right, m_rect.top);
            let _ = LineTo(hdc, m_rect.right, m_rect.bottom);
            let _ = LineTo(hdc, m_rect.left, m_rect.bottom);
            let _ = LineTo(hdc, m_rect.left, m_rect.top);
            SelectObject(hdc, op);
            let _ = DeleteObject(border_p);

            // Cabecera: Turno / Año 337 BC
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut turn_rect = RECT { left: 24, top: 82, right: 230, bottom: 102 };
            draw_ui_text(hdc, "Turno 2 | Julio, 337 a.C.", &mut turn_rect, DT_LEFT | DT_SINGLELINE);

            // Mapa mini geopolítico
            let map_inner = RECT { left: 24, top: 106, right: 232, bottom: 215 };
            let map_brush = CreateSolidBrush(COLORREF(0x001E293B));
            FillRect(hdc, &map_inner, map_brush);
            let _ = DeleteObject(map_brush);

            // Dibujar provincias miniatura
            let colors = [
                COLORREF(0x000284C7), COLORREF(0x00059669), COLORREF(0x00D97706),
                COLORREF(0x00DC2626), COLORREF(0x009333EA), COLORREF(0x00475569),
            ];
            for (i, col) in colors.iter().enumerate() {
                let mx = 40 + (i as i32 % 3) * 60;
                let my = 120 + (i as i32 / 3) * 40;
                let dot_rect = RECT { left: mx, top: my, right: mx + 45, bottom: my + 30 };
                let brush = CreateSolidBrush(*col);
                FillRect(hdc, &dot_rect, brush);
                let _ = DeleteObject(brush);
            }

            // Botones redondos de Zoom (+, -, ojo, globo)
            let zoom_icons = ["👁️", "🔍-", "🔍+", "🌐"];
            for (i, ic) in zoom_icons.iter().enumerate() {
                let bx = 26 + (i as i32) * 52;
                let by = 222;
                let z_rect = RECT { left: bx, top: by, right: bx + 44, bottom: by + 30 };
                let zb = CreateSolidBrush(COLORREF(0x00334155));
                FillRect(hdc, &z_rect, zb);
                let _ = DeleteObject(zb);

                SetTextColor(hdc, COLORREF(0x00FFFFFF));
                let mut zt = RECT { left: bx, top: by + 6, right: bx + 44, bottom: by + 28 };
                draw_ui_text(hdc, ic, &mut zt, DT_CENTER | DT_SINGLELINE);
            }
        }
    }

    unsafe fn render_map_overlay_controls(&self, hdc: HDC, width: i32, height: i32, state: &GameState, font_bold: HFONT, font_body: HFONT) {
        unsafe {
            // Panel de Control Rápido de la Provincia Seleccionada (Esquina Inferior Derecha)
            let p_rect = RECT { left: width - 380, top: height - 260, right: width - 20, bottom: height - 90 };
            let p_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &p_rect, p_brush);
            let _ = DeleteObject(p_brush);

            let border_p = CreatePen(PS_SOLID, 2, COLORREF(0x000B9EF5));
            let op = SelectObject(hdc, border_p);
            let _ = MoveToEx(hdc, p_rect.left, p_rect.top, None);
            let _ = LineTo(hdc, p_rect.right, p_rect.top);
            let _ = LineTo(hdc, p_rect.right, p_rect.bottom);
            let _ = LineTo(hdc, p_rect.left, p_rect.bottom);
            let _ = LineTo(hdc, p_rect.left, p_rect.top);
            SelectObject(hdc, op);
            let _ = DeleteObject(border_p);

            let prov = &state.provinces[state.selected_province];

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut pt = RECT { left: width - 365, top: height - 250, right: width - 30, bottom: height - 230 };
            let p_title = format!("📍 {}", prov.name);
            draw_ui_text(hdc, &p_title, &mut pt, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00E2E8F0));
            let mut pd = RECT { left: width - 365, top: height - 225, right: width - 30, bottom: height - 165 };
            let p_desc = format!(
                "Bioma: {} | {}\nGuarnición: {} hombres (HP: {:.0}/{:.0})\nEdificios: {} distritos construidos",
                prov.biome.name(),
                if prov.is_colonized { "Asentamiento Aliado" } else if prov.is_hostile { "Feudo Hostil Rebelde" } else { "Tierra Virgen" },
                prov.garrison_strength, prov.garrison_hp, prov.max_garrison_hp,
                prov.buildings.len()
            );
            draw_ui_text(hdc, &p_desc, &mut pd, DT_LEFT | DT_WORDBREAK);

            // Botones de Acción Rápida RTS (Mover Ejército / Construir Distrito)
            let btn_move = RECT { left: width - 365, top: height - 150, right: width - 200, bottom: height - 105 };
            let bm_brush = CreateSolidBrush(COLORREF(0x00DC2626)); // Rojo ataque
            FillRect(hdc, &btn_move, bm_brush);
            let _ = DeleteObject(bm_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut bmt = RECT { left: width - 365, top: height - 140, right: width - 200, bottom: height - 115 };
            draw_ui_text(hdc, "⚔️ DESPLEGAR", &mut bmt, DT_CENTER | DT_SINGLELINE);

            let btn_build = RECT { left: width - 190, top: height - 150, right: width - 30, bottom: height - 105 };
            let bb_brush = CreateSolidBrush(COLORREF(0x000284C7)); // Azul construcción
            FillRect(hdc, &btn_build, bb_brush);
            let _ = DeleteObject(bb_brush);

            let mut bbt = RECT { left: width - 190, top: height - 140, right: width - 30, bottom: height - 115 };
            draw_ui_text(hdc, "🏗️ CONSTRUIR", &mut bbt, DT_CENTER | DT_SINGLELINE);
        }
    }

    unsafe fn render_bottom_dock(&self, hdc: HDC, width: i32, height: i32, state: &GameState, font_bold: HFONT) {
        unsafe {
            // Barra Dock Inferior con Botones Redondos Metálicos (Demise of Nations style)
            let dock_w = 640;
            let dock_x = (width - dock_w) / 2;
            let dock_y = height - 76;

            let dock_rect = RECT { left: dock_x, top: dock_y, right: dock_x + dock_w, bottom: height - 8 };
            let dock_brush = CreateSolidBrush(COLORREF(0x000F172A));
            FillRect(hdc, &dock_rect, dock_brush);
            let _ = DeleteObject(dock_brush);

            let dock_border = CreatePen(PS_SOLID, 2, COLORREF(0x000B9EF5));
            let op = SelectObject(hdc, dock_border);
            let _ = MoveToEx(hdc, dock_x, dock_y, None);
            let _ = LineTo(hdc, dock_x + dock_w, dock_y);
            let _ = LineTo(hdc, dock_x + dock_w, height - 8);
            let _ = LineTo(hdc, dock_x, height - 8);
            let _ = LineTo(hdc, dock_x, dock_y);
            SelectObject(hdc, op);
            let _ = DeleteObject(dock_border);

            let buttons = [
                (TacticalTab::CampaignMap, "🗺️", "Mapa"),
                (TacticalTab::CityManager, "🏙️", "Ciudad"),
                (TacticalTab::TechTree, "🌐", "Árbol"),
                (TacticalTab::MilitaryCabinet, "✊", "Militar"),
                (TacticalTab::WondersAndLog, "ℹ️", "Crónica"),
            ];

            let btn_w = 90;
            for (i, (tab_type, icon, name)) in buttons.iter().enumerate() {
                let bx = dock_x + 15 + (i as i32) * btn_w;
                let by = dock_y + 8;
                let is_active = self.active_tab == *tab_type;

                let b_rect = RECT { left: bx, top: by, right: bx + 80, bottom: by + 50 };
                let bg_c = if is_active { COLORREF(0x000284C7) } else { COLORREF(0x001E293B) };
                let brush = CreateSolidBrush(bg_c);
                FillRect(hdc, &b_rect, brush);
                let _ = DeleteObject(brush);

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, if is_active { COLORREF(0x00FFFFFF) } else { COLORREF(0x0094A3B8) });
                let label = format!("{}\n{}", icon, name);
                let mut lt = RECT { left: bx, top: by + 6, right: bx + 80, bottom: by + 46 };
                draw_ui_text(hdc, &label, &mut lt, DT_CENTER | DT_WORDBREAK);
            }

            // Botón de Pasar Turno / Velocidad en la Derecha del Dock (Demise of Nations Checkmark style)
            let end_turn_rect = RECT { left: dock_x + dock_w - 140, top: dock_y + 8, right: dock_x + dock_w - 15, bottom: dock_y + 58 };
            let et_brush = CreateSolidBrush(COLORREF(0x00059669)); // Verde victoria
            FillRect(hdc, &end_turn_rect, et_brush);
            let _ = DeleteObject(et_brush);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let sp_lbl = format!("✔ {}", state.config.speed.name());
            let mut ett = RECT { left: end_turn_rect.left, top: end_turn_rect.top + 16, right: end_turn_rect.right, bottom: end_turn_rect.bottom };
            draw_ui_text(hdc, &sp_lbl, &mut ett, DT_CENTER | DT_SINGLELINE);
        }
    }

    unsafe fn render_city_modal(&self, hdc: HDC, width: i32, height: i32, _state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            let m_rect = RECT { left: (width - 700) / 2, top: 100, right: (width + 700) / 2, bottom: height - 100 };
            let bg_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &m_rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            let border_p = CreatePen(PS_SOLID, 2, COLORREF(0x000B9EF5));
            let op = SelectObject(hdc, border_p);
            let _ = MoveToEx(hdc, m_rect.left, m_rect.top, None);
            let _ = LineTo(hdc, m_rect.right, m_rect.top);
            let _ = LineTo(hdc, m_rect.right, m_rect.bottom);
            let _ = LineTo(hdc, m_rect.left, m_rect.bottom);
            let _ = LineTo(hdc, m_rect.left, m_rect.top);
            SelectObject(hdc, op);
            let _ = DeleteObject(border_p);

            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut tt = RECT { left: m_rect.left + 24, top: m_rect.top + 20, right: m_rect.right - 24, bottom: m_rect.top + 45 };
            draw_ui_text(hdc, "🏙️ GESTIÓN URBANA & EDIFICIOS HISTÓRICOS", &mut tt, DT_LEFT | DT_SINGLELINE);

            let mut y = m_rect.top + 60;
            let available_buildings = [
                BuildingType::Hearth, BuildingType::GrainPit, BuildingType::StoneQuarry,
                BuildingType::ShamanHut, BuildingType::MegalithCircle, BuildingType::BronzeForge,
                BuildingType::Forum, BuildingType::Watermill,
            ];

            SelectObject(hdc, font_body);
            for b in available_buildings {
                let def = get_building_definition(b);
                let b_rect = RECT { left: m_rect.left + 24, top: y, right: m_rect.right - 24, bottom: y + 36 };
                let br_brush = CreateSolidBrush(COLORREF(0x001E293B));
                FillRect(hdc, &b_rect, br_brush);
                let _ = DeleteObject(br_brush);

                SetTextColor(hdc, COLORREF(0x00FBBF24));
                SelectObject(hdc, font_bold);
                let mut lt = RECT { left: m_rect.left + 35, top: y + 8, right: m_rect.right - 35, bottom: y + 30 };
                let b_text = format!("🔨 Construir {} [🪵{} Mat, 🌾{} Comida] — {}", def.name, def.material_cost, def.food_cost, def.production_bonus_desc);
                draw_ui_text(hdc, &b_text, &mut lt, DT_LEFT | DT_SINGLELINE);

                y += 44;
            }
        }
    }

    unsafe fn render_tech_modal(&self, hdc: HDC, width: i32, height: i32, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            let m_rect = RECT { left: (width - 800) / 2, top: 90, right: (width + 800) / 2, bottom: height - 90 };
            let bg_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &m_rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut tt = RECT { left: m_rect.left + 24, top: m_rect.top + 20, right: m_rect.right - 24, bottom: m_rect.top + 45 };
            let t_str = format!("📜 ÁRBOL DE 8 RAMAS DE DESARROLLO — {}", state.current_era.name());
            draw_ui_text(hdc, &t_str, &mut tt, DT_LEFT | DT_SINGLELINE);

            let col_w = (m_rect.right - m_rect.left - 60) / 2;
            let y_off = m_rect.top + 60;

            for (i, tech) in state.era_technologies.iter().enumerate() {
                let col = i % 2;
                let row = i / 2;
                let cx = m_rect.left + 24 + (col as i32) * (col_w + 12);
                let cy = y_off + (row as i32) * 82;

                let card_rect = RECT { left: cx, top: cy, right: cx + col_w, bottom: cy + 74 };
                let card_brush = CreateSolidBrush(COLORREF(0x001E293B));
                FillRect(hdc, &card_rect, card_brush);
                let _ = DeleteObject(card_brush);

                SelectObject(hdc, font_bold);
                SetTextColor(hdc, COLORREF(0x00FBBF24));
                let mut ht = RECT { left: cx + 8, top: cy + 6, right: cx + col_w - 8, bottom: cy + 24 };
                let header = format!("{} {}", tech.branch.name(), tech.title);
                draw_ui_text(hdc, &header, &mut ht, DT_LEFT | DT_SINGLELINE);

                SelectObject(hdc, font_body);
                SetTextColor(hdc, COLORREF(0x00E2E8F0));
                let mut at = RECT { left: cx + 8, top: cy + 26, right: cx + col_w - 8, bottom: cy + 48 };
                let a_str = format!("A: {} ({})", tech.choice_a.name, tech.choice_a.buff_desc);
                draw_ui_text(hdc, &a_str, &mut at, DT_LEFT | DT_SINGLELINE);

                let mut bt = RECT { left: cx + 8, top: cy + 48, right: cx + col_w - 8, bottom: cy + 70 };
                let b_str = format!("B: {} ({})", tech.choice_b.name, tech.choice_b.buff_desc);
                draw_ui_text(hdc, &b_str, &mut bt, DT_LEFT | DT_SINGLELINE);
            }
        }
    }

    unsafe fn render_military_modal(&self, hdc: HDC, width: i32, height: i32, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            let m_rect = RECT { left: (width - 700) / 2, top: 100, right: (width + 700) / 2, bottom: height - 100 };
            let bg_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &m_rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut tt = RECT { left: m_rect.left + 24, top: m_rect.top + 20, right: m_rect.right - 24, bottom: m_rect.top + 45 };
            draw_ui_text(hdc, "⚔️ GABINETE DE GUERRA & DESPLIEGUE RTS", &mut tt, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00F87171));
            let mut mi = RECT { left: m_rect.left + 24, top: m_rect.top + 55, right: m_rect.right - 24, bottom: m_rect.top + 80 };
            let mil_info = format!("Poder Bélico Total: {:.0} | Divisiones Activas: {}", state.military_power, state.armies.len());
            draw_ui_text(hdc, &mil_info, &mut mi, DT_LEFT | DT_SINGLELINE);

            let mut y = m_rect.top + 90;
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00E2E8F0));
            for army in &state.armies {
                let def = get_unit_definition(army.unit_type);
                let prov_name = state.provinces.get(army.current_province_id).map(|p| p.name.as_str()).unwrap_or("En marcha");
                let a_str = format!(" 🛡️ {} — {} x{} (HP: {:.0}/{:.0}, Poder: {}) en {}", army.name, def.name, army.count, army.hp, army.max_hp, army.combat_power(), prov_name);
                let mut at = RECT { left: m_rect.left + 24, top: y, right: m_rect.right - 24, bottom: y + 25 };
                draw_ui_text(hdc, &a_str, &mut at, DT_LEFT | DT_SINGLELINE);
                y += 30;
            }
        }
    }

    unsafe fn render_wonders_modal(&self, hdc: HDC, width: i32, height: i32, state: &GameState, font_title: HFONT, font_body: HFONT, font_bold: HFONT) {
        unsafe {
            let m_rect = RECT { left: (width - 700) / 2, top: 100, right: (width + 700) / 2, bottom: height - 100 };
            let bg_brush = CreateSolidBrush(COLORREF(0x00111827));
            FillRect(hdc, &m_rect, bg_brush);
            let _ = DeleteObject(bg_brush);

            SelectObject(hdc, font_title);
            SetTextColor(hdc, COLORREF(0x0000F0FF));
            let mut tt = RECT { left: m_rect.left + 24, top: m_rect.top + 20, right: m_rect.right - 24, bottom: m_rect.top + 45 };
            draw_ui_text(hdc, "🏛️ GRANDES MARAVILLAS & CRÓNICA HISTÓRICA", &mut tt, DT_LEFT | DT_SINGLELINE);

            let mut y = m_rect.top + 60;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x00FBBF24));
            let mut wh = RECT { left: m_rect.left + 24, top: y, right: m_rect.right - 24, bottom: y + 25 };
            draw_ui_text(hdc, "Maravillas Emblemáticas de las 15 Edades:", &mut wh, DT_LEFT | DT_SINGLELINE);

            y += 30;
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            for wonder in state.wonders.iter().take(6) {
                let status = if wonder.is_completed { "✨ COMPLETADA" } else { "🔨 EN CONSTRUCCIÓN" };
                let w_str = format!(" • [{}] {} — Progreso: {:.1}% ({})", wonder.era.short_name(), wonder.name, wonder.progress, status);
                let mut wt = RECT { left: m_rect.left + 35, top: y, right: m_rect.right - 35, bottom: y + 20 };
                draw_ui_text(hdc, &w_str, &mut wt, DT_LEFT | DT_SINGLELINE);
                y += 24;
            }

            y += 20;
            SelectObject(hdc, font_bold);
            SetTextColor(hdc, COLORREF(0x0038BDF8));
            let mut lh = RECT { left: m_rect.left + 24, top: y, right: m_rect.right - 24, bottom: y + 25 };
            draw_ui_text(hdc, "Bitácora de Eventos Recientes:", &mut lh, DT_LEFT | DT_SINGLELINE);

            y += 28;
            SelectObject(hdc, font_body);
            SetTextColor(hdc, COLORREF(0x0094A3B8));
            for msg in state.event_log.iter().rev().take(6) {
                let log_str = format!(" • {}", msg);
                let mut lt = RECT { left: m_rect.left + 35, top: y, right: m_rect.right - 35, bottom: y + 20 };
                draw_ui_text(hdc, &log_str, &mut lt, DT_LEFT | DT_SINGLELINE);
                y += 22;
            }
        }
    }
}
