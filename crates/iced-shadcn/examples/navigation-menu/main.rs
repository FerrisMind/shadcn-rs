use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Font, Length};

use iced_shadcn::{
    AccentColor, NavigationMenuContentProps, NavigationMenuLinkProps, NavigationMenuListProps,
    NavigationMenuProps, NavigationMenuWrap, Theme, navigation_menu_content, navigation_menu_item,
    navigation_menu_link, navigation_menu_link_item, navigation_menu_list, navigation_menu_root,
    navigation_menu_trigger, navigation_menu_viewport,
};
use lucide_icons::{Icon as LucideIcon, LUCIDE_FONT_BYTES};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    open_value: String,
    last_action: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    OpenChanged(String),
    Navigate(&'static str),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpenChanged(value) => self.open_value = value,
            Message::Navigate(label) => self.last_action = Some(label),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let open = if self.open_value.is_empty() {
            None
        } else {
            Some(self.open_value.as_str())
        };

        let home_content = home_content(theme);
        let components_content = components_content(theme);
        let list_content = list_content(theme);
        let simple_content = simple_content(theme);
        let icons_content = icon_content(theme);

        let items = navigation_menu_list(vec![
            navigation_menu_item(
                navigation_menu_trigger("home", "Home"),
                navigation_menu_content(home_content)
                    .props(NavigationMenuContentProps::new().width(520.0)),
            ),
            navigation_menu_item(
                navigation_menu_trigger("components", "Components"),
                navigation_menu_content(components_content)
                    .props(NavigationMenuContentProps::new().width(600.0)),
            ),
            navigation_menu_link_item(
                "docs",
                text("Docs").size(13),
                Some(Message::Navigate("Docs")),
            ),
            navigation_menu_item(
                navigation_menu_trigger("list", "List"),
                navigation_menu_content(list_content)
                    .props(NavigationMenuContentProps::new().width(300.0)),
            ),
            navigation_menu_item(
                navigation_menu_trigger("simple", "Simple"),
                navigation_menu_content(simple_content)
                    .props(NavigationMenuContentProps::new().width(220.0)),
            ),
            navigation_menu_item(
                navigation_menu_trigger("icons", "With Icon"),
                navigation_menu_content(icons_content)
                    .props(NavigationMenuContentProps::new().width(220.0)),
            ),
        ]);

        let menu = navigation_menu_root(
            items,
            open,
            Some(Message::OpenChanged),
            NavigationMenuProps::new().viewport_component(navigation_menu_viewport()),
            NavigationMenuListProps::new()
                .wrap(NavigationMenuWrap::Wrap)
                .color(AccentColor::Blue)
                .high_contrast(true),
            theme,
        );

        let status = text(self.last_action.unwrap_or("Select a link"))
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            });

        let content = column![menu, status].spacing(16);
        preview(theme, content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn home_content<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let hero = container(
        column![
            text("shadcn/ui").size(16),
            text("Beautifully designed components built with Tailwind CSS.")
                .size(12)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
        ]
        .spacing(6),
    )
    .padding(16)
    .style(move |_t| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.muted)),
        border: iced::border::Border {
            color: theme.palette.border,
            width: 1.0,
            radius: theme.radius.md.into(),
        },
        ..iced::widget::container::Style::default()
    });

    let list = column![
        nav_list_item(
            theme,
            "Introduction",
            "Re-usable components built using Radix UI."
        ),
        nav_list_item(
            theme,
            "Installation",
            "How to install dependencies and structure your app."
        ),
        nav_list_item(
            theme,
            "Typography",
            "Styles for headings, paragraphs, lists..."
        ),
    ]
    .spacing(8)
    .width(Length::Fill);

    row![hero, list]
        .spacing(12)
        .align_y(Alignment::Start)
        .into()
}

fn components_content<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let left = column![
        nav_list_item(
            theme,
            "Alert Dialog",
            "A modal dialog that interrupts the user."
        ),
        nav_list_item(
            theme,
            "Hover Card",
            "Preview content available behind a link."
        ),
        nav_list_item(
            theme,
            "Progress",
            "Displays an indicator showing completion."
        ),
    ]
    .spacing(8);

    let right = column![
        nav_list_item(
            theme,
            "Scroll Area",
            "Visually or semantically separates content."
        ),
        nav_list_item(
            theme,
            "Tabs",
            "Layered sections of content displayed one at a time."
        ),
        nav_list_item(theme, "Tooltip", "Popup for hover or focus content."),
    ]
    .spacing(8);

    row![left, right].spacing(12).into()
}

fn list_content<'a>(theme: &'a Theme) -> Element<'a, Message> {
    column![
        nav_simple_link(theme, "Components", "Browse all components in the library."),
        nav_simple_link(theme, "Documentation", "Learn how to use the library."),
        nav_simple_link(theme, "Blog", "Read our latest blog posts."),
    ]
    .spacing(8)
    .width(Length::Fill)
    .into()
}

fn simple_content<'a>(theme: &'a Theme) -> Element<'a, Message> {
    column![
        nav_inline_link(theme, "Components"),
        nav_inline_link(theme, "Documentation"),
        nav_inline_link(theme, "Blocks"),
    ]
    .spacing(6)
    .width(Length::Fill)
    .into()
}

fn icon_content<'a>(theme: &'a Theme) -> Element<'a, Message> {
    column![
        nav_icon_link(theme, LucideIcon::CircleAlert, "Backlog"),
        nav_icon_link(theme, LucideIcon::Circle, "To Do"),
        nav_icon_link(theme, LucideIcon::CircleCheck, "Done"),
    ]
    .spacing(6)
    .width(Length::Fill)
    .into()
}

fn nav_list_item<'a>(
    theme: &'a Theme,
    title: &'static str,
    description: &'static str,
) -> Element<'a, Message> {
    let content = column![
        text(title).size(13),
        text(description)
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(4);

    navigation_menu_link(
        content,
        Some(Message::Navigate(title)),
        NavigationMenuLinkProps::new().padding(8.0).full_width(true),
        theme,
    )
}

fn nav_simple_link<'a>(
    theme: &'a Theme,
    title: &'static str,
    description: &'static str,
) -> Element<'a, Message> {
    let content = column![
        text(title).size(13),
        text(description)
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(4);

    navigation_menu_link(
        content,
        Some(Message::Navigate(title)),
        NavigationMenuLinkProps::new().padding(8.0).full_width(true),
        theme,
    )
}

fn nav_inline_link<'a>(theme: &'a Theme, label: &'static str) -> Element<'a, Message> {
    navigation_menu_link(
        text(label).size(13),
        Some(Message::Navigate(label)),
        NavigationMenuLinkProps::new().padding(8.0).full_width(true),
        theme,
    )
}

fn nav_icon_link<'a>(
    theme: &'a Theme,
    icon: LucideIcon,
    label: &'static str,
) -> Element<'a, Message> {
    let icon_font = Font::with_name("lucide");
    let icon_text = text(char::from(icon).to_string())
        .size(14)
        .font(icon_font)
        .style(move |_t| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        });

    let content = row![icon_text, text(label).size(13)]
        .spacing(8)
        .align_y(Alignment::Center);

    navigation_menu_link(
        content,
        Some(Message::Navigate(label)),
        NavigationMenuLinkProps::new().padding(8.0).full_width(true),
        theme,
    )
}

fn preview<'a, Message: 'a>(
    theme: &'a Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
}
