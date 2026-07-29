//! Interactive playground for `iced-shadcn-v2::ToggleGroup`.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example toggle_group`

use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Background, Element, Length, Task};

use iced_shadcn_v2::{
    StyleId, Theme, ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleGroupSelection,
    ToggleGroupSize, ToggleGroupType, ToggleGroupVariant, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    single: Option<String>,
    multiple: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Single(Option<String>),
    Multiple(Vec<String>),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Nova),
            single: Some("bold".to_owned()),
            multiple: vec!["bold".to_owned()],
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Toggle Group".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Single(value) => self.single = value,
            Message::Multiple(values) => self.multiple = values,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let single = ToggleGroup::new(theme)
            .group_type(ToggleGroupType::Single)
            .selection(ToggleGroupSelection::Single(self.single.clone()))
            .variant(ToggleGroupVariant::Outline)
            .push(ToggleGroupItem::text("bold", "Bold", theme))
            .push(ToggleGroupItem::text("italic", "Italic", theme))
            .push(ToggleGroupItem::text("underline", "Underline", theme))
            .on_change(Message::Single);

        let multiple = ToggleGroup::new(theme)
            .group_type(ToggleGroupType::Multiple)
            .selection(ToggleGroupSelection::Multiple(self.multiple.clone()))
            .variant(ToggleGroupVariant::Outline)
            .size(ToggleGroupSize::Sm)
            .push(ToggleGroupItem::icon("bold", text("B"), theme))
            .push(ToggleGroupItem::icon("italic", text("I"), theme))
            .push(ToggleGroupItem::icon("underline", text("U"), theme))
            .on_change_values(Message::Multiple);

        let spaced = ToggleGroup::new(theme)
            .group_type(ToggleGroupType::Multiple)
            .values(self.multiple.clone())
            .variant(ToggleGroupVariant::Outline)
            .spacing(2.0)
            .push(ToggleGroupItem::text("star", "Star", theme))
            .push(ToggleGroupItem::text("heart", "Heart", theme))
            .push(ToggleGroupItem::text("bookmark", "Bookmark", theme))
            .on_change_values(Message::Multiple);

        let vertical = ToggleGroup::new(theme)
            .group_type(ToggleGroupType::Multiple)
            .values(self.multiple.clone())
            .orientation(ToggleGroupOrientation::Vertical)
            .spacing(1.0)
            .size(ToggleGroupSize::Lg)
            .push(ToggleGroupItem::text("bold", "Bold", theme))
            .push(ToggleGroupItem::text("italic", "Italic", theme))
            .push(ToggleGroupItem::text("underline", "Underline", theme))
            .on_change_values(Message::Multiple);

        let disabled = ToggleGroup::new(theme)
            .group_type(ToggleGroupType::Single)
            .value("bold")
            .variant(ToggleGroupVariant::Outline)
            .disabled(true)
            .push(ToggleGroupItem::text("bold", "Bold", theme))
            .push(ToggleGroupItem::text("italic", "Italic", theme));

        let content = column![
            text("iced-shadcn-v2 Toggle Group")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: single/multiple, outline, orientation, spacing")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            section("Single selection", single, theme),
            section("Multiple icon selection", multiple, theme),
            section("Spaced items", spaced, theme),
            section("Vertical", vertical, theme),
            section("Disabled", disabled, theme),
            text(format_selection(
                "single",
                self.single.as_deref(),
                &self.multiple
            ))
            .size(13)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(18)
        .max_width(900)
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

fn section<'a>(
    label: &'static str,
    group: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(17)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        group.into(),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .into()
}

fn format_selection(label: &str, single: Option<&str>, multiple: &[String]) -> String {
    let single = single.unwrap_or("none");
    format!("{label}: {single}; multiple: [{}]", multiple.join(", "))
}
