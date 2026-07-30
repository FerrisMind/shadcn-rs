use iced::font::{Family, Weight};
use iced::mouse;
use iced::widget::text::{Rich, Span};
use iced::widget::{
    Column, Row, column, container, mouse_area, responsive, row, scrollable, space, stack, text,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Size};
use iced_shadcn::{
    AccentColor, AlertDialogProps, AlertProps, AlertVariant, BadgeProps, BadgeSize, BadgeVariant,
    BarChart, BreadcrumbProps, ButtonGroupItem, ButtonProps, ButtonSize, ButtonVariant, CardProps,
    CardSize, CardVariant, ChartGrid, ChartProps, CheckboxProps, CheckboxState, ControlSize,
    DropdownMenuEntry, DropdownMenuItem, DropdownMenuItemProps, DropdownMenuProps,
    InputGroupAddonAlign, InputGroupAddonProps, InputGroupInputProps, InputGroupProps, InputProps,
    InputSize, InputVariant, ItemProps, ProgressProps, ProgressSize, ProgressVariant,
    RadioDirection, RadioGroupProps, RadioItem, SelectProps, SelectSize, SeparatorOrientation,
    SeparatorProps, SidebarGroupLabelProps, SidebarGroupProps, SidebarMenuButtonProps,
    SidebarMenuButtonSize, SidebarProps, SidebarProviderProps, SwitchProps, SwitchSize,
    TextareaProps, TextareaSize, TextareaVariant, Theme, alert, alert_dialog, badge, breadcrumb,
    breadcrumb_item, breadcrumb_link, breadcrumb_list, breadcrumb_page, breadcrumb_separator,
    button_content, button_group, card as shadcn_card, chart, checkbox, dropdown_menu, icon_button,
    input, input_group, input_group_addon, input_group_input, item, radio_group, select, separator,
    sidebar, sidebar_group, sidebar_group_content, sidebar_group_label, sidebar_menu,
    sidebar_menu_button, sidebar_menu_item, sidebar_provider, switch, textarea,
};
use lucide_icons::iced::{
    icon_activity, icon_arrow_left_right, icon_arrow_right, icon_arrow_up, icon_bell,
    icon_book_open, icon_building_2, icon_calendar, icon_chart_bar, icon_chart_line,
    icon_chart_pie, icon_circle_question_mark, icon_credit_card, icon_ellipsis, icon_file_text,
    icon_github, icon_globe, icon_menu, icon_message_square, icon_moon, icon_palette, icon_plus,
    icon_refresh_cw, icon_search, icon_settings, icon_shield, icon_sun, icon_target,
    icon_trending_up, icon_user, icon_wallet, icon_x,
};

use super::app::{FooterLink, Message, PreviewApp};
use super::catalog::PreviewPage;

const MOBILE_BREAKPOINT: f32 = 768.0;
const DEMO_GAP: f32 = 22.0;

fn format_github_stars(stars: u64) -> String {
    if stars >= 1_000 {
        format!("{:.1}k", stars as f64 / 1_000.0)
    } else {
        stars.to_string()
    }
}

