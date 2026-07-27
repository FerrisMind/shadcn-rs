#[path = "iced_preview/mod.rs"]
mod iced_preview;

use iced_preview::PreviewApp;
use lucide_icons::LUCIDE_FONT_BYTES;

const INTER_REGULAR_FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/Inter-Regular.ttf");
const INTER_BOLD_FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/Inter-Bold.ttf");
const INTER_SEMIBOLD_FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/Inter-SemiBold.ttf");

pub fn main() -> iced::Result {
    iced::application(PreviewApp::default, PreviewApp::update, PreviewApp::view)
        .subscription(PreviewApp::subscription)
        .font(LUCIDE_FONT_BYTES)
        .font(INTER_REGULAR_FONT_BYTES)
        .font(INTER_SEMIBOLD_FONT_BYTES)
        .font(INTER_BOLD_FONT_BYTES)
        .default_font(iced::Font::with_name("Inter"))
        .run()
}
