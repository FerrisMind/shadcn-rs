use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Font, Length, Padding};
use lucide_icons::Icon as LucideIcon;

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, icon_button};
use crate::collapsible::{CollapsibleContentProps, CollapsibleProps, collapsible};
use crate::input::{InputProps, InputVariant, input};
use crate::theme::Theme;

#[cfg(feature = "wry")]
pub mod wry_backend;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WebPreviewBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl WebPreviewBounds {
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[must_use]
    pub fn normalized(self) -> Self {
        Self {
            x: self.x.max(0.0),
            y: self.y.max(0.0),
            width: self.width.max(1.0),
            height: self.height.max(1.0),
        }
    }
}

impl Default for WebPreviewBounds {
    fn default() -> Self {
        Self::new(0.0, 48.0, 1200.0, 640.0)
    }
}

#[derive(Clone, Debug)]
pub struct WebPreviewProps {
    pub title: String,
    pub url_placeholder: String,
    pub show_navigation: bool,
    pub show_root_border: bool,
    pub show_open_browser_button: bool,
    pub show_devtools_button: bool,
    pub show_console_toggle: bool,
    pub show_console_panel: bool,
    pub navigation_height: f32,
    pub console_height: f32,
    pub body_min_height: f32,
    pub console_max_entries: usize,
    pub default_bounds: WebPreviewBounds,
}

impl Default for WebPreviewProps {
    fn default() -> Self {
        Self {
            title: "Web preview".to_owned(),
            url_placeholder: "Enter URL...".to_owned(),
            show_navigation: true,
            show_root_border: true,
            show_open_browser_button: true,
            show_devtools_button: true,
            show_console_toggle: true,
            show_console_panel: true,
            navigation_height: 52.0,
            console_height: 180.0,
            body_min_height: 320.0,
            console_max_entries: 200,
            default_bounds: WebPreviewBounds::default(),
        }
    }
}