pub fn render(app: &PreviewApp) -> Element<'_, Message> {
    responsive(move |size| page(app, size))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn page(app: &PreviewApp, size: Size) -> Element<'_, Message> {
    let compact = size.width < MOBILE_BREAKPOINT;
    let mut content = column![topbar(app, compact), hero(app, compact)].width(Length::Fill);
    if let Some(notice) = app.landing_notice() {
        content = content.push(
            container(alert::<Message>(
                AlertProps::new(notice)
                    .title("Interaction complete")
                    .variant(AlertVariant::Success),
                app.theme(),
            ))
            .padding(Padding {
                top: 0.0,
                right: 24.0,
                bottom: 18.0,
                left: 24.0,
            }),
        );
    }
    let content = content
        .push(cards_demo(app, size))
        .push(footer(app, compact))
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
        text(format_github_stars(app.github_stars())).size(12)
    ]
    .spacing(4)
    .align_y(Alignment::Center);
    let github: Element<'a, Message> = button_content(
        github_content,
        Some(Message::OpenGithub),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size1),
        theme,
    )
    .into();
    let customizer_icon = if app.is_dark() {
        icon_sun()
    } else {
        icon_moon()
    };
    let customizer = iced_shadcn::icon_button(
        customizer_icon.size(if compact { 16 } else { 15 }),
        Some(Message::ToggleTheme),
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
                nav_link(
                    theme,
                    "Docs",
                    Message::OpenUrl("https://ui.shadcn.com/docs"),
                ),
                nav_link(
                    theme,
                    "Components",
                    Message::SelectPage(PreviewPage::Button)
                ),
                nav_link(
                    theme,
                    "Blocks",
                    Message::OpenUrl("https://ui.shadcn.com/blocks"),
                ),
                nav_link(
                    theme,
                    "Charts",
                    Message::OpenUrl("https://ui.shadcn.com/charts"),
                ),
                nav_link(theme, "Create", Message::SelectPage(PreviewPage::Button)),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
            space::horizontal(),
            input(
                app.search(),
                "Search documentation...",
                Some(Message::SearchChanged),
                InputProps::new()
                    .size(InputSize::Size2)
                    .variant(InputVariant::Soft),
                theme,
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
    separator(
        SeparatorProps::new()
            .orientation(SeparatorOrientation::Vertical)
            .length(16.0),
        theme,
    )
    .into()
}

fn hero<'a>(app: &'a PreviewApp, compact: bool) -> Element<'a, Message> {
    let theme = app.theme();
    let announcement = button_content(
        row![text("Introducing Rhea"), icon_arrow_right().size(13)]
            .spacing(5)
            .align_y(Alignment::Center),
        Some(Message::OpenUrl("https://ui.shadcn.com")),
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
    button_content(
        content,
        message,
        ButtonProps::new()
            .variant(ButtonVariant::Solid)
            .size(ButtonSize::Size1),
        theme,
    )
    .width(width)
    .into()
}

fn cards_demo<'a>(app: &'a PreviewApp, size: Size) -> Element<'a, Message> {
    if size.width < MOBILE_BREAKPOINT {
        return mobile_cards(app);
    }

    container(desktop_cards(app))
        .width(Length::Fill)
        .padding([0.0, 23.0])
        .into()
}

fn mobile_cards<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let mut content = column![
        ui_elements_card(app),
        contribution_history(theme),
        claimable_balance(theme),
        sidebar_nav(theme),
        savings_targets(theme),
    ]
    .spacing(DEMO_GAP)
    .width(Length::Fill)
    .padding([0.0, 12.0]);
    if app.landing_dividend_visible() {
        content = content.push(dividend_income(app));
    }
    content = content.push(qr_connect(theme));
    if app.landing_transfer_visible() {
        content = content.push(transfer_funds(app));
    }
    content = content.push(payments(theme));

    container(content)
        .width(Length::Fill)
        .padding(Padding::ZERO)
        .into()
}

fn desktop_cards<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let left = column![
        ui_elements_card(app),
        sidebar_nav(theme),
        savings_targets(theme)
    ]
    .spacing(DEMO_GAP)
    .width(Length::Fill)
    .align_x(Alignment::Center);
    let mut middle = column![contribution_history(theme), claimable_balance(theme)]
        .spacing(DEMO_GAP)
        .width(Length::Fill);
    if app.landing_dividend_visible() {
        middle = middle.push(dividend_income(app));
    }
    let mut right = column![qr_connect(theme)]
        .spacing(DEMO_GAP)
        .width(Length::Fill);
    if app.landing_transfer_visible() {
        right = right.push(transfer_funds(app));
    }
    let right = right.push(payments(theme));

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
    shadcn_card(
        content,
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size3)
            .show_shadow(false)
            .padding(0.0)
            .radius(theme.radius.lg),
        theme,
    )
    .width(Length::Fill)
}

