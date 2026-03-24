use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Color, Element, Length};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::tokens::accent_color;
use iced_shadcn::{
    AccentColor, ButtonProps, CheckboxCycle, CheckboxProps, CheckboxSize, CheckboxState,
    CheckboxVariant, TextProps, TextSize, TextWeight, Theme, button, checkbox, label, text,
};

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

    fn state_at(&self, index: usize) -> CheckboxState {
        self.states
            .get(index)
            .copied()
            .unwrap_or(CheckboxState::Unchecked)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border_color = theme.palette.border;
        let radius = theme.radius.md;

        let mut index = 0;
        let mut next_index = || {
            let current = index;
            index += 1;
            current
        };

        // Demo — basic checkbox
        let demo_idx = next_index();
        let demo_section = row![
            checkbox(
                self.state_at(demo_idx),
                Some(move |state| Message::Toggle(demo_idx, state)),
                CheckboxProps::new().size(CheckboxSize::Size2),
                theme,
            ),
            label("Accept terms and conditions", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        // With Text — checkbox + label + description
        let with_text_idx = next_index();
        let with_text_section = row![
            checkbox(
                self.state_at(with_text_idx),
                Some(move |state| Message::Toggle(with_text_idx, state)),
                CheckboxProps::new().size(CheckboxSize::Size2),
                theme,
            ),
            column![
                label("Accept terms and conditions", theme),
                muted_text(
                    "By clicking this checkbox, you agree to the terms and conditions.",
                    theme
                ),
            ]
            .spacing(4),
        ]
        .spacing(12)
        .align_y(Alignment::Start);

        // Disabled
        let disabled_section = row![
            row![
                checkbox(
                    CheckboxState::Unchecked,
                    None::<fn(CheckboxState) -> Message>,
                    CheckboxProps::new()
                        .size(CheckboxSize::Size2)
                        .disabled(true),
                    theme,
                ),
                muted_text("Unchecked disabled", theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            row![
                checkbox(
                    CheckboxState::Checked,
                    None::<fn(CheckboxState) -> Message>,
                    CheckboxProps::new()
                        .size(CheckboxSize::Size2)
                        .disabled(true),
                    theme,
                ),
                muted_text("Checked disabled", theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(24)
        .align_y(Alignment::Center);

        // Indeterminate
        let indet_idx = next_index();
        let indeterminate_section = row![
            checkbox(
                self.state_at(indet_idx),
                Some(move |state| Message::Toggle(indet_idx, state)),
                CheckboxProps::new()
                    .size(CheckboxSize::Size2)
                    .cycle(CheckboxCycle::TriState),
                theme,
            ),
            label("Select all items", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        // Card
        let card_idx = next_index();
        let card_state = self.state_at(card_idx);
        let card_active = card_state.is_checked();
        let accent = accent_color(&theme.palette, AccentColor::Blue);
        let card_border = if card_active {
            accent
        } else {
            theme.palette.border
        };
        let card_background = if card_active {
            apply_opacity(accent, 0.08)
        } else {
            theme.palette.card
        };
        let card_section = container(
            row![
                checkbox(
                    card_state,
                    Some(move |state| Message::Toggle(card_idx, state)),
                    CheckboxProps::new()
                        .size(CheckboxSize::Size2)
                        .color(AccentColor::Blue),
                    theme,
                ),
                column![
                    iced_text("Enable notifications").size(14).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(theme.palette.foreground),
                        }
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
        .width(Length::Fixed(400.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(card_background)),
            border: Border {
                radius: theme.radius.md.into(),
                width: 1.0,
                color: card_border,
            },
            ..iced::widget::container::Style::default()
        });

        // Variants grid
        let variant_header = row![
            container(caption("Variant", theme)).width(Length::Fixed(80.0)),
            container(caption("Default", theme)).width(Length::Fixed(60.0)),
            container(caption("High Contrast", theme)).width(Length::Fixed(100.0)),
            container(caption("Disabled", theme)).width(Length::Fixed(80.0)),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let mut variant_rows = column![variant_header].spacing(8);
        for variant in VARIANTS {
            let def_idx = next_index();
            let hc_idx = next_index();
            let dis_idx = next_index();

            let row_el = row![
                container(caption(variant_label(variant), theme)).width(Length::Fixed(80.0)),
                container(checkbox(
                    self.state_at(def_idx),
                    Some(move |state| Message::Toggle(def_idx, state)),
                    CheckboxProps::new()
                        .size(CheckboxSize::Size2)
                        .variant(variant),
                    theme,
                ))
                .width(Length::Fixed(60.0)),
                container(checkbox(
                    self.state_at(hc_idx),
                    Some(move |state| Message::Toggle(hc_idx, state)),
                    CheckboxProps::new()
                        .size(CheckboxSize::Size2)
                        .variant(variant)
                        .high_contrast(true),
                    theme,
                ))
                .width(Length::Fixed(100.0)),
                container(checkbox(
                    self.state_at(dis_idx),
                    None::<fn(CheckboxState) -> Message>,
                    CheckboxProps::new()
                        .size(CheckboxSize::Size2)
                        .variant(variant)
                        .disabled(true),
                    theme,
                ))
                .width(Length::Fixed(80.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            variant_rows = variant_rows.push(row_el);
        }

        // Sizes
        let mut sizes_el = row![].spacing(24).align_y(Alignment::Center);
        for size in SIZES {
            let s_idx = next_index();
            sizes_el = sizes_el.push(
                row![
                    checkbox(
                        self.state_at(s_idx),
                        Some(move |state| Message::Toggle(s_idx, state)),
                        CheckboxProps::new().size(size),
                        theme,
                    ),
                    caption(size_label(size), theme),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            );
        }

        // Colors
        let mut colors_el = row![].spacing(16).align_y(Alignment::Center);
        for color in COLORS {
            let c_idx = next_index();
            colors_el = colors_el.push(
                row![
                    checkbox(
                        self.state_at(c_idx),
                        Some(move |state| Message::Toggle(c_idx, state)),
                        CheckboxProps::new().size(CheckboxSize::Size2).color(color),
                        theme,
                    ),
                    caption(color_label(color), theme),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            );
        }

        // Form
        let form_idx = next_index();
        let form_section = column![
            row![
                checkbox(
                    self.state_at(form_idx),
                    Some(move |state| Message::Toggle(form_idx, state)),
                    CheckboxProps::new().size(CheckboxSize::Size2),
                    theme,
                ),
                column![
                    label("Accept terms and conditions", theme),
                    muted_text(
                        "You agree to our Terms of Service and Privacy Policy.",
                        theme
                    ),
                ]
                .spacing(4),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
            button("Submit", Some(Message::Submit), ButtonProps::new(), theme),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content = column![
            section(theme, "Demo", demo_section),
            section(theme, "With Text", with_text_section),
            section(theme, "Disabled", disabled_section),
            section(theme, "Indeterminate", indeterminate_section),
            section(theme, "Card", card_section),
            section(theme, "Variants", variant_rows),
            section(theme, "Sizes", sizes_el),
            section(theme, "Colors", colors_el),
            section(theme, "Form", form_section),
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
                    color: border_color,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

const VARIANTS: [CheckboxVariant; 3] = [
    CheckboxVariant::Surface,
    CheckboxVariant::Classic,
    CheckboxVariant::Soft,
];

const SIZES: [CheckboxSize; 3] = [
    CheckboxSize::Size1,
    CheckboxSize::Size2,
    CheckboxSize::Size3,
];

const COLORS: [AccentColor; 6] = [
    AccentColor::Blue,
    AccentColor::Green,
    AccentColor::Amber,
    AccentColor::Red,
    AccentColor::Purple,
    AccentColor::Gray,
];

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            states: default_states(),
        }
    }
}

fn default_states() -> Vec<CheckboxState> {
    let mut states = vec![
        // demo
        CheckboxState::Unchecked,
        // with text
        CheckboxState::Checked,
        // indeterminate
        CheckboxState::Indeterminate,
        // card
        CheckboxState::Checked,
    ];
    // variants: 3 variants × 3 columns (default, hc, disabled)
    for _ in VARIANTS {
        states.extend([
            CheckboxState::Unchecked,
            CheckboxState::Checked,
            CheckboxState::Unchecked,
        ]);
    }
    // sizes: 3
    for _ in SIZES {
        states.push(CheckboxState::Checked);
    }
    // colors: 6
    for _ in COLORS {
        states.push(CheckboxState::Checked);
    }
    // form
    states.push(CheckboxState::Unchecked);
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
            .size(TextSize::Size4)
            .weight(TextWeight::Medium),
        theme,
    );
    let bg = theme.palette.card;
    let border_c = theme.palette.border;
    let r = theme.radius.md;

    container(column![title, content.into()].spacing(12))
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: Border {
                radius: r.into(),
                width: 1.0,
                color: border_c,
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
        CheckboxVariant::Surface => "Surface",
        CheckboxVariant::Classic => "Classic",
        CheckboxVariant::Soft => "Soft",
    }
}

fn size_label(size: CheckboxSize) -> &'static str {
    match size {
        CheckboxSize::Size1 => "Size 1",
        CheckboxSize::Size2 => "Size 2",
        CheckboxSize::Size3 => "Size 3",
    }
}

fn color_label(color: AccentColor) -> &'static str {
    match color {
        AccentColor::Blue => "Blue",
        AccentColor::Green => "Green",
        AccentColor::Amber => "Amber",
        AccentColor::Red => "Red",
        AccentColor::Purple => "Purple",
        AccentColor::Gray => "Gray",
        _ => "Other",
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: opacity,
        ..color
    }
}
