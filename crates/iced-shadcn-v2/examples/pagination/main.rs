//! Interactive playground for `iced-shadcn-v2::Pagination` + `shadcn-common` theme knobs.
//!
//! Mirrors the `button` example layout: the same theme selects drive every
//! pagination demo below, so style packs, accents, fonts, and radii can be
//! compared live.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example pagination`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, ButtonSize, FontHeading, FontId, FontPack, Pagination,
    PaginationEllipsis, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious,
    RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font, pagination,
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

const BASIC_COUNT: usize = 95;
const WIDE_COUNT: usize = 500;
const CUSTOM_TOTAL: usize = 12;

struct Example {
    theme: Theme,
    basic_page: usize,
    basic_per_page: usize,
    wide_page: usize,
    simple_page: usize,
    custom_page: usize,
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
    Basic(usize),
    BasicPerPage(usize),
    Wide(usize),
    Simple(usize),
    Custom(usize),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            basic_page: 2,
            basic_per_page: 10,
            wide_page: 25,
            simple_page: 1,
            custom_page: 3,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Pagination".to_owned()
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
            Message::Basic(page) => {
                self.basic_page = page;
            }
            Message::BasicPerPage(per_page) => {
                self.basic_per_page = per_page;
                self.basic_page = 1;
            }
            Message::Wide(page) => {
                self.wide_page = page;
            }
            Message::Simple(page) => {
                self.simple_page = page;
            }
            Message::Custom(page) => {
                self.custom_page = page;
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
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "radius lg={:.0}px · control h={:.0}/{:.0}/{:.0} · sans={} · heading={}",
                theme.radius_scale().lg_px,
                theme.style.control_height_sm_px,
                theme.style.control_height_md_px,
                theme.style.control_height_lg_px,
                theme.font_pack().sans.title(),
                theme.font_heading().title(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let swatches = row![
            swatch("bg", p.background, p.border),
            swatch("fg", p.foreground, p.border),
            swatch("primary", p.primary, p.border),
            swatch("secondary", p.secondary, p.border),
            swatch("muted", p.muted, p.border),
            swatch("destructive", p.destructive, p.border),
            swatch("border", p.border, p.foreground),
        ]
        .spacing(8)
        .wrap();

        // shadcn `pagination-basic`: controlled count/perPage/page state.
        let basic = pagination(BASIC_COUNT, self.basic_page, theme)
            .per_page(self.basic_per_page)
            .on_page_change(Message::Basic);

        // shadcn `pagination-with-select`: the app owns perPage too.
        let per_page_switch = row([10usize, 20, 50].map(|per_page| {
            PaginationLink::new(per_page, theme)
                .size(ButtonSize::Sm)
                .active(self.basic_per_page == per_page)
                .on_press(Message::BasicPerPage(per_page))
                .into()
        }))
        .spacing(4)
        .align_y(Alignment::Center);

        // Many pages, wider sibling window, icon-only boundary controls.
        let wide = pagination(WIDE_COUNT, self.wide_page, theme)
            .sibling_count(2)
            .show_labels(false)
            .on_page_change(Message::Wide);

        // shadcn `pagination-simple`: previous/next only.
        let simple = pagination(BASIC_COUNT, self.simple_page, theme)
            .show_links(false)
            .on_page_change(Message::Simple);

        // Custom composition from the standalone subcomponents.
        let custom = self.custom_pagination(theme);

        // Disabled bar keeps its layout but ignores presses.
        let disabled = pagination(BASIC_COUNT, 3, theme)
            .disabled(true)
            .on_page_change(Message::Basic);

        let status = {
            let probe: Pagination<'_, Message> =
                pagination(BASIC_COUNT, self.basic_page, theme).per_page(self.basic_per_page);
            let (start, end) = probe.item_range().unwrap_or((0, 0));
            format!(
                "basic: page {} of {} · items {start}-{end} of {BASIC_COUNT} · simple: page {} · wide: page {}",
                self.basic_page,
                probe.total_pages(),
                self.simple_page,
                self.wide_page,
            )
        };

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Pagination")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte parity: count/perPage/page, siblingCount, ellipsis, prev/next")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            text(status)
                .size(14)
                .font(iced_font(theme.font_pack().mono))
                .color(p.foreground),
            controls,
            section_label("Palette", p.muted_foreground, theme.font_pack()),
            swatches,
            section_label(
                "Basic (count=95, controlled page)",
                p.muted_foreground,
                theme.font_pack()
            ),
            basic,
            section_label(
                "Items per page (controlled perPage)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            per_page_switch,
            section_label(
                "Sibling count 2 · icon-only controls (count=500)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            wide,
            section_label(
                "Simple (previous/next only)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            simple,
            section_label(
                "Custom composition from subcomponents",
                p.muted_foreground,
                theme.font_pack(),
            ),
            custom,
            section_label("Disabled", p.muted_foreground, theme.font_pack()),
            disabled,
        ]
        .spacing(16)
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
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }

    /// Manual layout built from `PaginationPrevious` / `PaginationLink` /
    /// `PaginationEllipsis` / `PaginationNext`, the way shadcn composes the
    /// subcomponents inside `Pagination.Content`.
    fn custom_pagination<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let mut items: Vec<Element<'a, Message>> = Vec::new();

        items.push(
            PaginationPrevious::new(theme)
                .label("Back")
                .disabled(self.custom_page == 1)
                .on_press_maybe(
                    (self.custom_page > 1).then(|| Message::Custom(self.custom_page - 1)),
                )
                .into(),
        );

        for item in iced_shadcn_v2::pagination::page_items(self.custom_page, CUSTOM_TOTAL, 1) {
            items.push(match item {
                PaginationItem::Page(page) => PaginationLink::new(page, theme)
                    .active(page == self.custom_page)
                    .size(ButtonSize::IconSm)
                    .on_press(Message::Custom(page))
                    .into(),
                // `PaginationItem` is non-exhaustive; render unknown
                // future slots as gaps.
                _ => PaginationEllipsis::new(theme)
                    .size(ButtonSize::IconSm)
                    .into(),
            });
        }

        items.push(
            PaginationNext::new(theme)
                .label("Forward")
                .disabled(self.custom_page == CUSTOM_TOTAL)
                .on_press_maybe(
                    (self.custom_page < CUSTOM_TOTAL)
                        .then(|| Message::Custom(self.custom_page + 1)),
                )
                .into(),
        );

        row(items).spacing(4).align_y(Alignment::Center).into()
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
