use iced::font::{Family, Weight};
use iced::widget::{
    Column, Row, button as iced_button, column, container, image, responsive, row, scrollable,
    space, stack, text,
};
use iced::{
    Alignment, Background, Border, Color, ContentFit, Element, Length, Padding, Rectangle, Size,
};
use iced_shadcn::{ButtonProps, ButtonSize, ButtonVariant, Theme, button_content};
use lucide_icons::iced::{
    icon_arrow_right, icon_badge_check, icon_github, icon_menu, icon_plus, icon_sliders_horizontal,
    icon_square, icon_sun_moon,
};

use super::app::{Message, PreviewApp};
use super::catalog::PreviewPage;

const MOBILE_BREAKPOINT: f32 = 768.0;
const DARK_CARDS: &[u8] =
    include_bytes!("../../../../../../shadcn-svelte/docs/static/img/registry/full-dark.png");
const LIGHT_CARDS: &[u8] =
    include_bytes!("../../../../../../shadcn-svelte/docs/static/img/registry/full-light.png");
const DEMO_GAP: f32 = 22.0;

pub fn render(app: &PreviewApp) -> Element<'_, Message> {
    responsive(move |size| page(app, size))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn page(app: &PreviewApp, size: Size) -> Element<'_, Message> {
    let compact = size.width < MOBILE_BREAKPOINT;
    let content = column![
        topbar(app, compact),
        hero(app, compact),
        cards_demo(app, size),
        footer(app, compact)
    ]
    .width(Length::Fill);

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .style({
            let background = app.theme().palette.background;
            move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                ..iced::widget::container::Style::default()
            }
        })
        .into()
}

