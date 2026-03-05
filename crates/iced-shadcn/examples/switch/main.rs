use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonVariant, SwitchProps, SwitchSize, SwitchVariant, TextProps,
    TextSize, TextWeight, Theme, button, label, switch, text,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    states: Vec<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(usize, bool),
    Noop,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(index, value) => {
                if let Some(state) = self.states.get_mut(index) {
                    *state = value;
                }
            }
            Message::Noop => {}
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

        let make_switch = |index: usize, props: SwitchProps| {
            switch(
                self.states[index],
                Some(move |value| Message::Toggle(index, value)),
                props,
                theme,
            )
        };

        // Demo — basic + with text
        let demo_content = column![
            row![
                make_switch(next_index(), SwitchProps::new().size(SwitchSize::Size2)),
                label("Airplane Mode", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                make_switch(next_index(), SwitchProps::new().size(SwitchSize::Size2)),
                column![
                    label("Email Notifications", theme),
                    muted_text("Receive email updates about your account activity.", theme),
                ]
                .spacing(4),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
        ]
        .spacing(16)
        .align_x(Alignment::Start);

        // Switch Form — email notifications card form
        let form_header = column![
            text(
                "Email Notifications",
                TextProps::new()
                    .size(TextSize::Size3)
                    .weight(TextWeight::Medium),
                theme
            ),
            muted_text("Manage your email preferences.", theme),
        ]
        .spacing(4);

        let form_items = column![
            form_item(
                row![
                    container(column![
                        label("Marketing emails", theme),
                        muted_text(
                            "Receive emails about new products, features, and more.",
                            theme
                        ),
                    ])
                    .width(Length::Fill),
                    make_switch(next_index(), SwitchProps::new().size(SwitchSize::Size2)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                theme,
            ),
            form_item(
                row![
                    container(column![
                        label("Security emails", theme),
                        muted_text("Receive emails about your account security.", theme),
                    ])
                    .width(Length::Fill),
                    make_switch(
                        next_index(),
                        SwitchProps::new().size(SwitchSize::Size2).disabled(true),
                    ),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                theme,
            ),
        ]
        .spacing(12);

        let form_content = column![
            form_header,
            form_items,
            row![button(
                "Submit",
                Some(Message::Noop),
                ButtonProps::new(),
                theme
            )]
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        // Variants grid
        let variant_header = row![
            container(caption("Variant", theme)).width(Length::Fixed(120.0)),
            caption("Off", theme),
            caption("On", theme),
            caption("Disabled Off", theme),
            caption("Disabled On", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let mut variant_rows: Vec<Element<'_, Message>> = Vec::new();
        for variant in VARIANTS {
            for high_contrast in [false, true] {
                let hc_label = if high_contrast {
                    format!("{} + HC", variant_label(variant))
                } else {
                    variant_label(variant).to_string()
                };

                let row_el = row![
                    container(caption(hc_label, theme)).width(Length::Fixed(120.0)),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Size2)
                            .variant(variant)
                            .high_contrast(high_contrast),
                    ),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Size2)
                            .variant(variant)
                            .high_contrast(high_contrast),
                    ),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Size2)
                            .variant(variant)
                            .high_contrast(high_contrast)
                            .disabled(true),
                    ),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Size2)
                            .variant(variant)
                            .high_contrast(high_contrast)
                            .disabled(true),
                    ),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                variant_rows.push(row_el.into());
            }
        }

        let variants_content = column![variant_header, column(variant_rows).spacing(8)].spacing(12);

        // Sizes
        let mut sizes_rows: Vec<Element<'_, Message>> = Vec::new();
        for size in SIZES {
            let row_el = row![
                container(caption(size_label(size), theme)).width(Length::Fixed(80.0)),
                make_switch(next_index(), SwitchProps::new().size(size)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            sizes_rows.push(row_el.into());
        }
        let sizes_content = column(sizes_rows).spacing(8);

        // Colors
        let colors_header = row![
            container(caption("Color", theme)).width(Length::Fixed(120.0)),
            caption("Off", theme),
            caption("On", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let mut color_rows: Vec<Element<'_, Message>> = Vec::new();
        for color in EXAMPLE_COLORS {
            let row_el = row![
                container(caption(color_label(color), theme)).width(Length::Fixed(120.0)),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::Size2).color(color),
                ),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::Size2).color(color),
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            color_rows.push(row_el.into());
        }
        let colors_content = column![colors_header, column(color_rows).spacing(8)].spacing(12);

        // Form (React Hook Form)
        let rhf_header = column![
            text(
                "Security Settings",
                TextProps::new()
                    .size(TextSize::Size3)
                    .weight(TextWeight::Medium),
                theme
            ),
            muted_text("Manage your account security preferences.", theme),
        ]
        .spacing(4);

        let rhf_content = column![
            rhf_header,
            form_item(
                row![
                    container(column![
                        label("Multi-factor authentication", theme),
                        muted_text(
                            "Enable multi-factor authentication to secure your account.",
                            theme
                        ),
                    ])
                    .width(Length::Fill),
                    make_switch(next_index(), SwitchProps::new().size(SwitchSize::Size2)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                theme,
            ),
            row![
                button(
                    "Reset",
                    Some(Message::Noop),
                    ButtonProps::new().variant(ButtonVariant::Outline),
                    theme
                ),
                button("Save", Some(Message::Noop), ButtonProps::new(), theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content = column![
            section(theme, "Demo", demo_content),
            section(theme, "Switch Form", form_content),
            section(theme, "Variants", variants_content),
            section(theme, "Sizes", sizes_content),
            section(theme, "Colors", colors_content),
            section(theme, "Form", rhf_content),
        ]
        .spacing(24)
        .align_x(Alignment::Start);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(24)
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
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            states: default_states(),
        }
    }
}

const VARIANTS: [SwitchVariant; 3] = [
    SwitchVariant::Classic,
    SwitchVariant::Surface,
    SwitchVariant::Soft,
];

const SIZES: [SwitchSize; 3] = [SwitchSize::Size1, SwitchSize::Size2, SwitchSize::Size3];

const EXAMPLE_COLORS: [AccentColor; 6] = [
    AccentColor::Blue,
    AccentColor::Green,
    AccentColor::Amber,
    AccentColor::Red,
    AccentColor::Purple,
    AccentColor::Gray,
];

fn default_states() -> Vec<bool> {
    let mut states = vec![
        false, true, // demo: airplane + email
        false, true, // switch form: marketing + security
    ];

    // variants: 3 × 2 HC × 4 states = 24
    for _variant in VARIANTS {
        for _high_contrast in [false, true] {
            states.extend([false, true, false, true]);
        }
    }

    // sizes: 3
    for size in SIZES {
        states.push(matches!(size, SwitchSize::Size2));
    }

    // colors: 6 × 2 (off, on) = 12
    for _ in EXAMPLE_COLORS {
        states.push(false);
        states.push(true);
    }

    // rhf form: 1
    states.push(false);

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

fn form_item<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let bg = theme.palette.background;
    let border_c = theme.palette.border;
    let r = theme.radius.md;

    container(content)
        .padding(12)
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

fn variant_label(variant: SwitchVariant) -> &'static str {
    match variant {
        SwitchVariant::Classic => "Classic",
        SwitchVariant::Surface => "Surface",
        SwitchVariant::Soft => "Soft",
    }
}

fn size_label(size: SwitchSize) -> &'static str {
    match size {
        SwitchSize::Size1 => "Size 1",
        SwitchSize::Size2 => "Size 2",
        SwitchSize::Size3 => "Size 3",
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
