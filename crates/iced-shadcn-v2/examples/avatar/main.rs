//! Interactive playground for `iced-shadcn-v2::Avatar`.
//!
//! The playground covers image handles, fallback content, size/radius
//! variants, status badges, overlapping groups, and group counts without
//! relying on network requests for image data.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example avatar`

use iced::widget::{column, container, row, scrollable, text};
use iced::{Background, Element, Length, Task};
use std::path::PathBuf;

use iced_shadcn_v2::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarRadius,
    AvatarSize, BaseColor, Button, ButtonVariant, FontId, StyleId, Theme, ThemeMode, fonts,
    iced_font,
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
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Style(StyleId),
    Mode(ThemeMode),
    Radius(RadiusOpt),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_base(BaseColor::Neutral),
            radius: RadiusOpt::Theme,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Avatar".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style),
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode),
            Message::Radius(radius) => self.radius = radius,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;
        let radius = self.radius.into();

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
                radius_button(RadiusOpt::Medium, self.radius, theme),
                radius_button(RadiusOpt::Large, self.radius, theme),
                radius_button(RadiusOpt::Full, self.radius, theme),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .width(Length::Fill);

        let preview = column![
            section_label("Sizes and fallback", theme),
            row![
                Avatar::new(theme)
                    .size(AvatarSize::Sm)
                    .radius(radius)
                    .image(sample_image())
                    .fallback_text("CN"),
                Avatar::new(theme)
                    .radius(radius)
                    .image(sample_image())
                    .fallback_text("ER"),
                Avatar::new(theme)
                    .size(AvatarSize::Lg)
                    .radius(radius)
                    .fallback(AvatarFallback::text("LR", theme)),
            ]
            .spacing(20)
            .align_y(iced::alignment::Vertical::Center),
            section_label("Badges", theme),
            row![
                Avatar::new(theme)
                    .size(AvatarSize::Sm)
                    .radius(radius)
                    .fallback_text("JZ")
                    .badge_dot(),
                Avatar::new(theme)
                    .radius(radius)
                    .fallback_text("PP")
                    .badge(AvatarBadge::icon(text("+").size(8), theme)),
                Avatar::new(theme)
                    .size(AvatarSize::Lg)
                    .radius(radius)
                    .image(sample_image())
                    .fallback_text("OK")
                    .badge(AvatarBadge::icon(text("✓").size(8), theme)),
            ]
            .spacing(20)
            .align_y(iced::alignment::Vertical::Center),
            section_label("Groups", theme),
            AvatarGroup::new(theme)
                .push(
                    Avatar::new(theme)
                        .size(AvatarSize::Sm)
                        .image(reference_image("01.png"))
                        .fallback_text("CN"),
                )
                .push(
                    Avatar::new(theme)
                        .size(AvatarSize::Sm)
                        .image(reference_image("02.png"))
                        .fallback_text("LR"),
                )
                .push(
                Avatar::new(theme)
                    .size(AvatarSize::Sm)
                    .image(reference_image("03.png"))
                    .fallback_text("ER"),
            )
                .count(AvatarGroupCount::icon(text("+").size(10), theme)),
            AvatarGroup::new(theme)
                .push(
                    Avatar::new(theme)
                        .image(reference_image("01.png"))
                        .fallback_text("CN"),
                )
                .push(
                    Avatar::new(theme)
                        .image(reference_image("02.png"))
                        .fallback_text("LR"),
                )
                .push(
                    Avatar::new(theme)
                        .image(reference_image("03.png"))
                        .fallback_text("ER"),
                )
                .count(AvatarGroupCount::text("+3", theme)),
            AvatarGroup::new(theme)
                .push(
                    Avatar::new(theme)
                        .size(AvatarSize::Lg)
                        .image(reference_image("01.png"))
                        .fallback_text("CN"),
                )
                .push(
                    Avatar::new(theme)
                        .size(AvatarSize::Lg)
                        .image(reference_image("02.png"))
                        .fallback_text("LR"),
                )
                .push(
                    Avatar::new(theme)
                        .size(AvatarSize::Lg)
                        .image(reference_image("03.png"))
                        .fallback_text("ER"),
                )
                .count(AvatarGroupCount::text("+3", theme)),
            text("Native image handles keep the demo deterministic; failed image decoding reveals the fallback slot.")
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
        ]
        .spacing(16)
        .width(Length::Fill);

        let content = column![
            text("iced-shadcn-v2 Avatar")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: image, fallback, badge, size variants, groups, and counts")
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
}

fn sample_image() -> AvatarImage {
    AvatarImage::from_rgba(16, 16, sample_pixels())
}

fn reference_image(file_name: &str) -> AvatarImage {
    AvatarImage::from_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../shadcn-svelte/docs/static/avatars")
            .join(file_name),
    )
}

fn sample_pixels() -> Vec<u8> {
    let mut pixels = Vec::with_capacity(16 * 16 * 4);

    for y in 0..16u8 {
        for x in 0..16u8 {
            let diagonal = x.abs_diff(y);
            pixels.extend([
                70 + x.saturating_mul(5),
                105 + y.saturating_mul(4),
                150 + diagonal,
                255,
            ]);
        }
    }

    pixels
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
    Button::text(style.as_str(), theme)
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
    Button::text(label, theme)
        .variant(ButtonVariant::Outline)
        .on_press(message(value))
        .into()
}

fn radius_button(radius: RadiusOpt, selected: RadiusOpt, theme: &Theme) -> Element<'_, Message> {
    Button::text(radius.label(), theme)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadiusOpt {
    Theme,
    None,
    Medium,
    Large,
    Full,
}

impl RadiusOpt {
    const fn label(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::None => "none",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::Full => "full",
        }
    }
}

impl From<RadiusOpt> for AvatarRadius {
    fn from(radius: RadiusOpt) -> Self {
        match radius {
            RadiusOpt::Theme => Self::Theme,
            RadiusOpt::None => Self::None,
            RadiusOpt::Medium => Self::Medium,
            RadiusOpt::Large => Self::Large,
            RadiusOpt::Full => Self::Full,
        }
    }
}
