//! Interactive playground for `iced-shadcn-v2::Sheet` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte sheet demos (With Form, No Close Button, Sides)
//! and adds rows for dismiss behaviors and controlled open state — same
//! playground shape as the button / dialog examples.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example sheet`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, Input, Label, RadiusId, Sheet,
    SheetBody, SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle, StyleId, Theme,
    ThemeMode, fonts, iced_font,
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
    profile_open: bool,
    name_value: String,
    username_value: String,
    saved: u32,
    side: SheetSide,
    side_open: bool,
    controlled_open: bool,
    open_changes: u32,
    last_open: Option<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Heading(Labelled<FontHeading>),
    Font(Labelled<FontId>),
    Radius(Labelled<RadiusId>),
    ProfileOpenChanged(bool),
    NameChanged(String),
    UsernameChanged(String),
    SaveProfile,
    Side(Labelled<SheetSide>),
    SideOpenChanged(bool),
    ToggleControlled,
    ControlledOpenChanged(bool),
    OpenChanged(bool),
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            profile_open: false,
            name_value: "Pedro Duarte".to_owned(),
            username_value: "@peduarte".to_owned(),
            saved: 0,
            side: SheetSide::Right,
            side_open: false,
            controlled_open: false,
            open_changes: 0,
            last_open: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Sheet".to_owned()
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
            Message::ProfileOpenChanged(open) => {
                self.profile_open = open;
            }
            Message::NameChanged(value) => {
                self.name_value = value;
            }
            Message::UsernameChanged(value) => {
                self.username_value = value;
            }
            Message::SaveProfile => {
                self.saved += 1;
                self.profile_open = false;
            }
            Message::Side(side) => {
                self.side = side.0;
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

        // shadcn-svelte "With Form": outline Open trigger, Edit-profile
        // header, two labeled inputs, Save + Close footer.
        let with_form =
            row![
                Sheet::new(
                    Button::text("Open", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    column![
                    Element::<Message>::from(
                        SheetHeader::new(theme)
                            .title(SheetTitle::text("Edit profile", theme))
                            .description(SheetDescription::text(
                                "Make changes to your profile here. Click save when you're done.",
                                theme,
                            ))
                    ),
                    Element::<Message>::from(
                        SheetBody::new(theme)
                            .push(form_field("Name", &self.name_value, Message::NameChanged, theme))
                            .push(form_field(
                                "Username",
                                &self.username_value,
                                Message::UsernameChanged,
                                theme,
                            ))
                    ),
                ],
                    theme,
                )
                .open(self.profile_open)
                .on_open_change(Message::ProfileOpenChanged)
                .footer(
                    SheetFooter::new(theme)
                        .push(
                            Button::text("Save changes", theme)
                                .on_press(Message::SaveProfile)
                                .width(Length::Fill),
                        )
                        .push(
                            Button::text("Close", theme)
                                .variant(ButtonVariant::Outline)
                                .on_press(Message::ProfileOpenChanged(false))
                                .width(Length::Fill),
                        ),
                ),
                text(format!("saved {} time(s)", self.saved))
                    .size(12)
                    .font(iced_font(theme.font_pack().mono))
                    .color(p.muted_foreground),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

        let no_close = Sheet::new(
            Button::text("No Close Button", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            SheetHeader::new(theme)
                .title(SheetTitle::text("No Close Button", theme))
                .description(SheetDescription::text(
                    "This sheet doesn't have a close button in the top-right corner. You can only close it using the backdrop or Esc.",
                    theme,
                )),
            theme,
        )
        .show_close_button(false);

        let sides = column![
            control_select(
                "Side",
                &SIDES,
                Some(Labelled(self.side)),
                Message::Side,
                theme,
            ),
            row![
                {
                    let mut sheet = Sheet::new(
                        Button::text(format!("{:?}", self.side).to_lowercase(), theme)
                            .variant(ButtonVariant::Outline)
                            .on_press(Message::Pressed),
                        column![
                            Element::<Message>::from(
                                SheetHeader::new(theme)
                                    .title(SheetTitle::text("Edit profile", theme))
                                    .description(SheetDescription::text(
                                        "Make changes to your profile here. Click save when you're done.",
                                        theme,
                                    ))
                            ),
                            Element::<Message>::from(
                                SheetBody::new(theme).push(
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
                    .side(self.side)
                    .open(self.side_open)
                    .on_open_change(Message::SideOpenChanged)
                    .footer(
                        SheetFooter::new(theme)
                            .push(
                                Button::text("Save changes", theme)
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

                    if self.side.is_horizontal_edge() {
                        sheet = sheet.max_height(400.0);
                    }

                    sheet
                },
            ]
            .spacing(12),
        ]
        .spacing(12);

        let behaviors = row![
            behavior_sheet(
                "Keeps open outside",
                "Backdrop clicks are ignored; press Esc or the X to close.",
                theme,
            )
            .close_on_click_outside(false),
            behavior_sheet(
                "No Esc",
                "Esc is ignored; click the backdrop or the X to close.",
                theme,
            )
            .close_on_escape(false),
            behavior_sheet(
                "Non-modal",
                "The window behind stays interactive (modal = false).",
                theme,
            )
            .modal(false),
            Sheet::new(
                Button::text("Disabled", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                SheetHeader::new(theme).title(SheetTitle::text("You should never see this", theme)),
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
                Sheet::new(
                    Button::text("Controlled trigger", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    SheetHeader::new(theme)
                        .title(SheetTitle::text("Controlled", theme))
                        .description(SheetDescription::text(
                            "Open state lives in the app; every request is reported.",
                            theme,
                        )),
                    theme,
                )
                .open(self.controlled_open)
                .on_open_change(Message::ControlledOpenChanged),
                Sheet::new(
                    Button::text("Watch onOpenChange", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    SheetHeader::new(theme)
                        .title(SheetTitle::text("Observed", theme))
                        .description(SheetDescription::text(
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
                "onOpenChange fired {} time(s), last = {}",
                self.open_changes,
                self.last_open
                    .map_or_else(|| "—".to_owned(), |open| open.to_string()),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(12);

        let content = column![
            text("iced-shadcn-v2 Sheet")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte Sheet.Root / Trigger / Content + Header, Footer, Close, ported onto an edge-docked iced overlay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("With Form", p.muted_foreground, theme.font_pack()),
            with_form,
            section_label("No Close Button", p.muted_foreground, theme.font_pack()),
            no_close,
            section_label("Sides", p.muted_foreground, theme.font_pack()),
            sides,
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

fn behavior_sheet<'a>(
    label: &'static str,
    description: &'static str,
    theme: &'a Theme,
) -> Sheet<'a, Message> {
    Sheet::new(
        Button::text(label, theme)
            .variant(ButtonVariant::Ghost)
            .on_press(Message::Pressed),
        SheetHeader::new(theme)
            .title(SheetTitle::text(label, theme))
            .description(SheetDescription::text(description, theme)),
        theme,
    )
}

fn form_field<'a>(
    label: &'static str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        Element::<Message>::from(Label::text(label, theme)),
        Input::new(theme)
            .value(value)
            .on_input(on_input)
            .width(Length::Fill),
    ]
    .spacing(12)
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

impl fmt::Display for Labelled<SheetSide> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            SheetSide::Top => f.write_str("top"),
            SheetSide::Right => f.write_str("right"),
            SheetSide::Bottom => f.write_str("bottom"),
            SheetSide::Left => f.write_str("left"),
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

const SIDES: [Labelled<SheetSide>; 4] = [
    Labelled(SheetSide::Top),
    Labelled(SheetSide::Right),
    Labelled(SheetSide::Bottom),
    Labelled(SheetSide::Left),
];