impl WebPreviewProps {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebPreviewConsoleLevel {
    Log,
    Info,
    Warn,
    Error,
    Debug,
}

impl WebPreviewConsoleLevel {
    fn label(&self) -> &'static str {
        match self {
            Self::Log => "LOG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Debug => "DEBUG",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebPreviewConsoleEntry {
    pub level: WebPreviewConsoleLevel,
    pub message: String,
    pub timestamp: String,
}

impl WebPreviewConsoleEntry {
    #[must_use]
    pub fn new(
        level: WebPreviewConsoleLevel,
        message: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        Self {
            level,
            message: message.into(),
            timestamp: timestamp.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebPreviewBackendEvent {
    PageLoadStarted {
        url: String,
    },
    PageLoadFinished {
        url: String,
    },
    UrlChanged {
        url: String,
    },
    TitleChanged {
        title: String,
    },
    HistoryState {
        can_go_back: bool,
        can_go_forward: bool,
    },
    Console(WebPreviewConsoleEntry),
    Error {
        message: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WebPreviewEffect {
    Attach {
        url: String,
        bounds: WebPreviewBounds,
    },
    Detach,
    Navigate(String),
    Back,
    Forward,
    Reload,
    OpenInBrowser(String),
    OpenDevTools,
    SetBounds(WebPreviewBounds),
}

#[derive(Clone, Debug, PartialEq)]
pub enum WebPreviewAction {
    Attach,
    Detach,
    UrlInputChanged(String),
    UrlSubmitted,
    Navigate(String),
    GoBack,
    GoForward,
    Reload,
    OpenInBrowser,
    OpenDevTools,
    ToggleConsole,
    SetConsoleOpen(bool),
    SetBounds(WebPreviewBounds),
    ClearConsole,
    Backend(WebPreviewBackendEvent),
}

#[derive(Clone, Debug)]
pub struct WebPreviewState {
    pub url_input: String,
    pub current_url: Option<String>,
    pub title: Option<String>,
    pub loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub console_open: bool,
    pub console_entries: Vec<WebPreviewConsoleEntry>,
    pub last_error: Option<String>,
    pub attached: bool,
    pub last_bounds: Option<WebPreviewBounds>,
}

impl Default for WebPreviewState {
    fn default() -> Self {
        Self::new("https://example.com")
    }
}

impl WebPreviewState {
    #[must_use]
    pub fn new(initial_url: impl Into<String>) -> Self {
        let initial_url = initial_url.into();

        Self {
            url_input: initial_url.clone(),
            current_url: Some(initial_url),
            title: None,
            loading: false,
            can_go_back: false,
            can_go_forward: false,
            console_open: false,
            console_entries: Vec::new(),
            last_error: None,
            attached: false,
            last_bounds: None,
        }
    }

    pub fn apply(
        &mut self,
        action: WebPreviewAction,
        props: &WebPreviewProps,
    ) -> Option<WebPreviewEffect> {
        match action {
            WebPreviewAction::Attach => {
                if self.attached {
                    return None;
                }

                self.attached = true;
                let url = self.url_input.trim().to_owned().replace(['\n', '\r'], "");
                let url = if url.is_empty() {
                    String::from("about:blank")
                } else {
                    url
                };
                let bounds = self
                    .last_bounds
                    .unwrap_or(props.default_bounds)
                    .normalized();
                self.last_bounds = Some(bounds);
                Some(WebPreviewEffect::Attach { url, bounds })
            }
            WebPreviewAction::Detach => {
                if !self.attached {
                    return None;
                }

                self.attached = false;
                Some(WebPreviewEffect::Detach)
            }
            WebPreviewAction::UrlInputChanged(value) => {
                self.url_input = value;
                None
            }
            WebPreviewAction::UrlSubmitted => {
                let url = self.url_input.trim().to_owned().replace(['\n', '\r'], "");

                if url.is_empty() {
                    return None;
                }

                self.url_input = url.clone();
                self.loading = true;
                Some(WebPreviewEffect::Navigate(url))
            }
            WebPreviewAction::Navigate(url) => {
                let url = url.trim().to_owned().replace(['\n', '\r'], "");
                if url.is_empty() {
                    return None;
                }

                self.url_input = url.clone();
                self.loading = true;
                Some(WebPreviewEffect::Navigate(url))
            }
            WebPreviewAction::GoBack => Some(WebPreviewEffect::Back),
            WebPreviewAction::GoForward => Some(WebPreviewEffect::Forward),
            WebPreviewAction::Reload => Some(WebPreviewEffect::Reload),
            WebPreviewAction::OpenInBrowser => {
                let url = self
                    .current_url
                    .clone()
                    .unwrap_or_else(|| self.url_input.trim().to_owned().replace(['\n', '\r'], ""));
                if url.is_empty() {
                    None
                } else {
                    Some(WebPreviewEffect::OpenInBrowser(url))
                }
            }
            WebPreviewAction::OpenDevTools => Some(WebPreviewEffect::OpenDevTools),
            WebPreviewAction::ToggleConsole => {
                self.console_open = !self.console_open;
                None
            }
            WebPreviewAction::SetConsoleOpen(open) => {
                self.console_open = open;
                None
            }
            WebPreviewAction::SetBounds(bounds) => {
                let normalized = bounds.normalized();
                self.last_bounds = Some(normalized);

                if self.attached {
                    Some(WebPreviewEffect::SetBounds(normalized))
                } else {
                    None
                }
            }
            WebPreviewAction::ClearConsole => {
                self.console_entries.clear();
                None
            }
            WebPreviewAction::Backend(event) => {
                self.apply_backend_event(event, props);
                None
            }
        }
    }

    fn apply_backend_event(&mut self, event: WebPreviewBackendEvent, props: &WebPreviewProps) {
        match event {
            WebPreviewBackendEvent::PageLoadStarted { url } => {
                self.loading = true;
                self.current_url = Some(url.clone());
                self.url_input = url;
            }
            WebPreviewBackendEvent::PageLoadFinished { url } => {
                self.loading = false;
                self.current_url = Some(url.clone());
                self.url_input = url;
            }
            WebPreviewBackendEvent::UrlChanged { url } => {
                self.current_url = Some(url.clone());
                self.url_input = url;
            }
            WebPreviewBackendEvent::TitleChanged { title } => {
                self.title = Some(title);
            }
            WebPreviewBackendEvent::HistoryState {
                can_go_back,
                can_go_forward,
            } => {
                self.can_go_back = can_go_back;
                self.can_go_forward = can_go_forward;
            }
            WebPreviewBackendEvent::Console(entry) => {
                self.push_console_entry(entry, props.console_max_entries);
            }
            WebPreviewBackendEvent::Error { message } => {
                self.last_error = Some(message.clone());
                self.loading = false;
                self.push_console_entry(
                    WebPreviewConsoleEntry::new(
                        WebPreviewConsoleLevel::Error,
                        message,
                        time_label_now(),
                    ),
                    props.console_max_entries,
                );
            }
        }
    }

    fn push_console_entry(&mut self, entry: WebPreviewConsoleEntry, max_entries: usize) {
        self.console_entries.push(entry);

        let keep = max_entries.max(1);
        if self.console_entries.len() > keep {
            let drop_count = self.console_entries.len() - keep;
            self.console_entries.drain(0..drop_count);
        }
    }
}

fn time_label_now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() % 86_400)
        .unwrap_or(0);
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{hours:02}:{minutes:02}:{secs:02}")
}

pub struct WebPreviewContext<'a, Message> {
    pub props: &'a WebPreviewProps,
    pub state: &'a WebPreviewState,
    pub theme: &'a Theme,
    pub(crate) on_action: Rc<dyn Fn(WebPreviewAction) -> Message + 'a>,
}

impl<'a, Message> Clone for WebPreviewContext<'a, Message> {
    fn clone(&self) -> Self {
        Self {
            props: self.props,
            state: self.state,
            theme: self.theme,
            on_action: Rc::clone(&self.on_action),
        }
    }
}

pub fn web_preview_root<'a, Message: Clone + 'a>(
    props: &'a WebPreviewProps,
    state: &'a WebPreviewState,
    on_action: impl Fn(WebPreviewAction) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let ctx = WebPreviewContext {
        props,
        state,
        theme,
        on_action: Rc::new(on_action),
    };

    let mut layout = column!()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(0);

    if ctx.props.show_navigation {
        layout = layout.push(web_preview_navigation(&ctx));
    }

    layout = layout.push(web_preview_body(&ctx));

    if ctx.props.show_console_panel {
        layout = layout.push(web_preview_console(&ctx));
    }

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.card)),
            text_color: Some(theme.palette.card_foreground),
            border: Border {
                radius: if props.show_root_border {
                    theme.radius.lg.into()
                } else {
                    0.0.into()
                },
                width: if props.show_root_border { 1.0 } else { 0.0 },
                color: theme.palette.border,
            },
            ..Default::default()
        })
        .into()
}

