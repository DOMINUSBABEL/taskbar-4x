use super::buffer::{BitmapBuffer, Color32};

pub struct SpriteAtlas {
    pub grass_tile: BitmapBuffer,
    pub wheat_tile: BitmapBuffer,
    pub cobble_tile: BitmapBuffer,
    pub water_tile: BitmapBuffer,
    pub dirt_tile: BitmapBuffer,

    pub cathedral: BitmapBuffer,
    pub windmill_base: BitmapBuffer,
    pub town_hall: BitmapBuffer,
    pub forge: BitmapBuffer,
    pub cottage: BitmapBuffer,
    pub granary: BitmapBuffer,
    pub barracks: BitmapBuffer,

    pub tree_oak: BitmapBuffer,
    pub tree_pine: BitmapBuffer,

    pub soldier_green: BitmapBuffer,
    pub soldier_blue: BitmapBuffer,
    pub soldier_red: BitmapBuffer,
    pub horseman: BitmapBuffer,
    pub peasant: BitmapBuffer,
    pub banner_flag: BitmapBuffer,
}

impl SpriteAtlas {
    pub fn generate() -> Self {
        Self {
            grass_tile: Self::gen_isometric_tile(0),
            wheat_tile: Self::gen_isometric_tile(1),
            cobble_tile: Self::gen_isometric_tile(2),
            water_tile: Self::gen_isometric_tile(3),
            dirt_tile: Self::gen_isometric_tile(4),

            cathedral: Self::gen_cathedral(),
            windmill_base: Self::gen_windmill_base(),
            town_hall: Self::gen_town_hall(),
            forge: Self::gen_forge(),
            cottage: Self::gen_cottage(),
            granary: Self::gen_granary(),
            barracks: Self::gen_barracks(),

            tree_oak: Self::gen_oak_tree(),
            tree_pine: Self::gen_pine_tree(),

            soldier_green: Self::gen_soldier(Color32::UNIFORM_GREEN),
            soldier_blue: Self::gen_soldier(Color32::UNIFORM_BLUE),
            soldier_red: Self::gen_soldier(Color32::UNIFORM_RED),
            horseman: Self::gen_horseman(),
            peasant: Self::gen_peasant(),
            banner_flag: Self::gen_banner(),
        }
    }

    // Genera mosaicos isométricos estándar 64x32 en forma de diamante
    fn gen_isometric_tile(tile_type: usize) -> BitmapBuffer {
        let w = 64;
        let h = 32;
        let mut buf = BitmapBuffer::new(w, h);

        let cx = 32.0f32;
        let cy = 16.0f32;

        for y in 0..h {
            for x in 0..w {
                let dx = (x as f32 - cx).abs() / 32.0;
                let dy = (y as f32 - cy).abs() / 16.0;

                // Forma de diamante isométrico
                if dx + dy <= 1.0 {
                    let hash = ((x * 7919 + y * 104729 + tile_type * 331) % 100) as f32 / 100.0;

                    let color = match tile_type {
                        0 => { // Hierba
                            if hash > 0.7 { Color32::GRASS_LIGHT }
                            else if hash < 0.3 { Color32::GRASS_DARK }
                            else { Color32::GRASS_MID }
                        }
                        1 => { // Campo de Trigo (Cossacks style)
                            if (x + y) % 3 == 0 {
                                Color32::WHEAT_LIGHT
                            } else if hash > 0.6 {
                                Color32::WHEAT_GOLD
                            } else if hash < 0.25 {
                                Color32::WHEAT_DARK
                            } else {
                                Color32::WHEAT_GOLD
                            }
                        }
                        2 => { // Adoquines de Ciudad
                            if x % 6 == 0 || y % 4 == 0 {
                                Color32::COBBLE_DARK
                            } else if hash > 0.6 {
                                Color32::COBBLE_LIGHT
                            } else {
                                Color32::COBBLE_MID
                            }
                        }
                        3 => { // Agua Fluvial
                            if (x * 2 + y * 3) % 7 == 0 {
                                Color32::WATER_LIGHT
                            } else if hash > 0.5 {
                                Color32::WATER_MID
                            } else {
                                Color32::WATER_DARK
                            }
                        }
                        _ => { // Camino de Tierra
                            if hash > 0.7 { Color32::DIRT_LIGHT }
                            else if hash < 0.3 { Color32::DIRT_DARK }
                            else { Color32::DIRT_MID }
                        }
                    };

                    // Sombrear los bordes para crear relieve de mosaico
                    let edge_factor = dx + dy;
                    if edge_factor > 0.92 {
                        let shaded = Color32::rgba((color.r as f32 * 0.7) as u8, (color.g as f32 * 0.7) as u8, (color.b as f32 * 0.7) as u8, 255);
                        buf.set_pixel(x, y, shaded);
                    } else {
                        buf.set_pixel(x, y, color);
                    }
                }
            }
        }

        buf
    }

