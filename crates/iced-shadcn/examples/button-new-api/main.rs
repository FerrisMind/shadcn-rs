use iced::border::Border;
use iced::time::{self, Duration};
use iced::widget::text::{Rich, Span};
use iced::widget::{column, container, row, scrollable};
use iced::{Alignment, Background, Element, Length, Subscription, mouse};

use iced_shadcn::{ButtonRadius, Theme, new_api::Button};
use lucide_icons::LUCIDE_FONT_BYTES;
use lucide_icons::iced::{
    icon_arrow_up, icon_arrow_up_right, icon_circle_fading_arrow_up, icon_git_branch,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .subscription(Example::subscription)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    progress: f32,
    link_hovered: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Pressed,
    LinkHover(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.progress += 0.02;
                if self.progress > 1.0 {
                    self.progress = 0.0;
                }
            }
            Message::Pressed => {}
            Message::LinkHover(hovered) => {
                self.link_hovered = hovered;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border_color = theme.palette.border;
        let radius = theme.radius.md;

        // Variants section
        let variants = column![
            tile(
                theme,
                "Primary (default)",
                Button::new("Button")
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(
                theme,
                "Secondary",
                Button::new("Secondary")
                    .secondary()
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(
                theme,
                "Outline",
                Button::new("Outline")
                    .outline()
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(
                theme,
                "Ghost",
                Button::new("Ghost")
                    .ghost()
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(
                theme,
                "Destructive",
                Button::new("Destructive")
                    .destructive()
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(theme, "Link", {
                let link_label = Rich::<(), Message>::with_spans(vec![
                    Span::new("Link").underline(self.link_hovered),
                ])
                .size(14);

                let btn = Button::new("")
                    .link_variant()
                    .on_press(Message::Pressed)
                    .icon(link_label)
                    .render(theme);

                iced::widget::mouse_area(btn)
                    .on_enter(Message::LinkHover(true))
                    .on_exit(Message::LinkHover(false))
                    .interaction(mouse::Interaction::Pointer)
            }),
        ]
        .spacing(12);

        // Icons section
        let icons = column![
            tile(
                theme,
                "Icon only (Square)",
                row![
                    Button::new("")
                        .outline()
                        .icon(icon_circle_fading_arrow_up().size(18))
                        .on_press(Message::Pressed)
                        .render(theme),
                    Button::new("")
                        .secondary()
                        .sm()
                        .icon(icon_git_branch().size(14))
                        .on_press(Message::Pressed)
                        .render(theme),
                    Button::new("")
                        .destructive()
                        .lg()
                        .icon(icon_arrow_up().size(20))
                        .on_press(Message::Pressed)
                        .render(theme),
                ]
                .spacing(8),
            ),
            tile(
                theme,
                "With leading icon",
                Button::new("New Branch")
                    .outline()
                    .sm()
                    .icon(icon_git_branch().size(12))
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(
                theme,
                "Rounded/Circle",
                row![
                    Button::new("")
                        .outline()
                        .radius(ButtonRadius::Full)
                        .icon(icon_arrow_up().size(16))
                        .on_press(Message::Pressed)
                        .render(theme),
                    Button::new("")
                        .secondary()
                        .radius(ButtonRadius::Large)
                        .icon(icon_git_branch().size(16))
                        .on_press(Message::Pressed)
                        .render(theme),
                ]
                .spacing(8),
            ),
        ]
        .spacing(12);

        // Sizes section
        let sizes = column![
            tile(
                theme,
                "Small",
                row![
                    Button::new("Small")
                        .outline()
                        .sm()
                        .on_press(Message::Pressed)
                        .render(theme),
                    Button::new("")
                        .outline()
                        .sm()
                        .icon(icon_arrow_up_right().size(12))
                        .on_press(Message::Pressed)
                        .render(theme),
                ]
                .spacing(8),
            ),
            tile(
                theme,
                "Default",
                row![
                    Button::new("Default")
                        .outline()
                        .on_press(Message::Pressed)
                        .render(theme),
                    Button::new("")
                        .outline()
                        .icon(icon_arrow_up_right().size(14))
                        .on_press(Message::Pressed)
                        .render(theme),
                ]
                .spacing(8),
            ),
            tile(
                theme,
                "Large",
                row![
                    Button::new("Large")
                        .outline()
                        .lg()
                        .on_press(Message::Pressed)
                        .render(theme),
                    Button::new("")
                        .outline()
                        .lg()
                        .icon(icon_arrow_up_right().size(16))
                        .on_press(Message::Pressed)
                        .render(theme),
                ]
                .spacing(8),
            ),
        ]
        .spacing(12);

        // States section
        let states = column![
            tile(
                theme,
                "Loading (automatic)",
                Button::new("Submit")
                    .outline()
                    .sm()
                    .loading(true)
                    .progress(self.progress)
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
            tile(
                theme,
                "Disabled",
                Button::new("Disabled")
                    .disabled(true)
                    .on_press(Message::Pressed)
                    .render(theme),
            ),
        ]
        .spacing(12);

        let content = column![
            section(theme, "Variants (New API)", variants),
            section(theme, "Icons (New API)", icons),
            section(theme, "Sizes (New API)", sizes),
            section(theme, "States (New API)", states),
        ]
        .spacing(20)
        .align_x(Alignment::Start);

        container(scrollable(content))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: border_color,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

fn section<'a, Message: 'a>(
    theme: &'a Theme,
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let title = iced::widget::text(title)
        .size(18)
        .style(move |_| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        });

    let bg = theme.palette.card;
    let border_c = theme.palette.border;
    let r = theme.radius.md;

    container(column![title, content.into()].spacing(12))
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: Border {
                radius: r.into(),
                width: 1.0,
                color: border_c,
            },
            ..iced::widget::container::Style::default()
        })
}

fn tile<'a, Message: 'a>(
    theme: &'a Theme,
    label: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let bg = theme.palette.background;
    let border_c = theme.palette.border;
    let r = theme.radius.md;
    let muted = theme.palette.muted_foreground;

    let label_text = iced::widget::text(label)
        .size(11)
        .style(move |_| iced::widget::text::Style { color: Some(muted) });

    container(column![label_text, content.into()].spacing(8))
        .padding(12)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: Border {
                radius: r.into(),
                width: 1.0,
                color: border_c,
            },
            ..iced::widget::container::Style::default()
        })
}