fn topbar<'a>(app: &'a PreviewApp, compact: bool) -> Element<'a, Message> {
    let theme = app.theme();
    let github_content = row![
        icon_github().size(14),
        text(if compact { "8.0k" } else { "9.0k" }).size(12)
    ]
    .spacing(4)
    .align_y(Alignment::Center);
    let github: Element<'a, Message> = if compact {
        container(github_content).padding([0.0, 14.0]).into()
    } else {
        github_content.into()
    };
    let customizer_icon = if compact {
        icon_sun_moon().size(16)
    } else {
        icon_sliders_horizontal().size(15)
    };
    let customizer = iced_shadcn::icon_button(
        customizer_icon,
        Some(if compact {
            Message::ToggleTheme
        } else {
            Message::Noop
        }),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size0),
        theme,
    );
    let new_button = white_button(
        theme,
        row![icon_plus().size(14), semibold("New", 14)]
            .spacing(5)
            .align_y(Alignment::Center),
        Some(Message::SelectPage(PreviewPage::Button)),
        Length::Shrink,
    );

    let content: Element<'a, Message> = if compact {
        row![
            icon_menu().size(18),
            text("Menu").size(18),
            space::horizontal(),
            github,
            divider(theme),
            space::horizontal().width(Length::Fixed(3.0)),
            customizer,
            new_button,
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    } else {
        row![
            row![
                nav_link(theme, "Home", Message::SelectPage(PreviewPage::Home)),
                nav_link(theme, "Docs", Message::Noop),
                nav_link(
                    theme,
                    "Components",
                    Message::SelectPage(PreviewPage::Button)
                ),
                nav_link(theme, "Blocks", Message::Noop),
                nav_link(theme, "Charts", Message::Noop),
                nav_link(theme, "Create", Message::SelectPage(PreviewPage::Button)),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
            space::horizontal(),
            container(
                iced::widget::text_input::TextInput::new("Search documentation...", app.search(),)
                    .on_input(Message::SearchChanged)
                    .padding([6.0, 12.0])
                    .size(14)
                    .style({
                        let muted = theme.palette.muted;
                        let foreground = theme.palette.foreground;
                        let muted_foreground = theme.palette.muted_foreground;
                        move |_theme, _status| iced::widget::text_input::Style {
                            background: Background::Color(muted),
                            border: Border {
                                radius: 12.0.into(),
                                width: 0.0,
                                color: Color::TRANSPARENT,
                            },
                            icon: muted_foreground,
                            placeholder: muted_foreground,
                            value: foreground,
                            selection: theme.palette.primary,
                        }
                    }),
            )
            .width(Length::Fixed(256.0)),
            space::horizontal().width(Length::Fixed(12.0)),
            divider(theme),
            github,
            customizer,
            divider(theme),
            new_button,
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(if compact { 56.0 } else { 64.0 }))
        .padding([0.0, 24.0])
        .align_y(iced::alignment::Vertical::Center)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn nav_link<'a>(theme: &'a Theme, label: &'a str, message: Message) -> Element<'a, Message> {
    button_content(
        text(label).size(12),
        Some(message),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size1),
        theme,
    )
    .into()
}

fn divider<'a>(theme: &'a Theme) -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fixed(1.0))
        .height(Length::Fixed(16.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.border)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn horizontal_divider<'a>(theme: &'a Theme) -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.border)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn hero<'a>(app: &'a PreviewApp, compact: bool) -> Element<'a, Message> {
    let theme = app.theme();
    let announcement = button_content(
        row![text("Introducing Rhea"), icon_arrow_right().size(13)]
            .spacing(5)
            .align_y(Alignment::Center),
        Some(Message::Noop),
        ButtonProps::new()
            .variant(ButtonVariant::Secondary)
            .size(ButtonSize::Size0),
        theme,
    );
    let heading_content: Element<'a, Message> = if compact {
        column![
            text("The Foundation for")
                .size(33.5)
                .font(semibold_font())
                .line_height(iced::widget::text::LineHeight::Relative(1.22)),
            text("your Design System")
                .size(34)
                .font(semibold_font())
                .line_height(iced::widget::text::LineHeight::Relative(1.22)),
        ]
        .spacing(0.0)
        .align_x(Alignment::Center)
        .into()
    } else {
        text("The Foundation for your Design System")
            .size(43)
            .font(semibold_font())
            .line_height(iced::widget::text::LineHeight::Relative(1.08))
            .into()
    };
    let heading = container(heading_content)
        .width(if compact {
            Length::Fill
        } else {
            Length::Fixed(920.0)
        })
        .align_x(iced::alignment::Horizontal::Center);
    let description_content: Element<'a, Message> = if compact {
        column![
            text("A set of beautifully designed components")
                .size(16)
                .line_height(iced::widget::text::LineHeight::Relative(1.5)),
            text("that you can customize, extend,")
                .size(16)
                .line_height(iced::widget::text::LineHeight::Relative(1.5)),
            text("and build on. Start here then make it")
                .size(16)
                .line_height(iced::widget::text::LineHeight::Relative(1.5)),
            text("your own. Open Source. Open Code.")
                .size(16)
                .line_height(iced::widget::text::LineHeight::Relative(1.5)),
        ]
        .spacing(0.0)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    } else {
        text("A set of beautifully designed components that you can customize, extend, and build on. Start here then make it your own. Open Source. Open Code.")
            .size(17.5)
            .line_height(iced::widget::text::LineHeight::Relative(1.54))
            .into()
    };
    let description = container(description_content)
        .width(if compact {
            Length::Fixed(315.0)
        } else {
            Length::Fixed(640.0)
        })
        .align_x(iced::alignment::Horizontal::Center);
    let actions = white_button(
        theme,
        row![semibold("Build Your Own", 14), icon_arrow_right().size(15)]
            .spacing(7)
            .align_y(Alignment::Center),
        Some(Message::SelectPage(PreviewPage::Button)),
        Length::Shrink,
    );

    column![
        announcement,
        space::vertical().height(Length::Fixed(if compact { 2.0 } else { 13.0 })),
        heading,
        space::vertical().height(Length::Fixed(if compact { 7.0 } else { 23.0 })),
        description,
        space::vertical().height(Length::Fixed(if compact { 18.0 } else { 26.0 })),
        actions,
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(if compact {
        Padding::new(0.0)
            .top(32.0)
            .right(24.0)
            .bottom(32.0)
            .left(24.0)
    } else {
        Padding::new(0.0)
            .top(80.0)
            .right(24.0)
            .bottom(71.0)
            .left(24.0)
    })
    .into()
}

fn white_button<'a>(
    theme: &'a Theme,
    content: impl Into<Element<'a, Message>>,
    message: Option<Message>,
    width: Length,
) -> Element<'a, Message> {
    let background = theme.palette.primary;
    let text_color = theme.palette.primary_foreground;
    let mut button = iced_button(content)
        .width(width)
        .height(Length::Fixed(31.0))
        .padding([5.0, 8.0])
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..Default::default()
        });
    if let Some(message) = message {
        button = button.on_press(message);
    }
    button.into()
}

