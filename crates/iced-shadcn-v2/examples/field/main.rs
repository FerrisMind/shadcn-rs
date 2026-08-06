//! Interactive playground for the complete `iced-shadcn-v2` field family.
//!
//! The composition mirrors shadcn-svelte's field demo: fieldsets, legends,
//! descriptions, horizontal controls, responsive fields, choice cards,
//! separators, and single/multiple validation errors.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example field`.

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};
use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, Checkbox, CheckboxState, Field, FieldContent,
    FieldDescription, FieldError, FieldErrorItem, FieldGroup, FieldLabel, FieldLegend,
    FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle, FontId, Input,
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
    name: String,
    email: String,
    choice: String,
    subscribe: CheckboxState,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Name(String),
    Email(String),
    Choice(CheckboxState),
    Subscribe(CheckboxState),
    Submit,
    Cancel,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            name: "Evil Rabbit".to_owned(),
            email: "not-an-email".to_owned(),
            choice: "kubernetes".to_owned(),
            subscribe: CheckboxState::Checked,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Field".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Name(value) => self.name = value,
            Message::Email(value) => self.email = value,
            Message::Choice(state) => {
                if matches!(state, CheckboxState::Checked) {
                    self.choice = "kubernetes".to_owned();
                }
            }
            Message::Subscribe(state) => self.subscribe = state,
            Message::Submit | Message::Cancel => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", palette.muted_foreground, theme),
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
        ]
        .spacing(8);

        let payment = FieldSet::new()
            .push(FieldLegend::text("Payment Method", theme))
            .push(FieldDescription::text(
                "All transactions are secure and encrypted",
                theme,
            ))
            .push(
                FieldGroup::new()
                    .push(
                        Field::new(theme)
                            .push(FieldLabel::text("Name on Card", theme))
                            .push(
                                Input::new(theme)
                                    .value(&self.name)
                                    .placeholder("John Doe")
                                    .on_input(Message::Name),
                            ),
                    )
                    .push(
                        row![
                            Field::new(theme)
                                .push(FieldLabel::text("Card Number", theme))
                                .push(
                                    Input::new(theme)
                                        .value("1234 5678 9012 3456")
                                        .placeholder("1234 5678 9012 3456"),
                                )
                                .push(
                                    FieldDescription::text("Enter your 16-digit number.", theme,)
                                ),
                            Field::new(theme)
                                .push(FieldLabel::text("CVV", theme))
                                .push(Input::new(theme).value("123").placeholder("123")),
                        ]
                        .spacing(16)
                        .width(Length::Fill),
                    )
                    .push(
                        row![
                            Field::new(theme)
                                .push(FieldLabel::text("Month", theme))
                                .push(Input::new(theme).placeholder("MM")),
                            Field::new(theme)
                                .push(FieldLabel::text("Year", theme))
                                .push(Input::new(theme).placeholder("YYYY")),
                        ]
                        .spacing(16)
                        .width(Length::Fill),
                    ),
            );

        let billing = FieldSet::new()
            .push(FieldLegend::text("Billing Address", theme))
            .push(FieldDescription::text(
                "The billing address associated with your payment method",
                theme,
            ))
            .push(
                FieldGroup::new().push(
                    Field::new(theme)
                        .orientation(FieldOrientation::Horizontal)
                        .push(
                            Checkbox::new(theme)
                                .state(CheckboxState::Checked)
                                .on_toggle(Message::Subscribe),
                        )
                        .push(
                            FieldLabel::text("Same as shipping address", theme)
                                .context(iced_shadcn_v2::LabelContext::AdjacentControl),
                        ),
                ),
            );

        let form = FieldGroup::new()
            .push(payment)
            .push(FieldSeparator::new(theme))
            .push(billing)
            .push(FieldSeparator::new(theme))
            .push(
                FieldSet::new().push(
                    FieldGroup::new().push(
                        Field::new(theme)
                            .push(FieldLabel::text("Comments", theme))
                            .push(Input::new(theme).placeholder("Add any additional comments")),
                    ),
                ),
            )
            .push(
                Field::new(theme)
                    .orientation(FieldOrientation::Horizontal)
                    .push(
                        Button::text("Submit", theme)
                            .variant(ButtonVariant::Default)
                            .on_press(Message::Submit),
                    )
                    .push(
                        Button::text("Cancel", theme)
                            .variant(ButtonVariant::Outline)
                            .on_press(Message::Cancel),
                    ),
            );

        let validation = FieldGroup::new()
            .push(
                Field::new(theme)
                    .invalid(true)
                    .push(FieldLabel::text("Email", theme))
                    .push(
                        Input::new(theme)
                            .value(&self.email)
                            .placeholder("you@example.com")
                            .invalid(true)
                            .on_input(Message::Email),
                    )
                    .push(
                        FieldError::new(theme)
                            .errors([FieldErrorItem::new("Enter a valid email address.")]),
                    ),
            )
            .push(
                Field::new(theme)
                    .push(FieldLabel::text("Multiple errors", theme))
                    .push(
                        FieldError::new(theme)
                            .errors(["First validation error", "Second validation error"]),
                    ),
            );

        let responsive = FieldGroup::new()
            .push(FieldLegend::text("Responsive layout", theme).variant(FieldLegendVariant::Label))
            .push(
                Field::new(theme)
                    .orientation(FieldOrientation::Responsive)
                    .push(
                        FieldContent::new()
                            .push(FieldLabel::text("Display name", theme))
                            .push(FieldDescription::text(
                                "Stacks on narrow layouts and becomes a row above 448 px.",
                                theme,
                            )),
                    )
                    .push(
                        Input::new(theme)
                            .value(&self.name)
                            .placeholder("Display name"),
                    ),
            );

        let choice_card = FieldLabel::new(
            Field::new(theme)
                .orientation(FieldOrientation::Horizontal)
                .push(
                    Checkbox::new(theme)
                        .state(if self.choice == "kubernetes" {
                            CheckboxState::Checked
                        } else {
                            CheckboxState::Unchecked
                        })
                        .on_toggle(Message::Choice),
                )
                .push(
                    FieldContent::new()
                        .push(FieldTitle::text("Kubernetes", theme))
                        .push(FieldDescription::text(
                            "Run GPU workloads on a K8s configured cluster.",
                            theme,
                        )),
                ),
            theme,
        )
        .choice_card(true)
        .selected(self.choice == "kubernetes");

        let preview = column![
            section_label("Field family", palette.muted_foreground, theme),
            container(form)
                .padding(24)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(palette.card)),
                    border: Border {
                        color: palette.border,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..container::Style::default()
                }),
            section_label(
                "Validation and responsive states",
                palette.muted_foreground,
                theme
            ),
            validation,
            responsive,
            choice_card,
        ]
        .spacing(16);

        let content = column![
            text("iced-shadcn-v2 Field")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Composable labels, controls, descriptions, and errors")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            preview,
        ]
        .spacing(16)
        .max_width(720)
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
    let palette = &theme.palette;

    row![
        text(label)
            .size(13)
            .width(80)
            .font(iced_font(theme.font_pack().sans))
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(iced_font(theme.font_pack().sans))
            .width(220)
            .style(move |_theme, _status| iced::widget::pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
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
