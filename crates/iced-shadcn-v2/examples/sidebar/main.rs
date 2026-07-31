//! Sidebar playground mirroring shadcn-svelte **sidebar-07**.
//!
//! Structure matches `docs/src/lib/registry/blocks/sidebar-07/`:
//! TeamSwitcher header, NavMain (collapsible Platform items), NavProjects,
//! NavUser footer, Rail, Inset with trigger + breadcrumb + muted cards.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example sidebar`

use std::fmt;

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{Space, column, container, pick_list, row, text};
use iced::{Alignment, Background, Border, Color, Element, Event, Length, Padding, Task, event};

use iced_shadcn_v2::{
    Avatar, AvatarFallback, AvatarRadius, AvatarSize, BaseColor, Breadcrumb, BreadcrumbLink,
    BreadcrumbPage, FontHeading, FontId, FontPack, RadiusId, Separator, SeparatorOrientation,
    Sidebar, SidebarCollapsible, SidebarContent, SidebarController, SidebarFooter, SidebarGroup,
    SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSide,
    SidebarTrigger, SidebarVariant, StyleId, Theme, ThemeMode, fonts, iced_font,
    matches_sidebar_shortcut,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

/// Platform nav entries from sidebar-07 `navMain` sample data.
const NAV_MAIN: [NavEntry; 4] = [
    NavEntry {
        title: "Playground",
        glyph: "▣",
        initially_open: true,
        items: &["History", "Starred", "Settings"],
    },
    NavEntry {
        title: "Models",
        glyph: "◎",
        initially_open: false,
        items: &["Genesis", "Explorer", "Quantum"],
    },
    NavEntry {
        title: "Documentation",
        glyph: "☰",
        initially_open: false,
        items: &["Introduction", "Get Started", "Tutorials", "Changelog"],
    },
    NavEntry {
        title: "Settings",
        glyph: "⚙",
        initially_open: false,
        items: &["General", "Team", "Billing", "Limits"],
    },
];

const PROJECTS: [(&str, &str); 3] = [
    ("Design Engineering", "◫"),
    ("Sales & Marketing", "◔"),
    ("Travel", "⌖"),
];

struct NavEntry {
    title: &'static str,
    glyph: &'static str,
    initially_open: bool,
    items: &'static [&'static str],
}

