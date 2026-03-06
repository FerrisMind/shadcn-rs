use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, CardProps, CardSize, CardVariant, ResizableDirection,
    ResizableHandleProps, ResizablePanelGroupProps, ResizablePanelProps, Theme, button, card,
    resizable_handle, resizable_panel, resizable_panel_group,
};

const HORIZONTAL_DEFAULT: [f32; 2] = [25.0, 75.0];
const VERTICAL_DEFAULT: [f32; 2] = [25.0, 75.0];
const HANDLE_DEFAULT: [f32; 2] = [25.0, 75.0];
const NESTED_OUTER_DEFAULT: [f32; 2] = [50.0, 50.0];
const NESTED_INNER_DEFAULT: [f32; 2] = [25.0, 75.0];
const CONTROLLED_DEFAULT: [f32; 2] = [30.0, 70.0];

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    horizontal_sizes: Vec<f32>,
    vertical_sizes: Vec<f32>,
    handle_sizes: Vec<f32>,
    nested_outer_sizes: Vec<f32>,
    nested_inner_sizes: Vec<f32>,
    controlled_sizes: Vec<f32>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            horizontal_sizes: HORIZONTAL_DEFAULT.into(),
            vertical_sizes: VERTICAL_DEFAULT.into(),
            handle_sizes: HANDLE_DEFAULT.into(),
            nested_outer_sizes: NESTED_OUTER_DEFAULT.into(),
            nested_inner_sizes: NESTED_INNER_DEFAULT.into(),
            controlled_sizes: CONTROLLED_DEFAULT.into(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    HorizontalResized(Vec<f32>),
    VerticalResized(Vec<f32>),
    HandleResized(Vec<f32>),
    NestedOuterResized(Vec<f32>),
    NestedInnerResized(Vec<f32>),
    ControlledResized(Vec<f32>),
    ResetAll,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::HorizontalResized(sizes) => self.horizontal_sizes = sizes,
            Message::VerticalResized(sizes) => self.vertical_sizes = sizes,
            Message::HandleResized(sizes) => self.handle_sizes = sizes,
            Message::NestedOuterResized(sizes) => self.nested_outer_sizes = sizes,
            Message::NestedInnerResized(sizes) => self.nested_inner_sizes = sizes,
            Message::ControlledResized(sizes) => self.controlled_sizes = sizes,
            Message::ResetAll => {
                self.horizontal_sizes = HORIZONTAL_DEFAULT.into();
                self.vertical_sizes = VERTICAL_DEFAULT.into();
                self.handle_sizes = HANDLE_DEFAULT.into();
                self.nested_outer_sizes = NESTED_OUTER_DEFAULT.into();
                self.nested_inner_sizes = NESTED_INNER_DEFAULT.into();
                self.controlled_sizes = CONTROLLED_DEFAULT.into();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            header(theme),
            controls(theme),
            demo_card(
                theme,
                "Horizontal",
                "Reference horizontal split. No min/max constraints, so the divider can travel to 0/100.",
                container(self.horizontal_demo())
                    .width(Length::Fill)
                    .height(Length::Fixed(200.0)),
            ),
            demo_card(
                theme,
                "Vertical",
                "Reference vertical split. Same API, different axis.",
                container(self.vertical_demo())
                    .width(Length::Fill)
                    .height(Length::Fixed(200.0)),
            ),
            demo_card(
                theme,
                "With Handle",
                "Visible grip variant, matching the dedicated handle example in the web references.",
                container(self.with_handle_demo())
                    .width(Length::Fill)
                    .height(Length::Fixed(200.0)),
            ),
            demo_card(
                theme,
                "Nested",
                "Outer horizontal group with an inner vertical group, like the default nested example.",
                container(self.nested_demo())
                    .width(Length::Fill)
                    .height(Length::Fixed(220.0)),
            ),
            demo_card(
                theme,
                "Controlled",
                "Externally managed layout state. The panels reflect the current Vec<f32> on every drag.",
                container(self.controlled_demo())
                    .width(Length::Fill)
                    .height(Length::Fixed(200.0)),
            ),
            coverage_card(
                theme,
                &self.horizontal_sizes,
                &self.vertical_sizes,
                &self.handle_sizes,
                &self.nested_outer_sizes,
                &self.nested_inner_sizes,
                &self.controlled_sizes,
            ),
        ]
        .spacing(20)
        .width(Length::Fill);

        app(
            theme,
            scrollable(container(content).width(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        )
    }

    fn horizontal_demo(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        resizable_panel_group(
            ResizablePanelGroupProps::new("horizontal").direction(ResizableDirection::Horizontal),
            &self.horizontal_sizes,
            Some(Message::HorizontalResized),
            theme,
            move |ctx| {
                vec![
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(25.0),
                        0,
                        panel(
                            theme,
                            "Sidebar",
                            format!("{:.1}% / {:.0}px", ctx.get_size(0), ctx.get_pixel_size(0)),
                            &[
                                "Plain handle",
                                "defaultSize only",
                                "0% to 100% resize range",
                            ],
                        ),
                    ),
                    resizable_handle(ctx, ResizableHandleProps::new(), 0, theme),
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(75.0),
                        1,
                        panel(
                            theme,
                            "Content",
                            format!("{:.1}% / {:.0}px", ctx.get_size(1), ctx.get_pixel_size(1)),
                            &["Mirrors shadcn-ui horizontal example"],
                        ),
                    ),
                ]
            },
        )
    }

    fn vertical_demo(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        resizable_panel_group(
            ResizablePanelGroupProps::new("vertical").direction(ResizableDirection::Vertical),
            &self.vertical_sizes,
            Some(Message::VerticalResized),
            theme,
            move |ctx| {
                vec![
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(25.0),
                        0,
                        panel(
                            theme,
                            "Header",
                            format!("{:.1}% / {:.0}px", ctx.get_size(0), ctx.get_pixel_size(0)),
                            &["Vertical direction", "No resize limits"],
                        ),
                    ),
                    resizable_handle(ctx, ResizableHandleProps::new(), 0, theme),
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(75.0),
                        1,
                        panel(
                            theme,
                            "Content",
                            format!("{:.1}% / {:.0}px", ctx.get_size(1), ctx.get_pixel_size(1)),
                            &["Matches the vertical reference example"],
                        ),
                    ),
                ]
            },
        )
    }

    fn with_handle_demo(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        resizable_panel_group(
            ResizablePanelGroupProps::new("with-handle").direction(ResizableDirection::Horizontal),
            &self.handle_sizes,
            Some(Message::HandleResized),
            theme,
            move |ctx| {
                vec![
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(25.0),
                        0,
                        panel(
                            theme,
                            "Sidebar",
                            format!("{:.1}% / {:.0}px", ctx.get_size(0), ctx.get_pixel_size(0)),
                            &["with_handle(true)", "Thin separator + wider grab area"],
                        ),
                    ),
                    resizable_handle(ctx, ResizableHandleProps::new().with_handle(true), 0, theme),
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(75.0),
                        1,
                        panel(
                            theme,
                            "Content",
                            format!("{:.1}% / {:.0}px", ctx.get_size(1), ctx.get_pixel_size(1)),
                            &["Dedicated visible-handle example"],
                        ),
                    ),
                ]
            },
        )
    }

    fn nested_demo(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let inner_sizes = &self.nested_inner_sizes;

        resizable_panel_group(
            ResizablePanelGroupProps::new("nested-outer").direction(ResizableDirection::Horizontal),
            &self.nested_outer_sizes,
            Some(Message::NestedOuterResized),
            theme,
            move |ctx| {
                vec![
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(50.0),
                        0,
                        panel(
                            theme,
                            "One",
                            format!("{:.1}% / {:.0}px", ctx.get_size(0), ctx.get_pixel_size(0)),
                            &["Outer horizontal group"],
                        ),
                    ),
                    resizable_handle(ctx, ResizableHandleProps::new(), 0, theme),
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(50.0),
                        1,
                        nested_right_panel(theme, inner_sizes),
                    ),
                ]
            },
        )
    }

    fn controlled_demo(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        resizable_panel_group(
            ResizablePanelGroupProps::new("controlled").direction(ResizableDirection::Horizontal),
            &self.controlled_sizes,
            Some(Message::ControlledResized),
            theme,
            move |ctx| {
                vec![
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(30.0),
                        0,
                        panel(
                            theme,
                            "Left",
                            format!("{:.0}%", ctx.get_size(0).round()),
                            &["Externally managed", "Live layout mirror"],
                        ),
                    ),
                    resizable_handle(ctx, ResizableHandleProps::new(), 0, theme),
                    resizable_panel(
                        ctx,
                        ResizablePanelProps::new(70.0),
                        1,
                        panel_owned(
                            theme,
                            "Right",
                            format!("{:.0}%", ctx.get_size(1).round()),
                            vec![format!("layout = {}", describe_sizes(&ctx_sizes(ctx, 2)))],
                        ),
                    ),
                ]
            },
        )
    }
}

