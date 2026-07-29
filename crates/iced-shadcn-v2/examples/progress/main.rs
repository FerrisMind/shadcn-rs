//! Interactive playground for `iced-shadcn-v2::Progress`.
//!
//! Mirrors shadcn-svelte's progress examples and exposes the iced-specific
//! value, max, orientation, color, radius, and indeterminate controls.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example progress`

use iced::widget::{column, container, row, scrollable, slider, text};
use iced::{Alignment, Background, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, Button, ButtonVariant, FontId, Progress, ProgressOrientation, ProgressVariant,
    StyleId, Theme, ThemeMode, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    value: f32,
    indeterminate: bool,
    animated: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
    ToggleIndeterminate,
    ToggleAnimated,
    ToggleTheme,
    Style(StyleId),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            value: 66.0,
            indeterminate: false,
            animated: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Progress".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ValueChanged(value) => self.value = value,
            Message::ToggleIndeterminate => self.indeterminate = !self.indeterminate,
            Message::ToggleAnimated => self.animated = !self.animated,
            Message::ToggleTheme => {
                let mode = match self.theme.mode() {
                    ThemeMode::Light => ThemeMode::Dark,
                    ThemeMode::Dark => ThemeMode::Light,
                };
                self.theme = self.theme.clone().with_mode(mode);
            }
            Message::Style(style) => self.theme = self.theme.clone().with_style(style),
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;
        let base = self.current_progress(theme);

        let controls = column![
            text("Value").size(13).color(palette.muted_foreground),
            slider(0.0..=100.0, self.value, Message::ValueChanged),
            row![
                Button::text(
                    if self.indeterminate {
                        "Determinate"
                    } else {
                        "Indeterminate"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleIndeterminate),
                Button::text(
                    if self.animated {
                        "Animation on"
                    } else {
                        "Animation off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleAnimated),
                Button::text(
                    if matches!(theme.mode(), ThemeMode::Dark) {
                        "Light theme"
                    } else {
                        "Dark theme"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleTheme),
            ]
            .spacing(8)
            .wrap(),
        ]
        .spacing(8);

        let progress_bars = column![
            labeled_bar(
                "0%",
                Progress::new(theme).value(0.0).animated(self.animated)
            ),
            labeled_bar(
                "25%",
                Progress::new(theme).value(25.0).animated(self.animated),
            ),
            labeled_bar(
                "50%",
                Progress::new(theme).value(50.0).animated(self.animated),
            ),
            labeled_bar(
                "75%",
                Progress::new(theme).value(75.0).animated(self.animated),
            ),
            labeled_bar(
                "100%",
                Progress::new(theme).value(100.0).animated(self.animated),
            ),
        ]
        .spacing(12);

        let variants = row![
            labeled_bar(
                "default",
                base.variant(ProgressVariant::Default)
                    .width(Length::Fixed(220.0)),
            ),
            labeled_bar(
                "surface",
                base.variant(ProgressVariant::Surface)
                    .width(Length::Fixed(220.0)),
            ),
            labeled_bar(
                "soft",
                base.variant(ProgressVariant::Soft)
                    .width(Length::Fixed(220.0)),
            ),
        ]
        .spacing(20)
        .align_y(Alignment::Center)
        .wrap();

        let colors = row![
            labeled_bar("primary", base.theme_primary().width(Length::Fixed(180.0)),),
            labeled_bar(
                "blue",
                base.color(AccentColor::Blue).width(Length::Fixed(180.0)),
            ),
            labeled_bar(
                "custom",
                base.custom_color(Color::from_rgb(0.12, 0.55, 0.42))
                    .track_color(Color::from_rgb(0.86, 0.89, 0.87))
                    .width(Length::Fixed(180.0)),
            ),
        ]
        .spacing(20)
        .align_y(Alignment::Center)
        .wrap();

        let orientations = row![
            labeled_bar("horizontal", base.width(Length::Fixed(220.0)),),
            column![
                text("vertical").size(11).color(palette.muted_foreground),
                vertical_progress(base),
            ]
            .spacing(8)
            .align_x(Alignment::Center),
        ]
        .spacing(28)
        .align_y(Alignment::Center);

        let style_buttons = row![
            Button::text("Vega", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Vega)),
            Button::text("Nova", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Nova)),
            Button::text("Maia", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Maia)),
            Button::text("Lyra", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Lyra)),
            Button::text("Mira", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Mira)),
            Button::text("Luma", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Luma)),
            Button::text("Sera", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Sera)),
            Button::text("Rhea", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Style(StyleId::Rhea)),
        ]
        .spacing(8)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 Progress")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: value, max, muted track, primary indicator")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Controlled value", theme),
            labeled_bar("current", base.width(Length::Fixed(420.0))),
            section_label("Progress bar", theme),
            progress_bars,
            section_label("Variants", theme),
            variants,
            section_label("Colors", theme),
            colors,
            section_label("Orientation", theme),
            orientations,
            section_label("Style-pack geometry", theme),
            style_buttons,
        ]
        .spacing(16)
        .max_width(960)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }

    fn current_progress<'a>(&self, theme: &'a Theme) -> Progress<'a> {
        Progress::new(theme)
            .max(100.0)
            .animated(self.animated)
            .value_maybe(if self.indeterminate {
                None
            } else {
                Some(self.value)
            })
    }
}

fn vertical_progress<'a>(progress: Progress<'a>) -> Progress<'a> {
    progress
        .orientation(ProgressOrientation::Vertical)
        .height(Length::Fixed(140.0))
}

fn labeled_bar<'a>(label: &'static str, progress: Progress<'a>) -> Element<'a, Message> {
    column![text(label).size(11), progress,].spacing(6).into()
}

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.muted_foreground)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_progress_inherits_indeterminate_state() {
        let mut example = Example::default();
        example.indeterminate = true;

        let vertical = vertical_progress(example.current_progress(&example.theme));
        let debug = format!("{vertical:?}");

        assert!(debug.contains("value: None"), "{debug}");
        assert!(debug.contains("orientation: Vertical"), "{debug}");
        assert!(debug.contains("animated: true"), "{debug}");
    }

    #[test]
    fn vertical_progress_preserves_style_and_controlled_settings() {
        let mut example = Example::default();
        example.value = 42.0;
        example.animated = false;

        let vertical = vertical_progress(example.current_progress(&example.theme));
        let debug = format!("{vertical:?}");

        assert!(debug.contains("value: Some(42.0)"), "{debug}");
        assert!(debug.contains("max: 100.0"), "{debug}");
        assert!(debug.contains("animated: false"), "{debug}");
        assert!(debug.contains("size: Default"), "{debug}");
        assert!(debug.contains("radius: None"), "{debug}");
        assert!(debug.contains("height: Some(Fixed(140.0))"), "{debug}");
    }
}