fn cards_demo<'a>(app: &'a PreviewApp, size: Size) -> Element<'a, Message> {
    if size.width < MOBILE_BREAKPOINT {
        return mobile_cards(app, size);
    }

    container(desktop_cards(app))
        .width(Length::Fill)
        .padding([0.0, 23.0])
        .into()
}

fn mobile_cards<'a>(app: &'a PreviewApp, size: Size) -> Element<'a, Message> {
    let bytes = if app.is_dark() {
        DARK_CARDS
    } else {
        LIGHT_CARDS
    };
    let handle = iced::widget::image::Handle::from_bytes(bytes.to_vec());
    let display_width = size.width * 1.4;
    let display_height = display_width * 2764.0 / 2560.0;
    let preview = container(
        image::Image::new(handle)
            .width(Length::Fill)
            .height(Length::Fixed(display_height))
            .crop(Rectangle {
                x: 0,
                y: 0,
                width: 1828,
                height: 2764,
            })
            .content_fit(ContentFit::Contain),
    )
    .width(Length::Fill)
    .clip(true)
    .style({
        let background = app.theme().palette.background;
        move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        }
    });

    container(preview)
        .width(Length::Fill)
        .clip(true)
        .padding(Padding::ZERO)
        .into()
}

fn desktop_cards<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let left = column![
        ui_elements_card(theme),
        sidebar_nav(theme),
        savings_targets(theme)
    ]
    .spacing(DEMO_GAP)
    .width(Length::Fill)
    .align_x(Alignment::Center);
    let middle = column![
        contribution_history(theme),
        claimable_balance(theme),
        dividend_income(theme)
    ]
    .spacing(DEMO_GAP)
    .width(Length::Fill);
    let right = column![qr_connect(theme), transfer_funds(theme), payments(theme)]
        .spacing(DEMO_GAP)
        .width(Length::Fill);

    let content = row![left, middle, right]
        .spacing(DEMO_GAP)
        .width(Length::Fill);
    let background = theme.palette.background;
    let muted = theme.palette.muted;
    let fade_mid = if app.is_dark() { background } else { muted };
    let mut fade = Column::new()
        .width(Length::Fill)
        .height(Length::Fixed(320.0));
    for index in 0..40 {
        let t = (index as f32 + 0.5) / 40.0;
        let (color, alpha) = if t < 0.25 {
            (fade_mid, 0.1 * t / 0.25)
        } else if t < 0.45 {
            (fade_mid, 0.1 + 0.3 * (t - 0.25) / 0.2)
        } else if t < 0.6 {
            (fade_mid, 0.4 + 0.4 * (t - 0.45) / 0.15)
        } else if t < 0.75 {
            (fade_mid, 0.8 - 0.1 * (t - 0.6) / 0.15)
        } else if t < 0.86 {
            (background, 0.7 * (t - 0.75) / 0.11)
        } else {
            (background, 0.45 + 0.55 * (t - 0.86) / 0.14)
        };
        fade = fade.push(
            container(text(""))
                .width(Length::Fill)
                .height(Length::Fixed(8.0))
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(Color { a: alpha, ..color })),
                    ..Default::default()
                }),
        );
    }
    let layered = stack![
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(muted)),
                ..Default::default()
            }),
        column![space::vertical(), fade]
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .width(Length::Fill)
    .height(Length::Fixed(1398.0));
    container(layered)
        .width(Length::Fill)
        .height(Length::Fixed(1398.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background)),
            ..Default::default()
        })
        .into()
}

fn card<'a>(
    theme: &'a Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    container(content)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: 24.0.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}

fn label<'a>(theme: &'a Theme, value: &'a str, size: u16) -> iced::widget::Text<'a> {
    text(value)
        .size(f32::from(size))
        .style(move |_theme| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        })
}

fn semibold_font() -> iced::Font {
    iced::Font {
        family: Family::Name("Inter"),
        weight: Weight::Semibold,
        ..iced::Font::DEFAULT
    }
}

fn semibold<'a>(value: &'a str, size: u16) -> iced::widget::Text<'a> {
    text(value).size(f32::from(size)).font(semibold_font())
}

fn card_title<'a>(value: &'a str, size: u16) -> iced::widget::Text<'a> {
    semibold(value, size)
}

