//! Interactive playground for `iced-shadcn-v2::Drawer` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte drawer demos (Scrollable Content, Sides) and adds
//! rows for dismiss behaviors, snap points, and controlled open state — same
//! playground shape as the button / sheet examples.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example drawer`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, Drawer, DrawerBody, DrawerDescription, DrawerDirection,
    DrawerFooter, DrawerHeader, DrawerTitle, FontHeading, FontId, FontPack, RadiusId, ScrollArea,
    StyleId, Theme, ThemeMode, fonts, iced_font,
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
    scrollable_open: bool,
    direction: DrawerDirection,
    side_open: bool,
    controlled_open: bool,
    open_changes: u32,
    last_open: Option<bool>,
    snap_point: Option<f32>,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Heading(Labelled<FontHeading>),
    Font(Labelled<FontId>),
    Radius(Labelled<RadiusId>),
    ScrollableOpenChanged(bool),
    Direction(Labelled<DrawerDirection>),
    SideOpenChanged(bool),
    ToggleControlled,
    ControlledOpenChanged(bool),
    OpenChanged(bool),
    SnapPointChanged(Option<f32>),
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            scrollable_open: false,
            direction: DrawerDirection::Bottom,
            side_open: false,
            controlled_open: false,
            open_changes: 0,
            last_open: None,
            snap_point: Some(0.5),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Drawer".to_owned()
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
            Message::ScrollableOpenChanged(open) => {
                self.scrollable_open = open;
            }
            Message::Direction(direction) => {
                self.direction = direction.0;
            }
            Message::SideOpenChanged(open) => {
                self.side_open = open;
            }
            Message::ToggleControlled => {
                self.controlled_open = !self.controlled_open;
            }
            Message::ControlledOpenChanged(open) => {
                self.controlled_open = open;
            }
            Message::OpenChanged(open) => {
                self.open_changes += 1;
                self.last_open = Some(open);
            }
            Message::SnapPointChanged(point) => {
                self.snap_point = point;
            }
            Message::Pressed => {}
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
        ]
        .spacing(8);

        // shadcn-svelte "Scrollable Content": outline trigger, Move Goal
        // header, lorem body in ScrollArea, Submit + Cancel footer — direction right.
        let scrollable_demo = row![
            Drawer::new(
                Button::text("Scrollable Content", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                column![
                    Element::<Message>::from(
                        DrawerHeader::new(theme)
                            .center(false)
                            .title(DrawerTitle::text("Move Goal", theme))
                            .description(DrawerDescription::text(
                                "Set your daily activity goal.",
                                theme,
                            ))
                    ),
                    Element::<Message>::from(
                        ScrollArea::new(
                            DrawerBody::new(theme).push(
                                column(
                                    (0..10)
                                        .map(|_| {
                                            text(
                                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
                                            )
                                            .size(14)
                                            .font(iced_font(theme.font_pack().sans))
                                            .color(p.foreground)
                                            .into()
                                        })
                                        .collect::<Vec<_>>(),
                                )
                                .spacing(16),
                            ),
                            theme,
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .bordered(false),
                    ),
                ]
                .height(Length::Fill),
                theme,
            )
            .direction(DrawerDirection::Right)
            .open(self.scrollable_open)
            .on_open_change(Message::ScrollableOpenChanged)
            .footer(
                DrawerFooter::new(theme)
                    .push(
                        Button::text("Submit", theme)
                            .on_press(Message::ScrollableOpenChanged(false))
                            .width(Length::Fill),
                    )
                    .push(
                        Button::text("Cancel", theme)
                            .variant(ButtonVariant::Outline)
                            .on_press(Message::ScrollableOpenChanged(false))
                            .width(Length::Fill),
                    ),
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let sides = column![
            control_select(
                "Direction",
                &DIRECTIONS,
                Some(Labelled(self.direction)),
                Message::Direction,
                theme,
            ),
            row![{
                let mut drawer = Drawer::new(
                    Button::text(format!("{:?}", self.direction).to_lowercase(), theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    column![
                        Element::<Message>::from(
                            DrawerHeader::new(theme)
                                .center(self.direction.centers_header())
                                .title(DrawerTitle::text("Move Goal", theme))
                                .description(DrawerDescription::text(
                                    "Set your daily activity goal.",
                                    theme,
                                ))
                        ),
                        Element::<Message>::from(
                            DrawerBody::new(theme).push(
                                text(
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                                )
                                .size(14)
                                .font(iced_font(theme.font_pack().sans))
                                .color(p.foreground)
                            )
                        ),
                    ],
                    theme,
                )
                .direction(self.direction)
                .open(self.side_open)
                .on_open_change(Message::SideOpenChanged)
                .footer(
                    DrawerFooter::new(theme)
                        .push(
                            Button::text("Submit", theme)
                                .on_press(Message::SideOpenChanged(false))
                                .width(Length::Fill),
                        )
                        .push(
                            Button::text("Cancel", theme)
                                .variant(ButtonVariant::Outline)
                                .on_press(Message::SideOpenChanged(false))
                                .width(Length::Fill),
                        ),
                );

                if self.direction.is_horizontal_edge() {
                    drawer = drawer.max_height(400.0);
                }

                drawer
            }]
            .spacing(12),
        ]
        .spacing(12);

        let snap = Drawer::new(
            Button::text("Snap points (0.4 / 0.7)", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            DrawerHeader::new(theme)
                .center(true)
                .title(DrawerTitle::text("Snap drawer", theme))
                .description(DrawerDescription::text(
                    "Drag the handle; release near a snap point to settle.",
                    theme,
                )),
            theme,
        )
        .direction(DrawerDirection::Bottom)
        .snap_points([0.4, 0.7])
        .active_snap_point(self.snap_point)
        .on_snap_point_change(Message::SnapPointChanged)
        .on_open_change(Message::OpenChanged);

        let behaviors = row![
            behavior_drawer(
                "Keeps open outside",
                "Backdrop clicks are ignored; press Esc or drag to close.",
                theme,
            )
            .close_on_click_outside(false),
            behavior_drawer(
                "No Esc",
                "Esc is ignored; click the backdrop or drag to close.",
                theme,
            )
            .close_on_escape(false),
            behavior_drawer(
                "Non-modal",
                "The window behind stays interactive (modal = false).",
                theme,
            )
            .modal(false),
            Drawer::new(
                Button::text("Disabled", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                DrawerHeader::new(theme)
                    .title(DrawerTitle::text("You should never see this", theme)),
                theme,
            )
            .disabled(true),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let controlled = column![
            row![
                Button::text(
                    if self.controlled_open {
                        "Force close"
                    } else {
                        "Force open"
                    },
                    theme,
                )
                .variant(ButtonVariant::Default)
                .on_press(Message::ToggleControlled),
                Drawer::new(
                    Button::text("Controlled trigger", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    DrawerHeader::new(theme)
                        .center(true)
                        .title(DrawerTitle::text("Controlled", theme))
                        .description(DrawerDescription::text(
                            "Open state lives in the app; every request is reported.",
                            theme,
                        )),
                    theme,
                )
                .open(self.controlled_open)
                .on_open_change(Message::ControlledOpenChanged),
                Drawer::new(
                    Button::text("Watch onOpenChange", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    DrawerHeader::new(theme)
                        .center(true)
                        .title(DrawerTitle::text("Observed", theme))
                        .description(DrawerDescription::text(
                            "Every open and dismiss request is counted below.",
                            theme,
                        )),
                    theme,
                )
                .on_open_change(Message::OpenChanged),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            text(format!(
                "onOpenChange fired {} time(s), last = {}, snap = {:?}",
                self.open_changes,
                self.last_open
                    .map_or_else(|| "—".to_owned(), |open| open.to_string()),
                self.snap_point,
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(12);

        let content = column![
            text("iced-shadcn-v2 Drawer")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte Drawer.Root / Trigger / Content + Header, Footer, Close, ported onto a vaul-style iced overlay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Scrollable Content", p.muted_foreground, theme.font_pack()),
            scrollable_demo,
            section_label("Sides", p.muted_foreground, theme.font_pack()),
            sides,
            section_label("Snap points", p.muted_foreground, theme.font_pack()),
            snap,
            section_label(
                "Dismiss behaviors / disabled",
                p.muted_foreground,
                theme.font_pack()
            ),
            behaviors,
            section_label(
                "Controlled open + onOpenChange",
                p.muted_foreground,
                theme.font_pack()
            ),
            controlled,
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

fn behavior_drawer<'a>(
    label: &'static str,
    description: &'static str,
    theme: &'a Theme,
) -> Drawer<'a, Message> {
    Drawer::new(
        Button::text(label, theme)
            .variant(ButtonVariant::Ghost)
            .on_press(Message::Pressed),
        DrawerHeader::new(theme)
            .center(true)
            .title(DrawerTitle::text(label, theme))
            .description(DrawerDescription::text(description, theme)),
        theme,
    )
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

impl fmt::Display for Labelled<DrawerDirection> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            DrawerDirection::Top => f.write_str("top"),
            DrawerDirection::Right => f.write_str("right"),
            DrawerDirection::Bottom => f.write_str("bottom"),
            DrawerDirection::Left => f.write_str("left"),
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

const DIRECTIONS: [Labelled<DrawerDirection>; 4] = [
    Labelled(DrawerDirection::Top),
    Labelled(DrawerDirection::Right),
    Labelled(DrawerDirection::Bottom),
    Labelled(DrawerDirection::Left),
];