    // 1. Gran Catedral / Basílica Histórica (140x160 px)
    fn gen_cathedral() -> BitmapBuffer {
        let w = 140;
        let h = 160;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra proyectada en el suelo
        for sy in 120..158 {
            for sx in 10..130 {
                let dx = (sx as f32 - 70.0) / 50.0;
                let dy = (sy as f32 - 139.0) / 18.0;
                if dx * dx + dy * dy <= 1.0 {
                    buf.set_pixel(sx, sy, Color32::SHADOW_ALPHA);
                }
            }
        }

        // Base de piedra y escalinata
        buf.draw_rect(20, 110, 100, 35, Color32::STONE_MID);
        buf.draw_rect(25, 105, 90, 10, Color32::STONE_LIGHT);
        buf.draw_rect(18, 140, 104, 6, Color32::STONE_SHADOW);

        // Fachada principal con arquerías
        buf.draw_rect(30, 45, 80, 65, Color32::STONE_LIGHT);
        buf.draw_rect(35, 50, 70, 60, Color32::STONE_MID);

        // Torre Izquierda
        buf.draw_rect(20, 20, 26, 90, Color32::STONE_LIGHT);
        buf.draw_rect(22, 10, 22, 12, Color32::STONE_HIGHLIGHT);
        // Aguja piramidal izquierda
        for i in 0..16 {
            let half = 12 - (i * 12 / 16);
            buf.draw_rect(33 - half, 10 - i, half * 2, 1, Color32::ROOF_RED_MID);
        }
        buf.draw_rect(32, -6 + 10, 2, 8, Color32::GOLD_ACCENT); // Cruz dorada

        // Torre Derecha
        buf.draw_rect(94, 20, 26, 90, Color32::STONE_LIGHT);
        buf.draw_rect(96, 10, 22, 12, Color32::STONE_HIGHLIGHT);
        // Aguja piramidal derecha
        for i in 0..16 {
            let half = 12 - (i * 12 / 16);
            buf.draw_rect(107 - half, 10 - i, half * 2, 1, Color32::ROOF_RED_MID);
        }
        buf.draw_rect(106, -6 + 10, 2, 8, Color32::GOLD_ACCENT); // Cruz dorada

        // Cúpula Central
        for cy in 25..55 {
            for cx in 48..92 {
                let dx = (cx as f32 - 70.0) / 20.0;
                let dy = (cy as f32 - 50.0) / 22.0;
                if dx * dx + dy * dy <= 1.0 && cy <= 50 {
                    let col = if cx < 65 { Color32::STONE_HIGHLIGHT } else { Color32::STONE_SHADOW };
                    buf.set_pixel(cx, cy, col);
                }
            }
        }
        // Linterna y Cruz Central
        buf.draw_rect(66, 16, 8, 12, Color32::STONE_LIGHT);
        buf.draw_rect(69, 6, 2, 10, Color32::GOLD_ACCENT);
        buf.draw_rect(67, 9, 6, 2, Color32::GOLD_ACCENT);

        // Gran Portal Arqueado y Rosetón
        buf.draw_rect(58, 85, 24, 30, Color32::STONE_DARK);
        buf.draw_rect(62, 80, 16, 8, Color32::STONE_DARK); // Arco portal
        buf.draw_rect(60, 56, 20, 20, Color32::STONE_SHADOW); // Rosetón
        buf.draw_rect(63, 59, 14, 14, Color32::GOLD_ACCENT);
        buf.draw_rect(67, 59, 6, 14, Color32::WATER_LIGHT);

        // Ventanales góticos en las torres
        buf.draw_rect(28, 40, 10, 24, Color32::STONE_DARK);
        buf.draw_rect(102, 40, 10, 24, Color32::STONE_DARK);
        buf.draw_rect(28, 70, 10, 20, Color32::STONE_DARK);
        buf.draw_rect(102, 70, 10, 20, Color32::STONE_DARK);

        buf
    }

