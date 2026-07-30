//! Interactive playground for `iced-shadcn-v2::NavigationMenu`.
//!
//! Mirrors the shadcn-svelte navigation-menu demos (with / without viewport)
//! and adds theme knobs like the button example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example navigation_menu`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, FontHeading, FontId, FontPack, NavigationMenu, NavigationMenuContentProps,
    NavigationMenuItem, NavigationMenuLinkProps, NavigationMenuListProps, NavigationMenuWrap,
    RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font, navigation_menu_content,
    navigation_menu_link, navigation_menu_trigger_style,
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
    open_value: String,
    last_action: Option<&'static str>,
    with_viewport: bool,
    show_indicator: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Heading(Labelled<FontHeading>),
    Font(Labelled<FontId>),
    Radius(Labelled<RadiusId>),
    OpenChanged(String),
    Navigate(&'static str),
    ToggleViewport,
    ToggleIndicator,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            open_value: String::new(),
            last_action: None,
            with_viewport: true,
            show_indicator: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 NavigationMenu".to_owned()
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
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::OpenChanged(value) => {
                self.open_value = value;
            }
            Message::Navigate(label) => {
                self.last_action = Some(label);
            }
            Message::ToggleViewport => {
                self.with_viewport = !self.with_viewport;
            }
            Message::ToggleIndicator => {
                self.show_indicator = !self.show_indicator;
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
            row![
                toggle_chip(
                    if self.with_viewport {
                        "Viewport: on"
                    } else {
                        "Viewport: off"
                    },
                    Message::ToggleViewport,
                    theme,
                ),
                toggle_chip(
                    if self.show_indicator {
                        "Pointer diamond: on"
                    } else {
                        "Pointer diamond: off"
                    },
                    Message::ToggleIndicator,
                    theme,
                ),
            ]
            .spacing(8),
            text("Pointer = diamond between open trigger and panel")
                .size(11)
                .color(p.muted_foreground),
        ]
        .spacing(8);

        let open = if self.open_value.is_empty() {
            None
        } else {
            Some(self.open_value.as_str())
        };

        let menu = if self.with_viewport {
            with_viewport_menu(theme, open)
        } else {
            without_viewport_menu(theme, open)
        }
        .indicator(self.show_indicator)
        .list_props(NavigationMenuListProps::new().wrap(NavigationMenuWrap::Wrap))
        .on_value_change(Message::OpenChanged);

        let status = text(format!(
            "open={:?} · last={}",
            if self.open_value.is_empty() {
                None
            } else {
                Some(self.open_value.as_str())
            },
            self.last_action.unwrap_or("—"),
        ))
        .size(12)
        .font(iced_font(theme.font_pack().mono))
        .color(p.muted_foreground);

        let body = column![
            section_label(
                if self.with_viewport {
                    "With Viewport"
                } else {
                    "Without Viewport"
                },
                p.foreground,
                theme.font_pack(),
            ),
            menu,
            status,
        ]
        .spacing(16);

        let content = row![
            container(controls)
                .padding(16)
                .width(Length::Fixed(280.0))
                .style(move |_| container::Style {
                    background: Some(Background::Color(p.card)),
                    border: Border {
                        color: p.border,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..container::Style::default()
                }),
            container(body).padding(24).width(Length::Fill),
        ]
        .spacing(16)
        .padding(16);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(p.background)),
                ..container::Style::default()
            })
            .into()
    }
}

