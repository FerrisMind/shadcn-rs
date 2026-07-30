//! Interactive playground for `iced-shadcn-v2::ContextMenu` + `shadcn-common`
//! theme knobs.
//!
//! Mirrors the shadcn-svelte context-menu docs demos (basic, sides, icons,
//! shortcuts, submenu, groups, checkboxes, radio, destructive, inset) plus
//! theme controls like the button / dropdown-menu examples.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example context_menu`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, ContextMenu, ContextMenuCheckboxItem, ContextMenuItem,
    ContextMenuLabel, ContextMenuRadioItem, ContextMenuSub, FontHeading, FontId, FontPack,
    MenuItemVariant, RadiusId, StyleId, Theme, ThemeMode, context_menu, fonts, iced_font,
};
use shadcn_common::FloatingSide;

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
    show_bookmarks: bool,
    show_urls: bool,
    theme_radio: ThemeOpt,
    open_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeOpt {
    Light,
    Dark,
    System,
}

impl ThemeOpt {
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
    ToggleBookmarks,
    ToggleUrls,
    ThemeRadio(ThemeOpt),
    Opened,
    Closed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            last_action: "—".to_owned(),
            show_bookmarks: true,
            show_urls: false,
            theme_radio: ThemeOpt::System,
            open_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Context Menu".to_owned()
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
            Message::ToggleBookmarks => {
                self.show_bookmarks = !self.show_bookmarks;
                self.last_action = format!("bookmarks={}", self.show_bookmarks);
            }
            Message::ToggleUrls => {
                self.show_urls = !self.show_urls;
                self.last_action = format!("urls={}", self.show_urls);
            }
            Message::ThemeRadio(theme) => {
                self.theme_radio = theme;
                self.last_action = format!("theme={}", theme.as_str());
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
        ]
        .spacing(8);

        let basic = context_menu("Right click here", theme)
            .width(192.0)
            .item(ContextMenuItem::new("Back").on_select(Message::Action("Back")))
            .item(
                ContextMenuItem::new("Forward")
                    .disabled(true)
                    .on_select(Message::Action("Forward")),
            )
            .item(ContextMenuItem::new("Reload").on_select(Message::Action("Reload")));

        let with_shortcuts = context_menu("Right click here", theme)
            .width(208.0)
            .item(
                ContextMenuItem::new("Back")
                    .shortcut("⌘[")
                    .on_select(Message::Action("Back")),
            )
            .item(
                ContextMenuItem::new("Forward")
                    .disabled(true)
                    .shortcut("⌘]")
                    .on_select(Message::Action("Forward")),
            )
            .item(
                ContextMenuItem::new("Reload")
                    .shortcut("⌘R")
                    .on_select(Message::Action("Reload")),
            )
            .separator()
            .item(
                ContextMenuItem::new("Save")
                    .shortcut("⌘S")
                    .on_select(Message::Action("Save")),
            )
            .item(
                ContextMenuItem::new("Save As...")
                    .shortcut("⇧⌘S")
                    .on_select(Message::Action("Save As...")),
            );

        let with_submenu = context_menu("Right click here", theme)
            .width(208.0)
            .item(
                ContextMenuItem::new("Copy")
                    .shortcut("⌘C")
                    .on_select(Message::Action("Copy")),
            )
            .item(
                ContextMenuItem::new("Cut")
                    .shortcut("⌘X")
                    .on_select(Message::Action("Cut")),
            )
            .submenu(
                ContextMenuSub::new("More Tools")
                    .item(
                        ContextMenuItem::new("Save Page...")
                            .on_select(Message::Action("Save Page...")),
                    )
                    .item(
                        ContextMenuItem::new("Create Shortcut...")
                            .on_select(Message::Action("Create Shortcut...")),
                    )
                    .item(
                        ContextMenuItem::new("Name Window...")
                            .on_select(Message::Action("Name Window...")),
                    )
                    .separator()
                    .item(
                        ContextMenuItem::new("Developer Tools")
                            .on_select(Message::Action("Developer Tools")),
                    )
                    .separator()
                    .item(
                        ContextMenuItem::new("Delete")
                            .variant(MenuItemVariant::Destructive)
                            .on_select(Message::Action("Delete")),
                    ),
            );