fn panel<'a>(
    theme: &'a Theme,
    content: impl Into<Element<'a, Message>>,
    padding: impl Into<Padding>,
) -> iced::widget::Container<'a, Message> {
    shadcn_card(
        content,
        CardProps::new()
            .variant(CardVariant::Surface)
            .show_shadow(false)
            .background(theme.palette.secondary)
            .border_color(Color::TRANSPARENT)
            .padding(0.0)
            .radius(16.0),
        theme,
    )
    .width(Length::Fill)
    .padding(padding)
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
    let variant = match kind {
        ChipKind::Light => BadgeVariant::Default,
        ChipKind::Muted => BadgeVariant::Secondary,
        ChipKind::Outline => BadgeVariant::Outline,
    };
    badge(
        value,
        BadgeProps::new().variant(variant).size(BadgeSize::Size1),
        theme,
    )
}

fn button_chip<'a>(
    value: &'a str,
    theme: &'a Theme,
    kind: ChipKind,
    message: Message,
) -> Element<'a, Message> {
    let variant = match kind {
        ChipKind::Light => ButtonVariant::Solid,
        ChipKind::Muted => ButtonVariant::Secondary,
        ChipKind::Outline => ButtonVariant::Outline,
    };
    button_content(
        semibold(value, 14),
        Some(message),
        ButtonProps::new().variant(variant).size(ButtonSize::Size1),
        theme,
    )
    .into()
}

#[derive(Clone, Copy)]
enum ChipKind {
    Light,
    Muted,
    Outline,
}

fn primary_chip_with_icon<'a>(
    value: &'a str,
    icon: iced::widget::Text<'a>,
    theme: &'a Theme,
    message: Message,
) -> Element<'a, Message> {
    button_content(
        row![semibold(value, 14), icon.size(14)]
            .spacing(7)
            .align_y(Alignment::Center),
        Some(message),
        ButtonProps::new()
            .variant(ButtonVariant::Solid)
            .size(ButtonSize::Size1),
        theme,
    )
    .into()
}

fn pending_badge<'a>(theme: &'a Theme) -> Element<'a, Message> {
    badge(
        "Pending Setup",
        BadgeProps::new()
            .variant(BadgeVariant::Outline)
            .size(BadgeSize::Size2),
        theme,
    )
}

