//! Interactive playground for `iced_shadcn_v2::Toaster`.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example sonner`.

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, stack, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button as ShadcnButton, ButtonVariant, FontHeading, FontId, RadiusId,
    StyleId, Theme, ThemeMode, ToastAction, ToastCallback, ToastId, ToastPosition, ToastPromise,
    ToastType, Toaster, fonts, iced_font, toast, toast_error, toast_info, toast_loading,
    toast_promise, toast_success, toast_warning,
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
    duration_ms: u64,
    visible_toasts: usize,
    gap: f32,
    offset: f32,
    width: f32,
    rich_colors: bool,
    invert: bool,
    close_button: bool,
    expand: bool,
    pause_on_hover: bool,
    pause_when_hidden: bool,
    animated: bool,
    last_toast: Option<ToastId>,
    promise: Option<ToastPromise>,
    dismissed_count: u32,
    auto_closed_count: u32,
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
    Position(Labelled<ToastPosition>),
    Duration(DurationOpt),
    Visible(VisibleOpt),
    Gap(GapOpt),
    Offset(OffsetOpt),
    Width(WidthOpt),
    ToggleRichColors,
    ToggleInvert,
    ToggleCloseButton,
    ToggleExpand,
    TogglePauseOnHover,
    TogglePauseWhenHidden,
    ToggleAnimated,
    ShowDefault,
    ShowSuccess,
    ShowInfo,
    ShowWarning,
    ShowError,
    ShowLoading,
    ShowPromise,
    PromiseSuccess,
    PromiseError,
    ShowDescription,
    ShowAction,
    ShowCancel,
    ShowPerToastOptions,
    ShowImportantStack,
    DismissLast,
    DismissAll,
    Undo,
    Retry,
    Cancel,
    Dismissed,
    AutoClosed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            position: ToastPosition::BottomRight,
            duration_ms: 4_000,
            visible_toasts: 3,
            gap: 14.0,
            offset: 24.0,
            width: 356.0,
            rich_colors: false,
            invert: false,
            close_button: false,
            expand: false,
            pause_on_hover: true,
            pause_when_hidden: false,
            animated: true,
            last_toast: None,
            promise: None,
            dismissed_count: 0,
            auto_closed_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Sonner".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Font(font) => self.theme = self.theme.clone().with_font(font.0),
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => self.theme = self.theme.clone().with_radius(radius.0),
            Message::Position(position) => self.position = position.0,
            Message::Duration(duration) => self.duration_ms = duration.0,
            Message::Visible(visible) => self.visible_toasts = visible.0,
            Message::Gap(gap) => self.gap = gap.0,
            Message::Offset(offset) => self.offset = offset.0,
            Message::Width(width) => self.width = width.0,
            Message::ToggleRichColors => self.rich_colors = !self.rich_colors,
            Message::ToggleInvert => self.invert = !self.invert,
            Message::ToggleCloseButton => self.close_button = !self.close_button,
            Message::ToggleExpand => self.expand = !self.expand,
            Message::TogglePauseOnHover => self.pause_on_hover = !self.pause_on_hover,
            Message::TogglePauseWhenHidden => self.pause_when_hidden = !self.pause_when_hidden,
            Message::ToggleAnimated => self.animated = !self.animated,
            Message::ShowDefault => {
                self.last_toast = Some(
                    toast("Event has been created")
                        .on_dismiss(ToastCallback::new(|| Message::Dismissed))
                        .on_auto_close(ToastCallback::new(|| Message::AutoClosed))
                        .show(),
                );
            }
            Message::ShowSuccess => self.last_toast = Some(toast_success("Event has been created")),
            Message::ShowInfo => {
                self.last_toast = Some(toast_info(
                    "Be at the area 10 minutes before the event time",
                ));
            }
            Message::ShowWarning => {
                self.last_toast =
                    Some(toast_warning("Event start time cannot be earlier than 8am"));
            }
            Message::ShowError => {
                self.last_toast = Some(toast_error("Event has not been created"));
            }
            Message::ShowLoading => self.last_toast = Some(toast_loading("Loading...")),
            Message::ShowPromise => {
                self.promise = Some(toast_promise("Creating event..."));
            }
            Message::PromiseSuccess => {
                if let Some(promise) = self.promise.take() {
                    self.last_toast = Some(promise.success("Event has been created"));
                }
            }
            Message::PromiseError => {
                if let Some(promise) = self.promise.take() {
                    self.last_toast = Some(promise.error("Event could not be created"));
                }
            }
            Message::ShowDescription => {
                self.last_toast = Some(
                    toast("Event has been created")
                        .description("Sunday, December 03, 2023 at 9:00 AM")
                        .show(),
                );
            }
            Message::ShowAction => {
                self.last_toast = Some(
                    toast("Event has been created")
                        .description("Undo is available for this change")
                        .action(ToastAction::new("Undo", || Message::Undo))
                        .show(),
                );
            }
            Message::ShowCancel => {
                self.last_toast = Some(
                    toast("Upload started")
                        .description("You can cancel this operation")
                        .action(ToastAction::new("Retry", || Message::Retry))
                        .cancel(ToastAction::new("Cancel", || Message::Cancel))
                        .show(),
                );
            }
            Message::ShowPerToastOptions => {
                self.last_toast = Some(
                    toast("Per-toast options")
                        .toast_type(ToastType::Warning)
                        .position(ToastPosition::TopLeft)
                        .rich_colors(true)
                        .invert(true)
                        .close_button(true)
                        .show(),
                );
            }
            Message::ShowImportantStack => {
                for index in 1..=5 {
                    let _ = toast(format!("Queued event {index}"))
                        .important(index == 5)
                        .show();
                }
            }
            Message::DismissLast => {
                if let Some(id) = self.last_toast {
                    iced_shadcn_v2::dismiss_toast(id);
                }
            }
            Message::DismissAll => iced_shadcn_v2::dismiss_all_toasts(),
            Message::Undo => self.last_toast = Some(toast_success("Change undone")),
            Message::Retry => self.last_toast = Some(toast_info("Retrying upload...")),
            Message::Cancel => self.last_toast = Some(toast_warning("Upload cancelled")),
            Message::Dismissed => self.dismissed_count += 1,
            Message::AutoClosed => self.auto_closed_count += 1,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let theme_controls = column![
            section_label("Theme (shadcn-common)", palette.muted_foreground, theme),
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

        let toaster_controls = column![
            section_label("Toaster configuration", palette.muted_foreground, theme),
            control_select(
                "Position",
                &POSITIONS,
                Some(Labelled(self.position)),
                Message::Position,
                theme,
            ),
            control_select(
                "Duration",
                &DURATIONS,
                Some(DurationOpt(self.duration_ms)),
                Message::Duration,
                theme,
            ),
            control_select(
                "Visible",
                &VISIBLE_TOASTS,
                Some(VisibleOpt(self.visible_toasts)),
                Message::Visible,
                theme,
            ),
            control_select("Gap", &GAPS, Some(GapOpt(self.gap)), Message::Gap, theme),
            control_select(
                "Offset",
                &OFFSETS,
                Some(OffsetOpt(self.offset)),
                Message::Offset,
                theme,
            ),
            control_select(
                "Width",
                &WIDTHS,
                Some(WidthOpt(self.width)),
                Message::Width,
                theme,
            ),
        ]
        .spacing(8);

        let toggles = row![
            toggle_button(
                theme,
                "Rich colors",
                self.rich_colors,
                Message::ToggleRichColors
            ),
            toggle_button(theme, "Invert", self.invert, Message::ToggleInvert),
            toggle_button(
                theme,
                "Close button",
                self.close_button,
                Message::ToggleCloseButton,
            ),
            toggle_button(theme, "Expand", self.expand, Message::ToggleExpand),
            toggle_button(
                theme,
                "Pause hover",
                self.pause_on_hover,
                Message::TogglePauseOnHover,
            ),
            toggle_button(
                theme,
                "Pause hidden",
                self.pause_when_hidden,
                Message::TogglePauseWhenHidden,
            ),
            toggle_button(theme, "Animated", self.animated, Message::ToggleAnimated),
        ]
        .spacing(8)
        .wrap();

        let toast_types = row![
            action_button(theme, "Default", Message::ShowDefault),
            action_button(theme, "Success", Message::ShowSuccess),
            action_button(theme, "Info", Message::ShowInfo),
            action_button(theme, "Warning", Message::ShowWarning),
            action_button(theme, "Error", Message::ShowError),
            action_button(theme, "Loading", Message::ShowLoading),
            action_button(theme, "Promise", Message::ShowPromise),
        ]
        .spacing(12)
        .wrap();

        let composed_toasts = row![
            action_button(theme, "Description", Message::ShowDescription),
            action_button(theme, "Action", Message::ShowAction),
            action_button(theme, "Action + cancel", Message::ShowCancel),
            action_button(theme, "Per-toast options", Message::ShowPerToastOptions),
            action_button(theme, "Important stack", Message::ShowImportantStack),
        ]
        .spacing(12)
        .wrap();

        let lifecycle = row![
            action_button(theme, "Promise success", Message::PromiseSuccess),
            action_button(theme, "Promise error", Message::PromiseError),
            action_button(theme, "Dismiss last", Message::DismissLast),
            action_button(theme, "Dismiss all", Message::DismissAll),
        ]
        .spacing(12)
        .wrap();

        let status = text(format!(
            "active={} · dismissed callbacks={} · auto-close callbacks={}",
            iced_shadcn_v2::active_toast_count(),
            self.dismissed_count,
            self.auto_closed_count,
        ))
        .size(12)
        .font(iced_font(theme.font_pack().mono))
        .color(palette.muted_foreground);

        let content = column![
            text("iced-shadcn-v2 Sonner")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Sonner-style overlay notifications with a typed iced message API")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            row![theme_controls, toaster_controls]
                .spacing(48)
                .align_y(Alignment::Start)
                .wrap(),
            section_label("Toaster knobs", palette.muted_foreground, theme),
            toggles,
            section_label("Toast types", palette.muted_foreground, theme),
            toast_types,
            section_label("Descriptions and actions", palette.muted_foreground, theme),
            composed_toasts,
            section_label("Lifecycle", palette.muted_foreground, theme),
            lifecycle,
            status,
        ]
        .spacing(16)
        .max_width(1_040)
        .padding(8);

        let content = container(scrollable(
            container(content).width(Length::Fill).padding(24),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        });

        let toaster: Toaster<'_, Message> = Toaster::new(theme)
            .position(self.position)
            .duration(self.duration_ms)
            .visible_toasts(self.visible_toasts)
            .gap(self.gap)
            .offset(self.offset)
            .width(self.width)
            .rich_colors(self.rich_colors)
            .invert(self.invert)
            .close_button(self.close_button)
            .expand(self.expand)
            .pause_on_hover(self.pause_on_hover)
            .pause_when_page_is_hidden(self.pause_when_hidden)
            .animated(self.animated);
        let toaster: Element<'_, Message> = toaster.into();

        stack![content, toaster]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn action_button<'a>(
    theme: &'a Theme,
    label: impl Into<String>,
    message: Message,
) -> Element<'a, Message> {
    ShadcnButton::text(label.into(), theme)
        .variant(ButtonVariant::Outline)
        .on_press(message)
        .into()
}

