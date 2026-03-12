use iced::border::Border;
use iced::widget::{column, container, row, scrollable, slider, text, text_input};
use iced::{Alignment, Background, Element, Length, Task};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use iced_shadcn::{
    AccentColor, BadgeProps, BadgeSize, BadgeVariant, ButtonProps, ButtonSize, ButtonVariant,
    InputProps, InputSize, InputVariant, ProgressProps, ProgressSize, ProgressVariant, Theme,
    badge, button, input, progress,
};
use lucide_icons::LUCIDE_FONT_BYTES;

const BUTTON_CODE: &str = include_str!("../../../iced-shadcn/examples/button/main.rs");
const BADGE_CODE: &str = include_str!("../../../iced-shadcn/examples/badge/main.rs");
const PROGRESS_CODE: &str = include_str!("../../../iced-shadcn/examples/progress/main.rs");
const INPUT_CODE: &str = include_str!("../../../iced-shadcn/examples/input/main.rs");

pub fn main() -> iced::Result {
    iced::application(PreviewApp::default, PreviewApp::update, PreviewApp::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PreviewPage {
    Button,
    Badge,
    Progress,
    Input,
}

impl PreviewPage {
    const ALL: [Self; 4] = [Self::Button, Self::Badge, Self::Progress, Self::Input];

    const fn title(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Badge => "Badge",
            Self::Progress => "Progress",
            Self::Input => "Input",
        }
    }

    const fn description(self) -> &'static str {
        match self {
            Self::Button => "Variants, sizes and states.",
            Self::Badge => "Status labels, variants and colors.",
            Self::Progress => "Determinate and loading indicators.",
            Self::Input => "Form fields and helper text.",
        }
    }

    const fn code(self) -> &'static str {
        match self {
            Self::Button => BUTTON_CODE,
            Self::Badge => BADGE_CODE,
            Self::Progress => PROGRESS_CODE,
            Self::Input => INPUT_CODE,
        }
    }

    #[cfg(target_arch = "wasm32")]
    const fn slug(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Badge => "badge",
            Self::Progress => "progress",
            Self::Input => "input",
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "button" => Some(Self::Button),
            "badge" => Some(Self::Badge),
            "progress" => Some(Self::Progress),
            "input" => Some(Self::Input),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Component(PreviewPage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComponentTab {
    Demo,
    Code,
}

impl ComponentTab {
    #[cfg(target_arch = "wasm32")]
    const fn slug(self) -> &'static str {
        match self {
            Self::Demo => "demo",
            Self::Code => "code",
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "demo" => Some(Self::Demo),
            "code" => Some(Self::Code),
            _ => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn parse_query(search: &str) -> (Option<PreviewPage>, Option<ComponentTab>) {
    let raw = search.strip_prefix('?').unwrap_or(search);
    let mut component = None;
    let mut tab = None;

    for part in raw.split('&') {
        if part.is_empty() {
            continue;
        }
        let mut kv = part.splitn(2, '=');
        let key = kv.next().unwrap_or_default();
        let value = kv.next().unwrap_or_default();
        match key {
            "component" => component = PreviewPage::from_slug(value),
            "tab" => tab = ComponentTab::from_slug(value),
            _ => {}
        }
    }

    (component, tab)
}

#[cfg(target_arch = "wasm32")]
fn read_initial_route() -> (Screen, ComponentTab) {
    let Some(window) = web_sys::window() else {
        return (Screen::Home, ComponentTab::Demo);
    };
    let Ok(search) = window.location().search() else {
        return (Screen::Home, ComponentTab::Demo);
    };

    let (component, tab) = parse_query(&search);
    if let Some(page) = component {
        (Screen::Component(page), tab.unwrap_or(ComponentTab::Demo))
    } else {
        (Screen::Home, ComponentTab::Demo)
    }
}

#[cfg(target_arch = "wasm32")]
fn sync_url(screen: Screen, tab: ComponentTab) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let location = window.location();
    let pathname = location.pathname().unwrap_or_else(|_| "/".to_owned());
    let wanted_query = match screen {
        Screen::Home => String::new(),
        Screen::Component(page) => format!("?component={}&tab={}", page.slug(), tab.slug()),
    };
    let current_query = location.search().unwrap_or_default();
    if current_query == wanted_query {
        return;
    }

    let url = format!("{pathname}{wanted_query}");
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }
}

struct PreviewApp {
    theme: Theme,
    screen: Screen,
    tab: ComponentTab,
    search: String,
    progress_value: f32,
    email: String,
    username: String,
}

impl Default for PreviewApp {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let (screen, tab) = read_initial_route();
        #[cfg(not(target_arch = "wasm32"))]
        let (screen, tab) = (Screen::Home, ComponentTab::Demo);

        Self {
            theme: Theme::default(),
            screen,
            tab,
            search: String::new(),
            progress_value: 62.0,
            email: String::new(),
            username: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    SearchChanged(String),
    OpenComponent(PreviewPage),
    SelectTab(ComponentTab),
    ProgressChanged(f32),
    EmailChanged(String),
    UsernameChanged(String),
    Noop,
}

impl PreviewApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GoHome => self.screen = Screen::Home,
            Message::SearchChanged(value) => self.search = value,
            Message::OpenComponent(page) => {
                self.screen = Screen::Component(page);
                self.tab = ComponentTab::Demo;
            }
            Message::SelectTab(tab) => self.tab = tab,
            Message::ProgressChanged(value) => self.progress_value = value,
            Message::EmailChanged(value) => self.email = value,
            Message::UsernameChanged(value) => self.username = value,
            Message::Noop => {}
        }

        #[cfg(target_arch = "wasm32")]
        sync_url(self.screen, self.tab);

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let header = self.header();
        let body = match self.screen {
            Screen::Home => self.home_view(),
            Screen::Component(page) => self.component_view(page),
        };

        container(column![header, body].spacing(12))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                ..iced::widget::container::Style::default()
            })
            .into()
    }

    fn header(&self) -> Element<'_, Message> {
        row![
            text("Components Preview").size(24),
            button(
                "Home",
                Some(Message::GoHome),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Size1),
                &self.theme,
            )
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .into()
    }

    fn home_view(&self) -> Element<'_, Message> {
        let needle = self.search.to_lowercase();
        let mut list = column![
            text("Browse component demos").size(14),
            text_input("Search components...", &self.search)
                .on_input(Message::SearchChanged)
                .padding(8),
        ]
        .spacing(10);

        for page in PreviewPage::ALL {
            if !needle.is_empty()
                && !page.title().to_lowercase().contains(&needle)
                && !page.description().to_lowercase().contains(&needle)
            {
                continue;
            }
            list = list.push(preview_card(
                &self.theme,
                page.title(),
                column![
                    text(page.description()).size(13),
                    button(
                        "Open",
                        Some(Message::OpenComponent(page)),
                        ButtonProps::new()
                            .variant(ButtonVariant::Solid)
                            .size(ButtonSize::Size1),
                        &self.theme,
                    )
                ]
                .spacing(8),
            ));
        }

        scrollable(list.spacing(10)).into()
    }

    fn component_view(&self, page: PreviewPage) -> Element<'_, Message> {
        let controls = row![
            text(page.title()).size(20),
            text(page.description())
                .size(13)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(self.theme.palette.muted_foreground),
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let tabs = row![
            button(
                "Demo",
                Some(Message::SelectTab(ComponentTab::Demo)),
                ButtonProps::new()
                    .variant(if self.tab == ComponentTab::Demo {
                        ButtonVariant::Solid
                    } else {
                        ButtonVariant::Outline
                    })
                    .size(ButtonSize::Size1),
                &self.theme,
            ),
            button(
                "Code",
                Some(Message::SelectTab(ComponentTab::Code)),
                ButtonProps::new()
                    .variant(if self.tab == ComponentTab::Code {
                        ButtonVariant::Solid
                    } else {
                        ButtonVariant::Outline
                    })
                    .size(ButtonSize::Size1),
                &self.theme,
            ),
        ]
        .spacing(8);

        let body: Element<'_, Message> = if self.tab == ComponentTab::Code {
            scrollable(container(text(page.code()).size(12)).padding(10)).into()
        } else {
            match page {
                PreviewPage::Button => self.page_buttons(),
                PreviewPage::Badge => self.page_badges(),
                PreviewPage::Progress => self.page_progress(),
                PreviewPage::Input => self.page_inputs(),
            }
        };

        column![controls, tabs, body].spacing(10).into()
    }

    fn page_buttons(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        row![
            preview_card(
                theme,
                "Variants",
                column![
                    button(
                        "Primary",
                        Some(Message::Noop),
                        ButtonProps::new()
                            .variant(ButtonVariant::Solid)
                            .size(ButtonSize::Size2),
                        theme,
                    ),
                    button(
                        "Outline",
                        Some(Message::Noop),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size2),
                        theme,
                    ),
                    button(
                        "Ghost",
                        Some(Message::Noop),
                        ButtonProps::new()
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Size2),
                        theme,
                    ),
                ]
                .spacing(8)
            ),
            preview_card(
                theme,
                "States",
                column![
                    button(
                        "Icon",
                        Some(Message::Noop),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size1),
                        theme,
                    ),
                    button(
                        "Loading",
                        Some(Message::Noop),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size1)
                            .loading(true),
                        theme,
                    ),
                ]
                .spacing(8)
            )
        ]
        .spacing(16)
        .align_y(Alignment::Start)
        .into()
    }

    fn page_badges(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        row![
            preview_card(
                theme,
                "Variants",
                column![
                    row![
                        badge(
                            "Default",
                            BadgeProps::new().variant(BadgeVariant::Default),
                            theme,
                        ),
                        badge(
                            "Secondary",
                            BadgeProps::new().variant(BadgeVariant::Secondary),
                            theme,
                        ),
                    ]
                    .spacing(8),
                    row![
                        badge(
                            "Outline",
                            BadgeProps::new().variant(BadgeVariant::Outline),
                            theme,
                        ),
                        badge(
                            "Destructive",
                            BadgeProps::new().variant(BadgeVariant::Destructive),
                            theme,
                        ),
                    ]
                    .spacing(8),
                ]
                .spacing(8)
            ),
            preview_card(
                theme,
                "Sizes & Colors",
                column![
                    row![
                        badge("Size 1", BadgeProps::new().size(BadgeSize::Size1), theme),
                        badge("Size 3", BadgeProps::new().size(BadgeSize::Size3), theme),
                    ]
                    .spacing(8),
                    row![
                        badge(
                            "Success",
                            BadgeProps::new().color(AccentColor::Green),
                            theme,
                        ),
                        badge("Error", BadgeProps::new().color(AccentColor::Red), theme),
                    ]
                    .spacing(8),
                ]
                .spacing(8)
            ),
        ]
        .spacing(16)
        .align_y(Alignment::Start)
        .into()
    }

    fn page_progress(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        column![
            preview_card(
                theme,
                "Determinate",
                column![
                    slider(0.0..=100.0, self.progress_value, Message::ProgressChanged).width(320),
                    container(progress(
                        ProgressProps::new()
                            .value(self.progress_value)
                            .size(ProgressSize::Size2)
                            .variant(ProgressVariant::Classic),
                        theme,
                    ))
                    .width(Length::Fixed(320.0)),
                ]
                .spacing(10)
            ),
            preview_card(
                theme,
                "Indeterminate",
                column![
                    container(progress(ProgressProps::new().indeterminate(), theme))
                        .width(Length::Fixed(320.0)),
                    container(progress(
                        ProgressProps::new()
                            .value(74.0)
                            .variant(ProgressVariant::Surface)
                            .color(AccentColor::Green),
                        theme,
                    ))
                    .width(Length::Fixed(320.0)),
                ]
                .spacing(10)
            ),
        ]
        .spacing(16)
        .into()
    }

    fn page_inputs(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        column![
            preview_card(
                theme,
                "Email",
                column![
                    text("Email").size(13),
                    input(
                        &self.email,
                        "you@example.com",
                        Some(Message::EmailChanged),
                        InputProps::new()
                            .size(InputSize::Size2)
                            .variant(InputVariant::Surface),
                        theme,
                    )
                    .width(Length::Fixed(320.0)),
                ]
                .spacing(8)
            ),
            preview_card(
                theme,
                "Username",
                column![
                    text("Username").size(13),
                    input(
                        &self.username,
                        "shadcn",
                        Some(Message::UsernameChanged),
                        InputProps::new()
                            .size(InputSize::Size2)
                            .variant(InputVariant::Surface),
                        theme,
                    )
                    .width(Length::Fixed(320.0)),
                    text("This is your public display name.")
                        .size(12)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(theme.palette.muted_foreground),
                        }),
                ]
                .spacing(8)
            ),
        ]
        .spacing(16)
        .into()
    }
}

fn preview_card<'a>(
    theme: &Theme,
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(column![text(title).size(14), content.into()].spacing(10))
        .padding(14)
        .width(Length::Fixed(360.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