pub fn web_preview_navigation<'a, Message: Clone + 'a>(
    ctx: &WebPreviewContext<'a, Message>,
) -> Element<'a, Message> {
    let theme = ctx.theme;
    let state = ctx.state;
    let props = ctx.props;
    let on_action = Rc::clone(&ctx.on_action);

    let icon = |glyph: LucideIcon| {
        text(char::from(glyph).to_string())
            .font(Font::with_name("lucide"))
            .size(14)
    };

    let back = icon_button(
        icon(LucideIcon::ArrowLeft),
        Some(on_action(WebPreviewAction::GoBack)),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size1),
        theme,
    );

    let forward = icon_button(
        icon(LucideIcon::ArrowRight),
        Some(on_action(WebPreviewAction::GoForward)),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size1),
        theme,
    );

    let reload = icon_button(
        icon(LucideIcon::RefreshCw),
        Some(on_action(WebPreviewAction::Reload)),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size1),
        theme,
    );

    let input_handler = {
        let on_action = Rc::clone(&on_action);
        move |value| on_action(WebPreviewAction::UrlInputChanged(value))
    };

    let url_input = input(
        &state.url_input,
        &props.url_placeholder,
        Some(input_handler),
        InputProps::new().variant(InputVariant::Surface),
        theme,
    )
    .on_submit(on_action(WebPreviewAction::UrlSubmitted))
    .width(Length::Fill);

    let mut top_row = row![back, forward, reload, url_input]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    if props.show_open_browser_button {
        top_row = top_row.push(icon_button(
            icon(LucideIcon::ExternalLink),
            Some(on_action(WebPreviewAction::OpenInBrowser)),
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1),
            theme,
        ));
    }

    if props.show_devtools_button {
        top_row = top_row.push(icon_button(
            icon(LucideIcon::Wrench),
            Some(on_action(WebPreviewAction::OpenDevTools)),
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1),
            theme,
        ));
    }

    if props.show_console_toggle {
        let toggle_icon = if state.console_open {
            LucideIcon::PanelBottomClose
        } else {
            LucideIcon::PanelBottomOpen
        };
        let toggle = icon_button(
            icon(toggle_icon),
            Some(on_action(WebPreviewAction::ToggleConsole)),
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1),
            theme,
        );
        top_row = top_row.push(toggle);
    }

    container(top_row.width(Length::Fill))
        .padding(Padding {
            top: 4.0,
            right: 8.0,
            bottom: 8.0,
            left: 8.0,
        })
        .height(Length::Fixed(props.navigation_height))
        .width(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.muted)),
            text_color: Some(theme.palette.foreground),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: theme.palette.border,
            },
            ..Default::default()
        })
        .into()
}