    // 2. Molino de Viento Clásico (96x110 px)
    fn gen_windmill_base() -> BitmapBuffer {
        let w = 96;
        let h = 110;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra
        for sy in 80..105 {
            for sx in 15..85 {
                let dx = (sx as f32 - 48.0) / 32.0;
                let dy = (sy as f32 - 92.0) / 10.0;
                if dx * dx + dy * dy <= 1.0 {
                    buf.set_pixel(sx, sy, Color32::SHADOW_ALPHA);
                }
            }
        }

        // Cuerpo cónico de piedra
        for y in 35..90 {
            let progress = (y - 35) as f32 / 55.0;
            let half_w = (14.0 + progress * 16.0) as i32;
            let cx = 48;
            for x in (cx - half_w)..(cx + half_w) {
                let col = if x < cx { Color32::STONE_LIGHT } else { Color32::STONE_SHADOW };
                buf.set_pixel(x as usize, y as usize, col);
            }
        }

        // Tejado de madera
        for i in 0..16 {
            let half = 18 - (i * 18 / 16);
            buf.draw_rect(48 - half, 35 - i, half * 2, 1, Color32::ROOF_RED_MID);
        }

        // Eje central de las aspas
        buf.draw_rect(45, 30, 6, 6, Color32::WOOD_DARK);
        buf.draw_rect(46, 31, 4, 4, Color32::METAL_SHINE);

        // Puerta y sacos de harina
        buf.draw_rect(43, 72, 10, 18, Color32::WOOD_DARK);
        buf.draw_rect(28, 82, 8, 8, Color32::WHEAT_LIGHT);
        buf.draw_rect(34, 84, 8, 7, Color32::WHEAT_GOLD);

        buf
    }

    // 3. Ayuntamiento / Mansión Señorial (110x120 px)
    fn gen_town_hall() -> BitmapBuffer {
        let w = 110;
        let h = 120;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra
        for sy in 85..115 {
            for sx in 10..100 {
                let dx = (sx as f32 - 55.0) / 42.0;
                let dy = (sy as f32 - 100.0) / 12.0;
                if dx * dx + dy * dy <= 1.0 {
                    buf.set_pixel(sx, sy, Color32::SHADOW_ALPHA);
                }
            }
        }

        // Fachada de dos pisos
        buf.draw_rect(15, 45, 80, 50, Color32::STONE_LIGHT);
        buf.draw_rect(18, 48, 74, 45, Color32::STONE_MID);

        // Columnas clásicas frontales
        for c in 0..4 {
            let cx = 25 + c * 18;
            buf.draw_rect(cx, 60, 6, 35, Color32::STONE_HIGHLIGHT);
        }

        // Frontón triangular renacentista
        for i in 0..20 {
            let half = 42 - (i * 42 / 20);
            buf.draw_rect(55 - half, 45 - i, half * 2, 1, Color32::ROOF_RED_LIGHT);
        }

        // Tejado de teja española
        buf.draw_rect(10, 42, 90, 6, Color32::ROOF_RED_DARK);

        // Estandarte heráldico en balcón central
        buf.draw_rect(50, 52, 10, 15, Color32::UNIFORM_RED);
        buf.draw_rect(53, 56, 4, 4, Color32::GOLD_ACCENT);

        // Puerta noble
        buf.draw_rect(48, 72, 14, 23, Color32::WOOD_DARK);

        buf
    }

    // 4. Herrería / Forja con Fuego (80x90 px)
    fn gen_forge() -> BitmapBuffer {
        let w = 80;
        let h = 90;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra
        buf.draw_rect(8, 65, 64, 18, Color32::SHADOW_ALPHA);

        // Muros de piedra tosca
        buf.draw_rect(12, 35, 56, 35, Color32::STONE_DARK);

        // Chimenea alta
        buf.draw_rect(48, 10, 14, 30, Color32::STONE_MID);
        buf.draw_rect(46, 8, 18, 4, Color32::STONE_HIGHLIGHT);

        // Tejado de cobertizo
        for i in 0..12 {
            let half = 30 - (i * 30 / 12);
            buf.draw_rect(40 - half, 35 - i, half * 2, 1, Color32::WOOD_DARK);
        }

        // Horno ardiente brillante
        buf.draw_rect(20, 48, 16, 18, Color32::BLACK);
        buf.draw_rect(22, 52, 12, 12, Color32::ROOF_RED_LIGHT);
        buf.draw_rect(25, 56, 6, 6, Color32::GOLD_ACCENT);

        // Yunque de hierro
        buf.draw_rect(44, 56, 8, 8, Color32::METAL_SHINE);
        buf.draw_rect(42, 54, 12, 3, Color32::METAL_SHINE);

        buf
    }

