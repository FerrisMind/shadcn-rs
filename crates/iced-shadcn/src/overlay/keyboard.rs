use iced::Event;
use iced::keyboard;
use iced::keyboard::key::{self, Key};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayCommand {
    Close,
}

pub fn command(event: &Event) -> Option<OverlayCommand> {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key::Named::Escape),
            ..
        }) => Some(OverlayCommand::Close),
        _ => None,
    }
}