pub fn web_preview_body<'a, Message: Clone + 'a>(
    ctx: &WebPreviewContext<'a, Message>,
) -> Element<'a, Message> {
    let theme = ctx.theme;
    let state = ctx.state;
    let message = if state.attached {
        ""
    } else {
        "WebView surface will appear here after attach."
    };

    container(
        container(
            text(message)
                .size(12)
                .style(move |_| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.background)),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: theme.palette.border,
        },
        ..Default::default()
    })
    .into()
}

pub fn web_preview_console<'a, Message: Clone + 'a>(
    ctx: &WebPreviewContext<'a, Message>,
) -> Element<'a, Message> {
    let theme = ctx.theme;
    let state = ctx.state;
    let props = ctx.props;
    let on_action = Rc::clone(&ctx.on_action);
    let icon = |glyph: LucideIcon| {
        text(char::from(glyph).to_string())
            .font(Font::with_name("lucide"))
            .size(14)
    };

    let content: Element<'a, Message> =
        if state.console_entries.is_empty() {
            container(text("No console output").size(12).style(move |_| {
                iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }
            }))
            .padding([8, 10])
            .width(Length::Fill)
            .into()
        } else {
            let entries = state
                .console_entries
                .iter()
                .map(|entry| {
                    let color = match entry.level {
                        WebPreviewConsoleLevel::Log | WebPreviewConsoleLevel::Info => {
                            theme.palette.foreground
                        }
                        WebPreviewConsoleLevel::Warn => theme.palette.primary,
                        WebPreviewConsoleLevel::Error => theme.palette.destructive,
                        WebPreviewConsoleLevel::Debug => theme.palette.muted_foreground,
                    };

                    row![
                        text(entry.timestamp.clone())
                            .size(11)
                            .width(Length::Fixed(56.0))
                            .style(move |_| iced::widget::text::Style {
                                color: Some(theme.palette.muted_foreground),
                            }),
                        text(entry.level.label())
                            .size(11)
                            .width(Length::Fixed(48.0))
                            .style(move |_| iced::widget::text::Style { color: Some(color) }),
                        text(entry.message.clone())
                            .size(12)
                            .width(Length::Fill)
                            .style(move |_| iced::widget::text::Style { color: Some(color) }),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Start)
                    .width(Length::Fill)
                    .into()
                })
                .collect::<Vec<Element<'a, Message>>>();

            scrollable(column(entries).spacing(6))
                .height(Length::Fixed(props.console_height))
                .width(Length::Fill)
                .into()
        };

    if props.show_console_toggle {
        if !state.console_open {
            return container(column![]).width(Length::Fill).into();
        }

        let header = row![
            row![
                text("Console").size(13),
                text(format!("({})", state.console_entries.len()))
                    .size(12)
                    .style(move |_| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .width(Length::Fill),
            icon_button(
                icon(LucideIcon::Trash2),
                Some(on_action(WebPreviewAction::ClearConsole)),
                ButtonProps::new()
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Size1),
                theme,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        return container(column![header, content].spacing(8).width(Length::Fill))
            .padding([6, 10])
            .width(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.muted)),
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: theme.palette.border,
                },
                ..Default::default()
            })
            .into();
    }

    let trigger = row![
        text("Console").size(13),
        text(format!("({})", state.console_entries.len()))
            .size(12)
            .style(move |_| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    collapsible(
        state.console_open,
        trigger,
        content,
        Some({
            let on_action = Rc::clone(&on_action);
            move |open| on_action(WebPreviewAction::SetConsoleOpen(open))
        }),
        CollapsibleContentProps::new().force_mount(false),
        CollapsibleProps::new().compact(true),
        theme,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props() -> WebPreviewProps {
        WebPreviewProps::new()
    }

    #[test]
    fn submit_url_commits_navigation_effect() {
        let mut state = WebPreviewState::new("https://initial");
        let props = props();

        state.apply(
            WebPreviewAction::UrlInputChanged("https://example.test/path".to_owned()),
            &props,
        );

        let effect = state.apply(WebPreviewAction::UrlSubmitted, &props);

        assert_eq!(
            effect,
            Some(WebPreviewEffect::Navigate(
                "https://example.test/path".to_owned()
            ))
        );
        assert!(state.loading);
        assert_eq!(state.url_input, "https://example.test/path");
    }

    #[test]
    fn open_devtools_returns_effect() {
        let mut state = WebPreviewState::new("https://initial");
        let props = props();

        let effect = state.apply(WebPreviewAction::OpenDevTools, &props);

        assert_eq!(effect, Some(WebPreviewEffect::OpenDevTools));
    }

    #[test]
    fn open_in_browser_uses_current_or_input_url() {
        let mut state = WebPreviewState::new("https://initial");
        let props = props();

        let effect = state.apply(WebPreviewAction::OpenInBrowser, &props);
        assert_eq!(
            effect,
            Some(WebPreviewEffect::OpenInBrowser(
                "https://initial".to_owned()
            ))
        );

        state.current_url = None;
        state.url_input = "https://input-only".to_owned();
        let effect = state.apply(WebPreviewAction::OpenInBrowser, &props);
        assert_eq!(
            effect,
            Some(WebPreviewEffect::OpenInBrowser(
                "https://input-only".to_owned()
            ))
        );
    }

    #[test]
    fn toggle_console_and_limit_entries() {
        let mut state = WebPreviewState::new("https://initial");
        let mut props = props();
        props.console_max_entries = 2;

        state.apply(WebPreviewAction::ToggleConsole, &props);
        assert!(state.console_open);

        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::Console(
                WebPreviewConsoleEntry::new(WebPreviewConsoleLevel::Info, "one", "t1"),
            )),
            &props,
        );
        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::Console(
                WebPreviewConsoleEntry::new(WebPreviewConsoleLevel::Warn, "two", "t2"),
            )),
            &props,
        );
        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::Console(
                WebPreviewConsoleEntry::new(WebPreviewConsoleLevel::Error, "three", "t3"),
            )),
            &props,
        );

        assert_eq!(state.console_entries.len(), 2);
        assert_eq!(state.console_entries[0].message, "two");
        assert_eq!(state.console_entries[1].message, "three");
    }

    #[test]
    fn backend_event_updates_loading_title_and_history() {
        let mut state = WebPreviewState::new("https://initial");
        let props = props();

        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::PageLoadStarted {
                url: "https://started".to_owned(),
            }),
            &props,
        );
        assert!(state.loading);
        assert_eq!(state.current_url.as_deref(), Some("https://started"));

        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::TitleChanged {
                title: "Page title".to_owned(),
            }),
            &props,
        );
        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::HistoryState {
                can_go_back: true,
                can_go_forward: false,
            }),
            &props,
        );
        state.apply(
            WebPreviewAction::Backend(WebPreviewBackendEvent::PageLoadFinished {
                url: "https://finished".to_owned(),
            }),
            &props,
        );

        assert_eq!(state.title.as_deref(), Some("Page title"));
        assert!(state.can_go_back);
        assert!(!state.can_go_forward);
        assert!(!state.loading);
        assert_eq!(state.current_url.as_deref(), Some("https://finished"));
    }

    #[test]
    fn attach_bounds_detach_effects_are_deterministic() {
        let mut state = WebPreviewState::new("https://start");
        let props = props();

        let attach = state.apply(WebPreviewAction::Attach, &props);
        assert!(matches!(attach, Some(WebPreviewEffect::Attach { .. })));
        assert!(state.attached);

        let bounds = WebPreviewBounds::new(10.0, 20.0, 100.0, 200.0);
        let effect = state.apply(WebPreviewAction::SetBounds(bounds), &props);
        assert_eq!(
            effect,
            Some(WebPreviewEffect::SetBounds(bounds.normalized()))
        );

        let detach = state.apply(WebPreviewAction::Detach, &props);
        assert_eq!(detach, Some(WebPreviewEffect::Detach));
        assert!(!state.attached);
    }
}