    // 5. Cabaña Rural / Casa de Colonos (70x80 px)
    fn gen_cottage() -> BitmapBuffer {
        let w = 70;
        let h = 80;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra
        buf.draw_rect(5, 55, 60, 16, Color32::SHADOW_ALPHA);

        // Muros de madera y yeso
        buf.draw_rect(10, 32, 50, 30, Color32::WOOD_LIGHT);
        // Vigas de madera entramadas
        buf.draw_rect(10, 32, 4, 30, Color32::WOOD_DARK);
        buf.draw_rect(56, 32, 4, 30, Color32::WOOD_DARK);
        buf.draw_rect(33, 32, 4, 30, Color32::WOOD_DARK);
        buf.draw_rect(10, 46, 50, 3, Color32::WOOD_DARK);

        // Tejado de paja / tejas
        for i in 0..16 {
            let half = 28 - (i * 28 / 16);
            buf.draw_rect(35 - half, 32 - i, half * 2, 1, Color32::WHEAT_DARK);
        }

        // Puerta y ventana
        buf.draw_rect(18, 48, 8, 14, Color32::WOOD_DARK);
        buf.draw_rect(40, 38, 8, 8, Color32::WATER_LIGHT);

        buf
    }

    // 6. Granero de Grano (85x85 px)
    fn gen_granary() -> BitmapBuffer {
        let w = 85;
        let h = 85;
        let mut buf = BitmapBuffer::new(w, h);

        buf.draw_rect(8, 60, 70, 16, Color32::SHADOW_ALPHA);
        buf.draw_rect(12, 30, 60, 35, Color32::WOOD_MID);

        // Tejado amplio a dos aguas
        for i in 0..18 {
            let half = 34 - (i * 34 / 18);
            buf.draw_rect(42 - half, 30 - i, half * 2, 1, Color32::ROOF_RED_MID);
        }

        // Puertas dobles de granero
        buf.draw_rect(32, 42, 20, 23, Color32::WOOD_DARK);
        buf.draw_rect(41, 42, 2, 23, Color32::BLACK);

        // Sacos y carretilla
        buf.draw_rect(16, 54, 8, 8, Color32::WHEAT_LIGHT);
        buf.draw_rect(22, 56, 6, 6, Color32::WHEAT_GOLD);

        buf
    }

    // 7. Cuartel de Tropas (95x95 px)
    fn gen_barracks() -> BitmapBuffer {
        let w = 95;
        let h = 95;
        let mut buf = BitmapBuffer::new(w, h);

        buf.draw_rect(10, 65, 75, 18, Color32::SHADOW_ALPHA);
        buf.draw_rect(15, 30, 65, 40, Color32::STONE_MID);

        // Almenas superiores defensivas
        for i in 0..5 {
            buf.draw_rect(15 + i * 13, 22, 8, 8, Color32::STONE_LIGHT);
        }

        // Puerta con reja de hierro
        buf.draw_rect(38, 45, 18, 25, Color32::WOOD_DARK);
        buf.draw_rect(40, 48, 14, 20, Color32::STONE_DARK);

        // Estandarte del regimiento
        buf.draw_rect(72, 12, 2, 25, Color32::WOOD_LIGHT);
        buf.draw_rect(74, 12, 12, 8, Color32::UNIFORM_BLUE);

        buf
    }

    // Árbol de Roble Frondoso (48x56 px)
    fn gen_oak_tree() -> BitmapBuffer {
        let w = 48;
        let h = 56;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra
        buf.draw_rect(10, 44, 28, 8, Color32::SHADOW_ALPHA);
        // Tronco
        buf.draw_rect(21, 30, 6, 18, Color32::WOOD_DARK);

        // Copa de follaje denso
        for y in 4..38 {
            for x in 4..44 {
                let dx = (x as f32 - 24.0) / 18.0;
                let dy = (y as f32 - 20.0) / 16.0;
                if dx * dx + dy * dy <= 1.0 {
                    let col = if x < 20 && y < 20 { Color32::GRASS_LIGHT }
                    else if y > 24 { Color32::GRASS_DARK }
                    else { Color32::GRASS_MID };
                    buf.set_pixel(x, y, col);
                }
            }
        }
        buf
    }

    // Árbol de Pino (32x48 px)
    fn gen_pine_tree() -> BitmapBuffer {
        let w = 32;
        let h = 48;
        let mut buf = BitmapBuffer::new(w, h);

        buf.draw_rect(8, 40, 16, 6, Color32::SHADOW_ALPHA);
        buf.draw_rect(14, 30, 4, 12, Color32::WOOD_DARK);

        for tier in 0..3 {
            let top_y = 6 + tier * 10;
            for i in 0..12 {
                let half = 4 + tier * 3 + (i * 5 / 12);
                buf.draw_rect(16 - half, top_y + i, half * 2, 1, Color32::GRASS_DARK);
            }
        }
        buf
    }

