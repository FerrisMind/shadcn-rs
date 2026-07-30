//! Interactive playground for `iced-shadcn-v2::NativeSelect` + `shadcn-common`
//! theme knobs.
//!
//! Mirrors the shadcn-svelte native-select docs demos (default, groups,
//! disabled, invalid) plus the iced-specific size / accent knobs.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example native_select`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, NativeSelect, NativeSelectGroup,
    NativeSelectSize, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font, native_select,
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
    status: Option<&'static str>,
    department: Option<&'static str>,
    role: Option<&'static str>,
    fruit: Option<&'static str>,
    open_count: u32,
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
    StatusPicked(&'static str),
    DepartmentPicked(&'static str),
    RolePicked(&'static str),
    FruitPicked(&'static str),
    Opened,
    Closed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            status: None,
            department: None,
            role: None,
            fruit: None,
            open_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 NativeSelect".to_owned()
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
            Message::StatusPicked(status) => {
                self.status = Some(status);
            }
            Message::DepartmentPicked(department) => {
                self.department = Some(department);
            }
            Message::RolePicked(role) => {
                self.role = Some(role);
            }
            Message::FruitPicked(fruit) => {
                self.fruit = Some(fruit);
            }
            Message::Opened => {
                self.open_count += 1;
            }
            Message::Closed => {}
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
        ]
        .spacing(8);

        // Docs demo: fruits with a disabled option.
        let demo = native_select("Select a fruit", theme)
            .option(("apple", "Apple"))
            .option(("banana", "Banana"))
            .option(("blueberry", "Blueberry"))
            .option(iced_shadcn_v2::NativeSelectOption::new("grapes", "Grapes").disabled(true))
            .option(("pineapple", "Pineapple"))
            .selected_maybe(self.fruit)
            .on_select(Message::FruitPicked)
            .on_open(Message::Opened)
            .on_close(Message::Closed);

        // Docs demo: opt-groups.
        let groups = native_select("Select department", theme)
            .group(
                NativeSelectGroup::new("Engineering")
                    .option(("frontend", "Frontend"))
                    .option(("backend", "Backend"))
                    .option(("devops", "DevOps")),
            )
            .group(
                NativeSelectGroup::new("Sales")
                    .option(("sales-rep", "Sales Rep"))
                    .option(("account-manager", "Account Manager"))
                    .option(("sales-director", "Sales Director")),
            )
            .group(
                NativeSelectGroup::new("Operations")
                    .option(("support", "Customer Support"))
                    .option(("product-manager", "Product Manager"))
                    .option(("ops-manager", "Operations Manager")),
            )
            .selected_maybe(self.department)
            .on_select(Message::DepartmentPicked);

        // Docs demo: disabled select.
        let disabled = native_select("Select priority", theme)
            .options([
                ("low", "Low"),
                ("medium", "Medium"),
                ("high", "High"),
                ("critical", "Critical"),
            ])
            .disabled(true)
            .on_select(Message::RolePicked);

        // Docs demo: invalid select.
        let invalid = native_select("Select role", theme)
            .options([
                ("admin", "Admin"),
                ("editor", "Editor"),
                ("viewer", "Viewer"),
                ("guest", "Guest"),
            ])
            .selected_maybe(self.role)
            .invalid(true)
            .on_select(Message::RolePicked);

        let sizes = row![
            NativeSelect::new(theme)
                .placeholder("sm")
                .options(STATUSES)
                .size(NativeSelectSize::Sm)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
            NativeSelect::new(theme)
                .placeholder("default")
                .options(STATUSES)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
            NativeSelect::new(theme)
                .placeholder("lg (iced extension)")
                .options(STATUSES)
                .size(NativeSelectSize::Lg)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let accents = row![
            NativeSelect::new(theme)
                .placeholder("Theme ring")
                .options(STATUSES)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
            NativeSelect::new(theme)
                .placeholder("Blue focus")
                .options(STATUSES)
                .color(AccentColor::Blue)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
            NativeSelect::new(theme)
                .placeholder("Rose focus")
                .options(STATUSES)
                .color(AccentColor::Rose)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let layout = row![
            NativeSelect::new(theme)
                .placeholder("Fixed 220px")
                .options(STATUSES)
                .width(Length::Fixed(220.0))
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
            NativeSelect::new(theme)
                .placeholder("Fill")
                .options(STATUSES)
                .width(Length::Fill)
                .selected_maybe(self.status)
                .on_select(Message::StatusPicked),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 NativeSelect")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(format!(
                "selected: fruit={:?} · department={:?} · role={:?} · opened {} times",
                self.fruit, self.department, self.role, self.open_count
            ))
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            controls,
            section_label("Default (docs demo)", p.muted_foreground, theme.font_pack()),
            demo,
            section_label("With groups", p.muted_foreground, theme.font_pack()),
            groups,
            section_label("Disabled", p.muted_foreground, theme.font_pack()),
            disabled,
            section_label(
                "Invalid (aria-invalid)",
                p.muted_foreground,
                theme.font_pack()
            ),
            invalid,
            section_label(
                "Sizes (sm · default · lg)",
                p.muted_foreground,
                theme.font_pack()
            ),
            sizes,
            section_label(
                "Per-select accent overlay (open to see the focus border)",
                p.muted_foreground,
                theme.font_pack()
            ),
            accents,
            section_label(
                "Width (w-fit default · fixed · fill)",
                p.muted_foreground,
                theme.font_pack()
            ),
            layout,
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
}

const STATUSES: [(&str, &str); 4] = [
    ("todo", "Todo"),
    ("in-progress", "In Progress"),
    ("done", "Done"),
    ("cancelled", "Cancelled"),
];

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