fn chip<'a>(value: &'a str, theme: &'a Theme, kind: ChipKind) -> Element<'a, Message> {
    let (background, foreground, border) = match kind {
        ChipKind::Light => (
            theme.palette.primary,
            theme.palette.primary_foreground,
            Color::TRANSPARENT,
        ),
        ChipKind::Muted => (
            theme.palette.secondary,
            theme.palette.foreground,
            Color::TRANSPARENT,
        ),
        ChipKind::Outline => (
            Color::TRANSPARENT,
            theme.palette.foreground,
            theme.palette.border,
        ),
    };
    iced_button(semibold(value, 12))
        .padding([2.0, 8.0])
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border: Border {
                radius: 16.0.into(),
                width: if border == Color::TRANSPARENT {
                    0.0
                } else {
                    1.0
                },
                color: border,
            },
            ..Default::default()
        })
        .into()
}

fn button_chip<'a>(value: &'a str, theme: &'a Theme, kind: ChipKind) -> Element<'a, Message> {
    let (background, foreground, border) = match kind {
        ChipKind::Light => (
            theme.palette.primary,
            theme.palette.primary_foreground,
            Color::TRANSPARENT,
        ),
        ChipKind::Muted => (
            theme.palette.secondary,
            theme.palette.foreground,
            Color::TRANSPARENT,
        ),
        ChipKind::Outline => (
            Color::TRANSPARENT,
            theme.palette.foreground,
            theme.palette.border,
        ),
    };
    iced_button(semibold(value, 14))
        .padding([6.0, 12.0])
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border: Border {
                radius: 16.0.into(),
                width: if border == Color::TRANSPARENT {
                    0.0
                } else {
                    1.0
                },
                color: border,
            },
            ..Default::default()
        })
        .into()
}

#[derive(Clone, Copy)]
enum ChipKind {
    Light,
    Muted,
    Outline,
}

fn chip_with_icon<'a>(
    value: &'a str,
    icon: iced::widget::Text<'a>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    iced_button(
        row![semibold(value, 14), icon.size(14)]
            .spacing(7)
            .align_y(Alignment::Center),
    )
    .padding([6.0, 12.0])
    .style(move |_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: theme.palette.foreground,
        border: Border {
            radius: 16.0.into(),
            width: 1.0,
            color: theme.palette.border,
        },
        ..Default::default()
    })
    .into()
}

fn primary_chip_with_icon<'a>(
    value: &'a str,
    icon: iced::widget::Text<'a>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    iced_button(
        row![semibold(value, 14), icon.size(14)]
            .spacing(7)
            .align_y(Alignment::Center),
    )
    .padding([6.0, 12.0])
    .style(move |_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(theme.palette.primary)),
        text_color: theme.palette.primary_foreground,
        border: Border {
            radius: 16.0.into(),
            ..Border::default()
        },
        ..Default::default()
    })
    .into()
}

fn radio_control<'a>(theme: &'a Theme, selected: bool) -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
        .style(move |_theme| iced::widget::container::Style {
            background: if selected {
                Some(Background::Color(theme.palette.secondary))
            } else {
                None
            },
            border: Border {
                radius: 8.0.into(),
                width: 2.0,
                color: if selected {
                    theme.palette.muted_foreground
                } else {
                    theme.palette.foreground
                },
            },
            ..Default::default()
        })
        .into()
}

fn checkbox_control<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let background = theme.palette.secondary;
    let foreground = theme.palette.foreground;
    container(icon_badge_check().size(13))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            text_color: Some(foreground),
            ..Default::default()
        })
        .into()
}

fn switch_control<'a>(theme: &'a Theme) -> Element<'a, Message> {
    container(
        container(text(""))
            .width(Length::Fixed(14.0))
            .height(Length::Fixed(14.0))
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.primary)),
                border: Border {
                    radius: 7.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
    )
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(18.0))
    .align_x(iced::alignment::Horizontal::Right)
    .align_y(iced::alignment::Vertical::Center)
    .padding(2.0)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.secondary)),
        border: Border {
            radius: 9.0.into(),
            ..Default::default()
        },
        text_color: Some(theme.palette.foreground),
        ..Default::default()
    })
    .into()
}