fn header<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;

    column![
        iced_text("Resizable demo").size(28),
        iced_text(
            "Covers the shared reference scenarios from shadcn-ui and shadcn-svelte: \
horizontal, vertical, with handle, nested, and controlled."
        )
        .size(14)
        .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
    ]
    .spacing(8)
    .into()
}

fn controls<'a>(theme: &'a Theme) -> Element<'a, Message> {
    row![
        button(
            "Reset all demos",
            Some(Message::ResetAll),
            ButtonProps::new()
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Size1),
            theme,
        ),
        iced_text("Examples are scrollable and intentionally unconstrained by default.")
            .size(12)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground)
            }),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn demo_card<'a>(
    theme: &'a Theme,
    title: &'a str,
    description: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;

    container(card(
        column![
            column![
                iced_text(title).size(18),
                iced_text(description)
                    .size(13)
                    .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            ]
            .spacing(6),
            content.into(),
        ]
        .spacing(16),
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size3),
        theme,
    ))
    .width(Length::Fill)
    .into()
}

fn coverage_card<'a>(
    theme: &'a Theme,
    horizontal_sizes: &[f32],
    vertical_sizes: &[f32],
    handle_sizes: &[f32],
    nested_outer_sizes: &[f32],
    nested_inner_sizes: &[f32],
    controlled_sizes: &[f32],
) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;
    let primary = theme.palette.primary;

    demo_card(
        theme,
        "Coverage",
        "Reference example coverage and current controlled state.",
        column![
            iced_text("Covered reference scenarios")
                .size(13)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(primary),
                }),
            iced_text("Horizontal, Vertical, With Handle, Nested, Controlled")
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            iced_text(format!("horizontal: {}", describe_sizes(horizontal_sizes)))
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            iced_text(format!("vertical: {}", describe_sizes(vertical_sizes)))
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            iced_text(format!("with handle: {}", describe_sizes(handle_sizes)))
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            iced_text(format!(
                "nested outer: {}",
                describe_sizes(nested_outer_sizes)
            ))
            .size(12)
            .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            iced_text(format!(
                "nested inner: {}",
                describe_sizes(nested_inner_sizes)
            ))
            .size(12)
            .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
            iced_text(format!("controlled: {}", describe_sizes(controlled_sizes)))
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
        ]
        .spacing(6),
    )
}

