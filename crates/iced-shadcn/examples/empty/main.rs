use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Font, Length};

use iced_shadcn::{
    AvatarProps, AvatarSize, AvatarVariant, ButtonProps, ButtonSize, ButtonVariant, CardProps,
    CardSize, CardVariant, EmptyContentProps, EmptyDescriptionProps, EmptyHeaderProps,
    EmptyMediaProps, EmptyMediaVariant, EmptyRootProps, EmptyTitleProps, InputGroupAddonAlign,
    InputGroupAddonProps, InputGroupInputProps, InputGroupProps, InputSize, KbdProps, Theme,
    avatar, button, button_content, card, empty_content, empty_description, empty_header,
    empty_media, empty_root, empty_title, input_group, input_group_addon, input_group_input, kbd,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    search_query: String,
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    Noop,
}

impl Default for Example {
    fn default() -> Self {
        let mut theme = Theme::default();
        theme.styles.empty.root_gap = theme.spacing.lg + theme.spacing.sm;
        theme.styles.empty.root_padding = theme.spacing.lg * 3.0;

        Self {
            theme,
            search_query: String::new(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(value) => self.search_query = value,
            Message::Noop => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            header(theme),
            demo_grid_row(
                theme,
                section_card(theme, "Default", self.default_demo()),
                section_card(theme, "Outline", self.outline_demo()),
            ),
            demo_grid_row(
                theme,
                section_card(theme, "Background", self.background_demo()),
                section_card(theme, "Avatar", self.avatar_demo()),
            ),
            demo_grid_row(
                theme,
                section_card(theme, "Avatar Group", self.avatar_group_demo()),
                section_card(theme, "InputGroup", self.input_group_demo()),
            ),
        ]
        .spacing(theme.spacing.lg + theme.spacing.sm)
        .width(Length::Fill);

        app(theme, scrollable(content).into())
    }

    fn default_demo(&self) -> Element<'_, Message> {
        empty_root(
            column![
                empty_header(
                    vec![
                        empty_media(
                            lucide_icon(
                                Icon::FolderCode,
                                self.theme.styles.empty.media_icon_size,
                                self.theme.palette.foreground,
                            ),
                            EmptyMediaProps::new().variant(EmptyMediaVariant::Icon),
                            &self.theme,
                        ),
                        empty_title("No Projects Yet", EmptyTitleProps::new(), &self.theme),
                        empty_description(
                            "You haven't created any projects yet. Get started by creating your first project.",
                            EmptyDescriptionProps::new(),
                            &self.theme,
                        ),
                    ],
                    EmptyHeaderProps::new(),
                    &self.theme,
                ),
                row![
                    Element::from(button(
                        "Create Project",
                        Some(Message::Noop),
                        ButtonProps::new().size(ButtonSize::Size2),
                        &self.theme,
                    )),
                    Element::from(button(
                        "Import Project",
                        Some(Message::Noop),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size2),
                        &self.theme,
                    )),
                ]
                .spacing(self.theme.spacing.sm)
                .align_y(Alignment::Center),
                button_content(
                    row![
                        text("Learn More")
                            .size(self.theme.styles.empty.description_size)
                            .style(|_| iced::widget::text::Style {
                                color: Some(self.theme.palette.muted_foreground),
                            }),
                        lucide_icon(
                            Icon::ArrowUpRight,
                            self.theme.styles.empty.description_size,
                            self.theme.palette.muted_foreground
                        )
                    ]
                    .spacing(self.theme.spacing.sm - self.theme.spacing.xs / 2.0)
                    .align_y(Alignment::Center),
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Link)
                        .size(ButtonSize::Size1),
                    &self.theme,
                ),
            ]
            .align_x(Alignment::Center)
            .spacing(self.theme.styles.empty.root_gap),
            demo_root_props(&self.theme),
            &self.theme,
        )
    }

    fn outline_demo(&self) -> Element<'_, Message> {
        empty_root(
            column![
                empty_header(
                    vec![
                        empty_media(
                            lucide_icon(
                                Icon::Cloud,
                                self.theme.styles.empty.media_icon_size,
                                self.theme.palette.foreground,
                            ),
                            EmptyMediaProps::new().variant(EmptyMediaVariant::Icon),
                            &self.theme,
                        ),
                        empty_title("Cloud Storage Empty", EmptyTitleProps::new(), &self.theme),
                        empty_description(
                            "Upload files to your cloud storage to access them anywhere.",
                            EmptyDescriptionProps::new(),
                            &self.theme,
                        ),
                    ],
                    EmptyHeaderProps::new(),
                    &self.theme,
                ),
                button(
                    "Upload Files",
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size1),
                    &self.theme,
                ),
            ]
            .align_x(Alignment::Center)
            .spacing(self.theme.styles.empty.root_gap),
            demo_root_props(&self.theme).bordered(true),
            &self.theme,
        )
    }

    fn background_demo(&self) -> Element<'_, Message> {
        empty_root(
            column![
                empty_header(
                    vec![
                        empty_media(
                            lucide_icon(
                                Icon::Bell,
                                self.theme.styles.empty.media_icon_size,
                                self.theme.palette.foreground,
                            ),
                            EmptyMediaProps::new().variant(EmptyMediaVariant::Icon),
                            &self.theme,
                        ),
                        empty_title("No Notifications", EmptyTitleProps::new(), &self.theme),
                        empty_description(
                            "You're all caught up. New notifications will appear here.",
                            EmptyDescriptionProps::new(),
                            &self.theme,
                        ),
                    ],
                    EmptyHeaderProps::new(),
                    &self.theme,
                ),
                button_content(
                    row![
                        lucide_icon(
                            Icon::RefreshCcw,
                            self.theme.spacing.lg,
                            self.theme.palette.foreground,
                        ),
                        text("Refresh")
                    ]
                    .spacing(self.theme.spacing.sm)
                    .align_y(Alignment::Center),
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size1),
                    &self.theme,
                ),
            ]
            .align_x(Alignment::Center)
            .spacing(self.theme.styles.empty.root_gap),
            demo_root_props(&self.theme).background(Color {
                a: 1.0,
                ..mix(
                    self.theme.palette.muted,
                    self.theme.palette.background,
                    0.35,
                )
            }),
            &self.theme,
        )
    }

    fn avatar_demo(&self) -> Element<'_, Message> {
        empty_root(
            column![
                empty_header(
                    vec![
                        empty_media(
                            avatar(
                                AvatarProps::new("SO")
                                    .size(AvatarSize::Size6)
                                    .variant(AvatarVariant::Soft),
                                &self.theme,
                            ),
                            EmptyMediaProps::new()
                                .variant(EmptyMediaVariant::Default)
                                .size(self.theme.styles.empty.media_size + self.theme.spacing.sm),
                            &self.theme,
                        ),
                        empty_title("User Offline", EmptyTitleProps::new(), &self.theme),
                        empty_description(
                            "This user is currently offline. You can leave a message to notify them or try again later.",
                            EmptyDescriptionProps::new(),
                            &self.theme,
                        ),
                    ],
                    EmptyHeaderProps::new(),
                    &self.theme,
                ),
                button(
                    "Leave Message",
                    Some(Message::Noop),
                    ButtonProps::new().size(ButtonSize::Size1),
                    &self.theme,
                ),
            ]
            .align_x(Alignment::Center)
            .spacing(self.theme.styles.empty.root_gap),
            demo_root_props(&self.theme),
            &self.theme,
        )
    }

    fn avatar_group_demo(&self) -> Element<'_, Message> {
        let avatars = row![
            avatar(
                AvatarProps::new("CN")
                    .size(AvatarSize::Size6)
                    .variant(AvatarVariant::Soft),
                &self.theme
            ),
            avatar(
                AvatarProps::new("ML")
                    .size(AvatarSize::Size6)
                    .variant(AvatarVariant::Soft),
                &self.theme
            ),
            avatar(
                AvatarProps::new("ER")
                    .size(AvatarSize::Size6)
                    .variant(AvatarVariant::Soft),
                &self.theme
            ),
        ]
        .spacing(self.theme.spacing.sm - self.theme.spacing.xs / 2.0)
        .align_y(Alignment::Center);

        empty_root(
            column![
                empty_header(
                    vec![
                        empty_media(
                            avatars,
                            EmptyMediaProps::new()
                                .variant(EmptyMediaVariant::Default)
                                .size(self.theme.styles.empty.media_size * 4.0),
                            &self.theme,
                        ),
                        empty_title("No Team Members", EmptyTitleProps::new(), &self.theme),
                        empty_description(
                            "Invite your team to collaborate on this project.",
                            EmptyDescriptionProps::new(),
                            &self.theme,
                        ),
                    ],
                    EmptyHeaderProps::new().max_width(
                        self.theme.styles.empty.header_max_width + self.theme.spacing.md
                    ),
                    &self.theme,
                ),
                button_content(
                    row![
                        lucide_icon(
                            Icon::Plus,
                            self.theme.spacing.lg,
                            self.theme.palette.primary_foreground,
                        ),
                        text("Invite Members")
                    ]
                    .spacing(self.theme.spacing.sm)
                    .align_y(Alignment::Center),
                    Some(Message::Noop),
                    ButtonProps::new().size(ButtonSize::Size1),
                    &self.theme,
                ),
            ]
            .align_x(Alignment::Center)
            .spacing(self.theme.styles.empty.root_gap),
            demo_root_props(&self.theme).bordered(true),
            &self.theme,
        )
    }

    fn input_group_demo(&self) -> Element<'_, Message> {
        let input = input_group(
            vec![
                input_group_input(
                    &self.search_query,
                    "Try searching for pages...",
                    Some(Message::SearchChanged),
                    InputGroupInputProps::new().size(InputSize::Size3),
                    &self.theme,
                ),
                input_group_addon(
                    lucide_icon(
                        Icon::Search,
                        self.theme.spacing.lg,
                        self.theme.palette.muted_foreground,
                    ),
                    InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineEnd),
                ),
                input_group_addon(
                    kbd("/", KbdProps::new(), &self.theme),
                    InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineEnd),
                ),
            ],
            InputGroupProps::new(),
            &self.theme,
        );

        empty_root(
            column![
                empty_header(
                    vec![
                        empty_title("404 - Not Found", EmptyTitleProps::new(), &self.theme),
                        empty_description(
                            "The page you're looking for doesn't exist. Try searching for what you need below.",
                            EmptyDescriptionProps::new(),
                            &self.theme,
                        ),
                    ],
                    EmptyHeaderProps::new(),
                    &self.theme,
                ),
                empty_content(
                    vec![
                        container(input)
                            .max_width(self.theme.styles.empty.content_max_width - self.theme.spacing.lg)
                            .width(Length::Fill)
                            .into(),
                        row![
                            text("Need help?")
                                .size(self.theme.styles.empty.description_size)
                                .style(|_| iced::widget::text::Style {
                                    color: Some(self.theme.palette.muted_foreground),
                                }),
                            Element::from(button(
                                "Contact support",
                                Some(Message::Noop),
                                ButtonProps::new()
                                    .variant(ButtonVariant::Link)
                                    .size(ButtonSize::Size1),
                                &self.theme,
                            )),
                        ]
                        .spacing(self.theme.spacing.xs)
                        .align_y(Alignment::Center)
                        .into(),
                    ],
                    EmptyContentProps::new(),
                    &self.theme,
                ),
            ]
            .spacing(self.theme.styles.empty.root_gap),
            demo_root_props(&self.theme),
            &self.theme,
        )
    }
}

