use iced::border::Border;
use iced::widget::text::{Rich, Span};
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Subscription, Task};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, InputProps, InputSize, InputVariant, ScrollAreaProps,
    ScrollAreaScrollbars, SidebarMenuButtonProps, SidebarProps, SidebarProviderProps, Theme,
    button, card, icon_button, input, scroll_area, sidebar, sidebar_content, sidebar_group,
    sidebar_group_content, sidebar_group_label, sidebar_header, sidebar_menu, sidebar_menu_button,
    sidebar_menu_item, sidebar_provider,
};
use lucide_icons::iced::{icon_moon, icon_sun};

use super::catalog::PreviewPage;
use super::demos;
use super::highlight::{TokenKind, rust_highlight_ranges};
#[cfg(target_arch = "wasm32")]
use super::route;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentTab {
    Demo,
    Code,
}

impl ComponentTab {
    #[cfg(target_arch = "wasm32")]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Demo => "demo",
            Self::Code => "code",
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "demo" => Some(Self::Demo),
            "code" => Some(Self::Code),
            _ => None,
        }
    }
}

pub struct PreviewApp {
    theme: Theme,
    theme_mode: ThemeMode,
    selected: PreviewPage,
    tab: ComponentTab,
    search: String,
    spinner_phase: f32,
    progress_values: Vec<f32>,
    email: String,
    username: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    SelectPage(PreviewPage),
    SelectTab(ComponentTab),
    ToggleTheme,
    #[cfg(target_arch = "wasm32")]
    HighlightTick,
    AnimationTick,
    ProgressChanged(Vec<f32>),
    EmailChanged(String),
    UsernameChanged(String),
    Noop,
}

impl Default for PreviewApp {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let (selected, tab) = route::read_initial_route();
        #[cfg(not(target_arch = "wasm32"))]
        let (selected, tab) = (PreviewPage::Button, ComponentTab::Demo);

        Self {
            theme: Theme::dark(),
            theme_mode: ThemeMode::Dark,
            selected,
            tab,
            search: String::new(),
            spinner_phase: 0.0,
            progress_values: vec![62.0],
            email: String::new(),
            username: String::new(),
        }
    }
}

