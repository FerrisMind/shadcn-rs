//! Interactive playground for `iced-shadcn-v2::Toaster` + `shadcn-common` theme knobs.
//!
//! Demonstrates the sonner toast notification system with all toast types,
//! positions, and theme configurations.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example sonner`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, FontHeading, FontId, RadiusId, StyleId, Theme, ThemeMode, ToastAction,
    ToastPosition, ToastType, Toaster, fonts, iced_font, toast, toast_error, toast_info,
    toast_loading, toast_success, toast_warning,
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
    position: ToastPosition,
    rich_colors: bool,
    close_button: bool,
    duration_ms: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Position(Labelled<ToastPosition>),
    ToggleRichColors,
    ToggleCloseButton,
    Duration(u64),
    ShowDefault,
    ShowSuccess,
    ShowInfo,
    ShowWarning,
    ShowError,
    ShowLoading,
    ShowWithDescription,
    ShowWithAction,
    ShowWithCancel,
    DismissAll,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            position: ToastPosition::BottomRight,
            rich_colors: false,
            close_button: false,
            duration_ms: 4000,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Sonner".to_owned()
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
            Message::Position(pos) => {
                self.position = pos.0;
            }
            Message::ToggleRichColors => {
                self.rich_colors = !self.rich_colors;
            }
            Message::ToggleCloseButton => {
                self.close_button = !self.close_button;
            }
            Message::Duration(d) => {
                self.duration_ms = d;
            }
            Message::ShowDefault => {
                toast("Event has been created").show();
            }
            Message::ShowSuccess => {
                toast_success("Event has been created");
            }
            Message::ShowInfo => {
                toast_info("Be at the area 10 minutes before the event time");
            }
            Message::ShowWarning => {
                toast_warning("Event start time cannot be earlier than 8am");
            }
            Message::ShowError => {
                toast_error("Event has not been created");
            }
            Message::ShowLoading => {
                toast_loading("Loading...");
            }
            Message::ShowWithDescription => {
                toast("Event has been created")
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .show();
            }
            Message::ShowWithAction => {
                toast("Event has been created")
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .action(ToastAction::label("Undo"))
                    .show();
            }
            Message::ShowWithCancel => {
                toast("Event has been created")
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .cancel(ToastAction::label("Cancel"))
                    .show();
            }
            Message::DismissAll => {
                iced_shadcn_v2::dismiss_all_toasts();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        // Theme controls
        let controls = column![
            section_label("Theme", p.muted_foreground, theme.font_pack()),
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
                "Position",
                &POSITIONS,
                Some(Labelled(self.position)),
                Message::Position,
                theme,
            ),
        ]
        .spacing(8)
        .width(Length::Fill);

        // Toast type buttons
        let type_label = section_label("Toast Types", p.muted_foreground, theme.font_pack());
        let type_buttons = row![
            padded_button(theme, "Default", Message::ShowDefault),
            padded_button(theme, "Success", Message::ShowSuccess),
            padded_button(theme, "Info", Message::ShowInfo),
            padded_button(theme, "Warning", Message::ShowWarning),
            padded_button(theme, "Error", Message::ShowError),
            padded_button(theme, "Loading", Message::ShowLoading),
        ]
        .spacing(8)
        .wrap();

        // Feature toggles
        let feature_label = section_label("Features", p.muted_foreground, theme.font_pack());
        let rich_colors_text = if self.rich_colors { "On" } else { "Off" };
        let close_btn_text = if self.close_button { "On" } else { "Off" };

        let features = row![
            padded_button(
                theme,
                &format!("Rich Colors: {rich_colors_text}"),
                Message::ToggleRichColors,
            ),
            padded_button(
                theme,
                &format!("Close Button: {close_btn_text}"),
                Message::ToggleCloseButton,
            ),
            padded_button(theme, "Dismiss All", Message::DismissAll),
        ]
        .spacing(8);

        // Duration selector
        let duration_label = section_label("Duration", p.muted_foreground, theme.font_pack());
        let durations = row![
            padded_button(theme, "2s", Message::Duration(2000)),
            padded_button(theme, "4s", Message::Duration(4000)),
            padded_button(theme, "6s", Message::Duration(6000)),
            padded_button(theme, "10s", Message::Duration(10000)),
            padded_button(theme, "Inf", Message::Duration(u64::MAX)),
        ]
        .spacing(8);

        //复合 buttons (with description, action, cancel)
        let compound_label =
            section_label("Compound Toasts", p.muted_foreground, theme.font_pack());
        let compound_buttons = row![
            padded_button(theme, "With Description", Message::ShowWithDescription),
            padded_button(theme, "With Action", Message::ShowWithAction),
            padded_button(theme, "With Cancel", Message::ShowWithCancel),
        ]
        .spacing(8);

        // Toaster widget (must be placed in the view tree)
        let toaster = Toaster::new(theme)
            .position(self.position)
            .duration(self.duration_ms)
            .rich_colors(self.rich_colors)
            .close_button(self.close_button);

        let content = column![
            controls,
            section_label("", p.muted_foreground, theme.font_pack()),
            type_label,
            type_buttons,
            section_label("", p.muted_foreground, theme.font_pack()),
            feature_label,
            features,
            section_label("", p.muted_foreground, theme.font_pack()),
            duration_label,
            durations,
            section_label("", p.muted_foreground, theme.font_pack()),
            compound_label,
            compound_buttons,
        ]
        .spacing(4)
        .width(Length::Fill)
        .padding(24);

        column![toaster.into(), scrollable(content)]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// --- Helper types ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            StyleId::Vega => write!(f, "Vega"),
            StyleId::Nova => write!(f, "Nova"),
            StyleId::Maia => write!(f, "Maia"),
            StyleId::Lyra => write!(f, "Lyra"),
            StyleId::Mira => write!(f, "Mira"),
            StyleId::Luma => write!(f, "Luma"),
            StyleId::Sera => write!(f, "Sera"),
            StyleId::Rhea => write!(f, "Rhea"),
            _ => write!(f, "Other"),
        }
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            BaseColor::Neutral => write!(f, "Neutral"),
            BaseColor::Gray => write!(f, "Gray"),
            BaseColor::Slate => write!(f, "Slate"),
            BaseColor::Zinc => write!(f, "Zinc"),
            BaseColor::Stone => write!(f, "Stone"),
            BaseColor::Blue => write!(f, "Blue"),
            BaseColor::Green => write!(f, "Green"),
            _ => write!(f, "Other"),
        }
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::Dark => write!(f, "Dark"),
            _ => write!(f, "Other"),
        }
    }
}