struct Example {
    theme: Theme,
    controller: SidebarController,
    collapsible: SidebarCollapsible,
    variant: SidebarVariant,
    side: SidebarSide,
    open_nav: [bool; 4],
    active_sub: &'static str,
    force_mobile: bool,
    last_viewport_width: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    Collapsible(Labelled<SidebarCollapsible>),
    Variant(Labelled<SidebarVariant>),
    Side(Labelled<SidebarSide>),
    Toggle,
    MobileOpen(bool),
    ToggleMobileSim,
    ToggleNav(usize),
    NavSub(&'static str),
    Shortcut,
    Viewport(f32),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Vega),
            controller: SidebarController::new(true),
            collapsible: SidebarCollapsible::Icon,
            variant: SidebarVariant::Sidebar,
            side: SidebarSide::Left,
            open_nav: [
                NAV_MAIN[0].initially_open,
                NAV_MAIN[1].initially_open,
                NAV_MAIN[2].initially_open,
                NAV_MAIN[3].initially_open,
            ],
            active_sub: "History",
            force_mobile: false,
            last_viewport_width: 1280.0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Sidebar".to_owned()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Character(c),
                modifiers,
                ..
            }) => {
                let ctrl_or_meta =
                    modifiers.contains(Modifiers::CTRL) || modifiers.contains(Modifiers::COMMAND);
                let ch = c.chars().next().unwrap_or('\0');
                if matches_sidebar_shortcut(ch, ctrl_or_meta) {
                    Some(Message::Shortcut)
                } else {
                    None
                }
            }
            _ => None,
        })
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
            Message::Collapsible(value) => {
                self.collapsible = value.0;
            }
            Message::Variant(value) => {
                self.variant = value.0;
            }
            Message::Side(value) => {
                self.side = value.0;
            }
            Message::Toggle | Message::Shortcut => {
                self.controller.toggle();
            }
            Message::MobileOpen(open) => {
                self.controller.set_open_mobile(open);
            }
            Message::ToggleMobileSim => {
                self.force_mobile = !self.force_mobile;
                if self.force_mobile {
                    self.controller.set_is_mobile(true);
                } else {
                    self.controller.set_viewport_width(self.last_viewport_width);
                }
            }
            Message::ToggleNav(index) => {
                if let Some(flag) = self.open_nav.get_mut(index) {
                    *flag = !*flag;
                }
            }
            Message::NavSub(route) => {
                self.active_sub = route;
            }
            Message::Viewport(width) => {
                self.last_viewport_width = width;
                if !self.force_mobile {
                    self.controller.set_viewport_width(width);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;
        let ctrl = &self.controller;
        let icon_mode = ctrl.is_collapsed() && self.collapsible == SidebarCollapsible::Icon;

        let sidebar = Sidebar::new(ctrl, theme)
            .side(self.side)
            .variant(self.variant)
            .collapsible(self.collapsible)
            .on_mobile_open_change(Message::MobileOpen)
            .header(SidebarHeader::new(theme).push(team_switcher(theme, ctrl, self.collapsible)))
            .content(
                SidebarContent::new(theme)
                    .push(nav_main(self, theme, ctrl, icon_mode))
                    .push(nav_projects(theme, ctrl, self.collapsible, icon_mode)),
            )
            .footer(SidebarFooter::new(theme).push(nav_user(theme, ctrl, self.collapsible)))
            .rail(SidebarRail::new(theme).on_press(Message::Toggle));

        let inset_header = container(
            row![
                SidebarTrigger::new(theme).on_press(Message::Toggle),
                Separator::new(theme)
                    .orientation(SeparatorOrientation::Vertical)
                    .length(Length::Fixed(16.0)),
                Breadcrumb::new(theme)
                    .push(BreadcrumbLink::text("Build Your Application", theme))
                    .push(BreadcrumbPage::text("Data Fetching", theme)),
            ]
            .spacing(8)
            .padding(Padding {
                top: 0.0,
                right: 16.0,
                bottom: 0.0,
                left: 16.0,
            })
            .align_y(Alignment::Center)
            .height(Length::Fixed(64.0)),
        )
        .width(Length::Fill)
        .height(Length::Fixed(64.0));

        let muted_card = |flex: Length| {
            container(Space::new())
                .width(flex)
                .height(Length::Fixed(128.0))
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color { a: 0.5, ..p.muted })),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 12.0.into(),
                    },
                    ..container::Style::default()
                })
        };

        let cards = row![
            muted_card(Length::Fill),
            muted_card(Length::Fill),
            muted_card(Length::Fill),
        ]
        .spacing(16)
        .width(Length::Fill);

        let big_panel = container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(Color { a: 0.5, ..p.muted })),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 12.0.into(),
                },
                ..container::Style::default()
            });

        let controls = theme_controls(self, theme);

        let inset = SidebarInset::new(theme)
            .variant(self.variant)
            .header(inset_header)
            .push(
                column![
                    container(column![cards, big_panel].spacing(16).height(Length::Fill))
                        .padding(Padding {
                            top: 0.0,
                            right: 16.0,
                            bottom: 16.0,
                            left: 16.0,
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),
                    container(controls)
                        .padding(16)
                        .width(Length::Fill)
                        .style(move |_| container::Style {
                            background: Some(Background::Color(p.background)),
                            border: Border {
                                color: p.border,
                                width: 1.0,
                                radius: 0.0.into(),
                            },
                            ..container::Style::default()
                        }),
                ]
                .height(Length::Fill),
            );

        let shell = if matches!(self.side, SidebarSide::Right) {
            row![inset, sidebar].height(Length::Fill)
        } else {
            row![sidebar, inset].height(Length::Fill)
        };

        SidebarProvider::new(theme)
            .on_viewport_change(Message::Viewport)
            .push(shell)
            .into()
    }
}

