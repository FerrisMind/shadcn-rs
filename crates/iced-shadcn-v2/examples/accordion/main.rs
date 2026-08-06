//! Interactive playground for `iced-shadcn-v2::Accordion`.
//!
//! The example mirrors the button playground: theme controls are live, both
//! controlled selection modes are shown, and disabled, animated, bordered,
//! and custom-content states are available in one screen.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example accordion`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Task};

use iced_shadcn_v2::{
    Accordion, AccordionContent, AccordionItem, AccordionValue, BaseColor, StyleId, Theme,
    ThemeMode, fonts, iced_font,
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
    disabled: bool,
    animated: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    SingleChanged(AccordionValue),
    MultipleChanged(AccordionValue),
    ToggleDisabled,
    ToggleAnimated,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Nova),
            single: Some("item-1".to_owned()),
            multiple: vec!["item-1".to_owned()],
            disabled: false,
            animated: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Accordion".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::SingleChanged(value) => {
                self.single = value.as_single().map(str::to_owned);
            }
            Message::MultipleChanged(value) => {
                self.multiple = value.as_multiple().to_vec();
            }
            Message::ToggleDisabled => {
                self.disabled = !self.disabled;
            }
            Message::ToggleAnimated => {
                self.animated = !self.animated;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label("Theme", theme),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            text(format!(
                "single={:?} multiple={:?} animated={} disabled={}",
                self.single, self.multiple, self.animated, self.disabled
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(8);

        let actions = row![
            iced_shadcn_v2::Button::text(
                if self.disabled {
                    "Enable items"
                } else {
                    "Disable items"
                },
                theme,
            )
            .on_press(Message::ToggleDisabled),
            iced_shadcn_v2::Button::text(
                if self.animated {
                    "Disable animation"
                } else {
                    "Enable animation"
                },
                theme,
            )
            .on_press(Message::ToggleAnimated),
        ]
        .spacing(10)
        .wrap();

        let single = Accordion::new(theme)
            .value(AccordionValue::single(self.single.clone()))
            .animated(self.animated)
            .disabled(self.disabled)
            .push(AccordionItem::text(
                "item-1",
                "Is it accessible?",
                "Yes. It follows the WAI-ARIA accordion pattern.",
                theme,
            ))
            .push(AccordionItem::text(
                "item-2",
                "Can I use custom content?",
                "Yes. Use AccordionContent::with_children when a panel needs multiple paragraphs or controls.",
                theme,
            ))
            .push(
                AccordionItem::text(
                    "item-3",
                    "Can one item be disabled?",
                    "Disabled items keep their controlled content state but do not emit a trigger message.",
                    theme,
                )
                .disabled(true),
            )
            .on_value_change(Message::SingleChanged);

        let multiple_content = AccordionContent::with_children(
            theme,
            [
                text("The multiple mode accepts an ordered Vec<String> value.")
                    .size(14)
                    .color(palette.foreground)
                    .into(),
                text("The root callback receives the complete next selection.")
                    .size(14)
                    .color(palette.foreground)
                    .into(),
            ],
        )
        .spacing(16.0)
        .background(iced_shadcn_v2::SemanticColor::Muted);

        let multiple = Accordion::new(theme)
            .multiple()
            .values(self.multiple.clone())
            .animated(self.animated)
            .bordered(true)
            .radius(8.0)
            .push(
                AccordionItem::new(theme)
                    .value("item-1")
                    .trigger(iced_shadcn_v2::AccordionTrigger::text(
                        "What does multiple mode return?",
                        theme,
                    ))
                    .content(multiple_content),
            )
            .push(AccordionItem::text(
                "item-2",
                "Does the value stay controlled?",
                "Yes. Feed the emitted AccordionValue back into the next view call.",
                theme,
            ))
            .on_value_change(Message::MultipleChanged);

        let content = column![
            text("iced-shadcn-v2 Accordion")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: single and multiple values, disabled items, animated content, and custom slots")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            actions,
            section("Single", single, theme),
            section("Multiple with a bordered root", multiple, theme),
        ]
        .spacing(18)
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

fn section<'a>(
    label: &'static str,
    body: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(17)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        body.into(),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .into()
}

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.foreground)
        .into()
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    row![
        text(label)
            .size(13)
            .width(70)
            .font(iced_font(theme.font_pack().sans))
            .color(theme.palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .width(Length::Fixed(220.0)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];
