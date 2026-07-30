//! Interactive playground for `iced-shadcn-v2::Resizable`.
//!
//! Mirrors the shadcn-svelte resizable examples (horizontal, vertical, with
//! handle, nested, controlled) and the theme knobs from the button example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example resizable`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, RadiusId, ResizableDirection,
    ResizableHandle, ResizableLayout, ResizablePane, ResizablePaneGroup, ResizableRadius, StyleId,
    Theme, ThemeMode, fonts, iced_font,
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
    horizontal_sizes: Vec<f32>,
    vertical_sizes: Vec<f32>,
    with_handle_sizes: Vec<f32>,
    nested_sizes: Vec<f32>,
    nested_inner_sizes: Vec<f32>,
    controlled_sizes: Vec<f32>,
    dragging: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    HorizontalLayout(ResizableLayout),
    VerticalLayout(ResizableLayout),
    WithHandleLayout(ResizableLayout),
    NestedLayout(ResizableLayout),
    NestedInnerLayout(ResizableLayout),
    ControlledLayout(ResizableLayout),
    Dragging(bool),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Nova),
            horizontal_sizes: vec![25.0, 75.0],
            vertical_sizes: vec![25.0, 75.0],
            with_handle_sizes: vec![25.0, 75.0],
            nested_sizes: vec![50.0, 50.0],
            nested_inner_sizes: vec![25.0, 75.0],
            controlled_sizes: vec![30.0, 70.0],
            dragging: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Resizable".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::HorizontalLayout(layout) => {
                self.horizontal_sizes = layout.into_inner();
            }
            Message::VerticalLayout(layout) => {
                self.vertical_sizes = layout.into_inner();
            }
            Message::WithHandleLayout(layout) => {
                self.with_handle_sizes = layout.into_inner();
            }
            Message::NestedLayout(layout) => {
                self.nested_sizes = layout.into_inner();
            }
            Message::NestedInnerLayout(layout) => {
                self.nested_inner_sizes = layout.into_inner();
            }
            Message::ControlledLayout(layout) => {
                self.controlled_sizes = layout.into_inner();
            }
            Message::Dragging(dragging) => {
                self.dragging = dragging;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                palette.muted_foreground,
                theme.font_pack()
            ),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme
            ),
            control_select(
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "dragging handle: {} · radius lg={:.0}px",
                self.dragging,
                theme.radius_scale().lg_px,
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(8);

        let horizontal = example_group(
            "Horizontal",
            theme,
            ResizableDirection::Horizontal,
            &self.horizontal_sizes,
            Message::HorizontalLayout,
            false,
        );

        let vertical = example_group(
            "Vertical",
            theme,
            ResizableDirection::Vertical,
            &self.vertical_sizes,
            Message::VerticalLayout,
            false,
        );

        let with_handle = example_group(
            "With Handle",
            theme,
            ResizableDirection::Horizontal,
            &self.with_handle_sizes,
            Message::WithHandleLayout,
            true,
        );

        let nested = nested_example(theme, &self.nested_sizes, &self.nested_inner_sizes);

        let controlled = controlled_example(theme, &self.controlled_sizes);

        let content = column![
            text("iced-shadcn-v2 Resizable")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Port of shadcn-svelte / paneforge resizable")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Examples", palette.muted_foreground, theme.font_pack()),
            horizontal,
            vertical,
            with_handle,
            nested,
            controlled,
        ]
        .spacing(24)
        .max_width(960)
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

fn example_group<'a>(
    title: &'static str,
    theme: &'a Theme,
    direction: ResizableDirection,
    sizes: &[f32],
    on_layout: fn(ResizableLayout) -> Message,
    with_handle: bool,
) -> Element<'a, Message> {
    let handle = if with_handle {
        ResizableHandle::new().with_handle(true)
    } else {
        ResizableHandle::new()
    };

    let group = ResizablePaneGroup::new(theme)
        .direction(direction)
        .sizes_slice(sizes)
        .width(Length::Fill)
        .height(Length::Fixed(200.0))
        .bordered(true)
        .radius(ResizableRadius::Large)
        .pane(ResizablePane::new(pane_cell("Sidebar", theme)).default_size(25.0))
        .handle(handle)
        .pane(ResizablePane::new(pane_cell("Content", theme)).default_size(75.0))
        .on_layout_change(on_layout)
        .on_dragging_change(Message::Dragging);

    column![
        text(title)
            .size(16)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.foreground),
        group
            .into_element()
            .unwrap_or_else(|error| error_text(error.to_string(), theme)),
    ]
    .spacing(8)
    .into()
}

