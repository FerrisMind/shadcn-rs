mod badge;
mod button;
mod input;
mod progress;
mod stepper;

use super::app::{Message, PreviewApp};
use super::catalog::PreviewPage;
use iced::Element;

pub fn render<'a>(selected: PreviewPage, app: &'a PreviewApp) -> Element<'a, Message> {
    match selected {
        PreviewPage::Home => super::home::render(app),
        PreviewPage::Button => button::render(app),
        PreviewPage::Badge => badge::render(app),
        PreviewPage::Progress => progress::render(app),
        PreviewPage::Stepper => stepper::render(app),
        PreviewPage::Input => input::render(app),
    }
}