fn toggle_button<'a>(
    theme: &'a Theme,
    label: &'a str,
    enabled: bool,
    message: Message,
) -> Element<'a, Message> {
    action_button(
        theme,
        format!("{label}: {}", if enabled { "on" } else { "off" }),
        message,
    )
}

fn section_label<'a>(label: &'a str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(88)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(210.0))
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

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.label())
    }
}

impl fmt::Display for Labelled<ToastPosition> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DurationOpt(u64);

impl fmt::Display for DurationOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            formatter.write_str("persistent")
        } else {
            write!(formatter, "{} ms", self.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GapOpt(f32);

impl fmt::Display for GapOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} px", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct OffsetOpt(f32);

impl fmt::Display for OffsetOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} px", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WidthOpt(f32);

impl fmt::Display for WidthOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} px", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VisibleOpt(usize);

impl fmt::Display for VisibleOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
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
            Some(color) => Self::Color(color),
            None => Self::None,
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::Color(color) => Some(color),
            Self::None => None,
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("none"),
            Self::Color(color) => formatter.write_str(color.as_str()),
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

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
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

const POSITIONS: [Labelled<ToastPosition>; 6] = [
    Labelled(ToastPosition::BottomRight),
    Labelled(ToastPosition::BottomLeft),
    Labelled(ToastPosition::BottomCenter),
    Labelled(ToastPosition::TopRight),
    Labelled(ToastPosition::TopLeft),
    Labelled(ToastPosition::TopCenter),
];

const DURATIONS: [DurationOpt; 6] = [
    DurationOpt(0),
    DurationOpt(2_000),
    DurationOpt(4_000),
    DurationOpt(6_000),
    DurationOpt(10_000),
    DurationOpt(30_000),
];

const VISIBLE_TOASTS: [VisibleOpt; 4] =
    [VisibleOpt(1), VisibleOpt(3), VisibleOpt(5), VisibleOpt(8)];

const GAPS: [GapOpt; 4] = [GapOpt(8.0), GapOpt(14.0), GapOpt(20.0), GapOpt(28.0)];
const OFFSETS: [OffsetOpt; 4] = [
    OffsetOpt(12.0),
    OffsetOpt(24.0),
    OffsetOpt(40.0),
    OffsetOpt(64.0),
];
const WIDTHS: [WidthOpt; 4] = [
    WidthOpt(300.0),
    WidthOpt(356.0),
    WidthOpt(420.0),
    WidthOpt(520.0),
];
