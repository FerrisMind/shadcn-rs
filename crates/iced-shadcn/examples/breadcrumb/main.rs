use iced::border::Border;
use iced::widget::{column, container, scrollable, text as iced_text};
use iced::{Background, Element, Length};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    BreadcrumbProps, Theme, breadcrumb, breadcrumb_ellipsis, breadcrumb_item, breadcrumb_link,
    breadcrumb_list, breadcrumb_page, breadcrumb_separator,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    NavigateTo(usize),
}

struct Example {
    theme: Theme,
    path: Vec<&'static str>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            path: vec!["Docs", "Components", "Navigation", "Breadcrumb"],
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(index) => self.path.truncate(index + 1),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            section_title("Interactive"),
            preview(theme, self.interactive_example()),
            section_title("Collapsed"),
            preview(theme, self.collapsed_example()),
            section_title("No Wrap"),
            preview(theme, self.no_wrap_example()),
            section_title("Custom Separator"),
            preview(theme, self.custom_separator_example()),
        ]
        .spacing(16)
        .width(Length::Fill);

        app(theme, scrollable(content).into())
    }

    fn interactive_example(&self) -> Element<'_, Message> {
        breadcrumb(&self.theme, BreadcrumbProps::new(), |ctx| {
            let mut items = Vec::new();

            for (index, label) in self.path.iter().enumerate() {
                let is_last = index + 1 == self.path.len();
                let item = if is_last {
                    breadcrumb_page(*label, ctx)
                } else {
                    breadcrumb_link(*label, Some(Message::NavigateTo(index)), ctx)
                };

                items.push(breadcrumb_item(ctx, vec![item]));

                if !is_last {
                    items.push(breadcrumb_separator(ctx, None));
                }
            }

            column![
                breadcrumb_list(ctx, items),
                iced_text(format!("Current page: {}", self.path.join(" / ")))
                    .size(13)
                    .style(|_| iced::widget::text::Style {
                        color: Some(self.theme.palette.muted_foreground),
                    }),
            ]
            .spacing(10)
            .into()
        })
    }

    fn collapsed_example(&self) -> Element<'_, Message> {
        breadcrumb(&self.theme, BreadcrumbProps::new(), |ctx| {
            breadcrumb_list(
                ctx,
                vec![
                    breadcrumb_item(ctx, vec![breadcrumb_link("Docs", None, ctx)]),
                    breadcrumb_separator(ctx, None),
                    breadcrumb_ellipsis(ctx),
                    breadcrumb_separator(ctx, None),
                    breadcrumb_item(ctx, vec![breadcrumb_link("Navigation", None, ctx)]),
                    breadcrumb_separator(ctx, None),
                    breadcrumb_item(ctx, vec![breadcrumb_page("Breadcrumb", ctx)]),
                ],
            )
        })
    }

    fn no_wrap_example(&self) -> Element<'_, Message> {
        breadcrumb(
            &self.theme,
            BreadcrumbProps::new().wrap(false).text_size(13.0),
            |ctx| {
                breadcrumb_list(
                    ctx,
                    vec![
                        breadcrumb_item(ctx, vec![breadcrumb_link("Workspace", None, ctx)]),
                        breadcrumb_separator(ctx, None),
                        breadcrumb_item(ctx, vec![breadcrumb_link("nova-ui", None, ctx)]),
                        breadcrumb_separator(ctx, None),
                        breadcrumb_item(ctx, vec![breadcrumb_link("views", None, ctx)]),
                        breadcrumb_separator(ctx, None),
                        breadcrumb_item(ctx, vec![breadcrumb_page("split.rs", ctx)]),
                    ],
                )
            },
        )
    }

    fn custom_separator_example(&self) -> Element<'_, Message> {
        breadcrumb(
            &self.theme,
            BreadcrumbProps::new()
                .separator_size(10.0)
                .item_spacing(8.0)
                .text_size(13.0),
            |ctx| {
                breadcrumb_list(
                    ctx,
                    vec![
                        breadcrumb_item(ctx, vec![breadcrumb_link("Settings", None, ctx)]),
                        breadcrumb_separator(ctx, Some("/".to_string())),
                        breadcrumb_item(ctx, vec![breadcrumb_link("Team", None, ctx)]),
                        breadcrumb_separator(ctx, Some("/".to_string())),
                        breadcrumb_item(ctx, vec![breadcrumb_page("Members", ctx)]),
                    ],
                )
            },
        )
    }
}

fn section_title(title: &str) -> Element<'_, Message> {
    iced_text(title).size(16).into()
}

fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
        .width(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
