use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonRadius, SliderProps, SliderSize, SliderVariant, TextProps, TextSize,
    TextWeight, Theme, slider, text, vertical_slider,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    values: Vec<Vec<f32>>,
}

#[derive(Debug, Clone)]
enum Message {
    Changed(usize, Vec<f32>),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Changed(index, value) => {
                if let Some(slot) = self.values.get_mut(index) {
                    *slot = value;
                }
            }
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

        let demo_index = next_index();
        let demo_slider = slider(
            0.0..=100.0,
            self.values[demo_index].clone(),
            Some(move |value| Message::Changed(demo_index, value)),
            SliderProps::new(),
            theme,
        )
        .width(Length::Fixed(360.0));
        let demo_section = column![demo_slider].spacing(8);

        let range_index = next_index();
        let range_values = &self.values[range_index];
        let low = range_values.first().copied().unwrap_or(0.0);
        let high = range_values.get(1).copied().unwrap_or(0.0);
        let range_slider = slider(
            0.0..=1000.0,
            range_values.clone(),
            Some(move |value| Message::Changed(range_index, value)),
            SliderProps::new(),
            theme,
        )
        .step(10.0)
        .min_steps_between_thumbs(1)
        .width(Length::Fill);
        let range_section = column![
            text(
                "Price Range",
                TextProps::new()
                    .size(TextSize::Three)
                    .weight(TextWeight::Medium),
                theme
            ),
            muted_text(
                format!(
                    "Set your budget range (${} - {}).",
                    low.round() as i32,
                    high.round() as i32
                ),
                theme
            ),
            range_slider
        ]
        .spacing(8);

        let mut variant_rows = column![].spacing(12);
        for variant in VARIANTS {
            let default_index = next_index();
            let high_index = next_index();
            let disabled_index = next_index();
            let base_props = SliderProps::new().variant(variant);
            let row = row![
                caption(variant_label(variant), theme).width(Length::Fixed(72.0)),
                slider(
                    0.0..=100.0,
                    self.values[default_index].clone(),
                    Some(move |value| Message::Changed(default_index, value)),
                    base_props,
                    theme
                )
                .width(Length::Fixed(180.0)),
                slider(
                    0.0..=100.0,
                    self.values[high_index].clone(),
                    Some(move |value| Message::Changed(high_index, value)),
                    base_props.high_contrast(true),
                    theme
                )
                .width(Length::Fixed(180.0)),
                slider(
                    0.0..=100.0,
                    self.values[disabled_index].clone(),
                    None::<fn(Vec<f32>) -> Message>,
                    base_props.disabled(true),
                    theme
                )
                .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            variant_rows = variant_rows.push(row);
        }

        let mut size_rows = column![].spacing(12);
        for size in SIZES {
            let size_index = next_index();
            let row = row![
                caption(size_label(size), theme).width(Length::Fixed(72.0)),
                slider(
                    0.0..=100.0,
                    self.values[size_index].clone(),
                    Some(move |value| Message::Changed(size_index, value)),
                    SliderProps::new().size(size),
                    theme
                )
                .width(Length::Fixed(300.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            size_rows = size_rows.push(row);
        }

        let mut vertical_row = row![].spacing(16).align_y(Alignment::End);
        for size in SIZES {
            let v_index = next_index();
            let slider = vertical_slider(
                0.0..=100.0,
                self.values[v_index].clone(),
                Some(move |value| Message::Changed(v_index, value)),
                SliderProps::new().size(size),
                theme,
            )
            .height(Length::Fixed(160.0));
            vertical_row = vertical_row.push(slider);
        }

        let mut radius_rows = column![].spacing(12);
        for radius in RADII {
            let radius_index = next_index();
            let row = row![
                caption(radius_label(radius), theme).width(Length::Fixed(72.0)),
                slider(
                    0.0..=100.0,
                    self.values[radius_index].clone(),
                    Some(move |value| Message::Changed(radius_index, value)),
                    SliderProps::new().radius(radius),
                    theme
                )
                .width(Length::Fixed(300.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            radius_rows = radius_rows.push(row);
        }

        let mut color_rows = column![].spacing(12);
        for chunk in COLORS.chunks(3) {
            let mut row = row![].spacing(12).align_y(Alignment::Center);
            for color in chunk {
                let color_index = next_index();
                row = row.push(
                    column![
                        caption(color_label(*color), theme),
                        slider(
                            0.0..=100.0,
                            self.values[color_index].clone(),
                            Some(move |value| Message::Changed(color_index, value)),
                            SliderProps::new().color(*color),
                            theme
                        )
                        .width(Length::Fixed(160.0)),
                    ]
                    .spacing(6),
                );
            }
            color_rows = color_rows.push(row);
        }

        let content = column![
            section(theme, "Slider Demo", demo_section),
            section(theme, "Range Slider", range_section),
            section(theme, "Variants", variant_rows),
            section(theme, "Sizes", size_rows),
            section(theme, "Vertical", vertical_row),
            section(theme, "Radius", radius_rows),
            section(theme, "Colors", color_rows),
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
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            values: default_values(),
        }
    }
}

const VARIANTS: [SliderVariant; 3] = [
    SliderVariant::Classic,
    SliderVariant::Surface,
    SliderVariant::Soft,
];

const SIZES: [SliderSize; 3] = [SliderSize::One, SliderSize::Two, SliderSize::Three];

const RADII: [ButtonRadius; 5] = [
    ButtonRadius::None,
    ButtonRadius::Small,
    ButtonRadius::Medium,
    ButtonRadius::Large,
    ButtonRadius::Full,
];

const COLORS: [AccentColor; 6] = [
    AccentColor::Gray,
    AccentColor::Blue,
    AccentColor::Green,
    AccentColor::Amber,
    AccentColor::Red,
    AccentColor::Purple,
];

fn default_values() -> Vec<Vec<f32>> {
    let mut values = Vec::new();
    values.push(vec![50.0]);
    values.push(vec![200.0, 800.0]);

    for (index, _) in VARIANTS.iter().enumerate() {
        let base = 30.0 + index as f32 * 10.0;
        values.push(vec![base]);
        values.push(vec![base]);
        values.push(vec![base]);
    }

    for (index, _) in SIZES.iter().enumerate() {
        values.push(vec![20.0 + index as f32 * 15.0]);
    }

    for (index, _) in SIZES.iter().enumerate() {
        values.push(vec![40.0 + index as f32 * 10.0]);
    }

    for (index, _) in RADII.iter().enumerate() {
        values.push(vec![25.0 + index as f32 * 10.0]);
    }

    for (index, _) in COLORS.iter().enumerate() {
        values.push(vec![15.0 + index as f32 * 12.0]);
    }

    values
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

fn variant_label(variant: SliderVariant) -> &'static str {
    match variant {
        SliderVariant::Classic => "classic",
        SliderVariant::Surface => "surface",
        SliderVariant::Soft => "soft",
    }
}

fn size_label(size: SliderSize) -> &'static str {
    match size {
        SliderSize::One => "size 1",
        SliderSize::Two => "size 2",
        SliderSize::Three => "size 3",
    }
}

fn radius_label(radius: ButtonRadius) -> &'static str {
    match radius {
        ButtonRadius::None => "radius 0",
        ButtonRadius::Small => "radius sm",
        ButtonRadius::Medium => "radius md",
        ButtonRadius::Large => "radius lg",
        ButtonRadius::Full => "radius full",
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