        let with_groups = context_menu("Right click here", theme)
            .width(224.0)
            .label("File")
            .item(
                ContextMenuItem::new("New File")
                    .shortcut("⌘N")
                    .on_select(Message::Action("New File")),
            )
            .item(
                ContextMenuItem::new("Open File")
                    .shortcut("⌘O")
                    .on_select(Message::Action("Open File")),
            )
            .item(
                ContextMenuItem::new("Save")
                    .shortcut("⌘S")
                    .on_select(Message::Action("Save")),
            )
            .separator()
            .label("Edit")
            .item(
                ContextMenuItem::new("Undo")
                    .shortcut("⌘Z")
                    .on_select(Message::Action("Undo")),
            )
            .item(
                ContextMenuItem::new("Redo")
                    .shortcut("⇧⌘Z")
                    .on_select(Message::Action("Redo")),
            )
            .separator()
            .item(
                ContextMenuItem::new("Cut")
                    .shortcut("⌘X")
                    .on_select(Message::Action("Cut")),
            )
            .item(
                ContextMenuItem::new("Copy")
                    .shortcut("⌘C")
                    .on_select(Message::Action("Copy")),
            )
            .item(
                ContextMenuItem::new("Paste")
                    .shortcut("⌘V")
                    .on_select(Message::Action("Paste")),
            )
            .separator()
            .item(
                ContextMenuItem::new("Delete")
                    .variant(MenuItemVariant::Destructive)
                    .shortcut("⌫")
                    .on_select(Message::Action("Delete")),
            );

        let with_checkboxes = context_menu("Right click here", theme)
            .width(224.0)
            .checkbox_item(
                ContextMenuCheckboxItem::new("Show Bookmarks Bar", self.show_bookmarks)
                    .on_toggle(Message::ToggleBookmarks),
            )
            .checkbox_item(
                ContextMenuCheckboxItem::new("Show Full URLs", self.show_urls)
                    .on_toggle(Message::ToggleUrls),
            )
            .checkbox_item(
                ContextMenuCheckboxItem::new("Show Developer Tools", true)
                    .on_toggle(Message::Action("Toggle DevTools")),
            );

        let with_radio = context_menu("Right click here", theme)
            .width(224.0)
            .label("People")
            .radio_item(
                ContextMenuRadioItem::new("Pedro Duarte", false)
                    .on_select(Message::Action("People:Pedro")),
            )
            .radio_item(
                ContextMenuRadioItem::new("Colm Tuite", false)
                    .on_select(Message::Action("People:Colm")),
            )
            .separator()
            .label("Theme")
            .radio_item(
                ContextMenuRadioItem::new("Light", self.theme_radio == ThemeOpt::Light)
                    .on_select(Message::ThemeRadio(ThemeOpt::Light)),
            )
            .radio_item(
                ContextMenuRadioItem::new("Dark", self.theme_radio == ThemeOpt::Dark)
                    .on_select(Message::ThemeRadio(ThemeOpt::Dark)),
            )
            .radio_item(
                ContextMenuRadioItem::new("System", self.theme_radio == ThemeOpt::System)
                    .on_select(Message::ThemeRadio(ThemeOpt::System)),
            );

        let with_destructive = context_menu("Right click here", theme).width(192.0).item(
            ContextMenuItem::new("Delete")
                .variant(MenuItemVariant::Destructive)
                .on_select(Message::Action("Delete")),
        );

        let with_inset = context_menu("Right click here", theme)
            .width(176.0)
            .label("Actions")
            .item(ContextMenuItem::new("Copy").on_select(Message::Action("Copy")))
            .item(ContextMenuItem::new("Cut").on_select(Message::Action("Cut")))
            .item(
                ContextMenuItem::new("Paste")
                    .inset(true)
                    .on_select(Message::Action("Paste")),
            )
            .separator()
            .label(ContextMenuLabel::new("Appearance").inset(true))
            .checkbox_item(
                ContextMenuCheckboxItem::new("Bookmarks", self.show_bookmarks)
                    .inset(true)
                    .on_toggle(Message::ToggleBookmarks),
            )
            .checkbox_item(
                ContextMenuCheckboxItem::new("Full URLs", self.show_urls)
                    .inset(true)
                    .on_toggle(Message::ToggleUrls),
            )
            .separator()
            .label(ContextMenuLabel::new("Theme").inset(true))
            .radio_item(
                ContextMenuRadioItem::new("Light", self.theme_radio == ThemeOpt::Light)
                    .inset(true)
                    .on_select(Message::ThemeRadio(ThemeOpt::Light)),
            )
            .radio_item(
                ContextMenuRadioItem::new("Dark", self.theme_radio == ThemeOpt::Dark)
                    .inset(true)
                    .on_select(Message::ThemeRadio(ThemeOpt::Dark)),
            )
            .radio_item(
                ContextMenuRadioItem::new("System", self.theme_radio == ThemeOpt::System)
                    .inset(true)
                    .on_select(Message::ThemeRadio(ThemeOpt::System)),
            )
            .separator()
            .submenu(ContextMenuSub::new("More Options").inset(true).item(
                ContextMenuItem::new("Save Page...").on_select(Message::Action("Save Page...")),
            ));

