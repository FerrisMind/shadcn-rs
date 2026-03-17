#[path = "iced_preview/mod.rs"]
mod iced_preview;

use iced_preview::PreviewApp;
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(PreviewApp::default, PreviewApp::update, PreviewApp::view)
        .subscription(PreviewApp::subscription)
        .font(LUCIDE_FONT_BYTES)
        .run()
}
