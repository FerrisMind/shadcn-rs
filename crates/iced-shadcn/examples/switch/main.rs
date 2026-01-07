use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonRadius, ButtonVariant, SwitchProps, SwitchSize, SwitchVariant,
    TextProps, TextSize, TextWeight, Theme, button, label, switch, text,
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

        let demo_content = column![
            row![
                make_switch(next_index(), SwitchProps::new().size(SwitchSize::Two)),
                label("Airplane Mode", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                make_switch(next_index(), SwitchProps::new().size(SwitchSize::Two)),
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

        let form_header = column![
            text(
                "Email Notifications",
                TextProps::new()
                    .size(TextSize::Three)
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
                    make_switch(next_index(), SwitchProps::new().size(SwitchSize::Two)),
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
                        SwitchProps::new().size(SwitchSize::Two).disabled(true),
                    ),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                theme,
            ),
        ]
        .spacing(12);

        let form_actions = row![button(
            "Submit",
            Some(Message::Noop),
            ButtonProps::new(),
            theme
        )];

        let form_content = column![form_header, form_items, form_actions]
            .spacing(12)
            .align_x(Alignment::Start);

        let field_content = row![
            container(column![
                label("Multi-factor authentication", theme),
                muted_text(
                    "Enable multi-factor authentication. If you do not have a two-factor device, \
you can use a one-time code sent to your email.",
                    theme
                ),
            ])
            .width(Length::Fill),
            make_switch(next_index(), SwitchProps::new().size(SwitchSize::Two)),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let rhf_header = column![
            text(
                "Security Settings",
                TextProps::new()
                    .size(TextSize::Three)
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
                    make_switch(next_index(), SwitchProps::new().size(SwitchSize::Two)),
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

        let tanstack_header = column![
            text(
                "Security Settings",
                TextProps::new()
                    .size(TextSize::Three)
                    .weight(TextWeight::Medium),
                theme
            ),
            muted_text(
                "Enable multi-factor authentication to secure your account.",
                theme
            ),
        ]
        .spacing(4);

        let tanstack_content = column![
            tanstack_header,
            form_item(
                row![
                    container(column![
                        label("Multi-factor authentication", theme),
                        muted_text(
                            "Receive a one-time code if you do not have a two-factor device.",
                            theme
                        ),
                    ])
                    .width(Length::Fill),
                    make_switch(next_index(), SwitchProps::new().size(SwitchSize::Two)),
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

        let variants_header = row![
            container(caption("Variant", theme)).width(Length::Fixed(160.0)),
            caption("Off", theme),
            caption("On", theme),
            caption("Disabled", theme),
            caption("Disabled On", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let mut variant_rows = Vec::new();
        for variant in VARIANTS {
            for high_contrast in [false, true] {
                let mut title = variant_label(variant).to_string();
                if high_contrast {
                    title.push_str(" + high contrast");
                }
                let row = row![
                    container(caption(title, theme)).width(Length::Fixed(160.0)),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Two)
                            .variant(variant)
                            .high_contrast(high_contrast),
                    ),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Two)
                            .variant(variant)
                            .high_contrast(high_contrast),
                    ),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Two)
                            .variant(variant)
                            .high_contrast(high_contrast)
                            .disabled(true),
                    ),
                    make_switch(
                        next_index(),
                        SwitchProps::new()
                            .size(SwitchSize::Two)
                            .variant(variant)
                            .high_contrast(high_contrast)
                            .disabled(true),
                    ),
                ]
                .spacing(12)
                .align_y(Alignment::Center);
                variant_rows.push(row);
            }
        }

        let variants_content = column![
            variants_header,
            column(variant_rows.into_iter().map(|row| row.into())).spacing(8)
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let size_rows = SIZES.iter().map(|size| {
            row![
                container(caption(format!("Size {}", size_label(*size)), theme))
                    .width(Length::Fixed(120.0)),
                make_switch(next_index(), SwitchProps::new().size(*size)),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        });

        let sizes_content = column(size_rows).spacing(8).align_x(Alignment::Start);

        let alignment_rows = ALIGNMENT_ITEMS.iter().map(|(size, text_size)| {
            row![
                make_switch(next_index(), SwitchProps::new().size(*size)),
                text(
                    "Agree to Terms and Conditions",
                    TextProps::new().size(*text_size),
                    theme
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        });

        let alignment_content = column(alignment_rows).spacing(8).align_x(Alignment::Start);

        let radius_header = row![
            container(caption("Radius", theme)).width(Length::Fixed(120.0)),
            caption("Size 1", theme),
            caption("Size 2", theme),
            caption("Size 3", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let radius_rows = RADII.iter().map(|radius| {
            row![
                container(caption(radius_label(*radius), theme)).width(Length::Fixed(120.0)),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::One).radius(*radius),
                ),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::Two).radius(*radius),
                ),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::Three).radius(*radius),
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        });

        let radius_content = column![radius_header, column(radius_rows).spacing(8)].spacing(12);

        let colors_header = row![
            container(caption("Color", theme)).width(Length::Fixed(120.0)),
            caption("Off", theme),
            caption("On", theme),
            caption("High Contrast", theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let color_rows = COLORS.iter().map(|color| {
            row![
                container(caption(color_label(*color), theme)).width(Length::Fixed(120.0)),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::Two).color(*color),
                ),
                make_switch(
                    next_index(),
                    SwitchProps::new().size(SwitchSize::Two).color(*color),
                ),
                make_switch(
                    next_index(),
                    SwitchProps::new()
                        .size(SwitchSize::Two)
                        .color(*color)
                        .high_contrast(true),
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        });

        let colors_content = column![
            colors_header,
            column(color_rows.map(|row| row.into())).spacing(8)
        ]
        .spacing(12);

        let content = column![
            section(theme, "Demo", demo_content),
            section(theme, "Switch Form", form_content),
            section(theme, "Field", field_content),
            section(theme, "Form (React Hook Form)", rhf_content),
            section(theme, "Form (Tanstack)", tanstack_content),
            section(theme, "Variants", variants_content),
            section(theme, "Sizes", sizes_content),
            section(theme, "Alignment", alignment_content),
            section(theme, "Radius", radius_content),
            section(theme, "Colors", colors_content),
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

const SIZES: [SwitchSize; 3] = [SwitchSize::One, SwitchSize::Two, SwitchSize::Three];

const RADII: [ButtonRadius; 5] = [
    ButtonRadius::None,
    ButtonRadius::Small,
    ButtonRadius::Medium,
    ButtonRadius::Large,
    ButtonRadius::Full,
];

const COLORS: [AccentColor; 26] = [
    AccentColor::Gray,
    AccentColor::Gold,
    AccentColor::Bronze,
    AccentColor::Brown,
    AccentColor::Yellow,
    AccentColor::Amber,
    AccentColor::Orange,
    AccentColor::Tomato,
    AccentColor::Red,
    AccentColor::Ruby,
    AccentColor::Crimson,
    AccentColor::Pink,
    AccentColor::Plum,
    AccentColor::Purple,
    AccentColor::Violet,
    AccentColor::Iris,
    AccentColor::Indigo,
    AccentColor::Blue,
    AccentColor::Cyan,
    AccentColor::Teal,
    AccentColor::Jade,
    AccentColor::Green,
    AccentColor::Grass,
    AccentColor::Lime,
    AccentColor::Mint,
    AccentColor::Sky,
];

const ALIGNMENT_ITEMS: [(SwitchSize, TextSize); 6] = [
    (SwitchSize::One, TextSize::One),
    (SwitchSize::One, TextSize::Two),
    (SwitchSize::Two, TextSize::Two),
    (SwitchSize::Two, TextSize::Three),
    (SwitchSize::Three, TextSize::Three),
    (SwitchSize::Three, TextSize::Four),
];

fn default_states() -> Vec<bool> {
    let mut states = Vec::new();

    states.push(false);
    states.push(true);
    states.push(false);
    states.push(true);
    states.push(false);
    states.push(false);
    states.push(false);

    for _variant in VARIANTS {
        for _high_contrast in [false, true] {
            states.extend([false, true, false, true]);
        }
    }

    for size in SIZES {
        states.push(matches!(size, SwitchSize::Two));
    }

    for (index, _item) in ALIGNMENT_ITEMS.iter().enumerate() {
        states.push(index % 2 == 1);
    }

    for _radius in RADII {
        for size in SIZES {
            states.push(matches!(size, SwitchSize::Two));
        }
    }

    for _color in COLORS {
        states.push(false);
        states.push(true);
        states.push(true);
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

fn form_item<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.background;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(12)
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

fn variant_label(variant: SwitchVariant) -> &'static str {
    match variant {
        SwitchVariant::Classic => "classic",
        SwitchVariant::Surface => "surface",
        SwitchVariant::Soft => "soft",
    }
}

fn size_label(size: SwitchSize) -> &'static str {
    match size {
        SwitchSize::One => "1",
        SwitchSize::Two => "2",
        SwitchSize::Three => "3",
    }
}

fn radius_label(radius: ButtonRadius) -> &'static str {
    match radius {
        ButtonRadius::None => "none",
        ButtonRadius::Small => "small",
        ButtonRadius::Medium => "medium",
        ButtonRadius::Large => "large",
        ButtonRadius::Full => "full",
    }
}

fn color_label(color: AccentColor) -> &'static str {
    match color {
        AccentColor::Gray => "gray",
        AccentColor::Gold => "gold",
        AccentColor::Bronze => "bronze",
        AccentColor::Brown => "brown",
        AccentColor::Yellow => "yellow",
        AccentColor::Amber => "amber",
        AccentColor::Orange => "orange",
        AccentColor::Tomato => "tomato",
        AccentColor::Red => "red",
        AccentColor::Ruby => "ruby",
        AccentColor::Crimson => "crimson",
        AccentColor::Pink => "pink",
        AccentColor::Plum => "plum",
        AccentColor::Purple => "purple",
        AccentColor::Violet => "violet",
        AccentColor::Iris => "iris",
        AccentColor::Indigo => "indigo",
        AccentColor::Blue => "blue",
        AccentColor::Cyan => "cyan",
        AccentColor::Teal => "teal",
        AccentColor::Jade => "jade",
        AccentColor::Green => "green",
        AccentColor::Grass => "grass",
        AccentColor::Lime => "lime",
        AccentColor::Mint => "mint",
        AccentColor::Sky => "sky",
    }
}
