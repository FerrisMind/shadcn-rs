//! Interactive playground for `iced-shadcn-v2::DropdownMenu` + `shadcn-common`
//! theme knobs.
//!
//! Mirrors the shadcn-svelte dropdown-menu docs demos (basic, checkboxes,
//! radio, shortcuts, destructive, submenu) plus theme controls like the
//! button example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example dropdown_menu`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuItem,
    DropdownMenuRadioItem, DropdownMenuSub, FontHeading, FontId, FontPack, MenuItemVariant,
    RadiusId, StyleId, Theme, ThemeMode, dropdown_menu, fonts, iced_font,
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
    last_action: String,
    email: bool,
    sms: bool,
    appearance: Appearance,
    open_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Appearance {
    Light,
    Dark,
    System,
}

impl Appearance {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
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
    Action(&'static str),
    ToggleEmail,
    ToggleSms,
    Appearance(Appearance),
    Opened,
    Closed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            last_action: "—".to_owned(),
            email: true,
            sms: false,
            appearance: Appearance::Light,
            open_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Dropdown Menu".to_owned()
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
            Message::Action(action) => {
                self.last_action = action.to_owned();
            }
            Message::ToggleEmail => {
                self.email = !self.email;
                self.last_action = format!("email={}", self.email);
            }
            Message::ToggleSms => {
                self.sms = !self.sms;
                self.last_action = format!("sms={}", self.sms);
            }
            Message::Appearance(appearance) => {
                self.appearance = appearance;
                self.last_action = format!("appearance={}", appearance.as_str());
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

        let basic = dropdown_menu("Open", theme)
            .width(224.0)
            .label("My Account")
            .item(
                DropdownMenuItem::new("Profile")
                    .shortcut("⇧⌘P")
                    .on_select(Message::Action("Profile")),
            )
            .item(
                DropdownMenuItem::new("Billing")
                    .shortcut("⌘B")
                    .on_select(Message::Action("Billing")),
            )
            .item(
                DropdownMenuItem::new("Settings")
                    .shortcut("⌘S")
                    .on_select(Message::Action("Settings")),
            )
            .item(
                DropdownMenuItem::new("Keyboard shortcuts")
                    .shortcut("⌘K")
                    .on_select(Message::Action("Keyboard shortcuts")),
            )
            .separator()
            .item(DropdownMenuItem::new("Team").on_select(Message::Action("Team")))
            .submenu(
                DropdownMenuSub::new("Invite users")
                    .item(DropdownMenuItem::new("Email").on_select(Message::Action("Invite Email")))
                    .item(
                        DropdownMenuItem::new("Message")
                            .on_select(Message::Action("Invite Message")),
                    )
                    .separator()
                    .item(DropdownMenuItem::new("More…").on_select(Message::Action("Invite More"))),
            )
            .item(
                DropdownMenuItem::new("New Team")
                    .shortcut("⌘+T")
                    .on_select(Message::Action("New Team")),
            )
            .separator()
            .item(DropdownMenuItem::new("GitHub").on_select(Message::Action("GitHub")))
            .item(DropdownMenuItem::new("Support").on_select(Message::Action("Support")))
            .item(DropdownMenuItem::new("API").disabled(true))
            .separator()
            .item(
                DropdownMenuItem::new("Log out")
                    .shortcut("⇧⌘Q")
                    .on_select(Message::Action("Log out")),
            )
            .on_open(Message::Opened)
            .on_close(Message::Closed);

        let checkboxes = dropdown_menu("Notifications", theme)
            .checkbox_item(
                DropdownMenuCheckboxItem::new("Email", self.email).on_toggle(Message::ToggleEmail),
            )
            .checkbox_item(
                DropdownMenuCheckboxItem::new("SMS", self.sms).on_toggle(Message::ToggleSms),
            );

        let radios = dropdown_menu("Appearance", theme)
            .radio_item(
                DropdownMenuRadioItem::new("Light", self.appearance == Appearance::Light)
                    .on_select(Message::Appearance(Appearance::Light)),
            )
            .radio_item(
                DropdownMenuRadioItem::new("Dark", self.appearance == Appearance::Dark)
                    .on_select(Message::Appearance(Appearance::Dark)),
            )
            .radio_item(
                DropdownMenuRadioItem::new("System", self.appearance == Appearance::System)
                    .on_select(Message::Appearance(Appearance::System)),
            );

        let shortcuts = dropdown_menu("Edit", theme)
            .item(
                DropdownMenuItem::new("Cut")
                    .shortcut("⌘X")
                    .on_select(Message::Action("Cut")),
            )
            .item(
                DropdownMenuItem::new("Copy")
                    .shortcut("⌘C")
                    .on_select(Message::Action("Copy")),
            )
            .item(
                DropdownMenuItem::new("Paste")
                    .shortcut("⌘V")
                    .on_select(Message::Action("Paste")),
            );

        let destructive = dropdown_menu("Account", theme)
            .item(DropdownMenuItem::new("Profile").on_select(Message::Action("Profile")))
            .separator()
            .item(
                DropdownMenuItem::new("Sign Out")
                    .variant(MenuItemVariant::Destructive)
                    .shortcut("⇧⌘Q")
                    .on_select(Message::Action("Sign Out")),
            );

        let submenu = DropdownMenu::new(theme)
            .trigger_label("File")
            .width(224.0)
            .item(
                DropdownMenuItem::new("New File")
                    .shortcut("⌘N")
                    .on_select(Message::Action("New File")),
            )
            .submenu(
                DropdownMenuSub::new("Open Recent")
                    .label("Recent Projects")
                    .item(
                        DropdownMenuItem::new("Project Alpha")
                            .on_select(Message::Action("Project Alpha")),
                    )
                    .item(
                        DropdownMenuItem::new("Project Beta")
                            .on_select(Message::Action("Project Beta")),
                    )
                    .submenu(
                        DropdownMenuSub::new("More Projects").item(
                            DropdownMenuItem::new("Project Gamma")
                                .on_select(Message::Action("Project Gamma")),
                        ),
                    )
                    .separator()
                    .item(DropdownMenuItem::new("Browse…").on_select(Message::Action("Browse"))),
            )
            .separator()
            .item(
                DropdownMenuItem::new("Save")
                    .shortcut("⌘S")
                    .on_select(Message::Action("Save")),
            );

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Dropdown Menu")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(format!(
                "last={} · email={} · sms={} · appearance={} · opened {} times",
                self.last_action,
                self.email,
                self.sms,
                self.appearance.as_str(),
                self.open_count
            ))
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            controls,
            section_label("Basic (docs demo)", p.muted_foreground, theme.font_pack()),
            basic,
            section_label("Checkboxes", p.muted_foreground, theme.font_pack()),
            checkboxes,
            section_label("Radio group", p.muted_foreground, theme.font_pack()),
            radios,
            section_label("Shortcuts", p.muted_foreground, theme.font_pack()),
            shortcuts,
            section_label("Destructive", p.muted_foreground, theme.font_pack()),
            destructive,
            section_label("Submenu (nested)", p.muted_foreground, theme.font_pack()),
            submenu,
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