impl fmt::Display for Labelled<ToastPosition> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ToastPosition::BottomRight => write!(f, "Bottom Right"),
            ToastPosition::BottomLeft => write!(f, "Bottom Left"),
            ToastPosition::BottomCenter => write!(f, "Bottom Center"),
            ToastPosition::TopRight => write!(f, "Top Right"),
            ToastPosition::TopLeft => write!(f, "Top Left"),
            ToastPosition::TopCenter => write!(f, "Top Center"),
            _ => write!(f, "Other"),
        }
    }
}

// --- Static option arrays ---

static STYLES: &[Labelled<StyleId>] = &[
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

static BASES: &[Labelled<BaseColor>] = &[
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Gray),
    Labelled(BaseColor::Slate),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Blue),
    Labelled(BaseColor::Green),
];

static MODES: &[Labelled<ThemeMode>] = &[Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

static POSITIONS: &[Labelled<ToastPosition>] = &[
    Labelled(ToastPosition::BottomRight),
    Labelled(ToastPosition::BottomLeft),
    Labelled(ToastPosition::BottomCenter),
    Labelled(ToastPosition::TopRight),
    Labelled(ToastPosition::TopLeft),
    Labelled(ToastPosition::TopCenter),
];

// --- UI helpers ---

fn section_label<'a>(
    label: &'a str,
    color: iced::Color,
    font_pack: &iced_shadcn_v2::FontPack,
) -> Element<'a, Message> {
    if label.is_empty() {
        return container(text(""))
            .width(Length::Fill)
            .height(Length::Fixed(4.0))
            .into();
    }

    text(label)
        .size(14)
        .color(color)
        .font(iced_font(font_pack.sans))
        .into()
}

fn control_select<'a, T: Clone + PartialEq + 'static>(
    label: &'a str,
    options: &'a [T],
    selected: Option<T>,
    on_select: fn(T) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    row![
        text(label)
            .width(Length::Fixed(80.0))
            .size(13)
            .color(theme.palette.foreground)
            .font(iced_font(theme.font_pack().sans)),
        pick_list(options, selected, on_select)
            .width(Length::Fixed(160.0))
            .text_size(13),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn padded_button<'a>(theme: &Theme, label: &'a str, message: Message) -> Element<'a, Message> {
    iced::widget::button(
        text(label)
            .size(13)
            .color(theme.palette.foreground)
            .font(iced_font(theme.font_pack().sans)),
    )
    .padding([6, 12])
    .on_press(message)
    .into()
}
