use super::super::app::preview_card;
use super::super::app::{Message, PreviewApp};
use iced::widget::{column, container};
use iced::{Element, Length};
use iced_shadcn::{AccentColor, ProgressProps, ProgressVariant, SliderProps, progress, slider};

pub fn render<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let value = app.progress_value();

    column![
        preview_card(
            theme,
            "Determinate",
            column![
                slider(
                    0.0..=100.0,
                    app.progress_values().clone(),
                    Some(Message::ProgressChanged),
                    SliderProps::new(),
                    theme,
                )
                .width(Length::Fixed(320.0)),
                container(progress(ProgressProps::new().value(value), theme))
                    .width(Length::Fixed(320.0)),
            ]
            .spacing(10),
        ),
        preview_card(
            theme,
            "Indeterminate",
            column![
                container(progress(ProgressProps::new().indeterminate(), theme))
                    .width(Length::Fixed(320.0)),
                container(progress(
                    ProgressProps::new()
                        .value(74.0)
                        .variant(ProgressVariant::Surface)
                        .color(AccentColor::Green),
                    theme,
                ))
                .width(Length::Fixed(320.0)),
            ]
            .spacing(10),
        ),
    ]
    .spacing(16)
    .into()
}