fn ui_elements_card<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let field = input_group(
        vec![
            input_group_input(
                app.landing_name(),
                "Name",
                Some(Message::LandingNameChanged),
                InputGroupInputProps::new().size(InputSize::Size2),
                theme,
            ),
            input_group_addon(
                icon_search().size(14),
                InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineEnd),
            ),
        ],
        InputGroupProps::new(),
        theme,
    );
    let message = textarea(
        app.landing_message(),
        "Message",
        Some(Message::LandingMessageChanged),
        TextareaProps::new()
            .size(TextareaSize::Size2)
            .variant(TextareaVariant::Surface)
            .rows(2),
        theme,
    )
    .height(Length::Fixed(64.0));
    let radio = radio_group(
        Some(app.landing_radio()),
        vec![RadioItem::new("", 0usize), RadioItem::new("", 1usize)],
        Message::LandingRadioChanged,
        RadioGroupProps::new()
            .direction(RadioDirection::Horizontal)
            .size(ControlSize::Sm),
        theme,
    );
    let toggles = row![
        chip("Badge", theme, ChipKind::Light),
        chip("Secondary", theme, ChipKind::Muted),
        space::horizontal(),
        radio,
        checkbox(
            CheckboxState::from(app.landing_checkbox()),
            Some(|state: CheckboxState| Message::LandingCheckboxChanged(state.is_checked())),
            CheckboxProps::new(),
            theme,
        ),
        switch(
            app.landing_switch(),
            Some(Message::LandingSwitchChanged),
            SwitchProps::new().size(SwitchSize::Size1),
            theme,
        ),
    ]
    .spacing(8)
    .align_y(Alignment::Center);
    let menu = dropdown_menu(
        button_content(
            icon_arrow_up().size(14),
            Some(Message::LandingAction("quick-actions")),
            ButtonProps::new()
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Size0),
            theme,
        ),
        vec![
            DropdownMenuEntry::Label("Quick Actions".into()),
            DropdownMenuEntry::Item(DropdownMenuItem::new(
                "Mute Conversation",
                Some(Message::LandingAction("mute-conversation")),
            )),
            DropdownMenuEntry::Item(DropdownMenuItem::new(
                "Mark as Read",
                Some(Message::LandingAction("mark-read")),
            )),
            DropdownMenuEntry::Item(DropdownMenuItem::new(
                "Block User",
                Some(Message::LandingAction("block-user")),
            )),
            DropdownMenuEntry::Separator,
            DropdownMenuEntry::Item(
                DropdownMenuItem::new(
                    "Delete Conversation",
                    Some(Message::LandingAction("delete-conversation")),
                )
                .props(DropdownMenuItemProps::new().color(AccentColor::Red)),
            ),
        ],
        DropdownMenuProps::new().width(190),
        theme,
    );
    let button_group = button_group(
        vec![
            ButtonGroupItem::new(
                semibold("Button Group", 14),
                Some(Message::LandingAction("button-group")),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Size1),
            ),
            ButtonGroupItem::new(
                menu,
                None,
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Size0),
            ),
        ],
        theme,
    );
    let bottom = row![
        button_chip(
            "Alert Dialog",
            theme,
            ChipKind::Outline,
            Message::LandingAction("open-alert"),
        ),
        space::horizontal(),
        button_group,
    ]
    .spacing(8);

    let base: Element<'a, Message> = card(
        theme,
        column![
            row![
                primary_chip_with_icon(
                    "Button",
                    icon_arrow_right(),
                    theme,
                    Message::LandingAction("button"),
                ),
                button_chip(
                    "Secondary",
                    theme,
                    ChipKind::Muted,
                    Message::LandingAction("secondary"),
                ),
                button_chip(
                    "Outline",
                    theme,
                    ChipKind::Outline,
                    Message::LandingAction("outline"),
                ),
            ]
            .spacing(10),
            field,
            message,
            toggles,
            bottom,
        ]
        .spacing(18)
        .padding(22.0),
    )
    .height(Length::Fixed(318.0))
    .into();

    alert_dialog(
        base,
        app.landing_dialog_open(),
        AlertDialogProps::new(
            "Allow accessory to connect?",
            "Do you want to allow the USB accessory to connect to this device and your data?",
            Message::LandingAction("alert-confirm"),
            Message::LandingAction("alert-cancel"),
        )
        .confirm_label("Allow")
        .cancel_label("Don't allow"),
        theme,
    )
}

fn sidebar_nav<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let planning = sidebar_group_card(
        theme,
        "Planning",
        &["Documents", "Budget", "Reports", "Goals", "Calendar"],
        None,
    );
    let support = sidebar_group_card(
        theme,
        "Support",
        &["Help Center", "Docs", "Contact Us", "Status", "Community"],
        None,
    );
    let overview = sidebar_group_card(
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
    let account = sidebar_group_card(
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

fn sidebar_group_card<'a>(
    theme: &'a Theme,
    heading: &'a str,
    values: &'a [&'a str],
    active: Option<usize>,
) -> Element<'a, Message> {
    sidebar_provider(
        SidebarProviderProps::new(true)
            .expanded_width(174.0)
            .collapsed_width(174.0)
            .animate(false),
        None::<fn(bool) -> Message>,
        |ctx| {
            card(
                theme,
                sidebar(ctx, SidebarProps::new().border(false), theme, |ctx| {
                    let mut entries = Vec::with_capacity(values.len());
                    for (index, value) in values.iter().enumerate() {
                        entries.push(sidebar_menu_item(vec![
                            container(sidebar_icon(heading, index))
                                .width(Length::Fixed(22.0))
                                .center_x(Length::Fill)
                                .into(),
                            sidebar_menu_button(
                                SidebarMenuButtonProps::new(*value)
                                    .size(SidebarMenuButtonSize::Sm)
                                    .active(active == Some(index)),
                                Some(Message::LandingAction("sidebar-item")),
                                ctx,
                                theme,
                            ),
                        ]));
                    }
                    sidebar_group(
                        ctx,
                        SidebarGroupProps::new().spacing(8.0),
                        vec![
                            sidebar_group_label(SidebarGroupLabelProps::new(heading), ctx, theme),
                            sidebar_group_content(vec![sidebar_menu(entries)]),
                        ],
                    )
                }),
            )
            .height(Length::Fixed(226.0))
            .into()
        },
    )
}