fn pending_badge<'a>(theme: &'a Theme) -> Element<'a, Message> {
    container(
        row![
            container(text(""))
                .width(Length::Fixed(8.0))
                .height(Length::Fixed(8.0))
                .style(|_theme| iced::widget::container::Style {
                    background: Some(Background::Color(Color::from_rgb8(0xf0, 0xb4, 0x00))),
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            semibold("Pending Setup", 13),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
    )
    .padding([3.0, 8.0])
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 16.0.into(),
            width: 1.0,
            color: theme.palette.border,
        },
        ..Default::default()
    })
    .into()
}

fn ui_elements_card<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let field = container(
        row![
            label(theme, "Name", 14),
            space::horizontal(),
            icon_square().size(14)
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([7.0, 12.0])
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.secondary)),
        border: Border {
            radius: 16.0.into(),
            ..Border::default()
        },
        ..Default::default()
    });
    let message = container(label(theme, "Message", 14))
        .width(Length::Fill)
        .height(Length::Fixed(64.0))
        .padding([12.0, 12.0])
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.secondary)),
            border: Border {
                radius: 16.0.into(),
                ..Border::default()
            },
            ..Default::default()
        });
    let toggles = row![
        chip("Badge", theme, ChipKind::Light),
        chip("Secondary", theme, ChipKind::Muted),
        space::horizontal(),
        radio_control(theme, false),
        radio_control(theme, true),
        checkbox_control(theme),
        switch_control(theme),
    ]
    .spacing(8)
    .align_y(Alignment::Center);
    let bottom = row![
        button_chip("Alert Dialog", theme, ChipKind::Outline),
        space::horizontal(),
        chip_with_icon("Button Group", icon_square(), theme),
    ]
    .spacing(8);

    card(
        theme,
        column![
            row![
                primary_chip_with_icon("Button", icon_arrow_right(), theme),
                button_chip("Secondary", theme, ChipKind::Muted),
                button_chip("Outline", theme, ChipKind::Outline),
            ]
            .spacing(10),
            field,
            message,
            toggles,
            bottom,
        ]
        .spacing(24)
        .padding(22.0),
    )
    .height(Length::Fixed(318.0))
    .into()
}

fn sidebar_nav<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let planning = sidebar_group(
        theme,
        "Planning",
        &["Documents", "Budget", "Reports", "Goals", "Calendar"],
        None,
    );
    let support = sidebar_group(
        theme,
        "Support",
        &["Help Center", "Docs", "Contact Us", "Status", "Community"],
        None,
    );
    let overview = sidebar_group(
        theme,
        "Overview",
        &[
            "Analytics",
            "Transactions",
            "Investments",
            "Accounts",
            "Spending",
        ],
        Some(0),
    );
    let account = sidebar_group(
        theme,
        "Account",
        &[
            "Profile",
            "Billing",
            "Notifications",
            "Security",
            "Appearance",
        ],
        Some(1),
    );
    row![
        column![planning, overview]
            .spacing(DEMO_GAP)
            .width(Length::Fill),
        column![support, account]
            .spacing(DEMO_GAP)
            .width(Length::Fill),
    ]
    .spacing(DEMO_GAP)
    .into()
}

fn sidebar_group<'a>(
    theme: &'a Theme,
    heading: &'a str,
    values: &'a [&'a str],
    active: Option<usize>,
) -> iced::widget::Container<'a, Message> {
    let mut items = Column::new().spacing(12.0);
    for (index, value) in values.iter().enumerate() {
        let item: Element<'a, Message> = row![sidebar_icon(heading, index), text(*value).size(14)]
            .spacing(9)
            .align_y(Alignment::Center)
            .into();
        let item: Element<'a, Message> = if active == Some(index) {
            container(item)
                .width(Length::Fill)
                .padding([5.0, 8.0])
                .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(Background::Color(theme.palette.secondary)),
                    border: Border {
                        radius: 12.0.into(),
                        ..Border::default()
                    },
                    ..Default::default()
                })
                .into()
        } else {
            item
        };
        items = items.push(item);
    }
    card(
        theme,
        column![label(theme, heading, 12), items]
            .spacing(14)
            .padding(20.0),
    )
    .height(Length::Fixed(226.0))
}

fn sidebar_icon<'a>(heading: &str, index: usize) -> iced::widget::Text<'a> {
    let _ = (heading, index);
    icon_square().size(15)
}