fn nested_example<'a>(
    theme: &'a Theme,
    sizes: &[f32],
    inner_sizes: &[f32],
) -> Element<'a, Message> {
    let inner = ResizablePaneGroup::new(theme)
        .direction(ResizableDirection::Vertical)
        .sizes_slice(inner_sizes)
        .pane(ResizablePane::new(pane_cell("Two", theme)).default_size(25.0))
        .handle(ResizableHandle::new())
        .pane(ResizablePane::new(pane_cell("Three", theme)).default_size(75.0))
        .width(Length::Fill)
        .height(Length::Fill)
        .on_layout_change(Message::NestedInnerLayout);

    let outer = ResizablePaneGroup::new(theme)
        .direction(ResizableDirection::Horizontal)
        .sizes_slice(sizes)
        .width(Length::Fill)
        .height(Length::Fixed(200.0))
        .bordered(true)
        .radius(ResizableRadius::Large)
        .pane(ResizablePane::new(pane_cell("One", theme)).default_size(50.0))
        .handle(ResizableHandle::new())
        .pane(
            ResizablePane::new(
                inner
                    .into_element()
                    .unwrap_or_else(|error| error_text(error.to_string(), theme)),
            )
            .default_size(50.0),
        )
        .on_layout_change(Message::NestedLayout)
        .on_dragging_change(Message::Dragging);

    column![
        text("Nested")
            .size(16)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.foreground),
        outer
            .into_element()
            .unwrap_or_else(|error| error_text(error.to_string(), theme)),
    ]
    .spacing(8)
    .into()
}

fn controlled_example<'a>(theme: &'a Theme, sizes: &[f32]) -> Element<'a, Message> {
    let left = sizes.first().copied().unwrap_or(30.0).round() as i32;
    let right = sizes.get(1).copied().unwrap_or(70.0).round() as i32;

    let group = ResizablePaneGroup::new(theme)
        .direction(ResizableDirection::Horizontal)
        .sizes_slice(sizes)
        .width(Length::Fill)
        .height(Length::Fixed(200.0))
        .bordered(true)
        .radius(ResizableRadius::Large)
        .pane(
            ResizablePane::new(pane_cell(&format!("{left}%"), theme))
                .default_size(30.0)
                .min_size(20.0),
        )
        .handle(ResizableHandle::new())
        .pane(
            ResizablePane::new(pane_cell(&format!("{right}%"), theme))
                .default_size(70.0)
                .min_size(20.0),
        )
        .on_layout_change(Message::ControlledLayout)
        .on_dragging_change(Message::Dragging);

    column![
        text("Controlled")
            .size(16)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.foreground),
        group
            .into_element()
            .unwrap_or_else(|error| error_text(error.to_string(), theme)),
    ]
    .spacing(8)
    .into()
}

fn pane_cell<'a>(label: &str, theme: &'a Theme) -> Element<'a, Message> {
    let label = label.to_owned();

    container(
        text(label)
            .size(15)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.foreground),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(24.0)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_| container::Style {
        text_color: Some(theme.palette.foreground),
        ..container::Style::default()
    })
    .into()
}

fn error_text(message: String, theme: &Theme) -> Element<'_, Message> {
    text(message)
        .size(13)
        .color(theme.palette.destructive)
        .into()
}

fn section_label<'a>(
    label: &'static str,
    color: iced::Color,
    pack: FontPack,
) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
            .style(move |_theme, _status| pick_list::Style {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
        }
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

const ACCENTS: [AccentOpt; 5] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Amber),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 3] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::GeistMono),
];

const HEADINGS: [Labelled<FontHeading>; 3] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