fn team_switcher<'a>(
    theme: &'a Theme,
    ctrl: &'a SidebarController,
    collapsible: SidebarCollapsible,
) -> Element<'a, Message> {
    let p = &theme.palette;
    // Web: `aspect-square size-8 rounded-lg` — same footprint as NavUser avatar.
    let logo = Avatar::new(theme)
        .size(AvatarSize::Custom(32.0))
        .radius(AvatarRadius::Large)
        .fallback(
            AvatarFallback::text("A", theme)
                .color(p.sidebar_primary_foreground)
                .background(p.sidebar_primary),
        );

    SidebarMenu::new(theme)
        .push(
            SidebarMenuItem::new(theme).push(
                SidebarMenuButton::text("Acme Inc", ctrl, theme)
                    .subtitle("Enterprise")
                    .size(SidebarMenuButtonSize::Lg)
                    .collapsible(collapsible)
                    .tooltip("Acme Inc")
                    .leading_icon(logo)
                    .trailing_icon(chevron_up_down(p.sidebar_foreground)),
            ),
        )
        .into()
}

fn nav_user<'a>(
    theme: &'a Theme,
    ctrl: &'a SidebarController,
    collapsible: SidebarCollapsible,
) -> Element<'a, Message> {
    let p = &theme.palette;
    let avatar = Avatar::new(theme)
        .size(AvatarSize::Custom(32.0))
        .radius(AvatarRadius::Large)
        .fallback(
            AvatarFallback::text("CN", theme)
                .color(p.sidebar_primary_foreground)
                .background(p.sidebar_primary),
        );

    SidebarMenu::new(theme)
        .push(
            SidebarMenuItem::new(theme).push(
                SidebarMenuButton::text("shadcn", ctrl, theme)
                    .subtitle("m@example.com")
                    .size(SidebarMenuButtonSize::Lg)
                    .collapsible(collapsible)
                    .tooltip("shadcn")
                    .leading_icon(avatar)
                    .trailing_icon(chevron_up_down(p.sidebar_foreground)),
            ),
        )
        .into()
}

fn nav_main<'a>(
    state: &'a Example,
    theme: &'a Theme,
    ctrl: &'a SidebarController,
    icon_mode: bool,
) -> Element<'a, Message> {
    let mut menu = SidebarMenu::new(theme);
    for (index, entry) in NAV_MAIN.iter().enumerate() {
        menu = menu.push(nav_main_item(state, theme, ctrl, index, entry));
    }

    SidebarGroup::new(theme)
        .icon_mode(icon_mode)
        .label(SidebarGroupLabel::text("Platform", theme))
        .content(SidebarGroupContent::new(theme).push(menu))
        .into()
}

fn nav_main_item<'a>(
    state: &'a Example,
    theme: &'a Theme,
    ctrl: &'a SidebarController,
    index: usize,
    entry: &'a NavEntry,
) -> Element<'a, Message> {
    let p = &theme.palette;
    let open = state.open_nav.get(index).copied().unwrap_or(false);
    let chevron = if open { "▾" } else { "▸" };

    let mut item = SidebarMenuItem::new(theme).push(
        SidebarMenuButton::text(entry.title, ctrl, theme)
            .collapsible(state.collapsible)
            .tooltip(entry.title)
            .leading_icon(text(entry.glyph).size(14).color(p.sidebar_foreground))
            .trailing_icon(text(chevron).size(14).color(p.sidebar_foreground))
            .on_press(Message::ToggleNav(index)),
    );

    if open {
        let mut sub = SidebarMenuSub::new(ctrl, theme);
        for &sub_title in entry.items {
            sub = sub.push(
                SidebarMenuSubItem::new(theme).push(
                    SidebarMenuSubButton::text(sub_title, theme)
                        .active(state.active_sub == sub_title)
                        .on_press(Message::NavSub(sub_title)),
                ),
            );
        }
        item = item.push(sub);
    }

    item.into()
}

