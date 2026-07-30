//! Interactive playground for `iced-shadcn-v2::Dialog` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte dialog demo (outline "Open Dialog" trigger with
//! the Edit-profile form, Cancel/Save footer, and the top-right close button)
//! and adds rows for dismiss behaviors, the close button toggle, custom
//! width, and controlled open state.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example dialog`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, Dialog, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, FontHeading, FontId, FontPack, Input, Label, RadiusId, StyleId, Theme, ThemeMode,
    fonts, iced_font,
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
            controlled_open: false,
            open_changes: 0,
            last_open: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Dialog".to_owned()
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

        // The shadcn-svelte dialog demo: outline trigger, Edit-profile
        // header, two labeled inputs, Cancel + Save footer. Controlled so
        // the footer buttons can close it.
        let demo = row![
            Dialog::new(
                Button::text("Open Dialog", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                column![
                    Element::<Message>::from(
                        DialogHeader::new(theme)
                            .title(DialogTitle::text("Edit profile", theme))
                            .description(DialogDescription::text(
                                "Make changes to your profile here. Click save when you're done.",
                                theme,
                            ))
                    ),
                    column![
                        form_field("Name", &self.name_value, Message::NameChanged, theme),
                        form_field(
                            "Username",
                            &self.username_value,
                            Message::UsernameChanged,
                            theme
                        ),
                    ]
                    .spacing(16),
                ]
                .spacing(24),
                theme,
            )
            .max_width(425.0)
            .open(self.profile_open)
            .on_open_change(Message::ProfileOpenChanged)
            .footer(
                DialogFooter::new(theme)
                    .push(
                        Button::text("Cancel", theme)
                            .variant(ButtonVariant::Outline)
                            .on_press(Message::ProfileOpenChanged(false)),
                    )
                    .push(Button::text("Save changes", theme).on_press(Message::SaveProfile)),
            ),
            text(format!("saved {} time(s)", self.saved))
                .size(12)
                .font(iced_font(theme.font_pack().mono))
                .color(p.muted_foreground),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        // A plain uncontrolled dialog: header only, dismissed by the close
        // button, the backdrop, or Esc.
        let uncontrolled = row![Dialog::new(
            Button::text("Share", theme)
                .variant(ButtonVariant::Secondary)
                .on_press(Message::Pressed),
            DialogHeader::new(theme)
                .title(DialogTitle::text("Share link", theme))
                .description(DialogDescription::text(
                    "Anyone who has this link will be able to view this.",
                    theme,
                )),
            theme,
        )]
        .spacing(12);

        let behaviors = row![
            behavior_dialog(
                "No close button",
                "The top-right X is hidden (showCloseButton = false).",
                theme,
            )
            .show_close_button(false),
            behavior_dialog(
                "Keeps open outside",
                "Backdrop clicks are ignored; press Esc or the X to close.",
                theme,
            )
            .close_on_click_outside(false),
            behavior_dialog(
                "No Esc",
                "Esc is ignored; click the backdrop or the X to close.",
                theme,
            )
            .close_on_escape(false),
            behavior_dialog(
                "Non-modal",
                "The window behind stays interactive (modal = false).",
                theme,
            )
            .modal(false),
            behavior_dialog("Narrow (max 320)", "sm:max-w-[320px] equivalent.", theme,)
                .max_width(320.0),
            Dialog::new(
                Button::text("Disabled", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                DialogHeader::new(theme)
                    .title(DialogTitle::text("You should never see this", theme)),
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
                Dialog::new(
                    Button::text("Controlled trigger", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    DialogHeader::new(theme)
                        .title(DialogTitle::text("Controlled", theme))
                        .description(DialogDescription::text(
                            "Open state lives in the app; every request is reported.",
                            theme,
                        )),
                    theme,
                )
                .open(self.controlled_open)
                .on_open_change(Message::ControlledOpenChanged),
                Dialog::new(
                    Button::text("Watch onOpenChange", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    DialogHeader::new(theme)
                        .title(DialogTitle::text("Observed", theme))
                        .description(DialogDescription::text(
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
            text("iced-shadcn-v2 Dialog")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte Dialog.Root / Trigger / Content + Header, Footer, Close, ported onto a modal iced overlay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Basic (Edit profile)", p.muted_foreground, theme.font_pack()),
            demo,
            section_label("Uncontrolled", p.muted_foreground, theme.font_pack()),
            uncontrolled,
            section_label(
                "Dismiss behaviors / close button / width / disabled",
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

/// A dismiss-behavior demo dialog with a title/description explaining it.
fn behavior_dialog<'a>(
    label: &'static str,
    description: &'static str,
    theme: &'a Theme,
) -> Dialog<'a, Message> {
    Dialog::new(
        Button::text(label, theme)
            .variant(ButtonVariant::Ghost)
            .on_press(Message::Pressed),
        DialogHeader::new(theme)
            .title(DialogTitle::text(label, theme))
            .description(DialogDescription::text(description, theme)),
        theme,
    )
}

/// One vertical field of the Edit-profile form: label over input
/// (`grid gap-3`).
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