fn with_viewport_menu<'a>(theme: &'a Theme, open: Option<&'a str>) -> NavigationMenu<'a, Message> {
    let hero = container(
        column![
            text("shadcn/ui").size(16),
            text("Beautifully designed components built with Tailwind CSS.")
                .size(12)
                .color(theme.palette.muted_foreground),
        ]
        .spacing(6),
    )
    .padding(16)
    .width(Length::FillPortion(1))
    .style(move |_| container::Style {
        background: Some(Background::Color(theme.palette.muted)),
        border: Border {
            color: theme.palette.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    });

    let getting_started = row![
        hero,
        column![
            list_item(
                theme,
                "Introduction",
                "Re-usable components built with Tailwind CSS."
            ),
            list_item(
                theme,
                "Installation",
                "How to install dependencies and structure your app.",
            ),
            list_item(
                theme,
                "Typography",
                "Styles for headings, paragraphs, lists...etc",
            ),
        ]
        .spacing(4)
        .width(Length::FillPortion(1)),
    ]
    .spacing(12)
    .width(Length::Fixed(520.0));

    let components = [
        (
            "Alert Dialog",
            "A modal dialog that interrupts the user with important content.",
        ),
        (
            "Hover Card",
            "For sighted users to preview content available behind a link.",
        ),
        (
            "Progress",
            "Displays an indicator showing the completion progress of a task.",
        ),
        ("Scroll-area", "Visually or semantically separates content."),
        (
            "Tabs",
            "A set of layered sections of content displayed one at a time.",
        ),
        (
            "Tooltip",
            "A popup that displays information related to an element.",
        ),
    ];

    let left = column![
        list_item(theme, components[0].0, components[0].1),
        list_item(theme, components[1].0, components[1].1),
        list_item(theme, components[2].0, components[2].1),
    ]
    .spacing(4);
    let right = column![
        list_item(theme, components[3].0, components[3].1),
        list_item(theme, components[4].0, components[4].1),
        list_item(theme, components[5].0, components[5].1),
    ]
    .spacing(4);
    let components_grid = row![left, right].spacing(12).width(Length::Fixed(560.0));

    NavigationMenu::new(theme)
        .viewport(true)
        .value_maybe(open.map(str::to_owned))
        .item(
            NavigationMenuItem::trigger("home", "Home").content(
                navigation_menu_content(getting_started, theme)
                    .props(NavigationMenuContentProps::new().width(540.0)),
            ),
        )
        .item(
            NavigationMenuItem::trigger("components", "Components").content(
                navigation_menu_content(components_grid, theme)
                    .props(NavigationMenuContentProps::new().width(580.0)),
            ),
        )
        .item(NavigationMenuItem::link(
            "docs",
            text("Docs").size(14),
            Some(Message::Navigate("Docs")),
        ))
        .item(
            NavigationMenuItem::trigger("list", "List").content(
                navigation_menu_content(
                    column![
                        list_item(theme, "Components", "Browse all components in the library."),
                        list_item(theme, "Documentation", "Learn how to use the library."),
                        list_item(theme, "Blog", "Read our latest blog posts."),
                    ]
                    .spacing(4)
                    .width(Length::Fixed(280.0)),
                    theme,
                )
                .props(NavigationMenuContentProps::new().width(300.0)),
            ),
        )
        .item(
            NavigationMenuItem::trigger("simple", "Simple").content(
                navigation_menu_content(
                    column![
                        simple_link(theme, "Components"),
                        simple_link(theme, "Documentation"),
                        simple_link(theme, "Blocks"),
                    ]
                    .spacing(2)
                    .width(Length::Fixed(200.0)),
                    theme,
                )
                .props(NavigationMenuContentProps::new().width(220.0)),
            ),
        )
        .item(
            NavigationMenuItem::trigger("icons", "With Icon").content(
                navigation_menu_content(
                    column![
                        icon_link(theme, "Backlog"),
                        icon_link(theme, "To Do"),
                        icon_link(theme, "Done"),
                    ]
                    .spacing(2)
                    .width(Length::Fixed(200.0)),
                    theme,
                )
                .props(NavigationMenuContentProps::new().width(220.0)),
            ),
        )
}

fn without_viewport_menu<'a>(
    theme: &'a Theme,
    open: Option<&'a str>,
) -> NavigationMenu<'a, Message> {
    let list = column![
        list_item(theme, "Components", "Browse all components in the library."),
        list_item(theme, "Documentation", "Learn how to use the library."),
        list_item(theme, "Blog", "Read our latest blog posts."),
    ]
    .spacing(4)
    .width(Length::Fixed(280.0));

    let simple = column![
        simple_link(theme, "Components"),
        simple_link(theme, "Documentation"),
        simple_link(theme, "Blocks"),
    ]
    .spacing(2)
    .width(Length::Fixed(200.0));

    let icons = column![
        icon_link(theme, "Backlog"),
        icon_link(theme, "To Do"),
        icon_link(theme, "Done"),
    ]
    .spacing(2)
    .width(Length::Fixed(200.0));

    NavigationMenu::new(theme)
        .viewport(false)
        .value_maybe(open.map(str::to_owned))
        .item(NavigationMenuItem::link(
            "docs",
            text("Documentation").size(14),
            Some(Message::Navigate("Documentation")),
        ))
        .item(
            NavigationMenuItem::trigger("list", "List").content(
                navigation_menu_content(list, theme)
                    .props(NavigationMenuContentProps::new().width(300.0)),
            ),
        )
        .item(
            NavigationMenuItem::trigger("simple", "Simple List").content(
                navigation_menu_content(simple, theme)
                    .props(NavigationMenuContentProps::new().width(220.0)),
            ),
        )
        .item(
            NavigationMenuItem::trigger("icons", "With Icon").content(
                navigation_menu_content(icons, theme)
                    .props(NavigationMenuContentProps::new().width(220.0)),
            ),
        )
}