fn sidebar_icon<'a>(heading: &str, index: usize) -> iced::widget::Text<'a> {
    let icon = match (heading, index) {
        ("Planning", 0) => icon_file_text(),
        ("Planning", 1) => icon_wallet(),
        ("Planning", 2) => icon_chart_bar(),
        ("Planning", 3) => icon_target(),
        ("Planning", 4) => icon_calendar(),
        ("Support", 0) => icon_circle_question_mark(),
        ("Support", 1) => icon_book_open(),
        ("Support", 2) => icon_message_square(),
        ("Support", 3) => icon_activity(),
        ("Support", 4) => icon_globe(),
        ("Overview", 0) => icon_chart_line(),
        ("Overview", 1) => icon_arrow_left_right(),
        ("Overview", 2) => icon_trending_up(),
        ("Overview", 3) => icon_building_2(),
        ("Overview", 4) => icon_chart_pie(),
        ("Account", 0) => icon_user(),
        ("Account", 1) => icon_credit_card(),
        ("Account", 2) => icon_bell(),
        ("Account", 3) => icon_shield(),
        ("Account", 4) => icon_palette(),
        _ => icon_settings(),
    };
    icon.size(15)
}

fn savings_targets<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let target = |name: &'a str,
                  amount: &'a str,
                  progress: f32,
                  achieved: &'a str,
                  achieved_amount: &'a str|
     -> Element<'a, Message> {
        panel(
            theme,
            column![
                item(ItemProps::new(name), theme),
                semibold(amount, 32),
                iced_shadcn::progress(
                    ProgressProps::new()
                        .value(progress * 100.0)
                        .max(100.0)
                        .size(ProgressSize::Size2)
                        .variant(ProgressVariant::Surface)
                        .color(AccentColor::Blue),
                    theme,
                ),
                row![
                    label(theme, achieved, 14),
                    space::horizontal(),
                    card_title(achieved_amount, 14)
                ],
            ]
            .spacing(8),
            0.0,
        )
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

fn contribution_history<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let contribution_chart = chart(
        ChartProps::new()
            .show_legend(false)
            .show_grid(ChartGrid::new(false, false))
            .show_x(false)
            .height(176.0)
            .margin([2.0, 4.0]),
        theme,
        |plot| {
            BarChart::new(vec![
                (0.0, 800.0),
                (1.0, 1100.0),
                (2.0, 900.0),
                (3.0, 1300.0),
                (4.0, 750.0),
                (5.0, 1400.0),
            ])
            .label("Contributions")
            .color(theme.palette.chart_2)
            .bar_width(0.62)
            .show(plot);
        },
    );
    let month_labels = row![
        label(theme, "Dec", 12),
        label(theme, "Jan", 12),
        label(theme, "Feb", 12),
        label(theme, "Mar", 12),
        label(theme, "Apr", 12),
        label(theme, "May", 12),
    ]
    .spacing(12)
    .width(Length::Fill)
    .align_y(Alignment::Center);
    card(
        theme,
        column![
            column![
                card_title("Contribution History", 17),
                label(theme, "Last 6 months of activity", 14),
            ]
            .spacing(7.0),
            column![contribution_chart, month_labels].spacing(0),
            row![
                muted_item(theme, "UPCOMING", "May 2024", "Scheduled"),
                muted_item(theme, "SAVINGS PLAN", "Accelerated", "Recurring"),
            ]
            .spacing(12),
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
    panel(
        theme,
        column![
            label(theme, eyebrow, 11),
            card_title(title, 16),
            label(theme, detail, 14),
        ]
        .spacing(5.0)
        .padding(16.0),
        0.0,
    )
    .height(Length::Fixed(100.0))
    .into()
}

fn claimable_balance<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let details = panel(
        theme,
        column![
            detail_item(theme, "Net Royalties", "1,248.75"),
            detail_item(theme, "Processing Fee", "-37.46"),
            separator(SeparatorProps::new(), theme),
            detail_item(theme, "Total Ready to Claim", "1,211.29 USD"),
        ]
        .spacing(4),
        16.0,
    );
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

fn detail_item<'a>(theme: &'a Theme, title: &'a str, description: &'a str) -> Element<'a, Message> {
    item(ItemProps::new(title).description(description), theme)
}