    // Soldado de Línea / Regimiento (16x24 px estilo Cossacks)
    fn gen_soldier(coat_color: Color32) -> BitmapBuffer {
        let w = 16;
        let h = 24;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra
        buf.draw_rect(3, 21, 10, 3, Color32::SHADOW_ALPHA);

        // Botas y Pantalones
        buf.draw_rect(5, 17, 3, 5, Color32::BLACK);
        buf.draw_rect(9, 17, 3, 5, Color32::BLACK);
        buf.draw_rect(5, 13, 7, 5, Color32::WHITE);

        // Chaqueta / Casaca de uniforme
        buf.draw_rect(4, 7, 9, 7, coat_color);
        buf.draw_rect(7, 7, 3, 7, Color32::WHITE); // Pechera blanca

        // Cabeza y Sombrero Tricornio
        buf.draw_rect(6, 4, 5, 4, Color32::SKIN_LIGHT);
        buf.draw_rect(4, 2, 9, 3, Color32::BLACK); // Tricornio
        buf.draw_rect(7, 1, 3, 1, Color32::GOLD_ACCENT);

        // Mosquete / Fusil con bayoneta
        buf.draw_rect(13, 1, 1, 20, Color32::WOOD_DARK);
        buf.draw_rect(13, 0, 1, 6, Color32::METAL_SHINE); // Bayoneta

        buf
    }

    // Caballero / Jinete (28x32 px)
    fn gen_horseman() -> BitmapBuffer {
        let w = 28;
        let h = 32;
        let mut buf = BitmapBuffer::new(w, h);

        // Sombra de caballo
        buf.draw_rect(4, 26, 20, 5, Color32::SHADOW_ALPHA);

        // Cuerpo de caballo (marrón noble)
        buf.draw_rect(6, 14, 16, 9, Color32::WOOD_MID);
        // Patas
        buf.draw_rect(6, 22, 3, 6, Color32::WOOD_DARK);
        buf.draw_rect(19, 22, 3, 6, Color32::WOOD_DARK);
        // Cuello y cabeza
        buf.draw_rect(18, 8, 6, 8, Color32::WOOD_MID);
        buf.draw_rect(22, 6, 5, 5, Color32::WOOD_LIGHT);

        // Jinete encima
        buf.draw_rect(10, 8, 6, 8, Color32::UNIFORM_BLUE);
        buf.draw_rect(11, 4, 5, 5, Color32::SKIN_LIGHT);
        buf.draw_rect(10, 2, 7, 3, Color32::GOLD_ACCENT); // Casco reluciente

        // Lanza con gallardete
        buf.draw_rect(17, 0, 1, 26, Color32::WOOD_DARK);
        buf.draw_rect(18, 0, 6, 4, Color32::UNIFORM_RED);

        buf
    }

    // Campesino Cosechador (14x20 px)
    fn gen_peasant() -> BitmapBuffer {
        let w = 14;
        let h = 20;
        let mut buf = BitmapBuffer::new(w, h);

        buf.draw_rect(2, 17, 10, 3, Color32::SHADOW_ALPHA);
        buf.draw_rect(4, 13, 6, 5, Color32::DIRT_DARK);
        buf.draw_rect(4, 7, 6, 7, Color32::WHITE);
        buf.draw_rect(5, 3, 4, 4, Color32::SKIN_LIGHT);
        buf.draw_rect(4, 2, 6, 2, Color32::WHEAT_GOLD); // Sombrero de paja
        buf.draw_rect(10, 5, 3, 8, Color32::METAL_SHINE); // Hoz

        buf
    }

    // Estandarte Imperial (20x36 px)
    fn gen_banner() -> BitmapBuffer {
        let w = 20;
        let h = 36;
        let mut buf = BitmapBuffer::new(w, h);

        buf.draw_rect(3, 0, 2, 36, Color32::WOOD_LIGHT);
        buf.draw_rect(2, 0, 4, 3, Color32::GOLD_ACCENT); // Águila de remate
        buf.draw_rect(5, 3, 14, 16, Color32::UNIFORM_RED);
        buf.draw_rect(8, 6, 8, 8, Color32::GOLD_ACCENT); // Escudo imperial

        buf
    }
}
