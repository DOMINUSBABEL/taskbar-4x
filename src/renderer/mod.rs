pub mod diorama;
pub mod tactical;
pub mod menu;
pub mod pixel_art;
pub mod iso_map;

pub use diorama::DioramaRenderer;
pub use tactical::{TacticalRenderer, TacticalTab};
pub use menu::{MenuRenderer, MenuScreen};
pub use pixel_art::{BitmapBuffer, Color32, SpriteAtlas};
pub use iso_map::IsoWorldRenderer;

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

pub unsafe fn draw_ui_text(hdc: HDC, text: &str, rect: &mut RECT, format: DRAW_TEXT_FORMAT) {
    unsafe {
        let mut wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let len = wide.len() - 1;
        let _ = DrawTextW(hdc, &mut wide[..len], rect, format);
    }
}

pub unsafe fn blit_buffer_to_hdc(hdc: HDC, dest_x: i32, dest_y: i32, buf: &BitmapBuffer) {
    unsafe {
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: buf.width as i32,
                biHeight: -(buf.height as i32), // Negativo para top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD::default()],
        };

        let _ = SetDIBitsToDevice(
            hdc,
            dest_x,
            dest_y,
            buf.width as u32,
            buf.height as u32,
            0,
            0,
            0,
            buf.height as u32,
            buf.pixels.as_ptr() as *const _,
            &bmi,
            DIB_RGB_COLORS,
        );
    }
}
