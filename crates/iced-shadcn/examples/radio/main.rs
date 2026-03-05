use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonVariant, ControlSize, RadioDirection, RadioGroupProps,
    RadioItem, TextProps, TextSize, TextWeight, Theme, button, label, radio_group, text,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Plan {
    Starter,
    Pro,
    Team,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Notification {
    All,
    Mentions,
    Nothing,
}

#[derive(Default)]
struct Example {
    theme: Theme,
    demo_selection: Option<Plan>,
    notification_selection: Option<Notification>,
    horizontal_selection: Option<Plan>,
    disabled_selection: Option<Plan>,
    size_sm: Option<Plan>,
    size_md: Option<Plan>,
    size_lg: Option<Plan>,
    color_blue: Option<Notification>,
    color_green: Option<Notification>,
    color_amber: Option<Notification>,
    high_contrast: Option<Plan>,
    form_selection: Option<Plan>,
    form_error: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Demo(Plan),
    Notification(Notification),
    Horizontal(Plan),
    Disabled(Plan),
    SizeSm(Plan),
    SizeMd(Plan),
    SizeLg(Plan),
    ColorBlue(Notification),
    ColorGreen(Notification),
    ColorAmber(Notification),
    HighContrast(Plan),
    Form(Plan),
    Submit,
    Reset,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Demo(v) => self.demo_selection = Some(v),
            Message::Notification(v) => self.notification_selection = Some(v),
            Message::Horizontal(v) => self.horizontal_selection = Some(v),
            Message::Disabled(v) => self.disabled_selection = Some(v),
            Message::SizeSm(v) => self.size_sm = Some(v),
            Message::SizeMd(v) => self.size_md = Some(v),
            Message::SizeLg(v) => self.size_lg = Some(v),
            Message::ColorBlue(v) => self.color_blue = Some(v),
            Message::ColorGreen(v) => self.color_green = Some(v),
            Message::ColorAmber(v) => self.color_amber = Some(v),
            Message::HighContrast(v) => self.high_contrast = Some(v),
            Message::Form(v) => {
                self.form_selection = Some(v);
                self.form_error = None;
            }
            Message::Submit => {
                if self.form_selection.is_none() {
                    self.form_error =
                        Some("You must select a subscription plan to continue.".to_string());
                } else {
                    self.form_error = None;
                }
            }
            Message::Reset => {
                self.form_selection = None;
                self.form_error = None;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border_color = theme.palette.border;
        let radius = theme.radius.md;

        // Demo — with descriptions
        let demo_section = column![
            label("Display density", theme),
            muted_text("Choose how compact the UI is.", theme),
            radio_group(
                self.demo_selection,
                plan_items_with_desc(),
                Message::Demo,
                RadioGroupProps::new(),
                theme,
            ),
        ]
        .spacing(8);

        // Notifications — simple list
        let notifications_section = column![
            label("Notify me about...", theme),
            radio_group(
                self.notification_selection,
                notification_items(),
                Message::Notification,
                RadioGroupProps::new(),
                theme,
            ),
        ]
        .spacing(8);

        // Horizontal direction
        let horizontal_section = radio_group(
            self.horizontal_selection,
            plan_items(),
            Message::Horizontal,
            RadioGroupProps::new().direction(RadioDirection::Horizontal),
            theme,
        );

        // Disabled
        let disabled_section = radio_group(
            self.disabled_selection,
            plan_items(),
            Message::Disabled,
            RadioGroupProps::new().disabled(true),
            theme,
        );

        // Sizes
        let sizes_section = column![
            row![
                caption("Sm", theme).width(Length::Fixed(48.0)),
                radio_group(
                    self.size_sm,
                    plan_items(),
                    Message::SizeSm,
                    RadioGroupProps::new()
                        .size(ControlSize::Sm)
                        .direction(RadioDirection::Horizontal),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                caption("Md", theme).width(Length::Fixed(48.0)),
                radio_group(
                    self.size_md,
                    plan_items(),
                    Message::SizeMd,
                    RadioGroupProps::new()
                        .size(ControlSize::Md)
                        .direction(RadioDirection::Horizontal),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                caption("Lg", theme).width(Length::Fixed(48.0)),
                radio_group(
                    self.size_lg,
                    plan_items(),
                    Message::SizeLg,
                    RadioGroupProps::new()
                        .size(ControlSize::Lg)
                        .direction(RadioDirection::Horizontal),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(12);

        // Colors
        let colors_section = column![
            row![
                caption("Blue", theme).width(Length::Fixed(72.0)),
                radio_group(
                    self.color_blue,
                    notification_items(),
                    Message::ColorBlue,
                    RadioGroupProps::new()
                        .color(AccentColor::Blue)
                        .direction(RadioDirection::Horizontal),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                caption("Green", theme).width(Length::Fixed(72.0)),
                radio_group(
                    self.color_green,
                    notification_items(),
                    Message::ColorGreen,
                    RadioGroupProps::new()
                        .color(AccentColor::Green)
                        .direction(RadioDirection::Horizontal),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                caption("Amber", theme).width(Length::Fixed(72.0)),
                radio_group(
                    self.color_amber,
                    notification_items(),
                    Message::ColorAmber,
                    RadioGroupProps::new()
                        .color(AccentColor::Amber)
                        .direction(RadioDirection::Horizontal),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(12);

        // High contrast
        let high_contrast_section = radio_group(
            self.high_contrast,
            plan_items(),
            Message::HighContrast,
            RadioGroupProps::new().high_contrast(true),
            theme,
        );

        // Form with validation
        let form_header = column![
            label("Plan", theme),
            muted_text("You can upgrade or downgrade your plan at any time.", theme),
        ]
        .spacing(4);

        let form_radio = radio_group(
            self.form_selection,
            plan_items_with_desc(),
            Message::Form,
            RadioGroupProps::new(),
            theme,
        );

        let error_text: Element<'_, Message> = if let Some(err) = &self.form_error {
            let color = theme.palette.destructive;
            iced_text(err.as_str())
                .size(13)
                .style(move |_| iced::widget::text::Style { color: Some(color) })
                .into()
        } else {
            column![].into()
        };

        let form_section = column![
            form_header,
            form_radio,
            error_text,
            row![
                button("Save", Some(Message::Submit), ButtonProps::new(), theme,),
                button(
                    "Reset",
                    Some(Message::Reset),
                    ButtonProps::new().variant(ButtonVariant::Outline),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content = column![
            section(theme, "Demo", demo_section),
            section(theme, "Notifications", notifications_section),
            section(theme, "Horizontal", horizontal_section),
            section(theme, "Disabled", disabled_section),
            section(theme, "Sizes", sizes_section),
            section(theme, "Colors", colors_section),
            section(theme, "High Contrast", high_contrast_section),
            section(theme, "Form", form_section),
        ]
        .spacing(16);

        let content = scrollable(content).height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .padding(32)
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

fn plan_items() -> Vec<RadioItem<Plan>> {
    vec![
        RadioItem::new("Starter", Plan::Starter),
        RadioItem::new("Pro", Plan::Pro),
        RadioItem::new("Team", Plan::Team),
    ]
}

fn plan_items_with_desc() -> Vec<RadioItem<Plan>> {
    vec![
        RadioItem::new("Starter", Plan::Starter)
            .description("For everyday use with basic features."),
        RadioItem::new("Pro", Plan::Pro).description("For professionals who need more features."),
        RadioItem::new("Team", Plan::Team).description("Best for teams and organizations."),
    ]
}

fn notification_items() -> Vec<RadioItem<Notification>> {
    vec![
        RadioItem::new("All", Notification::All),
        RadioItem::new("Mentions", Notification::Mentions),
        RadioItem::new("Nothing", Notification::Nothing),
    ]
}

fn section<'a, Message: 'a>(
    theme: &Theme,
    title: impl iced::widget::text::IntoFragment<'a>,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let title = text(
        title,
        TextProps::new()
            .size(TextSize::Size4)
            .weight(TextWeight::Medium),
        theme,
    );
    let background = theme.palette.card;
    let border_color = theme.palette.border;
    let radius = theme.radius.md;

    container(column![title, content.into()].spacing(12))
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border_color,
            },
            ..iced::widget::container::Style::default()
        })
}

fn muted_text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    let color = theme.palette.muted_foreground;
    iced_text(content)
        .size(13)
        .style(move |_theme| iced::widget::text::Style { color: Some(color) })
}

fn caption<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    let color = theme.palette.muted_foreground;
    iced_text(content)
        .size(12)
        .style(move |_theme| iced::widget::text::Style { color: Some(color) })
}
