//! Interactive playground for `iced-shadcn-v2::NativeSelect`.
//!
//! Run with: `cargo run -p iced-shadcn-v2 --example native_select`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, NativeSelect, NativeSelectItem, NativeSelectOptGroup,
    NativeSelectOption, NativeSelectSize, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    status: Option<String>,
    department: Option<String>,
    priority: Option<String>,
    invalid: bool,
    disabled: bool,
    changes: u32,
    last_event: String,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Radius(Labelled<RadiusId>),
    StatusChanged(String),
    DepartmentChanged(String),
    PriorityChanged(String),
    ToggleInvalid,
    ToggleDisabled,
    Opened,
    Closed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            status: Some("todo".to_owned()),
            department: None,
            priority: Some("medium".to_owned()),
            invalid: false,
            disabled: false,
            changes: 0,
            last_event: "No interaction yet".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Native Select".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Radius(radius) => self.theme = self.theme.clone().with_radius(radius.0),
            Message::StatusChanged(value) => {
                self.status = Some(value);
                self.changes += 1;
                self.last_event = "status changed".to_owned();
            }
            Message::DepartmentChanged(value) => {
                self.department = Some(value);
                self.changes += 1;
                self.last_event = "grouped option changed".to_owned();
            }
            Message::PriorityChanged(value) => {
                self.priority = Some(value);
                self.changes += 1;
                self.last_event = "priority changed".to_owned();
            }
            Message::ToggleInvalid => self.invalid = !self.invalid,
            Message::ToggleDisabled => self.disabled = !self.disabled,
            Message::Opened => self.last_event = "menu opened".to_owned(),
            Message::Closed => self.last_event = "menu closed".to_owned(),
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;
        let sans = iced_font(theme.font_pack().sans);

        let controls = column![
            section_label("Theme", palette.muted_foreground, theme),
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
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme
            ),
            text(format!(
                "pack={} · h(sm/default)={:.0}/{:.0}px · font={}",
                theme.style_id().as_str(),
                theme.style.control_height_sm_px,
                theme.style.control_height_md_px,
                theme.font_pack().sans.title(),
            ))
            .size(12)
            .font(sans)
            .color(palette.muted_foreground),
        ]
        .spacing(8);

        let status = NativeSelect::with_options(
            theme,
            [
                NativeSelectOption::new("todo".to_owned(), "Todo"),
                NativeSelectOption::new("in-progress".to_owned(), "In Progress"),
                NativeSelectOption::new("done".to_owned(), "Done"),
                NativeSelectOption::new("cancelled".to_owned(), "Cancelled").disabled(true),
            ],
            self.status.clone(),
        )
        .placeholder("Select status")
        .on_change(Message::StatusChanged)
        .on_open(Message::Opened)
        .on_close(Message::Closed)
        .width(Length::Fixed(230.0));

        let grouped = NativeSelect::with_items(
            theme,
            [
                NativeSelectItem::option(NativeSelectOption::new(
                    "unassigned".to_owned(),
                    "Unassigned",
                )),
                NativeSelectItem::opt_group(
                    NativeSelectOptGroup::new("Engineering")
                        .push(NativeSelectOption::new("frontend".to_owned(), "Frontend"))
                        .push(NativeSelectOption::new("backend".to_owned(), "Backend"))
                        .push(
                            NativeSelectOption::new("legacy".to_owned(), "Legacy").disabled(true),
                        ),
                ),
                NativeSelectItem::opt_group(
                    NativeSelectOptGroup::new("Operations")
                        .push(NativeSelectOption::new(
                            "support".to_owned(),
                            "Customer Support",
                        ))
                        .push(NativeSelectOption::new("ops".to_owned(), "Operations")),
                ),
            ],
            self.department.clone(),
        )
        .placeholder("Select department")
        .on_change(Message::DepartmentChanged)
        .menu_height(Length::Fixed(170.0))
        .width(Length::Fixed(230.0));

        let priority = NativeSelect::with_options(
            theme,
            [
                NativeSelectOption::new("low".to_owned(), "Low"),
                NativeSelectOption::new("medium".to_owned(), "Medium"),
                NativeSelectOption::new("high".to_owned(), "High"),
            ],
            self.priority.clone(),
        )
        .size(NativeSelectSize::Sm)
        .on_change(Message::PriorityChanged)
        .width(Length::Fixed(150.0));

        let states = row![
            Button::text("Toggle invalid", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleInvalid),
            Button::text("Toggle disabled", theme)
                .variant(ButtonVariant::Secondary)
                .on_press(Message::ToggleDisabled),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let shared_state = NativeSelect::with_options(
            theme,
            [
                NativeSelectOption::new("enabled".to_owned(), "Enabled"),
                NativeSelectOption::new("paused".to_owned(), "Paused"),
            ],
            Some("enabled".to_owned()),
        )
        .invalid(self.invalid)
        .disabled(self.disabled)
        .on_change(Message::StatusChanged)
        .width(Length::Fixed(230.0));

        let content = column![
            text("iced-shadcn-v2 Native Select")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Typed controlled values, placeholders, optgroups, disabled entries, and native keyboard navigation")
                .size(14)
                .font(sans)
                .color(palette.muted_foreground),
            controls,
            section_label("Basic / controlled", palette.muted_foreground, theme),
            row![text("Status").width(120), status]
                .spacing(12)
                .align_y(Alignment::Center),
            section_label("Optgroups and disabled options", palette.muted_foreground, theme),
            row![text("Department").width(120), grouped]
                .spacing(12)
                .align_y(Alignment::Center),
            section_label("Small size", palette.muted_foreground, theme),
            row![text("Priority").width(120), priority]
                .spacing(12)
                .align_y(Alignment::Center),
            section_label("Invalid / disabled state", palette.muted_foreground, theme),
            states,
            row![text("Shared state").width(120), shared_state]
                .spacing(12)
                .align_y(Alignment::Center),
            text(format!("Changes: {} · last event: {}", self.changes, self.last_event))
                .size(13)
                .font(sans)
                .color(palette.foreground),
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
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.label())
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

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