impl PreviewApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(value) => {
                self.search = value;
                self.ensure_valid_selected();
            }
            Message::SelectPage(page) => self.selected = page,
            Message::SelectTab(tab) => self.tab = tab,
            Message::ToggleTheme => self.toggle_theme(),
            #[cfg(target_arch = "wasm32")]
            Message::HighlightTick => {}
            Message::AnimationTick => {
                self.spinner_phase = (self.spinner_phase + 0.025).fract();
            }
            Message::ProgressChanged(values) => {
                self.progress_values = values;
            }
            Message::EmailChanged(value) => self.email = value,
            Message::UsernameChanged(value) => self.username = value,
            Message::Noop => {}
        }

        #[cfg(target_arch = "wasm32")]
        route::sync_url(self.selected, self.tab);

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let needs_animation =
            self.tab == ComponentTab::Demo && self.selected == PreviewPage::Button;

        #[cfg(target_arch = "wasm32")]
        {
            if needs_animation {
                return iced::time::every(std::time::Duration::from_millis(16))
                    .map(|_| Message::AnimationTick);
            }
            if self.tab == ComponentTab::Code {
                return iced::time::every(std::time::Duration::from_millis(350))
                    .map(|_| Message::HighlightTick);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if needs_animation {
                return iced::time::every(std::time::Duration::from_millis(16))
                    .map(|_| Message::AnimationTick);
            }
        }

        Subscription::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let theme = self.theme.clone();
        let background = theme.palette.background;
        let shell = row![self.sidebar_view(), self.detail_view()]
            .spacing(12)
            .height(Length::Fill);

        container(shell)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                text_color: Some(theme.palette.foreground),
                ..iced::widget::container::Style::default()
            })
            .into()
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn progress_values(&self) -> &Vec<f32> {
        &self.progress_values
    }

    pub fn progress_value(&self) -> f32 {
        self.progress_values.first().copied().unwrap_or(0.0)
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn spinner_phase(&self) -> f32 {
        self.spinner_phase
    }

    fn filtered_pages(&self) -> Vec<PreviewPage> {
        let needle = self.search.to_lowercase();
        PreviewPage::ALL
            .into_iter()
            .filter(|page| {
                needle.is_empty()
                    || page.title().to_lowercase().contains(&needle)
                    || page.description().to_lowercase().contains(&needle)
            })
            .collect()
    }

    fn ensure_valid_selected(&mut self) {
        let filtered = self.filtered_pages();
        if filtered.is_empty() {
            return;
        }
        if !filtered.contains(&self.selected) {
            self.selected = filtered[0];
        }
    }

    fn toggle_theme(&mut self) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
        self.theme = match self.theme_mode {
            ThemeMode::Dark => Theme::dark(),
            ThemeMode::Light => Theme::light(),
        };
    }

    fn sidebar_view(&self) -> Element<'_, Message> {
        let filtered = self.filtered_pages();
        sidebar_provider(
            SidebarProviderProps::new(true).expanded_width(272.0),
            None::<fn(bool) -> Message>,
            |ctx| {
                sidebar(ctx, SidebarProps::new().padding(8.0), &self.theme, |ctx| {
                    let header = sidebar_header(
                        ctx,
                        column![
                            row![
                                text("iced-shadcn").size(18),
                                icon_button(
                                    if self.theme_mode == ThemeMode::Dark {
                                        icon_sun().size(14)
                                    } else {
                                        icon_moon().size(14)
                                    },
                                    Some(Message::ToggleTheme),
                                    ButtonProps::new()
                                        .variant(ButtonVariant::Outline)
                                        .size(ButtonSize::Size1),
                                    &self.theme,
                                )
                            ]
                            .align_y(Alignment::Center)
                            .spacing(8),
                            text("Component preview").size(12).style(|_t| {
                                iced::widget::text::Style {
                                    color: Some(self.theme.palette.muted_foreground),
                                }
                            }),
                            input(
                                &self.search,
                                "Search components...",
                                Some(Message::SearchChanged),
                                InputProps::new()
                                    .size(InputSize::Size2)
                                    .variant(InputVariant::Surface),
                                &self.theme,
                            )
                            .width(Length::Fill),
                        ]
                        .spacing(8),
                    );

                    let mut items = Vec::new();
                    for page in filtered {
                        items.push(sidebar_menu_item(vec![sidebar_menu_button(
                            SidebarMenuButtonProps::new(page.title()).active(self.selected == page),
                            Some(Message::SelectPage(page)),
                            ctx,
                            &self.theme,
                        )]));
                    }

                    let group = sidebar_group(
                        ctx,
                        iced_shadcn::SidebarGroupProps::new(),
                        vec![
                            sidebar_group_label(
                                iced_shadcn::SidebarGroupLabelProps::new("Components"),
                                ctx,
                                &self.theme,
                            ),
                            sidebar_group_content(vec![sidebar_menu(items)]),
                        ],
                    );

                    column![header, sidebar_content(ctx, scrollable(group))]
                        .spacing(8)
                        .into()
                })
            },
        )
    }

    fn detail_view(&self) -> Element<'_, Message> {
        let selected = self.selected;
        let heading = column![
            text(selected.title()).size(24),
            text(selected.description())
                .size(13)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(self.theme.palette.muted_foreground),
                }),
        ]
        .spacing(4);

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
        .spacing(8)
        .align_y(Alignment::Center);

        let body: Element<'_, Message> = if self.tab == ComponentTab::Code {
            let code = container(render_highlighted_code(&self.theme, selected.code()))
                .width(Length::Fill)
                .padding(2);
            let scroller = scroll_area(
                code,
                ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Both),
                &self.theme,
            )
            .width(Length::Fill)
            .height(Length::Fill);
            card(
                scroller,
                iced_shadcn::CardProps::new().show_shadow(false),
                &self.theme,
            )
            .height(Length::Fill)
            .into()
        } else {
            demos::render(selected, self)
        };

        let panel_bg = self.theme.palette.card;
        let panel_border = self.theme.palette.border;
        let panel_radius = self.theme.radius.md;

        container(column![heading, tabs, body].spacing(10))
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(panel_bg)),
                text_color: Some(self.theme.palette.card_foreground),
                border: Border {
                    radius: panel_radius.into(),
                    width: 1.0,
                    color: panel_border,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

fn render_highlighted_code<'a>(theme: &'a Theme, source: &'a str) -> Rich<'a, (), Message> {
    let ranges = rust_highlight_ranges(source);
    if ranges.is_empty() {
        return Rich::with_spans(vec![Span::new(source).color(theme.palette.foreground)])
            .font(iced::Font::MONOSPACE)
            .size(13);
    }

    let mut color_buf: Vec<Option<(iced::Color, u8)>> = vec![None; source.len()];
    for range in ranges {
        let (color, priority) = match range.kind {
            TokenKind::Comment => (theme.palette.muted_foreground, 1),
            TokenKind::Keyword => (theme.palette.primary, 3),
            TokenKind::String => (iced::Color::from_rgb8(0x9E, 0xD0, 0x9E), 2),
            TokenKind::Type => (iced::Color::from_rgb8(0x8A, 0xB8, 0xFF), 2),
            TokenKind::Function => (iced::Color::from_rgb8(0xE7, 0xC5, 0x8A), 2),
            TokenKind::Number => (iced::Color::from_rgb8(0xF0, 0x9D, 0x9D), 2),
            TokenKind::Attribute => (iced::Color::from_rgb8(0xD8, 0x9B, 0xFF), 2),
        };
        for cell in color_buf.iter_mut().take(range.end).skip(range.start) {
            match cell {
                Some((_, existing_priority)) if *existing_priority > priority => {}
                _ => *cell = Some((color, priority)),
            }
        }
    }

    let base = theme.palette.foreground;
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < source.len() {
        let (color, _) = color_buf[i].unwrap_or((base, 0));
        let mut j = i + 1;
        while j < source.len() {
            let (next_color, _) = color_buf[j].unwrap_or((base, 0));
            if next_color != color {
                break;
            }
            j += 1;
        }
        if let Some(chunk) = source.get(i..j) {
            spans.push(Span::new(chunk).color(color));
        }
        i = j;
    }

    Rich::with_spans(spans).font(iced::Font::MONOSPACE).size(13)
}

pub fn preview_card<'a>(
    theme: &'a Theme,
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
            text_color: Some(theme.palette.card_foreground),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