fn savings_targets<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let target = |name: &'a str,
                  amount: &'a str,
                  progress: f32,
                  achieved: &'a str,
                  achieved_amount: &'a str|
     -> Element<'a, Message> {
        column![
            label(theme, name, 12),
            semibold(amount, 30),
            progress_bar(theme, progress),
            row![
                label(theme, achieved, 14),
                space::horizontal(),
                card_title(achieved_amount, 14)
            ],
        ]
        .spacing(10)
        .padding(16.0)
        .width(Length::Fill)
        .into()
    };
    card(
        theme,
        column![
            card_title("Savings Targets", 17),
            label(theme, "Active milestones for 2024 across your portfolio.\nMonitor how close you are to each savings goal.", 14),
            target(
                "RETIREMENT",
                "$420,000",
                0.65,
                "65% achieved",
                "$273,000",
            ),
            target(
                "REAL ESTATE",
                "$85,000",
                0.32,
                "32% achieved",
                "$27,200",
            ),
            space::vertical(),
            container(label(theme, "You have not met your targets for this year.", 14))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        ]
        .spacing(16.0)
        .padding(20.0)
        .height(Length::Fill),
    )
    .height(Length::Fixed(503.0))
    .into()
}

fn progress_bar<'a>(theme: &'a Theme, progress: f32) -> Element<'a, Message> {
    let filled = container(text(""))
        .width(Length::FillPortion((progress * 100.0) as u16))
        .height(Length::Fixed(8.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.primary)),
            ..Default::default()
        });
    let remaining = container(text(""))
        .width(Length::FillPortion(((1.0 - progress) * 100.0) as u16))
        .height(Length::Fixed(8.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.secondary)),
            ..Default::default()
        });
    container(row![filled, remaining].height(Length::Fixed(8.0)))
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.secondary)),
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            ..Default::default()
        })
        .into()
}

fn contribution_history<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let amounts = [
        ("Dec", 0.57),
        ("Jan", 0.79),
        ("Feb", 0.64),
        ("Mar", 0.93),
        ("Apr", 0.54),
        ("May", 1.0),
    ];
    let mut chart = Row::new().spacing(12.0).height(Length::Fixed(200.0));
    for (month, amount) in amounts {
        let bar = container(text(""))
            .width(Length::Fill)
            .height(Length::Fixed(177.0 * amount))
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.muted_foreground)),
                border: Border {
                    radius: 7.0.into(),
                    ..Border::default()
                },
                ..Default::default()
            });
        chart = chart.push(
            column![space::vertical(), bar, label(theme, month, 12)]
                .spacing(8.0)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        );
    }
    card(
        theme,
        column![
            column![
                card_title("Contribution History", 17),
                label(theme, "Last 6 months of activity", 14),
            ]
            .spacing(7.0),
            chart,
            container(
                row![
                    muted_item(theme, "UPCOMING", "May 2024", "Scheduled"),
                    muted_item(theme, "SAVINGS PLAN", "Accelerated", "Recurring"),
                ]
                .spacing(12.0),
            )
            .width(Length::Fill)
            .padding(Padding {
                top: 11.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            }),
            space::vertical(),
            white_button(theme, semibold("View Full Report", 14), None, Length::Fill,),
        ]
        .spacing(14.0)
        .padding(Padding {
            top: 20.0,
            right: 20.0,
            bottom: 22.0,
            left: 20.0,
        }),
    )
    .height(Length::Fixed(503.0))
    .into()
}

fn muted_item<'a>(
    theme: &'a Theme,
    eyebrow: &'a str,
    title: &'a str,
    detail: &'a str,
) -> Element<'a, Message> {
    container(
        column![
            label(theme, eyebrow, 11),
            card_title(title, 16),
            label(theme, detail, 14),
        ]
        .spacing(5.0),
    )
    .width(Length::Fill)
    .padding(16.0)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.secondary)),
        border: Border {
            radius: 16.0.into(),
            ..Border::default()
        },
        ..Default::default()
    })
    .height(Length::Fixed(100.0))
    .into()
}