fn header<'a>(theme: &'a Theme) -> Element<'a, Message> {
    column![
        text("Empty").size(self::page_heading_size(theme)),
        text("Parity demo for the shadcn-svelte Empty component scenarios.")
            .size(theme.styles.empty.description_size)
            .style(move |_| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(theme.spacing.sm)
    .into()
}

fn page_heading_size(theme: &Theme) -> f32 {
    theme.styles.empty.title_size + theme.spacing.lg - theme.spacing.xs / 2.0
}

fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(theme.spacing.lg + theme.spacing.sm)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn section_card<'a>(
    theme: &Theme,
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    card(
        column![text(title).size(18), content.into()]
            .spacing(theme.spacing.md)
            .width(Length::Fill),
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size2),
        theme,
    )
    .width(Length::Fill)
    .into()
}

fn demo_grid_row<'a>(
    theme: &Theme,
    left: Element<'a, Message>,
    right: Element<'a, Message>,
) -> Element<'a, Message> {
    row![
        container(left).width(Length::FillPortion(1)),
        container(right).width(Length::FillPortion(1)),
    ]
    .spacing(theme.spacing.lg + theme.spacing.sm)
    .width(Length::Fill)
    .align_y(Alignment::Start)
    .into()
}

fn lucide_icon<'a, Message: 'a>(icon: Icon, size: f32, color: Color) -> Element<'a, Message> {
    text(char::from(icon).to_string())
        .font(Font::with_name("lucide"))
        .size(size)
        .style(move |_| iced::widget::text::Style { color: Some(color) })
        .into()
}

fn mix(a: Color, b: Color, t: f32) -> Color {
    let clamped = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * clamped,
        g: a.g + (b.g - a.g) * clamped,
        b: a.b + (b.b - a.b) * clamped,
        a: a.a + (b.a - a.a) * clamped,
    }
}

fn demo_root_props(theme: &Theme) -> EmptyRootProps {
    let _ = theme;
    EmptyRootProps::new()
}