        // Side variants: pin placement to each edge of the cursor.
        let with_sides_top = context_menu("Right click (top)", theme)
            .width(176.0)
            .side(FloatingSide::Top)
            .item(ContextMenuItem::new("Back").on_select(Message::Action("Back")))
            .item(ContextMenuItem::new("Forward").on_select(Message::Action("Forward")))
            .item(ContextMenuItem::new("Reload").on_select(Message::Action("Reload")));
        let with_sides_bottom = context_menu("Right click (bottom)", theme)
            .width(176.0)
            .side(FloatingSide::Bottom)
            .item(ContextMenuItem::new("Back").on_select(Message::Action("Back")))
            .item(ContextMenuItem::new("Forward").on_select(Message::Action("Forward")))
            .item(ContextMenuItem::new("Reload").on_select(Message::Action("Reload")));
        let with_sides_left = context_menu("Right click (left)", theme)
            .width(176.0)
            .side(FloatingSide::Left)
            .item(ContextMenuItem::new("Back").on_select(Message::Action("Back")))
            .item(ContextMenuItem::new("Forward").on_select(Message::Action("Forward")))
            .item(ContextMenuItem::new("Reload").on_select(Message::Action("Reload")));
        let with_sides_right = context_menu("Right click (right)", theme)
            .width(176.0)
            .side(FloatingSide::Right)
            .item(ContextMenuItem::new("Back").on_select(Message::Action("Back")))
            .item(ContextMenuItem::new("Forward").on_select(Message::Action("Forward")))
            .item(ContextMenuItem::new("Reload").on_select(Message::Action("Reload")));

        // Controlled / open-change demo.
        let with_open_change = ContextMenu::new(theme)
            .trigger_label("Right click (on_open_change)")
            .width(176.0)
            .item(ContextMenuItem::new("Back").on_select(Message::Action("Back")))
            .item(ContextMenuItem::new("Forward").on_select(Message::Action("Forward")))
            .on_open(Message::Opened)
            .on_close(Message::Closed);

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Context Menu")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(format!(
                "last={} · bookmarks={} · urls={} · theme={} · opened {} times",
                self.last_action,
                self.show_bookmarks,
                self.show_urls,
                self.theme_radio.as_str(),
                self.open_count
            ))
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            controls,
            section_label("Basic (docs demo)", p.muted_foreground, theme.font_pack()),
            basic,
            section_label("With shortcuts", p.muted_foreground, theme.font_pack()),
            with_shortcuts,
            section_label("With submenu", p.muted_foreground, theme.font_pack()),
            with_submenu,
            section_label(
                "With groups, labels & separators",
                p.muted_foreground,
                theme.font_pack()
            ),
            with_groups,
            section_label("With checkboxes", p.muted_foreground, theme.font_pack()),
            with_checkboxes,
            section_label("With radio group", p.muted_foreground, theme.font_pack()),
            with_radio,
            section_label("With destructive", p.muted_foreground, theme.font_pack()),
            with_destructive,
            section_label("With inset", p.muted_foreground, theme.font_pack()),
            with_inset,
            section_label("With sides", p.muted_foreground, theme.font_pack()),
            row![with_sides_top, with_sides_bottom].spacing(12),
            row![with_sides_left, with_sides_right].spacing(12),
            section_label(
                "With on_open / on_close",
                p.muted_foreground,
                theme.font_pack()
            ),
            with_open_change,
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

const BASES: [Labelled<BaseColor>; 4] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
];

const ACCENTS: [AccentOpt; 9] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Rose),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 2] = [Labelled(FontId::Geist), Labelled(FontId::Inter)];

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