fn dividend_income<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let holdings = [
        ("Vanguard", "450 Shares", [380.0, 420.0, 390.0, 652.0]),
        ("S&P 500 VOO", "112 Shares", [180.0, 210.0, 320.0, 218.0]),
        ("Apple AAPL", "85 Shares", [60.0, 70.0, 120.0, 90.0]),
        ("Realty Income", "320 Shares", [240.0, 260.0, 280.0, 360.0]),
    ];
    let mut list = Column::new().spacing(12.0);
    for (name, shares, bars) in holdings {
        let holding_chart = chart(
            ChartProps::new()
                .show_legend(false)
                .show_grid(ChartGrid::new(false, false))
                .show_x(false)
                .height(32.0)
                .margin([0.0, 0.0]),
            theme,
            |plot| {
                BarChart::new(vec![
                    (0.0, bars[0]),
                    (1.0, bars[1]),
                    (2.0, bars[2]),
                    (3.0, bars[3]),
                ])
                .color(theme.palette.chart_2)
                .bar_width(0.72)
                .show(plot);
            },
        );
        list = list.push(panel(
            theme,
            row![
                container(item(ItemProps::new(name).description(shares), theme))
                    .width(Length::Fill),
                holding_chart,
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            0.0,
        ));
    }
    card(
        theme,
        column![
            row![
                card_title("Q2 Dividend Income", 17),
                space::horizontal(),
                icon_button(
                    icon_x(),
                    Some(Message::LandingAction("dismiss-dividend")),
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size0),
                    theme,
                ),
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

fn transfer_funds<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    const FROM_ACCOUNTS: [&str; 2] = [
        "Main Checking (...8402) — $12,450.00",
        "Business (...7731) — $8,920.00",
    ];
    const TO_ACCOUNTS: [&str; 2] = [
        "High Yield Savings (...1192) — $42,100.00",
        "Investment (...3349) — $18,200.00",
    ];
    let from = FROM_ACCOUNTS
        .iter()
        .copied()
        .find(|value| *value == app.landing_from_account());
    let to = TO_ACCOUNTS
        .iter()
        .copied()
        .find(|value| *value == app.landing_to_account());
    let amount = input_group(
        vec![
            input_group_addon(text("$").size(14), InputGroupAddonProps::new()),
            input_group_input(
                app.landing_amount(),
                "1,200.00",
                Some(Message::LandingAmountChanged),
                InputGroupInputProps::new().size(InputSize::Size2),
                theme,
            ),
        ],
        InputGroupProps::new(),
        theme,
    );
    let from_account = select(
        &FROM_ACCOUNTS,
        from,
        "Select account",
        |value: &str| Message::LandingFromAccountChanged(value.to_owned()),
        SelectProps::new().size(SelectSize::Size2),
        theme,
    )
    .width(Length::Fill);
    let to_account = select(
        &TO_ACCOUNTS,
        to,
        "Select account",
        |value: &str| Message::LandingToAccountChanged(value.to_owned()),
        SelectProps::new().size(SelectSize::Size2),
        theme,
    )
    .width(Length::Fill);
    let summary = panel(
        theme,
        column![
            detail_item(theme, "Estimated arrival", "Today, Apr 14"),
            separator(SeparatorProps::new(), theme),
            detail_item(theme, "Transaction fee", "$0.00"),
            separator(SeparatorProps::new(), theme),
            detail_item(theme, "Total amount", "$1,200.00"),
        ]
        .spacing(4),
        16.0,
    );
    card(
        theme,
        column![
            row![
                card_title("Transfer Funds", 17),
                space::horizontal(),
                icon_button(
                    icon_x(),
                    Some(Message::LandingAction("dismiss-transfer")),
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size0),
                    theme,
                )
            ],
            label(theme, "Move money between your connected accounts.", 14),
            column![card_title("Amount to Transfer", 14), amount].spacing(8),
            column![card_title("From Account", 14), from_account].spacing(8),
            column![card_title("To Account", 14), to_account].spacing(8),
            summary,
            space::vertical(),
            white_button(
                theme,
                semibold("Confirm Transfer", 14),
                Some(Message::LandingAction("confirm-transfer")),
                Length::Fill,
            ),
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
            "Change transfer limit" => icon_settings(),
            "Scheduled transfers" => icon_calendar(),
            _ => icon_refresh_cw(),
        };
        button_content(
            row![
                leading_icon.size(16),
                container(item(ItemProps::new(title).description(description), theme))
                    .width(Length::Fill),
                icon_arrow_right().size(16),
            ]
            .spacing(10),
            Some(Message::LandingAction("payment-action")),
            ButtonProps::new()
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Size3),
            theme,
        )
        .width(Length::Fill)
    };
    let breadcrumbs = breadcrumb(theme, BreadcrumbProps::new().text_size(14.0), |ctx| {
        let account_menu = dropdown_menu(
            button_content(
                icon_ellipsis().size(16),
                Some(Message::LandingAction("account-options")),
                ButtonProps::new()
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Size0),
                theme,
            ),
            vec![
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "Profile",
                    Some(Message::LandingAction("profile")),
                )),
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "Statements",
                    Some(Message::LandingAction("statements")),
                )),
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "Documents",
                    Some(Message::LandingAction("documents")),
                )),
            ],
            DropdownMenuProps::new().width(160),
            theme,
        );
        breadcrumb_list(
            ctx,
            vec![
                breadcrumb_item(
                    ctx,
                    vec![breadcrumb_link(
                        "Home",
                        Some(Message::LandingAction("breadcrumb-home")),
                        ctx,
                    )],
                ),
                breadcrumb_separator(ctx, None),
                breadcrumb_item(ctx, vec![account_menu]),
                breadcrumb_separator(ctx, None),
                breadcrumb_item(ctx, vec![breadcrumb_page("Payments", ctx)]),
            ],
        )
    });
    card(
        theme,
        column![
            breadcrumbs,
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
    let size = if compact { 12.0 } else { 14.0 };
    let muted = theme.palette.muted_foreground;
    let link_color = theme.palette.primary;

    let muted_text = move |value: &'static str| {
        text(value)
            .size(size)
            .style(move |_theme| iced::widget::text::Style { color: Some(muted) })
    };

    container(
        row![
            muted_text("Built by "),
            footer_link(
                "shadcn",
                "https://ui.shadcn.com",
                FooterLink::Shadcn,
                app.footer_link_hovered(FooterLink::Shadcn),
                size,
                link_color,
            ),
            muted_text(". Ported to "),
            footer_link(
                "iced",
                "https://iced.rs",
                FooterLink::Iced,
                app.footer_link_hovered(FooterLink::Iced),
                size,
                link_color,
            ),
            muted_text(" by FerrisMind."),
        ]
        .spacing(0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([
        if compact { 77.0 } else { 54.0 },
        if compact { 16.0 } else { 24.0 },
    ])
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}

fn footer_link<'a>(
    label: &'static str,
    url: &'static str,
    link: FooterLink,
    hovered: bool,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    let content =
        Rich::<(), Message>::with_spans(vec![Span::new(label).color(color).underline(hovered)])
            .size(size);

    mouse_area(content)
        .on_press(Message::OpenUrl(url))
        .on_enter(Message::FooterLinkHover(link, true))
        .on_exit(Message::FooterLinkHover(link, false))
        .interaction(mouse::Interaction::Pointer)
        .into()
}
