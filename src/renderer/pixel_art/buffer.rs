// 32-Bit RGBA Software Sprite & Pixel Art Generation Engine
// Genera texturas y sprites procedurales con iluminación, sombreado y estética 32-bit (Cossacks / Imperium / AoE II)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color32 {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Color32 {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { b, g, r, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { b, g, r, a: 255 }
    }

    pub const TRANSPARENT: Self = Self { b: 0, g: 0, r: 0, a: 0 };
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    // Paleta Clásica Cossacks & Imperium
    pub const GRASS_LIGHT: Self = Self::rgb(92, 133, 44);
    pub const GRASS_MID: Self = Self::rgb(68, 102, 34);
    pub const GRASS_DARK: Self = Self::rgb(45, 71, 24);
    
    pub const WHEAT_GOLD: Self = Self::rgb(212, 168, 67);
    pub const WHEAT_LIGHT: Self = Self::rgb(238, 203, 106);
    pub const WHEAT_DARK: Self = Self::rgb(166, 124, 38);
    pub const WHEAT_DIRT: Self = Self::rgb(107, 76, 36);

    pub const COBBLE_LIGHT: Self = Self::rgb(156, 150, 138);
    pub const COBBLE_MID: Self = Self::rgb(112, 106, 96);
    pub const COBBLE_DARK: Self = Self::rgb(74, 69, 61);

    pub const DIRT_LIGHT: Self = Self::rgb(140, 110, 75);
    pub const DIRT_MID: Self = Self::rgb(105, 80, 52);
    pub const DIRT_DARK: Self = Self::rgb(71, 53, 33);

    pub const WATER_LIGHT: Self = Self::rgb(72, 140, 184);
    pub const WATER_MID: Self = Self::rgb(42, 95, 140);
    pub const WATER_DARK: Self = Self::rgb(25, 60, 95);

    pub const STONE_HIGHLIGHT: Self = Self::rgb(220, 218, 210);
    pub const STONE_LIGHT: Self = Self::rgb(185, 180, 170);
    pub const STONE_MID: Self = Self::rgb(140, 135, 125);
    pub const STONE_SHADOW: Self = Self::rgb(90, 85, 78);
    pub const STONE_DARK: Self = Self::rgb(55, 52, 48);

    pub const ROOF_RED_LIGHT: Self = Self::rgb(189, 73, 50);
    pub const ROOF_RED_MID: Self = Self::rgb(148, 48, 30);
    pub const ROOF_RED_DARK: Self = Self::rgb(99, 28, 16);

    pub const WOOD_LIGHT: Self = Self::rgb(168, 122, 77);
    pub const WOOD_MID: Self = Self::rgb(122, 85, 48);
    pub const WOOD_DARK: Self = Self::rgb(74, 50, 26);

    pub const GOLD_ACCENT: Self = Self::rgb(235, 185, 52);
    pub const GOLD_SHADOW: Self = Self::rgb(166, 122, 22);

    pub const UNIFORM_GREEN: Self = Self::rgb(26, 99, 52);
    pub const UNIFORM_BLUE: Self = Self::rgb(35, 76, 140);
    pub const UNIFORM_RED: Self = Self::rgb(168, 38, 38);
    pub const SKIN_LIGHT: Self = Self::rgb(232, 190, 153);
    pub const SKIN_SHADOW: Self = Self::rgb(186, 142, 106);
    pub const METAL_SHINE: Self = Self::rgb(215, 225, 235);
    pub const SHADOW_ALPHA: Self = Self::rgba(0, 0, 0, 110);
}

pub struct BitmapBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>, // 0xAARRGGBB en formato compatible Win32 DIB
}

impl BitmapBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color32) {
        if x < self.width && y < self.height {
            if color.a == 255 {
                let p = ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
                self.pixels[y * self.width + x] = p;
            } else if color.a > 0 {
                // Alpha blending
                let idx = y * self.width + x;
                let existing = self.pixels[idx];
                let ex_r = ((existing >> 16) & 0xFF) as u32;
                let ex_g = ((existing >> 8) & 0xFF) as u32;
                let ex_b = (existing & 0xFF) as u32;

                let alpha = color.a as u32;
                let inv_alpha = 255 - alpha;

                let out_r = ((color.r as u32 * alpha) + (ex_r * inv_alpha)) / 255;
                let out_g = ((color.g as u32 * alpha) + (ex_g * inv_alpha)) / 255;
                let out_b = ((color.b as u32 * alpha) + (ex_b * inv_alpha)) / 255;

                self.pixels[idx] = (0xFF << 24) | (out_r << 16) | (out_g << 8) | out_b;
            }
        }
    }

    pub fn fill(&mut self, color: Color32) {
        let p = ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        self.pixels.fill(p);
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color32) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                    self.set_pixel(px as usize, py as usize, color);
                }
            }
        }
    }

    // Blit de sprite con soporte de transparencia y tintado
    pub fn blit_sprite(&mut self, sprite: &BitmapBuffer, dest_x: i32, dest_y: i32) {
        for sy in 0..sprite.height {
            for sx in 0..sprite.width {
                let px = dest_x + sx as i32;
                let py = dest_y + sy as i32;
                if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                    let sp_pixel = sprite.pixels[sy * sprite.width + sx];
                    let a = ((sp_pixel >> 24) & 0xFF) as u8;
                    if a > 0 {
                        let r = ((sp_pixel >> 16) & 0xFF) as u8;
                        let g = ((sp_pixel >> 8) & 0xFF) as u8;
                        let b = (sp_pixel & 0xFF) as u8;
                        self.set_pixel(px as usize, py as usize, Color32 { b, g, r, a });
                    }
                }
            }
        }
    }
}
