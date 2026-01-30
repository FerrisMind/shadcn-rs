use iced::border::Border;
use iced::widget::{column, container, text};
use iced::{Background, Element, Length};

use iced_shadcn::{
    AccentColor, TabsListProps, TabsListVariant, TabsRootProps, Theme, tabs_content, tabs_contents,
    tabs_list, tabs_root, tabs_trigger,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    active: String,
}

#[derive(Debug, Clone)]
enum Message {
    TabChanged(String),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabChanged(value) => self.active = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let active = if self.active.is_empty() {
            "account"
        } else {
            &self.active
        };

        let items = || {
            vec![
                tabs_trigger("account", "Account"),
                tabs_trigger("documents", "Documents"),
                tabs_trigger("settings", "Settings"),
            ]
        };
        let contents = || {
            tabs_contents(
                vec![
                    tabs_content("account", text("Account content").size(14)),
                    tabs_content("documents", text("Documents content").size(14)),
                    tabs_content("settings", text("Settings content").size(14)),
                ],
                active,
            )
        };

        let indigo = tabs_root(
            tabs_list(
                items(),
                active,
                Some(Message::TabChanged),
                TabsRootProps::new(),
                TabsListProps::new()
                    .variant(TabsListVariant::Line)
                    .color(AccentColor::Indigo),
                theme,
            ),
            contents(),
        );

        let cyan = tabs_root(
            tabs_list(
                items(),
                active,
                Some(Message::TabChanged),
                TabsRootProps::new(),
                TabsListProps::new()
                    .variant(TabsListVariant::Line)
                    .color(AccentColor::Cyan),
                theme,
            ),
            contents(),
        );

        let orange = tabs_root(
            tabs_list(
                items(),
                active,
                Some(Message::TabChanged),
                TabsRootProps::new(),
                TabsListProps::new()
                    .variant(TabsListVariant::Line)
                    .color(AccentColor::Orange),
                theme,
            ),
            contents(),
        );

        let content = column![indigo, cyan, orange].spacing(24);
        app(theme, preview(theme, content).into())
    }
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
        .width(Length::Fixed(440.0))
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