fn list_item<'a>(
    theme: &'a Theme,
    title: &'static str,
    description: &'static str,
) -> Element<'a, Message> {
    let body = column![
        text(title).size(14).font(iced_font(theme.font_pack().sans)),
        text(description)
            .size(12)
            .color(theme.palette.muted_foreground),
    ]
    .spacing(4);

    navigation_menu_link(
        body,
        Some(Message::Navigate(title)),
        navigation_menu_trigger_style()
            .full_width(true)
            .padding(8.0),
        theme,
    )
}

fn simple_link<'a>(theme: &'a Theme, label: &'static str) -> Element<'a, Message> {
    // Full-width inside a fixed panel (same pattern as List / With Icon).
    // Overlay caps width via content_props, so Fill cannot grow to the window.
    navigation_menu_link(
        text(label).size(14),
        Some(Message::Navigate(label)),
        NavigationMenuLinkProps::new().full_width(true).padding(8.0),
        theme,
    )
}

fn icon_link<'a>(theme: &'a Theme, label: &'static str) -> Element<'a, Message> {
    let body = row![
        alert_circle_icon(theme.palette.foreground),
        text(label).size(14),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    navigation_menu_link(
        body,
        Some(Message::Navigate(label)),
        NavigationMenuLinkProps::new().full_width(true).padding(8.0),
        theme,
    )
}

/// Lucide-style `circle-alert` (matches the shadcn-svelte "With Icon" demo).
fn alert_circle_icon(color: Color) -> Element<'static, Message> {
    use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke};

    #[derive(Debug)]
    struct AlertCircle {
        color: Color,
    }

    impl<Message> Program<Message> for AlertCircle {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &iced::Renderer,
            _theme: &iced::Theme,
            bounds: iced::Rectangle,
            _cursor: iced::mouse::Cursor,
        ) -> Vec<Geometry> {
            let size = bounds.width.min(bounds.height);
            let mut frame = Frame::new(renderer, iced::Size::new(size, size));
            let center = iced::Point::new(size / 2.0, size / 2.0);
            let radius = size * 0.38;
            let stroke = Stroke::default()
                .with_width((size * 0.10).clamp(1.0, 1.75))
                .with_color(self.color)
                .with_line_cap(canvas::LineCap::Round)
                .with_line_join(canvas::LineJoin::Round);

            frame.stroke(&Path::circle(center, radius), stroke);
            frame.stroke(
                &Path::line(
                    iced::Point::new(center.x, center.y - radius * 0.45),
                    iced::Point::new(center.x, center.y + radius * 0.15),
                ),
                stroke,
            );
            frame.fill(
                &Path::circle(
                    iced::Point::new(center.x, center.y + radius * 0.42),
                    size * 0.06,
                ),
                self.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas::Canvas::new(AlertCircle { color })
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
        .into()
}

fn toggle_chip<'a>(label: &'a str, on_press: Message, theme: &'a Theme) -> Element<'a, Message> {
    iced::widget::button(text(label).size(12))
        .on_press(on_press)
        .padding([6, 10])
        .style(move |_, status| {
            let hovered = matches!(
                status,
                iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed
            );
            iced::widget::button::Style {
                background: Some(Background::Color(if hovered {
                    theme.palette.muted
                } else {
                    theme.palette.secondary
                })),
                text_color: theme.palette.foreground,
                border: Border {
                    color: theme.palette.border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..iced::widget::button::Style::default()
            }
        })
        .into()
}

fn section_label<'a>(label: &'a str, color: Color, fonts: FontPack) -> Element<'a, Message> {
    text(label)
        .size(13)
        .font(iced_font(fonts.sans))
        .color(color)
        .into()
}

fn control_select<'a, T>(
    label: &'a str,
    options: &'static [Labelled<T>],
    selected: Option<Labelled<T>>,
    on_select: impl Fn(Labelled<T>) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Copy + PartialEq + 'static,
    Labelled<T>: fmt::Display,
{
    column![
        text(label)
            .size(11)
            .color(theme.palette.muted_foreground)
            .font(iced_font(theme.font_pack().mono)),
        pick_list(options, selected, on_select).width(Length::Fill),
    ]
    .spacing(4)
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
