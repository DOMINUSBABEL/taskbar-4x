use super::pixel_art::{BitmapBuffer, SpriteAtlas, Color32};
use crate::engine::state::GameState;

pub struct IsoWorldRenderer {
    pub atlas: SpriteAtlas,
    pub camera_x: i32,
    pub camera_y: i32,
    pub anim_time: f32,
    pub backbuffer: BitmapBuffer,
}

impl IsoWorldRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            atlas: SpriteAtlas::generate(),
            camera_x: 0,
            camera_y: 0,
            anim_time: 0.0,
            backbuffer: BitmapBuffer::new(width, height),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.backbuffer.width != width || self.backbuffer.height != height {
            self.backbuffer = BitmapBuffer::new(width, height);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.anim_time += dt;
    }

    pub fn render_world(&mut self, state: &GameState) {
        let w = self.backbuffer.width;
        let h = self.backbuffer.height;

        // Fondo de mapa geopolítico (Demise of Nations style)
        self.backbuffer.fill(Color32::rgb(32, 44, 58)); // Océano / Golfo profundo

        let map_origin_x = (w as i32 / 2) + self.camera_x;
        let map_origin_y = 70 + self.camera_y;

        // 1. Dibujar Provincias y Territorios con Fronteras Geopolíticas
        for prov in &state.provinces {
            let px = map_origin_x + ((prov.x - 0.5) * (w as f32 * 0.8)) as i32;
            let py = map_origin_y + ((prov.y - 0.4) * (h as f32 * 0.75)) as i32;

            let (territory_color, border_color) = if prov.is_colonized {
                (Color32::rgb(142, 68, 48), Color32::rgb(212, 118, 88)) // Territorio Persa/Romano (Rojo/Terracota)
            } else if prov.is_hostile {
                (Color32::rgb(65, 82, 98), Color32::rgb(120, 140, 160)) // Territorio Bárbaro Hostil (Pizarra azulada)
            } else {
                (Color32::rgb(118, 102, 68), Color32::rgb(175, 155, 110)) // Tierra Virgen Inexplorada (Ocre)
            };

            // Dibujar territorio poligonal hexagonal/oval
            let radius_x = 75;
            let radius_y = 48;
            for dy in -radius_y..=radius_y {
                for dx in -radius_x..=radius_x {
                    let nx = dx as f32 / radius_x as f32;
                    let ny = dy as f32 / radius_y as f32;
                    let dist = nx * nx + ny * ny;
                    if dist <= 1.0 {
                        let is_border = dist > 0.85;
                        let col = if is_border { border_color } else { territory_color };
                        let sx = px + dx;
                        let sy = py + dy;
                        if sx >= 0 && sx < w as i32 && sy >= 0 && sy < h as i32 {
                            self.backbuffer.set_pixel(sx as usize, sy as usize, col);
                        }
                    }
                }
            }

            // Anillo de selección si la provincia está activa
            if prov.id == state.selected_province {
                for dy in -54..=54 {
                    for dx in -82..=82 {
                        let nx = dx as f32 / 82.0;
                        let ny = dy as f32 / 54.0;
                        let dist = nx * nx + ny * ny;
                        if dist > 0.90 && dist <= 1.0 {
                            let sx = px + dx;
                            let sy = py + dy;
                            if sx >= 0 && sx < w as i32 && sy >= 0 && sy < h as i32 {
                                self.backbuffer.set_pixel(sx as usize, sy as usize, Color32::GOLD_ACCENT);
                            }
                        }
                    }
                }
            }

            // Dibujar Asentamiento / Ciudad Isométrica en la Provincia
            let is_capital = prov.id == 0;
            if is_capital {
                self.backbuffer.blit_sprite(&self.atlas.cathedral, px - 70, py - 95);
                self.backbuffer.blit_sprite(&self.atlas.town_hall, px + 25, py - 55);
            } else if prov.is_colonized {
                self.backbuffer.blit_sprite(&self.atlas.town_hall, px - 55, py - 65);
                self.backbuffer.blit_sprite(&self.atlas.granary, px + 15, py - 40);
            } else if prov.is_hostile {
                self.backbuffer.blit_sprite(&self.atlas.barracks, px - 48, py - 55);
            } else {
                self.backbuffer.blit_sprite(&self.atlas.cottage, px - 35, py - 45);
            }

            // Barra de Salud de la Guarnición si está dañada o en combate
            if prov.garrison_hp < prov.max_garrison_hp && prov.max_garrison_hp > 0.0 {
                let bar_w = 40;
                let hp_ratio = (prov.garrison_hp / prov.max_garrison_hp).clamp(0.0, 1.0);
                let fill_w = (bar_w as f32 * hp_ratio) as i32;

                self.backbuffer.draw_rect(px - 20, py - 105, bar_w, 5, Color32::BLACK);
                self.backbuffer.draw_rect(px - 19, py - 104, fill_w, 3, Color32::UNIFORM_RED);
            }
        }

        // 2. Dibujar Rutas RTS de Movimiento de Ejércitos (Líneas de Puntos Demise of Nations)
        for army in &state.armies {
            if army.is_moving {
                let start_x = map_origin_x + ((army.world_x - 0.5) * (w as f32 * 0.8)) as i32;
                let start_y = map_origin_y + ((army.world_y - 0.4) * (h as f32 * 0.75)) as i32;
                let end_x = map_origin_x + ((army.target_x - 0.5) * (w as f32 * 0.8)) as i32;
                let end_y = map_origin_y + ((army.target_y - 0.4) * (h as f32 * 0.75)) as i32;

                let steps = 20;
                for s in 0..=steps {
                    let t = s as f32 / steps as f32;
                    let dot_x = start_x + ((end_x - start_x) as f32 * t) as i32;
                    let dot_y = start_y + ((end_y - start_y) as f32 * t) as i32;

                    // Puntos circulares de marcha
                    self.backbuffer.draw_rect(dot_x - 2, dot_y - 2, 4, 4, Color32::GOLD_ACCENT);
                }
            }
        }

        // 3. Dibujar Ejércitos en el Mapa (Soldados y Banderas en Movimiento)
        for army in &state.armies {
            let ax = map_origin_x + ((army.world_x - 0.5) * (w as f32 * 0.8)) as i32;
            let ay = map_origin_y + ((army.world_y - 0.4) * (h as f32 * 0.75)) as i32;

            let march_bob = if army.is_moving { ((self.anim_time * 8.0).sin() * 2.0) as i32 } else { 0 };

            // Dibujar formación de regimiento
            self.backbuffer.blit_sprite(&self.atlas.soldier_green, ax - 12, ay - 14 + march_bob);
            self.backbuffer.blit_sprite(&self.atlas.soldier_green, ax + 2, ay - 14 + march_bob);
            self.backbuffer.blit_sprite(&self.atlas.banner_flag, ax - 5, ay - 30 + march_bob);

            // Efecto de Combate Activo (Chispas e impacto)
            if army.in_combat {
                let spark_x = ax + ((self.anim_time * 15.0).sin() * 8.0) as i32;
                let spark_y = ay - 20 + ((self.anim_time * 20.0).cos() * 8.0) as i32;
                self.backbuffer.draw_rect(spark_x - 3, spark_y - 3, 6, 6, Color32::WHITE);
                self.backbuffer.draw_rect(spark_x - 1, spark_y - 1, 2, 2, Color32::GOLD_ACCENT);
            }

            // Barra de vida del ejército
            let hp_w = 30;
            let hp_r = (army.hp / army.max_hp.max(1.0)).clamp(0.0, 1.0);
            let fill_hp = (hp_w as f32 * hp_r) as i32;
            self.backbuffer.draw_rect(ax - 15, ay + 12, hp_w, 4, Color32::BLACK);
            self.backbuffer.draw_rect(ax - 14, ay + 13, fill_hp, 2, Color32::GRASS_LIGHT);
        }

        // 4. Menú Radial de Acciones RTS (Demise of Nations Style)
        if state.radial_menu_open {
            let (rx_norm, ry_norm) = state.radial_pos;
            let rx = map_origin_x + ((rx_norm - 0.5) * (w as f32 * 0.8)) as i32;
            let ry = map_origin_y + ((ry_norm - 0.4) * (h as f32 * 0.75)) as i32;

            self.draw_radial_action_menu(rx, ry);
        }

        // Viñeta atmosférica sutil
        self.apply_vignette();
    }

    // Menú radial con botones circulares metálicos (Demise of Nations style)
    fn draw_radial_action_menu(&mut self, cx: i32, cy: i32) {
        let nodes = [
            (0, -50, "Atacar / Mover"),
            (50, 0, "Inspeccionar"),
            (0, 50, "Construir"),
            (-50, 0, "Cancelar"),
        ];

        // Líneas de conexión blancas finas
        for (dx, dy, _) in nodes {
            let steps = 15;
            for s in 0..steps {
                let t = s as f32 / steps as f32;
                let lx = cx + (dx as f32 * t) as i32;
                let ly = cy + (dy as f32 * t) as i32;
                self.backbuffer.draw_rect(lx, ly, 1, 1, Color32::WHITE);
            }
        }

        // Botones circulares metálicos con aro exterior dorado
        for (dx, dy, _) in nodes {
            let bx = cx + dx;
            let by = cy + dy;

            for py in -16..=16 {
                for px in -16..=16 {
                    let dist = (px * px + py * py) as f32;
                    if dist <= 256.0 {
                        let is_rim = dist >= 196.0;
                        let col = if is_rim { Color32::GOLD_ACCENT } else { Color32::rgb(35, 45, 55) };
                        let sx = bx + px;
                        let sy = by + py;
                        if sx >= 0 && sx < self.backbuffer.width as i32 && sy >= 0 && sy < self.backbuffer.height as i32 {
                            self.backbuffer.set_pixel(sx as usize, sy as usize, col);
                        }
                    }
                }
            }
        }
    }

    fn apply_vignette(&mut self) {
        let w = self.backbuffer.width;
        let h = self.backbuffer.height;

        let max_dist = ((w * w + h * h) as f32).sqrt() * 0.58;
        let center_x = w as f32 / 2.0;
        let center_y = h as f32 / 2.0;

        for y in 0..h {
            for x in 0..w {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist > max_dist * 0.75 {
                    let factor = ((dist - max_dist * 0.75) / (max_dist * 0.25)).clamp(0.0, 1.0);
                    let alpha = (factor * 90.0) as u8;
                    self.backbuffer.set_pixel(x, y, Color32::rgba(0, 0, 0, alpha));
                }
            }
        }
    }
}