fn claimable_balance<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let details = container(
        column![
            row![
                label(theme, "Net Royalties", 14),
                space::horizontal(),
                card_title("1,248.75", 14)
            ],
            row![
                label(theme, "Processing Fee", 14),
                space::horizontal(),
                card_title("-37.46", 14)
            ],
            horizontal_divider(theme),
            row![
                label(theme, "Total Ready to Claim", 14),
                space::horizontal(),
                card_title("1,211.29 USD", 14)
            ],
        ]
        .spacing(12.0),
    )
    .padding(16.0)
    .width(Length::Fill)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.secondary)),
        border: Border {
            radius: 16.0.into(),
            ..Border::default()
        },
        ..Default::default()
    });
    card(
        theme,
        column![
            label(theme, "Claimable Balance", 14),
            semibold("1,211.29", 34),
            pending_badge(theme),
            details,
            label(theme, "Once your bank is connected, balances over $10.00 are automatically eligible for monthly distribution on the 15th of each month.", 14),
        ]
        .spacing(12.0)
        .padding(20.0),
    )
    .height(Length::Fixed(391.0))
    .into()
}

fn dividend_income<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let holdings = [
        ("Vanguard", "450 Shares", [0.55, 0.72, 0.58, 0.9]),
        ("S&P 500 VOO", "112 Shares", [0.4, 0.56, 0.82, 0.6]),
        ("Apple AAPL", "85 Shares", [0.3, 0.42, 0.68, 0.48]),
        ("Reality Income", "320 Shares", [0.2, 0.35, 0.48, 0.72]),
    ];
    let mut list = Column::new().spacing(12.0);
    for (name, shares, bars) in holdings {
        let mut bar_row = Row::new().spacing(4.0).align_y(Alignment::End);
        for height in bars {
            bar_row = bar_row.push(
                container(text(""))
                    .width(Length::Fixed(22.0))
                    .height(Length::Fixed(32.0 * height))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(theme.palette.muted_foreground)),
                        border: Border {
                            radius: 5.0.into(),
                            ..Border::default()
                        },
                        ..Default::default()
                    }),
            );
        }
        list = list.push(
            container(row![
                column![card_title(name, 14), label(theme, shares, 13)].spacing(4.0),
                space::horizontal(),
                bar_row,
            ])
            .width(Length::Fill)
            .padding(14.0)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.secondary)),
                border: Border {
                    radius: 16.0.into(),
                    ..Border::default()
                },
                ..Default::default()
            }),
        );
    }
    card(
        theme,
        column![
            row![
                card_title("Q2 Dividend Income", 17),
                space::horizontal(),
                muted_icon_button(theme, icon_square()),
            ],
            label(
                theme,
                "Quarterly dividend payouts across your portfolio holdings.",
                14
            ),
            list,
        ]
        .spacing(14.0)
        .padding(20.0),
    )
    .height(Length::Fixed(460.0))
    .into()
}

const QR_CELLS: [&str; 21] = [
    "111111100101101111111",
    "100000101001001000001",
    "101110101111101011101",
    "101110100100001011101",
    "101110101010101011101",
    "100000100111001000001",
    "111111101010101111111",
    "000000001101000000000",
    "101011111001111010110",
    "010100001110010101001",
    "111010111011101111010",
    "001101000101000010101",
    "110111101111010111011",
    "000000001001010001010",
    "111111101101111101001",
    "100000100010001001111",
    "101110101011101110100",
    "101110100110100010011",
    "101110101000111101110",
    "100000101101000011001",
    "111111101011101101111",
];

const QR_CELL_SIZE: f32 = 160.0 / 21.0;

fn qr_connect<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let mut qr = Column::new().spacing(0.0);
    for line in QR_CELLS {
        let mut qr_line = Row::new().spacing(0.0);
        for cell in line.chars() {
            let color = if cell == '1' {
                Color::BLACK
            } else {
                Color::WHITE
            };
            qr_line = qr_line.push(
                container(text(""))
                    .width(Length::Fixed(QR_CELL_SIZE))
                    .height(Length::Fixed(QR_CELL_SIZE))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(color)),
                        ..Default::default()
                    }),
            );
        }
        qr = qr.push(qr_line);
    }
    let code = container(qr)
        .padding(16.0)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                radius: 12.0.into(),
                width: 1.0,
                color: Color::from_rgb8(0xe5, 0xe5, 0xe5),
            },
            ..Default::default()
        });
    card(
        theme,
        column![
            space::vertical().height(Length::Fixed(15.0)),
            container(code)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            space::vertical().height(Length::Fixed(3.0)),
            container(card_title("Scan to connect your mobile device", 16))
                .width(Length::Fill)
                .align_x(Alignment::Center),
            container(label(
                theme,
                "Open the Ledger mobile app and\nscan this code to link your device.",
                14
            ))
            .width(Length::Fill)
            .align_x(Alignment::Center),
        ]
        .spacing(10.0)
        .padding(20.0)
        .align_x(Alignment::Center),
    )
    .height(Length::Fixed(348.0))
    .into()
}

