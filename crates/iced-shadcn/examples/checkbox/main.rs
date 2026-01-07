use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Color, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonSize, CheckboxProps, CheckboxSize, CheckboxState,
    CheckboxVariant, LabelProps, TextProps, TextSize, TextWeight, Theme, button, checkbox, label,
    label_with_props, text,
};
use iced_shadcn::tokens::accent_color;
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    states: Vec<CheckboxState>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(usize, CheckboxState),
    Submit,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(index, value) => {
                if let Some(state) = self.states.get_mut(index) {
                    *state = value;
                }
            }
            Message::Submit => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let mut index = 0;
        let mut next_index = || {
            let current = index;
            index += 1;
            current
        };

        let demo_basic_index = next_index();
        let demo_terms_index = next_index();
        let demo_card_index = next_index();

        let demo_section = column![
            row![
                checkbox(
                    self.state_at(demo_basic_index),
                    Some(move |state| Message::Toggle(demo_basic_index, state)),
                    CheckboxProps::new().size(CheckboxSize::Two),
                    theme,
                ),
                label("Accept terms and conditions", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                checkbox(
                    self.state_at(demo_terms_index),
                    Some(move |state| Message::Toggle(demo_terms_index, state)),
                    CheckboxProps::new().size(CheckboxSize::Two),
                    theme,
                ),
                column![
                    label("Accept terms and conditions", theme),
                    muted_text(
                        "By clicking this checkbox, you agree to the terms and conditions.",
                        theme
                    ),
                ]
                .spacing(6),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
            row![
                checkbox(
                    CheckboxState::Unchecked,
                    None::<fn(CheckboxState) -> Message>,
                    CheckboxProps::new()
                        .size(CheckboxSize::Two)
                        .disabled(true),
                    theme,
                ),
                label("Enable notifications", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            {
                let card_state = self.state_at(demo_card_index);
                let card_active = card_state.is_checked();
                let accent = accent_color(&theme.palette, AccentColor::Blue);
                let card_border = if card_active { accent } else { theme.palette.border };
                let card_background = if card_active {
                    apply_opacity(accent, 0.08)
                } else {
                    theme.palette.card
                };

                container(
                    row![
                        checkbox(
                            card_state,
                            Some(move |state| Message::Toggle(demo_card_index, state)),
                            CheckboxProps::new()
                                .size(CheckboxSize::Two)
                                .color(AccentColor::Blue),
                            theme,
                        ),
                        column![
                            iced_text("Enable notifications")
                                .size(14)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(theme.palette.foreground),
                                }),
                            muted_text(
                                "You can enable or disable notifications at any time.",
                                theme
                            ),
                        ]
                        .spacing(6),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Start),
                )
                .padding(12)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(card_background)),
                    border: Border {
                        radius: theme.radius.md.into(),
                        width: 1.0,
                        color: card_border,
                    },
                    ..iced::widget::container::Style::default()
                })
            },
        ]
        .spacing(12);

        let with_text_index = next_index();
        let with_text_section = row![
            checkbox(
                self.state_at(with_text_index),
                Some(move |state| Message::Toggle(with_text_index, state)),
                CheckboxProps::new().size(CheckboxSize::Two),
                theme,
            ),
            column![
                label("Accept terms and conditions", theme),
                muted_text("You agree to our Terms of Service and Privacy Policy.", theme),
            ]
            .spacing(4),
        ]
        .spacing(12)
        .align_y(Alignment::Start);

        let disabled_section = row![
            checkbox(
                CheckboxState::Unchecked,
                None::<fn(CheckboxState) -> Message>,
                CheckboxProps::new()
                    .size(CheckboxSize::Two)
                    .disabled(true),
                theme,
            ),
            label_with_props(
                "Accept terms and conditions",
                LabelProps::new().disabled(true),
                theme,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let form_single_index = next_index();
        let form_single_card = container(
            row![
                checkbox(
                    self.state_at(form_single_index),
                    Some(move |state| Message::Toggle(form_single_index, state)),
                    CheckboxProps::new().size(CheckboxSize::Two),
                    theme,
                ),
                column![
                    label("Use different settings for my mobile devices", theme),
                    muted_text(
                        "You can manage your mobile notifications in the mobile settings page.",
                        theme
                    ),
                ]
                .spacing(6),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
        )
        .padding(16)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.card)),
            border: Border {
                radius: theme.radius.md.into(),
                width: 1.0,
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        });

        let form_single_section = column![
            form_single_card,
            button(
                "Submit",
                Some(Message::Submit),
                ButtonProps::new().size(ButtonSize::Two),
                theme,
            ),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let mut form_items = column![].spacing(8);
        for item in FORM_ITEMS {
            let item_state_index = next_index();
            let checkbox_props = CheckboxProps::new().size(CheckboxSize::Two);
            let item_row = row![
                checkbox(
                    self.state_at(item_state_index),
                    Some(move |state| Message::Toggle(item_state_index, state)),
                    checkbox_props,
                    theme,
                ),
                text(
                    item,
                    TextProps::new()
                        .size(TextSize::Two)
                        .weight(TextWeight::Regular),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            form_items = form_items.push(item_row);
        }

        let form_multiple_section = column![
            text(
                "Sidebar",
                TextProps::new()
                    .size(TextSize::Three)
                    .weight(TextWeight::Medium),
                theme,
            ),
            muted_text("Select the items you want to display in the sidebar.", theme),
            form_items,
            button(
                "Submit",
                Some(Message::Submit),
                ButtonProps::new().size(ButtonSize::Two),
                theme,
            ),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let indeterminate_index = next_index();
        let indeterminate_section = row![
            checkbox(
                self.state_at(indeterminate_index),
                Some(move |state| Message::Toggle(indeterminate_index, state)),
                CheckboxProps::new().size(CheckboxSize::Two),
                theme,
            ),
            label("Select all", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let variant_header = row![
            caption("variant", theme).width(Length::Fixed(140.0)),
            caption("unchecked", theme),
            caption("checked", theme),
            caption("indeterminate", theme),
            caption("disabled", theme),
            caption("disabled checked", theme),
            caption("disabled indet", theme),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let mut variant_rows = column![variant_header].spacing(8);
        for variant in VARIANTS {
            let base_props = CheckboxProps::new().size(CheckboxSize::Two).variant(variant);
            let row_label = caption(variant_label(variant), theme)
                .width(Length::Fixed(140.0));
            let unchecked_index = next_index();
            let checked_index = next_index();
            let indeterminate_index = next_index();
            let row = row![
                row_label,
                checkbox(
                    self.state_at(unchecked_index),
                    Some(move |state| Message::Toggle(unchecked_index, state)),
                    base_props,
                    theme,
                ),
                checkbox(
                    self.state_at(checked_index),
                    Some(move |state| Message::Toggle(checked_index, state)),
                    base_props,
                    theme,
                ),
                checkbox(
                    self.state_at(indeterminate_index),
                    Some(move |state| Message::Toggle(indeterminate_index, state)),
                    base_props,
                    theme,
                ),
                checkbox(
                    CheckboxState::Unchecked,
                    None::<fn(CheckboxState) -> Message>,
                    base_props.disabled(true),
                    theme,
                ),
                checkbox(
                    CheckboxState::Checked,
                    None::<fn(CheckboxState) -> Message>,
                    base_props.disabled(true),
                    theme,
                ),
                checkbox(
                    CheckboxState::Indeterminate,
                    None::<fn(CheckboxState) -> Message>,
                    base_props.disabled(true),
                    theme,
                ),
            ]
            .spacing(16)
            .align_y(Alignment::Center);

            let high_label = caption(
                format!("{} + high contrast", variant_label(variant)),
                theme,
            )
            .width(Length::Fixed(140.0));
            let high_unchecked_index = next_index();
            let high_checked_index = next_index();
            let high_indeterminate_index = next_index();
            let high_row = row![
                high_label,
                checkbox(
                    self.state_at(high_unchecked_index),
                    Some(move |state| Message::Toggle(high_unchecked_index, state)),
                    base_props.high_contrast(true),
                    theme,
                ),
                checkbox(
                    self.state_at(high_checked_index),
                    Some(move |state| Message::Toggle(high_checked_index, state)),
                    base_props.high_contrast(true),
                    theme,
                ),
                checkbox(
                    self.state_at(high_indeterminate_index),
                    Some(move |state| Message::Toggle(high_indeterminate_index, state)),
                    base_props.high_contrast(true),
                    theme,
                ),
                checkbox(
                    CheckboxState::Unchecked,
                    None::<fn(CheckboxState) -> Message>,
                    base_props.high_contrast(true).disabled(true),
                    theme,
                ),
                checkbox(
                    CheckboxState::Checked,
                    None::<fn(CheckboxState) -> Message>,
                    base_props.high_contrast(true).disabled(true),
                    theme,
                ),
                checkbox(
                    CheckboxState::Indeterminate,
                    None::<fn(CheckboxState) -> Message>,
                    base_props.high_contrast(true).disabled(true),
                    theme,
                ),
            ]
            .spacing(16)
            .align_y(Alignment::Center);

            variant_rows = variant_rows.push(row).push(high_row);
        }

        let mut size_rows = column![].spacing(12);
        for size in SIZES {
            let unchecked_index = next_index();
            let checked_index = next_index();
            let row = row![
                caption(size_label(size), theme).width(Length::Fixed(72.0)),
                checkbox(
                    self.state_at(unchecked_index),
                    Some(move |state| Message::Toggle(unchecked_index, state)),
                    CheckboxProps::new().size(size),
                    theme,
                ),
                checkbox(
                    self.state_at(checked_index),
                    Some(move |state| Message::Toggle(checked_index, state)),
                    CheckboxProps::new().size(size),
                    theme,
                ),
            ]
            .spacing(16)
            .align_y(Alignment::Center);

            size_rows = size_rows.push(row);
        }

        let mut color_rows = column![].spacing(12);
        for (group_label, high_contrast) in [("default", false), ("high contrast", true)] {
            let mut group_rows = column![caption(group_label, theme)].spacing(8);
            for chunk in COLORS.chunks(3) {
                let mut row = row![].spacing(16).align_y(Alignment::Center);
                for color in chunk {
                    let color_index = next_index();
                    let mut props = CheckboxProps::new()
                        .size(CheckboxSize::Two)
                        .color(*color);
                    if high_contrast {
                        props = props.high_contrast(true);
                    }
                    row = row.push(
                        row![
                            checkbox(
                                self.state_at(color_index),
                                Some(move |state| Message::Toggle(color_index, state)),
                                props,
                                theme,
                            ),
                            caption(color_label(*color), theme),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    );
                }
                group_rows = group_rows.push(row);
            }
            color_rows = color_rows.push(group_rows);
        }

        let mut alignment_rows = column![].spacing(12);
        for (text_size, checkbox_size) in ALIGNMENT_ROWS {
            let alignment_index = next_index();
            let row = row![
                checkbox(
                    self.state_at(alignment_index),
                    Some(move |state| Message::Toggle(alignment_index, state)),
                    CheckboxProps::new().size(checkbox_size),
                    theme,
                ),
                iced_text("Agree to Terms and Conditions")
                    .size(text_size)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(theme.palette.foreground),
                    }),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            alignment_rows = alignment_rows.push(row);
        }

        let content = column![
            section(theme, "Checkbox Demo", demo_section),
            section(theme, "Checkbox With Text", with_text_section),
            section(theme, "Checkbox Disabled", disabled_section),
            section(theme, "Form (Single)", form_single_section),
            section(theme, "Form (Multiple)", form_multiple_section),
            section(theme, "Indeterminate", indeterminate_section),
            section(theme, "Variants", variant_rows),
            section(theme, "Sizes", size_rows),
            section(theme, "Colors", color_rows),
            section(theme, "Alignment", alignment_rows),
        ]
        .spacing(16);

        let content = scrollable(content).height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .padding(32)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: border,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }

    fn state_at(&self, index: usize) -> CheckboxState {
        self.states
            .get(index)
            .copied()
            .unwrap_or(CheckboxState::Unchecked)
    }
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            states: default_states(),
        }
    }
}

const VARIANTS: [CheckboxVariant; 3] = [
    CheckboxVariant::Classic,
    CheckboxVariant::Surface,
    CheckboxVariant::Soft,
];

const SIZES: [CheckboxSize; 3] = [CheckboxSize::One, CheckboxSize::Two, CheckboxSize::Three];

const COLORS: [AccentColor; 6] = [
    AccentColor::Gray,
    AccentColor::Blue,
    AccentColor::Green,
    AccentColor::Amber,
    AccentColor::Red,
    AccentColor::Purple,
];

const FORM_ITEMS: [&str; 6] = [
    "Recents",
    "Home",
    "Applications",
    "Desktop",
    "Downloads",
    "Documents",
];

const ALIGNMENT_ROWS: [(u32, CheckboxSize); 6] = [
    (12, CheckboxSize::One),
    (14, CheckboxSize::One),
    (14, CheckboxSize::Two),
    (16, CheckboxSize::Two),
    (16, CheckboxSize::Three),
    (18, CheckboxSize::Three),
];

fn default_states() -> Vec<CheckboxState> {
    let mut states = vec![
        CheckboxState::Unchecked,
        CheckboxState::Checked,
        CheckboxState::Checked,
        CheckboxState::Unchecked,
        CheckboxState::Checked,
        CheckboxState::Checked,
        CheckboxState::Checked,
    ];
    for _ in 0..(FORM_ITEMS.len() - 2) {
        states.push(CheckboxState::Unchecked);
    }

    states.push(CheckboxState::Indeterminate);

    for _variant in VARIANTS {
        for _ in 0..2 {
            states.push(CheckboxState::Unchecked);
            states.push(CheckboxState::Checked);
            states.push(CheckboxState::Indeterminate);
        }
    }

    for _ in SIZES {
        states.push(CheckboxState::Unchecked);
        states.push(CheckboxState::Checked);
    }

    for _ in 0..2 {
        for _ in COLORS {
            states.push(CheckboxState::Checked);
        }
    }

    for _ in ALIGNMENT_ROWS {
        states.push(CheckboxState::Unchecked);
    }

    states
}

fn section<'a, Message: 'a>(
    theme: &Theme,
    title: impl iced::widget::text::IntoFragment<'a>,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let title = text(
        title,
        TextProps::new()
            .size(TextSize::Four)
            .weight(TextWeight::Medium),
        theme,
    );
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(column![title, content.into()].spacing(12))
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}

fn muted_text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    let color = theme.palette.muted_foreground;
    iced_text(content)
        .size(13)
        .style(move |_theme| iced::widget::text::Style { color: Some(color) })
}

fn caption<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    let color = theme.palette.muted_foreground;
    iced_text(content)
        .size(12)
        .style(move |_theme| iced::widget::text::Style { color: Some(color) })
}

fn variant_label(variant: CheckboxVariant) -> &'static str {
    match variant {
        CheckboxVariant::Classic => "classic",
        CheckboxVariant::Surface => "surface",
        CheckboxVariant::Soft => "soft",
    }
}

fn size_label(size: CheckboxSize) -> &'static str {
    match size {
        CheckboxSize::One => "size 1",
        CheckboxSize::Two => "size 2",
        CheckboxSize::Three => "size 3",
    }
}

fn color_label(color: AccentColor) -> &'static str {
    match color {
        AccentColor::Gray => "gray",
        AccentColor::Blue => "blue",
        AccentColor::Green => "green",
        AccentColor::Amber => "amber",
        AccentColor::Red => "red",
        AccentColor::Purple => "purple",
        _ => "accent",
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}
