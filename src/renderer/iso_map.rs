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

    pub fn render_world(&mut self, _state: &GameState) {
        let w = self.backbuffer.width;
        let _h = self.backbuffer.height;

        // Limpiar fondo con color de tierra base
        self.backbuffer.fill(Color32::rgb(35, 48, 22));

        // Origen isométrico centrado en pantalla
        let origin_x = (w as i32 / 2) + self.camera_x;
        let origin_y = 60 + self.camera_y;

        let grid_size = 18;

        // 1. Dibujar Mosaicos de Terreno Isométrico (64x32)
        for iso_y in 0..grid_size {
            for iso_x in 0..grid_size {
                let screen_x = origin_x + (iso_x - iso_y) * 32;
                let screen_y = origin_y + (iso_x + iso_y) * 16;

                // Determinar tipo de terreno
                let tile = if iso_x >= 12 && iso_y <= 8 {
                    &self.atlas.wheat_tile // Gran campo de trigo al norte/este (Cossacks style)
                } else if iso_x >= 5 && iso_x <= 10 && iso_y >= 5 && iso_y <= 11 {
                    &self.atlas.cobble_tile // Plaza central y calles empedradas
                } else if iso_y >= 14 && iso_y <= 15 {
                    &self.atlas.water_tile // Río / Canal en el sur
                } else if iso_x == 4 || iso_y == 4 {
                    &self.atlas.dirt_tile // Caminos de tierra conectores
                } else {
                    &self.atlas.grass_tile // Praderas verdes
                };

                self.backbuffer.blit_sprite(tile, screen_x - 32, screen_y);
            }
        }

        // 2. Dibujar Edificios Isométricos en Orden de Profundidad (Z-Sort)
        
        // Molino en los campos de trigo
        let mill_screen_x = origin_x + (14 - 3) * 32 - 48;
        let mill_screen_y = origin_y + (14 + 3) * 16 - 85;
        self.backbuffer.blit_sprite(&self.atlas.windmill_base, mill_screen_x, mill_screen_y);

        // Aspas giratorias del molino
        let blade_angle = self.anim_time * 2.5;
        self.draw_windmill_blades(mill_screen_x + 48, mill_screen_y + 32, blade_angle);

        // Granero junto al trigo
        let granary_x = origin_x + (12 - 7) * 32 - 42;
        let granary_y = origin_y + (12 + 7) * 16 - 65;
        self.backbuffer.blit_sprite(&self.atlas.granary, granary_x, granary_y);

        // Gran Basílica / Catedral en el centro de la ciudad
        let cath_x = origin_x + (7 - 7) * 32 - 70;
        let cath_y = origin_y + (7 + 7) * 16 - 130;
        self.backbuffer.blit_sprite(&self.atlas.cathedral, cath_x, cath_y);

        // Palacio / Ayuntamiento
        let town_x = origin_x + (5 - 9) * 32 - 55;
        let town_y = origin_y + (5 + 9) * 16 - 95;
        self.backbuffer.blit_sprite(&self.atlas.town_hall, town_x, town_y);

        // Herrería / Forja con humo animado
        let forge_x = origin_x + (10 - 5) * 32 - 40;
        let forge_y = origin_y + (10 + 5) * 16 - 70;
        self.backbuffer.blit_sprite(&self.atlas.forge, forge_x, forge_y);
        self.draw_smoke_particles(forge_x + 55, forge_y + 8);

        // Cuartel Militar
        let barr_x = origin_x + (9 - 10) * 32 - 48;
        let barr_y = origin_y + (9 + 10) * 16 - 75;
        self.backbuffer.blit_sprite(&self.atlas.barracks, barr_x, barr_y);

        // Cabañas residenciales
        let cot1_x = origin_x + (3 - 6) * 32 - 35;
        let cot1_y = origin_y + (3 + 6) * 16 - 60;
        self.backbuffer.blit_sprite(&self.atlas.cottage, cot1_x, cot1_y);

        let cot2_x = origin_x + (2 - 8) * 32 - 35;
        let cot2_y = origin_y + (2 + 8) * 16 - 60;
        self.backbuffer.blit_sprite(&self.atlas.cottage, cot2_x, cot2_y);

        // Bosque de Robles y Pinos en los flancos
        let trees = [(1, 2), (2, 1), (0, 4), (16, 12), (15, 14), (17, 10), (1, 12), (2, 14)];
        for (tx, ty) in trees {
            let tree_x = origin_x + (tx - ty) * 32 - 24;
            let tree_y = origin_y + (tx + ty) * 16 - 45;
            if (tx + ty) % 2 == 0 {
                self.backbuffer.blit_sprite(&self.atlas.tree_oak, tree_x, tree_y);
            } else {
                self.backbuffer.blit_sprite(&self.atlas.tree_pine, tree_x, tree_y);
            }
        }

        // 3. Campesinos trabajando en el campo de trigo (Cossacks Peasant style)
        for i in 0..4 {
            let px = origin_x + (13 + (i % 2) * 2 - (2 + (i / 2) * 2)) * 32 - 7;
            let py = origin_y + (13 + (i % 2) * 2 + (2 + (i / 2) * 2)) * 16 - 15;
            self.backbuffer.blit_sprite(&self.atlas.peasant, px, py);
        }

        // 4. Batallón de Infantería de Línea en Formación Rectangular (Cossacks Regiment style)
        // Regimiento de 36 soldados en formación 9x4 con estandarte y oficial
        let reg_base_x = origin_x + (11 - 10) * 32 + 20;
        let reg_base_y = origin_y + (11 + 10) * 16 - 20;

        let march_bob = ((self.anim_time * 6.0).sin() * 1.5) as i32;

        for row in 0..4 {
            for col in 0..9 {
                let sx = reg_base_x + (col as i32 * 12) - (row as i32 * 6);
                let sy = reg_base_y + (row as i32 * 10) + (col as i32 * 4) + march_bob;
                self.backbuffer.blit_sprite(&self.atlas.soldier_green, sx, sy);
            }
        }
        // Estandarte en el frente del regimiento
        self.backbuffer.blit_sprite(&self.atlas.banner_flag, reg_base_x + 50, reg_base_y + 15 + march_bob);

        // 5. Escuadrón de Caballería en Marcha
        for c in 0..3 {
            let cx = origin_x + (5 - 3 + c) * 32 - 14;
            let cy = origin_y + (5 + 3 + c) * 16 - 25;
            self.backbuffer.blit_sprite(&self.atlas.horseman, cx, cy);
        }

        // 6. Efecto de Niebla Atmosférica y Viñeta en los Bordes
        self.apply_vignette();
    }

    fn draw_windmill_blades(&mut self, cx: i32, cy: i32, angle: f32) {
        let _length = 28.0f32;
        for i in 0..4 {
            let a = angle + (i as f32) * (std::f32::consts::PI / 2.0);
            let cos_a = a.cos();
            let sin_a = a.sin();

            for step in 0..26 {
                let r = step as f32;
                let bx = cx + (r * cos_a) as i32;
                let by = cy + (r * sin_a * 0.7) as i32; // Escala isométrica
                
                if step % 4 == 0 {
                    // Aspa de tela blanca
                    for w in -2..=2 {
                        let wx = bx + (-sin_a * w as f32) as i32;
                        let wy = by + (cos_a * w as f32) as i32;
                        if wx >= 0 && wx < self.backbuffer.width as i32 && wy >= 0 && wy < self.backbuffer.height as i32 {
                            self.backbuffer.set_pixel(wx as usize, wy as usize, Color32::WHITE);
                        }
                    }
                } else {
                    // Viga de madera central
                    if bx >= 0 && bx < self.backbuffer.width as i32 && by >= 0 && by < self.backbuffer.height as i32 {
                        self.backbuffer.set_pixel(bx as usize, by as usize, Color32::WOOD_DARK);
                    }
                }
            }
        }
    }

    fn draw_smoke_particles(&mut self, cx: i32, cy: i32) {
        for i in 0..6 {
            let t = (self.anim_time * 2.0 + (i as f32 * 0.8)) % 3.0;
            let px = cx + ((t * 8.0).sin() * 5.0) as i32 + (t * 4.0) as i32;
            let py = cy - (t * 18.0) as i32;
            let size = (3.0 + t * 4.0) as i32;
            let alpha = ((1.0 - (t / 3.0)) * 180.0) as u8;

            let smoke_col = Color32::rgba(220, 225, 230, alpha);
            self.backbuffer.draw_rect(px - size / 2, py - size / 2, size, size, smoke_col);
        }
    }

    fn apply_vignette(&mut self) {
        let w = self.backbuffer.width;
        let h = self.backbuffer.height;

        let max_dist = ((w * w + h * h) as f32).sqrt() * 0.55;
        let center_x = w as f32 / 2.0;
        let center_y = h as f32 / 2.0;

        for y in 0..h {
            for x in 0..w {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist > max_dist * 0.7 {
                    let factor = ((dist - max_dist * 0.7) / (max_dist * 0.3)).clamp(0.0, 1.0);
                    let alpha = (factor * 120.0) as u8;
                    self.backbuffer.set_pixel(x, y, Color32::rgba(0, 0, 0, alpha));
                }
            }
        }
    }
}
