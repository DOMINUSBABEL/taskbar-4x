pub mod diorama;
pub mod tactical;

pub use diorama::DioramaRenderer;
pub use tactical::{TacticalRenderer, TacticalTab};

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

pub unsafe fn draw_ui_text(hdc: HDC, text: &str, rect: &mut RECT, format: DRAW_TEXT_FORMAT) {
    unsafe {
        let mut wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let len = wide.len() - 1;
        let _ = DrawTextW(hdc, &mut wide[..len], rect, format);
    }
}