fn muted_icon_button<'a>(theme: &'a Theme, icon: iced::widget::Text<'a>) -> Element<'a, Message> {
    container(icon.size(16))
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(28.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.secondary)),
            border: Border {
                radius: 16.0.into(),
                ..Border::default()
            },
            ..Default::default()
        })
        .into()
}

fn transfer_funds<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let field = |title: &'a str, value: &'a str| -> Element<'a, Message> {
        column![
            card_title(title, 14),
            container(text(value).size(14))
                .width(Length::Fill)
                .padding([7.0, 12.0])
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(theme.palette.secondary)),
                    border: Border {
                        radius: 16.0.into(),
                        ..Border::default()
                    },
                    ..Default::default()
                })
        ]
        .spacing(8.0)
        .into()
    };
    let summary = container(
        column![
            row![
                label(theme, "Estimated arrival", 14),
                space::horizontal(),
                card_title("Today, Apr 14", 14)
            ],
            horizontal_divider(theme),
            row![
                label(theme, "Transaction fee", 14),
                space::horizontal(),
                card_title("$0.00", 14)
            ],
            horizontal_divider(theme),
            row![
                card_title("Total amount", 14),
                space::horizontal(),
                card_title("$1,200.00", 14)
            ],
        ]
        .spacing(12.0),
    )
    .width(Length::Fill)
    .padding(16.0)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.secondary)),
        border: Border {
            radius: 16.0.into(),
            ..Border::default()
        },
        ..Default::default()
    });
    card(
        theme,
        column![
            row![
                card_title("Transfer Funds", 17),
                space::horizontal(),
                muted_icon_button(theme, icon_square())
            ],
            label(theme, "Move money between your connected accounts.", 14),
            field("Amount to Transfer", "$ 1,200.00"),
            field("From Account", "Main Checking (...8402) — $12,450.00"),
            field("To Account", "High Yield Savings (...1192) — $42,100.00"),
            summary,
            space::vertical(),
            white_button(theme, semibold("Confirm Transfer", 14), None, Length::Fill,),
        ]
        .spacing(14.0)
        .padding(20.0),
    )
    .height(Length::Fixed(604.0))
    .into()
}

fn payments<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let payment = |title: &'a str, description: &'a str| {
        let leading_icon = match title {
            "Change transfer limit" => icon_square(),
            "Scheduled transfers" => icon_square(),
            _ => icon_square(),
        };
        container(
            row![
                leading_icon.size(16),
                column![card_title(title, 14), label(theme, description, 13)].spacing(4.0),
                space::horizontal(),
                icon_arrow_right().size(16),
            ]
            .spacing(10.0),
        )
        .width(Length::Fill)
        .padding(16.0)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.secondary)),
            border: Border {
                radius: 16.0.into(),
                ..Border::default()
            },
            ..Default::default()
        })
    };
    card(
        theme,
        column![
            row![
                label(theme, "Home", 14),
                icon_square().size(14),
                icon_square().size(15),
                icon_square().size(14),
                space::horizontal(),
                card_title("Payments", 14)
            ],
            payment(
                "Change transfer limit",
                "Adjust how much you can send from your balance."
            ),
            payment(
                "Scheduled transfers",
                "Set up a transfer to send at a later date."
            ),
            payment(
                "Recurring card payments",
                "Manage your repeated card transactions."
            ),
        ]
        .spacing(14.0)
        .padding(20.0),
    )
    .height(Length::Fixed(390.0))
    .into()
}

fn footer<'a>(app: &'a PreviewApp, compact: bool) -> Element<'a, Message> {
    let theme = app.theme();
    container(
        text("Built by shadcn. Ported to Svelte by Huntabyte & CokaKoala.")
            .size(if compact { 12 } else { 14 })
            .style(move |_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    )
    .width(Length::Fill)
    .padding([
        if compact { 77.0 } else { 54.0 },
        if compact { 16.0 } else { 24.0 },
    ])
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}
