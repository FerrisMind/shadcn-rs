//! Interactive playground for `iced-shadcn-v2::Alert`.
//!
//! The examples mirror shadcn-svelte's alert demo: default and destructive
//! variants, icon/title/description composition, arbitrary description
//! content, and the top-right action slot.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example alert`

use iced::widget::{column, container, row, scrollable, text};
use iced::{Background, Element, Length, Task};

use iced_shadcn_v2::{
    Alert, AlertAction, AlertDescription, AlertRadius, AlertTitle, AlertVariant, BaseColor,
    ButtonSize, ButtonVariant, FontId, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    radius: RadiusOpt,
    notice: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Style(StyleId),
    Mode(ThemeMode),
    Radius(RadiusOpt),
    Action,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_base(BaseColor::Neutral),
            radius: RadiusOpt::Theme,
            notice: "Ready".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Alert".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style),
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode),
            Message::Radius(radius) => self.radius = radius,
            Message::Action => self.notice = "The alert action emitted a message".to_owned(),
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label("Style pack", theme),
            style_buttons(theme),
            section_label("Mode", theme),
            row![
                option_button("light", ThemeMode::Light, theme, Message::Mode),
                option_button("dark", ThemeMode::Dark, theme, Message::Mode),
            ]
            .spacing(8),
            section_label("Radius", theme),
            row![
                radius_button(RadiusOpt::Theme, self.radius, theme),
                radius_button(RadiusOpt::None, self.radius, theme),
                radius_button(RadiusOpt::Small, self.radius, theme),
                radius_button(RadiusOpt::Medium, self.radius, theme),
                radius_button(RadiusOpt::Large, self.radius, theme),
                radius_button(RadiusOpt::Xl, self.radius, theme),
            ]
            .spacing(8)
            .wrap(),
        ]
        .spacing(8)
        .width(Length::Fill);

        let preview = column![
            section_label("Preview", theme),
            self.success_alert(theme),
            self.title_only_alert(theme),
            self.destructive_alert(theme),
            self.action_alert(theme),
            text(&self.notice)
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
        ]
        .spacing(12)
        .width(Length::Fill);

        let content = column![
            text("iced-shadcn-v2 Alert")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: variants, icon, typed typography, arbitrary content, and action")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            preview,
        ]
        .spacing(16)
        .max_width(760)
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

    fn success_alert<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        Alert::new(theme)
            .radius(self.radius.into())
            .icon(glyph("✓", theme))
            .title(AlertTitle::text(
                "Success! Your changes have been saved",
                theme,
            ))
            .description(AlertDescription::text(
                "This is an alert with icon, title and description.",
                theme,
            ))
            .into()
    }

    fn title_only_alert<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        Alert::new(theme)
            .radius(self.radius.into())
            .icon(glyph("✦", theme))
            .title(AlertTitle::text(
                "This Alert has a title and an icon. No description.",
                theme,
            ))
            .into()
    }

    fn destructive_alert<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let details = column![
            text("Please verify your billing information and try again."),
            text("•  Check your card details"),
            text("•  Ensure sufficient funds"),
            text("•  Verify billing address"),
        ]
        .spacing(4)
        .width(Length::Fill);

        Alert::new(theme)
            .variant(AlertVariant::Destructive)
            .radius(self.radius.into())
            .icon(glyph("!", theme))
            .title(AlertTitle::text("Unable to process your payment.", theme))
            .description(AlertDescription::new(details, theme))
            .into()
    }

    fn action_alert<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        Alert::new(theme)
            .radius(self.radius.into())
            .icon(glyph("!", theme))
            .title(AlertTitle::text(
                "The selected emails have been marked as spam.",
                theme,
            ))
            .action(AlertAction::new(
                iced_shadcn_v2::Button::text("Undo", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Xs)
                    .on_press(Message::Action),
            ))
            .into()
    }
}

fn style_buttons(theme: &Theme) -> Element<'_, Message> {
    row![
        style_button(StyleId::Vega, theme),
        style_button(StyleId::Nova, theme),
        style_button(StyleId::Maia, theme),
        style_button(StyleId::Lyra, theme),
        style_button(StyleId::Mira, theme),
        style_button(StyleId::Luma, theme),
        style_button(StyleId::Sera, theme),
        style_button(StyleId::Rhea, theme),
    ]
    .spacing(8)
    .wrap()
    .into()
}

fn style_button(style: StyleId, theme: &Theme) -> Element<'_, Message> {
    iced_shadcn_v2::Button::text(style.as_str(), theme)
        .variant(ButtonVariant::Outline)
        .on_press(Message::Style(style))
        .into()
}

fn option_button<'a, T>(
    label: &'static str,
    value: T,
    theme: &'a Theme,
    message: impl Fn(T) -> Message,
) -> Element<'a, Message>
where
    T: Copy + 'static,
{
    iced_shadcn_v2::Button::text(label, theme)
        .variant(ButtonVariant::Outline)
        .on_press(message(value))
        .into()
}

fn radius_button(radius: RadiusOpt, selected: RadiusOpt, theme: &Theme) -> Element<'_, Message> {
    iced_shadcn_v2::Button::text(radius.label(), theme)
        .variant(if radius == selected {
            ButtonVariant::Secondary
        } else {
            ButtonVariant::Outline
        })
        .on_press(Message::Radius(radius))
        .into()
}

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.muted_foreground)
        .into()
}

fn glyph<'a>(glyph: &'a str, theme: &'a Theme) -> Element<'a, Message> {
    text(glyph)
        .size(16)
        .font(iced_font(theme.font_pack().sans))
        .color(theme.palette.foreground)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadiusOpt {
    Theme,
    None,
    Small,
    Medium,
    Large,
    Xl,
}

impl RadiusOpt {
    const fn label(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::None => "none",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::Xl => "xl",
        }
    }
}

impl From<RadiusOpt> for AlertRadius {
    fn from(radius: RadiusOpt) -> Self {
        match radius {
            RadiusOpt::Theme => Self::Theme,
            RadiusOpt::None => Self::None,
            RadiusOpt::Small => Self::Small,
            RadiusOpt::Medium => Self::Medium,
            RadiusOpt::Large => Self::Large,
            RadiusOpt::Xl => Self::Xl,
        }
    }
}