fn nested_right_panel<'a>(theme: &'a Theme, sizes: &'a [f32]) -> Element<'a, Message> {
    resizable_panel_group(
        ResizablePanelGroupProps::new("nested-inner").direction(ResizableDirection::Vertical),
        sizes,
        Some(Message::NestedInnerResized),
        theme,
        move |ctx| {
            vec![
                resizable_panel(
                    ctx,
                    ResizablePanelProps::new(25.0),
                    0,
                    panel(
                        theme,
                        "Two",
                        format!("{:.1}% / {:.0}px", ctx.get_size(0), ctx.get_pixel_size(0)),
                        &["Inner vertical group"],
                    ),
                ),
                resizable_handle(ctx, ResizableHandleProps::new(), 0, theme),
                resizable_panel(
                    ctx,
                    ResizablePanelProps::new(75.0),
                    1,
                    panel(
                        theme,
                        "Three",
                        format!("{:.1}% / {:.0}px", ctx.get_size(1), ctx.get_pixel_size(1)),
                        &["Matches the nested reference example"],
                    ),
                ),
            ]
        },
    )
}

fn panel<'a>(
    theme: &'a Theme,
    title: &'a str,
    metric: String,
    lines: &[&'a str],
) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;
    let primary = theme.palette.primary;
    let border = theme.palette.border;
    let background = theme.palette.card;

    let mut details = column![
        iced_text(title).size(16),
        iced_text(metric)
            .size(13)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(primary)
            }),
    ]
    .spacing(6);

    for line in lines {
        details = details.push(
            iced_text(*line)
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    container(details)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: iced::Border {
                color: border,
                width: 1.0,
                radius: theme.radius.md.into(),
            },
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn panel_owned<'a>(
    theme: &'a Theme,
    title: &'a str,
    metric: String,
    lines: Vec<String>,
) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;
    let primary = theme.palette.primary;
    let border = theme.palette.border;
    let background = theme.palette.card;

    let mut details = column![
        iced_text(title).size(16),
        iced_text(metric)
            .size(13)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(primary)
            }),
    ]
    .spacing(6);

    for line in lines {
        details = details.push(
            iced_text(line)
                .size(12)
                .style(move |_theme| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    container(details)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: iced::Border {
                color: border,
                width: 1.0,
                radius: theme.radius.md.into(),
            },
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn describe_sizes(sizes: &[f32]) -> String {
    sizes
        .iter()
        .map(|size| format!("{size:.1}%"))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn ctx_sizes(ctx: &iced_shadcn::ResizableContext<'_, Message>, len: usize) -> Vec<f32> {
    (0..len).map(|index| ctx.get_size(index)).collect()
}

fn app<'a>(theme: &'a Theme, content: Element<'a, Message>) -> Element<'a, Message> {
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