fn nav_projects<'a>(
    theme: &'a Theme,
    ctrl: &'a SidebarController,
    collapsible: SidebarCollapsible,
    icon_mode: bool,
) -> Element<'a, Message> {
    if icon_mode {
        return Space::new().width(0).height(0).into();
    }

    let p = &theme.palette;
    let mut menu = SidebarMenu::new(theme);
    for &(name, glyph) in &PROJECTS {
        menu = menu.push(
            SidebarMenuItem::new(theme).push(
                SidebarMenuButton::text(name, ctrl, theme)
                    .collapsible(collapsible)
                    .tooltip(name)
                    .leading_icon(text(glyph).size(14).color(p.sidebar_foreground))
                    .on_press(Message::NavSub(name)),
            ),
        );
    }
    menu = menu.push(
        SidebarMenuItem::new(theme).push(
            SidebarMenuButton::text("More", ctrl, theme)
                .collapsible(collapsible)
                .leading_icon(text("···").size(14).color(Color {
                    a: 0.7,
                    ..p.sidebar_foreground
                })),
        ),
    );

    SidebarGroup::new(theme)
        .icon_mode(false)
        .label(SidebarGroupLabel::text("Projects", theme))
        .content(SidebarGroupContent::new(theme).push(menu))
        .into()
}

fn chevron_up_down<'a, Message: 'a>(color: Color) -> Element<'a, Message> {
    text("⇅").size(14).color(color).into()
}

fn theme_controls<'a>(state: &'a Example, theme: &'a Theme) -> Element<'a, Message> {
    let p = &theme.palette;
    column![
        section_label(
            "Theme (shadcn-common)",
            p.muted_foreground,
            theme.font_pack()
        ),
        row![
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
        ]
        .spacing(12),
        row![
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
        .spacing(12),
        section_label("Sidebar", p.muted_foreground, theme.font_pack()),
        row![
            control_select(
                "Collapse",
                &COLLAPSIBLES,
                Some(Labelled(state.collapsible)),
                Message::Collapsible,
                theme,
            ),
            control_select(
                "Variant",
                &VARIANTS,
                Some(Labelled(state.variant)),
                Message::Variant,
                theme,
            ),
            control_select(
                "Side",
                &SIDES,
                Some(Labelled(state.side)),
                Message::Side,
                theme,
            ),
        ]
        .spacing(12),
        text(format!(
            "open={} · mobile={} · active={} · Ctrl+B toggles",
            state.controller.open(),
            state.controller.is_mobile(),
            state.active_sub,
        ))
        .size(12)
        .font(iced_font(theme.font_pack().mono))
        .color(p.muted_foreground),
        iced_shadcn_v2::Button::text(
            if state.force_mobile {
                "Disable mobile simulation"
            } else {
                "Simulate mobile sheet"
            },
            theme,
        )
        .variant(iced_shadcn_v2::ButtonVariant::Outline)
        .on_press(Message::ToggleMobileSim),
    ]
    .spacing(8)
    .width(Length::Fill)
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
            .width(64)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(140.0))
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
        .size(14)
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

impl fmt::Display for Labelled<SidebarCollapsible> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
            _ => "unknown",
        })
    }
}

impl fmt::Display for Labelled<SidebarVariant> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
            _ => "unknown",
        })
    }
}

impl fmt::Display for Labelled<SidebarSide> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
            _ => "unknown",
        })
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

const COLLAPSIBLES: [Labelled<SidebarCollapsible>; 3] = [
    Labelled(SidebarCollapsible::Offcanvas),
    Labelled(SidebarCollapsible::Icon),
    Labelled(SidebarCollapsible::None),
];

const VARIANTS: [Labelled<SidebarVariant>; 3] = [
    Labelled(SidebarVariant::Sidebar),
    Labelled(SidebarVariant::Floating),
    Labelled(SidebarVariant::Inset),
];

const SIDES: [Labelled<SidebarSide>; 2] =
    [Labelled(SidebarSide::Left), Labelled(SidebarSide::Right)];
