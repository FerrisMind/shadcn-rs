//! Interactive playground for `iced-shadcn-v2::Carousel` + theme knobs.
//!
//! Mirrors the shadcn-svelte carousel docs demos: the default demo (full-width
//! slides in a card), size demo (1/3 per view), orientation demo (vertical),
//! and the autoplay plugin demo.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example carousel`

use std::fmt;
use std::time::Duration;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Card, CardContent, Carousel, CarouselAlign, CarouselItem, CarouselOrientation,
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
    demo_selected: usize,
    size_selected: usize,
    orientation_selected: usize,
    autoplay_selected: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    DemoSelected(usize),
    SizeSelected(usize),
    OrientationSelected(usize),
    AutoplaySelected(usize),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            demo_selected: 0,
            size_selected: 0,
            orientation_selected: 0,
            autoplay_selected: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Carousel".to_owned()
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
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::DemoSelected(index) => {
                self.demo_selected = index;
            }
            Message::SizeSelected(index) => {
                self.size_selected = index;
            }
            Message::OrientationSelected(index) => {
                self.orientation_selected = index;
            }
            Message::AutoplaySelected(index) => {
                self.autoplay_selected = index;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label("Theme", p.muted_foreground, theme.font_pack()),
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
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
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

        let demo_carousel = demo_section(theme, self.demo_selected);
        let size_carousel = size_section(theme, self.size_selected);
        let orientation_carousel = orientation_section(theme, self.orientation_selected);
        let autoplay_carousel = autoplay_section(theme, self.autoplay_selected);

        let content = column![
            text("iced-shadcn-v2 Carousel")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte Carousel port with full snap-scroll, drag, loop, autoplay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label(
                "Default (full-width slides)",
                p.muted_foreground,
                theme.font_pack()
            ),
            demo_carousel,
            text(format!("Slide {} of 5", self.demo_selected + 1))
                .size(13)
                .color(p.muted_foreground),
            section_label("Size (basis-1/3)", p.muted_foreground, theme.font_pack()),
            size_carousel,
            section_label(
                "Orientation (vertical)",
                p.muted_foreground,
                theme.font_pack()
            ),
            orientation_carousel,
            section_label(
                "Autoplay (2 s delay, loops)",
                p.muted_foreground,
                theme.font_pack()
            ),
            autoplay_carousel,
        ]
        .spacing(16)
        .max_width(520)
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

fn demo_section<'a>(theme: &'a Theme, selected: usize) -> Element<'a, Message> {
    let mut carousel = Carousel::new(theme)
        .selected(selected)
        .on_select(Message::DemoSelected)
        .width(Length::Fill);

    for index in 0..5 {
        carousel = carousel.push(CarouselItem::new(slide_card(index, theme)));
    }

    carousel.into()
}

fn size_section<'a>(theme: &'a Theme, selected: usize) -> Element<'a, Message> {
    let mut carousel = Carousel::new(theme)
        .selected(selected)
        .on_select(Message::SizeSelected)
        .align(CarouselAlign::Start)
        .item_basis(1.0 / 3.0)
        .width(Length::Fill);

    for index in 0..5 {
        carousel = carousel.push(CarouselItem::new(slide_card(index, theme)));
    }

    carousel.into()
}

fn orientation_section<'a>(theme: &'a Theme, selected: usize) -> Element<'a, Message> {
    let mut carousel = Carousel::new(theme)
        .selected(selected)
        .on_select(Message::OrientationSelected)
        .orientation(CarouselOrientation::Vertical)
        .align(CarouselAlign::Start)
        .item_basis(0.5)
        .height(Length::Fixed(200.0))
        .width(Length::Fixed(200.0));

    for index in 0..5 {
        carousel = carousel.push(CarouselItem::new(slide_card(index, theme)));
    }

    carousel.into()
}

fn autoplay_section<'a>(theme: &'a Theme, selected: usize) -> Element<'a, Message> {
    let mut carousel = Carousel::new(theme)
        .selected(selected)
        .on_select(Message::AutoplaySelected)
        .autoplay(Duration::from_millis(2000))
        .autoplay_stop_on_interaction(true)
        .looped(true)
        .width(Length::Fill);

    for index in 0..5 {
        carousel = carousel.push(CarouselItem::new(slide_card(index, theme)));
    }

    carousel.into()
}

fn slide_card<'a>(index: usize, theme: &'a Theme) -> Element<'a, Message> {
    let p = &theme.palette;

    Card::new(theme)
        .width(Length::Fill)
        .style_override(|mut style| {
            style.shadow = Default::default();
            style
        })
        .content(
            CardContent::new(theme).push(
                container(
                    text(format!("{}", index + 1))
                        .size(32)
                        .font(iced_font(theme.font_pack().heading))
                        .color(p.card_foreground),
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fixed(140.0)),
            ),
        )
        .into()
}

// ---- shared helper utilities (same as button example) ----

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

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
