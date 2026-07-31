//! Interactive playground for `iced-shadcn-v2::Chart` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte chart demos (bar / area / line / pie built on
//! layerchart), rendered on an iced canvas. Hover any chart for the shadcn
//! tooltip; theme selects use `iced::widget::pick_list` like the button
//! example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example chart`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Chart, ChartAxis, ChartColor, ChartCurve, ChartIndicator, ChartSeries,
    FontHeading, FontId, FontPack, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font,
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
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Radius(Labelled<RadiusId>),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
        }
    }
}

const MONTHS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];
const DESKTOP: [f64; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f64; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
const VISITORS: [f64; 6] = [186.0, 205.0, -207.0, 173.0, -209.0, 214.0];
const BROWSERS: [&str; 5] = ["Chrome", "Safari", "Firefox", "Edge", "Other"];
const BROWSER_VISITORS: [f64; 5] = [275.0, 200.0, 187.0, 173.0, 90.0];

fn month_short(month: &str) -> String {
    month.chars().take(3).collect()
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Chart".to_owned()
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
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                p.muted_foreground,
                theme.font_pack()
            ),
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
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
        ]
        .spacing(8);

        let swatches = row![
            swatch("chart-1", p.chart_1, p.border),
            swatch("chart-2", p.chart_2, p.border),
            swatch("chart-3", p.chart_3, p.border),
            swatch("chart-4", p.chart_4, p.border),
            swatch("chart-5", p.chart_5, p.border),
        ]
        .spacing(8)
        .wrap();

        let bar_charts = row![
            chart_block(
                "Bar Chart",
                "January - June 2024",
                Chart::bar(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP))
                    .axis(ChartAxis::X)
                    .bar_radius(8.0)
                    .category_format(month_short)
                    .tooltip_hide_label(true)
                    .into(),
                theme,
            ),
            chart_block(
                "Bar Chart - Multiple",
                "January - June 2024",
                Chart::bar(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP).color(ChartColor::Chart1))
                    .series(ChartSeries::new("Mobile", MOBILE).color(ChartColor::Chart2))
                    .axis(ChartAxis::X)
                    .category_format(month_short)
                    .tooltip_indicator(ChartIndicator::Dashed)
                    .into(),
                theme,
            ),
            chart_block(
                "Bar Chart - Stacked + Legend",
                "January - June 2024",
                Chart::bar(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP))
                    .series(ChartSeries::new("Mobile", MOBILE))
                    .stacked(true)
                    .legend(true)
                    .axis(ChartAxis::X)
                    .category_format(month_short)
                    .into(),
                theme,
            ),
            chart_block(
                "Bar Chart - Horizontal",
                "January - June 2024",
                Chart::bar(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP))
                    .horizontal(true)
                    .axis(ChartAxis::Y)
                    .grid(false)
                    .category_format(month_short)
                    .tooltip_hide_label(true)
                    .into(),
                theme,
            ),
            chart_block(
                "Bar Chart - Negative",
                "January - June 2024",
                Chart::bar(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Visitors", VISITORS).point_colors(
                        VISITORS.iter().map(|value| {
                            Some(if *value >= 0.0 {
                                ChartColor::Chart1
                            } else {
                                ChartColor::Chart2
                            })
                        }),
                    ),)
                    .axis(ChartAxis::None)
                    .bar_radius(0.0)
                    .category_format(month_short)
                    .tooltip_hide_label(true)
                    .tooltip_hide_indicator(true)
                    .into(),
                theme,
            ),
        ]
        .spacing(16)
        .wrap();

        let area_line_charts = row![
            chart_block(
                "Area Chart",
                "Showing total visitors for the last 6 months",
                Chart::area(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP))
                    .curve(ChartCurve::Natural)
                    .axis(ChartAxis::X)
                    .category_format(month_short)
                    .tooltip_indicator(ChartIndicator::Line)
                    .into(),
                theme,
            ),
            chart_block(
                "Area Chart - Stacked",
                "Showing total visitors for the last 6 months",
                Chart::area(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Mobile", MOBILE).color(ChartColor::Chart2))
                    .series(ChartSeries::new("Desktop", DESKTOP).color(ChartColor::Chart1))
                    .stacked(true)
                    .curve(ChartCurve::Natural)
                    .axis(ChartAxis::X)
                    .category_format(month_short)
                    .into(),
                theme,
            ),
            chart_block(
                "Line Chart",
                "Showing total visitors for the last 6 months",
                Chart::line(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP))
                    .curve(ChartCurve::Natural)
                    .axis(ChartAxis::X)
                    .category_format(month_short)
                    .tooltip_hide_label(true)
                    .into(),
                theme,
            ),
            chart_block(
                "Line Chart - Step",
                "Showing total visitors for the last 6 months",
                Chart::line(theme)
                    .categories(MONTHS)
                    .series(ChartSeries::new("Desktop", DESKTOP).color(ChartColor::Chart2))
                    .curve(ChartCurve::Step)
                    .axis(ChartAxis::X)
                    .category_format(month_short)
                    .into(),
                theme,
            ),
        ]
        .spacing(16)
        .wrap();

        let pie_charts = row![
            chart_block(
                "Pie Chart",
                "January - June 2024",
                Chart::pie(theme)
                    .categories(BROWSERS)
                    .series(ChartSeries::new("Visitors", BROWSER_VISITORS))
                    .into(),
                theme,
            ),
            chart_block(
                "Pie Chart - Donut + Legend",
                "January - June 2024",
                Chart::pie(theme)
                    .categories(BROWSERS)
                    .series(ChartSeries::new("Visitors", BROWSER_VISITORS))
                    .donut(0.6)
                    .legend(true)
                    .into(),
                theme,
            ),
        ]
        .spacing(16)
        .wrap();

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Chart")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Canvas port of the shadcn-svelte charts (layerchart) — hover for tooltips")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Chart palette", p.muted_foreground, theme.font_pack()),
            swatches,
            section_label("Bar charts", p.muted_foreground, theme.font_pack()),
            bar_charts,
            section_label("Area / line charts", p.muted_foreground, theme.font_pack()),
            area_line_charts,
            section_label("Pie charts", p.muted_foreground, theme.font_pack()),
            pie_charts,
        ]
        .spacing(16)
        .max_width(1040)
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
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

/// Card-like block around one chart: title, description, plot.
fn chart_block<'a>(
    title: &'static str,
    description: &'static str,
    chart: Element<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = theme.palette;

    container(
        column![
            text(title)
                .size(16)
                .font(iced_font(theme.font_pack().heading))
                .color(p.card_foreground),
            text(description)
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            chart,
        ]
        .spacing(8),
    )
    .width(Length::Fixed(480.0))
    .padding(20)
    .style(move |_| container::Style {
        background: Some(Background::Color(p.card)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 12.0.into(),
        },
        ..container::Style::default()
    })
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
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(p.background),
                text_color: p.foreground,
                placeholder_color: p.muted_foreground,
                handle_color: p.muted_foreground,
                border: Border {
                    color: p.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
        .into()
}

fn swatch<'a>(label: &'static str, fill: Color, border: Color) -> Element<'a, Message> {
    column![
        container(text(""))
            .width(36)
            .height(36)
            .style(move |_| container::Style {
                background: Some(Background::Color(fill)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(label).size(10).color(border),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
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

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
];

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
