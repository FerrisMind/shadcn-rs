use std::process::Command;
use std::time::Duration;

use iced::border::Border;
use iced::widget::container;
use iced::{Element, Length, Subscription, Task, window};

use iced_shadcn::{
    Theme, WebPreviewAction, WebPreviewBackendEvent, WebPreviewBounds, WebPreviewEffect,
    WebPreviewProps, WebPreviewState, web_preview_root,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    if let Err(code) = ensure_clean_runtime() {
        std::process::exit(code);
    }

    clear_snap_env();
    init_linux_wry_runtime();

    iced::application(Example::default, Example::update, Example::view)
        .subscription(Example::subscription)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[cfg(all(feature = "wry", target_os = "linux"))]
fn ensure_clean_runtime() -> Result<(), i32> {
    if std::env::var_os("WEB_PREVIEW_SANITIZED").is_some() {
        return Ok(());
    }

    let exe = std::env::current_exe().map_err(|_| 1)?;
    let args = std::env::args_os().skip(1);
    let current_env = std::env::vars_os().collect::<Vec<_>>();

    let mut command = Command::new(exe);
    command.env_clear();
    command.args(args);
    command.env("WEB_PREVIEW_SANITIZED", "1");
    command.env(
        "LD_LIBRARY_PATH",
        "/lib/x86_64-linux-gnu:/usr/lib/x86_64-linux-gnu",
    );
    command.env("NO_AT_BRIDGE", "1");

    for (key, value) in current_env {
        let key = key.to_string_lossy();
        if key == "WEB_PREVIEW_SANITIZED"
            || key == "NO_AT_BRIDGE"
            || key == "GTK_MODULES"
            || key.starts_with("SNAP")
        {
            continue;
        }

        let value = value.to_string_lossy();
        if key == "PATH" || key == "XDG_DATA_DIRS" || key == "XDG_CONFIG_DIRS" {
            let cleaned = value
                .split(':')
                .filter(|segment| !segment.is_empty() && !segment.contains("/snap/"))
                .collect::<Vec<_>>()
                .join(":");
            if !cleaned.is_empty() {
                command.env(key.as_ref(), cleaned);
            }
            continue;
        }

        if value.contains("/snap/") {
            continue;
        }

        command.env(key.as_ref(), value.as_ref());
    }

    let status = command.spawn().map_err(|_| 1)?.wait().map_err(|_| 1)?;
    Err(status.code().unwrap_or(1))
}

#[cfg(not(all(feature = "wry", target_os = "linux")))]
fn ensure_clean_runtime() -> Result<(), i32> {
    Ok(())
}

#[cfg(all(feature = "wry", target_os = "linux"))]
fn init_linux_wry_runtime() {
    gtk::init().expect("failed to initialize GTK for web-preview");
}

#[cfg(not(all(feature = "wry", target_os = "linux")))]
fn init_linux_wry_runtime() {}

fn clear_snap_env() {
    // Remove snap-specific environment inherited from the editor shell before GTK/WebKit starts.
    // This keeps WebKit child processes on the system runtime instead of snap-provided libs.
    for key in [
        "SNAP",
        "SNAP_NAME",
        "SNAP_INSTANCE_NAME",
        "SNAP_REVISION",
        "SNAP_ARCH",
        "SNAP_DATA",
        "SNAP_COMMON",
        "SNAP_USER_DATA",
        "SNAP_USER_COMMON",
        "SNAP_LIBRARY_PATH",
        "LD_AUDIT",
        "LD_DEBUG",
        "LD_DEBUG_OUTPUT",
        "LD_PRELOAD",
        "LD_ORIGIN_PATH",
        "NO_AT_BRIDGE",
        "GTK_MODULES",
    ] {
        unsafe {
            std::env::remove_var(key);
        }
    }

    // Force WebKit and its helper processes to resolve libc/libpthread from the host system.
    // This avoids snap-provided glibc fragments being picked up through inherited launcher state.
    unsafe {
        if cfg!(target_os = "linux") {
            std::env::set_var(
                "LD_LIBRARY_PATH",
                "/lib/x86_64-linux-gnu:/usr/lib/x86_64-linux-gnu",
            );
            std::env::set_var("NO_AT_BRIDGE", "1");
        }
    }
}

const CARD_MAX_WIDTH: f32 = 960.0;
const CARD_MAX_HEIGHT: f32 = 640.0;
const CARD_MIN_MARGIN: f32 = 24.0;
const CARD_BORDER_WIDTH: f32 = 1.0;
const CARD_RADIUS: f32 = 12.0;
const CARD_CONTENT_INSET: f32 = 6.0;

struct Example {
    theme: Theme,
    props: WebPreviewProps,
    preview: WebPreviewState,
    window_id: Option<window::Id>,
    last_window_size: Option<iced::Size>,
}

#[derive(Debug, Clone)]
enum Message {
    Preview(WebPreviewAction),
    WindowEvent((window::Id, window::Event)),
    Tick,
}

impl Default for Example {
    fn default() -> Self {
        let mut props = WebPreviewProps::new();
        props.title = "Web Preview".to_owned();
        props.navigation_height = 44.0;
        props.show_root_border = false;
        props.show_console_panel = false;
        props.show_console_toggle = false;
        props.show_open_browser_button = true;
        props.show_devtools_button = true;

        Self {
            theme: Theme::dark(),
            props,
            preview: WebPreviewState::new("https://iced.rs"),
            window_id: None,
            last_window_size: None,
        }
    }
}

impl Example {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            window::events().map(Message::WindowEvent),
            iced::time::every(Duration::from_millis(33)).map(|_| Message::Tick),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Preview(action) => {
                let should_sync_bounds = matches!(
                    action,
                    WebPreviewAction::ToggleConsole | WebPreviewAction::SetConsoleOpen(_)
                );
                let mut tasks = Vec::new();

                if let Some(effect) = self.preview.apply(action, &self.props) {
                    tasks.push(self.run_effect(effect));
                }
                if should_sync_bounds && let Some(effect) = self.sync_bounds_effect() {
                    tasks.push(self.run_effect(effect));
                }

                Task::batch(tasks)
            }
            Message::WindowEvent((id, event)) => match event {
                window::Event::Opened { size, .. } => {
                    self.window_id = Some(id);
                    self.last_window_size = Some(size);

                    let mut tasks = Vec::new();
                    if let Some(effect) = self.preview.apply(WebPreviewAction::Attach, &self.props)
                    {
                        tasks.push(self.run_effect(effect));
                    }
                    if let Some(effect) = self.sync_bounds_effect() {
                        tasks.push(self.run_effect(effect));
                    }

                    Task::batch(tasks)
                }
                window::Event::Resized(size) => {
                    if Some(id) != self.window_id {
                        return Task::none();
                    }

                    self.last_window_size = Some(size);
                    if let Some(effect) = self.sync_bounds_effect() {
                        self.run_effect(effect)
                    } else {
                        Task::none()
                    }
                }
                window::Event::Closed => {
                    if Some(id) == self.window_id {
                        self.window_id = None;
                        self.preview.apply(WebPreviewAction::Detach, &self.props);
                    }
                    Task::none()
                }
                _ => Task::none(),
            },
            Message::Tick => {
                #[cfg(feature = "wry")]
                iced_shadcn::wry_backend::pump_gtk_events();

                for event in drain_backend_events() {
                    self.preview
                        .apply(WebPreviewAction::Backend(event), &self.props);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = web_preview_root(&self.props, &self.preview, Message::Preview, &self.theme);
        let theme = self.theme.clone();
        let window_size = self
            .last_window_size
            .unwrap_or_else(|| iced::Size::new(1280.0, 800.0));
        let card = self.card_bounds_for_window(window_size);

        let card_block = container(container(content).width(Length::Fill).height(Length::Fill))
            .width(Length::Fixed(card.width))
            .height(Length::Fixed(card.height))
            .padding(CARD_BORDER_WIDTH + CARD_CONTENT_INSET)
            .style(move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(theme.palette.muted)),
                text_color: Some(theme.palette.foreground),
                border: Border {
                    radius: CARD_RADIUS.into(),
                    width: CARD_BORDER_WIDTH,
                    color: theme.palette.ring,
                },
                ..Default::default()
            });

        container(card_block)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn run_effect(&self, effect: WebPreviewEffect) -> Task<Message> {
        let effect = match effect {
            WebPreviewEffect::OpenInBrowser(url) => {
                return Task::perform(async move { open_in_system_browser(url) }, |_| {
                    Message::Tick
                });
            }
            other => other,
        };

        let Some(window_id) = self.window_id else {
            return Task::none();
        };

        #[cfg(feature = "wry")]
        {
            iced_shadcn::wry_backend::run(effect, window_id).map(|_| Message::Tick)
        }

        #[cfg(not(feature = "wry"))]
        {
            let _ = (window_id, effect);
            Task::none()
        }
    }

    fn sync_bounds_effect(&mut self) -> Option<WebPreviewEffect> {
        let size = self.last_window_size?;
        let bounds = self.bounds_for_size(size);
        self.preview
            .apply(WebPreviewAction::SetBounds(bounds), &self.props)
    }

    fn bounds_for_size(&self, size: iced::Size) -> WebPreviewBounds {
        let card = self.card_bounds_for_window(size);
        let frame_inset = CARD_BORDER_WIDTH + CARD_CONTENT_INSET;
        let inner_x = card.x + frame_inset;
        let inner_y = card.y + frame_inset;
        let inner_width = (card.width - frame_inset * 2.0).max(1.0);
        let inner_height = (card.height - frame_inset * 2.0).max(1.0);
        let nav_height = if self.props.show_navigation {
            self.props.navigation_height
        } else {
            0.0
        };
        let console_height = 0.0;
        let body_height = (inner_height - nav_height - console_height).max(1.0);

        WebPreviewBounds::new(inner_x, inner_y + nav_height, inner_width, body_height)
    }

    fn card_bounds_for_window(&self, size: iced::Size) -> WebPreviewBounds {
        let max_width = (size.width - CARD_MIN_MARGIN * 2.0).max(320.0);
        let max_height = (size.height - CARD_MIN_MARGIN * 2.0).max(280.0);
        let width = CARD_MAX_WIDTH.min(max_width);
        let height = CARD_MAX_HEIGHT.min(max_height);
        let x = ((size.width - width) / 2.0).max(0.0);
        let y = ((size.height - height) / 2.0).max(0.0);
        WebPreviewBounds::new(x, y, width, height)
    }
}

fn open_in_system_browser(url: String) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(url)
            .spawn()?;
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
        return Ok(());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(url).spawn()?;
        return Ok(());
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn drain_backend_events() -> Vec<WebPreviewBackendEvent> {
    #[cfg(feature = "wry")]
    {
        iced_shadcn::wry_backend::drain_events()
    }

    #[cfg(not(feature = "wry"))]
    {
        Vec::new()
    }
}
