use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Task};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, InputProps, InputSize, InputVariant,
    SeparatorOrientation, SeparatorProps, StepperItem, StepperOrientation, StepperProps, Theme,
    button, button_content, input, label, separator, stepper, stepper_description,
    stepper_indicator, stepper_item, stepper_next, stepper_previous, stepper_title,
    stepper_trigger,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShippingMethod {
    Standard,
    Express,
    Overnight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaymentMethod {
    Card,
    OpenSourceSpecial,
}

#[derive(Debug, Clone)]
enum Message {
    BasicStepChanged(usize),
    BasicNext,
    BasicPrevious,
    IconsStepChanged(usize),
    VerticalStepChanged(usize),
    FormStepChanged(usize),
    StreetChanged(String),
    CityChanged(String),
    StateChanged(String),
    ZipChanged(String),
    ShippingChanged(ShippingMethod),
    PaymentChanged(PaymentMethod),
    FormNext,
    FormPrevious,
    CompleteOrder,
}

struct Example {
    theme: Theme,
    basic_step: usize,
    icons_step: usize,
    vertical_step: usize,
    form_step: usize,
    street: String,
    city: String,
    state: String,
    zip: String,
    shipping_method: Option<ShippingMethod>,
    payment_method: Option<PaymentMethod>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            basic_step: 2,
            icons_step: 2,
            vertical_step: 2,
            form_step: 1,
            street: String::new(),
            city: String::new(),
            state: String::new(),
            zip: String::new(),
            shipping_method: None,
            payment_method: None,
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BasicStepChanged(step) => self.basic_step = step,
            Message::BasicNext => {
                if self.basic_step < 4 {
                    self.basic_step += 1;
                }
            }
            Message::BasicPrevious => {
                if self.basic_step > 1 {
                    self.basic_step -= 1;
                }
            }
            Message::IconsStepChanged(step) => self.icons_step = step,
            Message::VerticalStepChanged(step) => self.vertical_step = step,
            Message::FormStepChanged(step) => self.form_step = step,
            Message::StreetChanged(value) => self.street = value,
            Message::CityChanged(value) => self.city = value,
            Message::StateChanged(value) => self.state = value,
            Message::ZipChanged(value) => self.zip = value,
            Message::ShippingChanged(value) => self.shipping_method = Some(value),
            Message::PaymentChanged(value) => self.payment_method = Some(value),
            Message::FormNext => {
                if self.can_advance_form() && self.form_step < 4 {
                    self.form_step += 1;
                }
            }
            Message::FormPrevious => {
                if self.form_step > 1 {
                    self.form_step -= 1;
                }
            }
            Message::CompleteOrder => {
                self.basic_step = 2;
                self.icons_step = 2;
                self.vertical_step = 2;
                self.form_step = 1;
                self.street.clear();
                self.city.clear();
                self.state.clear();
                self.zip.clear();
                self.shipping_method = None;
                self.payment_method = None;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let outer_bg = theme.palette.background;

        let content = column![
            self.basic_section(),
            section_header(
                theme,
                "Icons",
                "Add an icon to the Indicator component to display an icon.",
            ),
            demo_card(theme, self.icons_section()),
            section_header(
                theme,
                "Vertical",
                "Add orientation=\"vertical\" to the Nav component to make the stepper vertical.",
            ),
            demo_card(theme, self.vertical_section()),
            section_header(
                theme,
                "Form",
                "Create a multi-step form with the Stepper component.",
            ),
            demo_card(theme, self.form_section())
        ]
        .spacing(28)
        .width(Length::Fill);

        container(scrollable(content))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(outer_bg)),
                ..iced::widget::container::Style::default()
            })
            .into()
    }

    fn basic_section(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let nav = stepper(
            basic_step_items(theme),
            self.basic_step,
            Some(Message::BasicStepChanged),
            StepperProps::new()
                .orientation(StepperOrientation::Horizontal)
                .item_spacing(4.0)
                .content_spacing(0.0)
                .separator_thickness(4.0),
            theme,
        );

        let controls = row![
            stepper_previous(
                "Previous",
                Some(Message::BasicPrevious),
                self.basic_step,
                4,
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Size2),
                theme,
            ),
            container(stepper_next(
                "Next",
                Some(Message::BasicNext),
                self.basic_step,
                4,
                ButtonProps::new()
                    .variant(ButtonVariant::Default)
                    .size(ButtonSize::Size2),
                theme,
            ),)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center);

        demo_card(
            theme,
            column![nav, controls].spacing(20).width(Length::Fill),
        )
        .into()
    }

    fn icons_section(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        stepper(
            icons_step_items(theme),
            self.icons_step,
            Some(Message::IconsStepChanged),
            StepperProps::new()
                .orientation(StepperOrientation::Horizontal)
                .item_spacing(4.0)
                .content_spacing(4.0)
                .separator_thickness(4.0),
            theme,
        )
    }

    fn vertical_section(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        container(stepper(
            vertical_step_items(theme),
            self.vertical_step,
            Some(Message::VerticalStepChanged),
            StepperProps::new()
                .orientation(StepperOrientation::Vertical)
                .item_spacing(4.0)
                .content_spacing(4.0)
                .separator_thickness(4.0),
            theme,
        ))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into()
    }

    fn form_section(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let max_valid_step = self.max_valid_step();
        let nav = stepper(
            form_step_items(max_valid_step, theme),
            self.form_step,
            Some(Message::FormStepChanged),
            StepperProps::new()
                .orientation(StepperOrientation::Horizontal)
                .item_spacing(4.0)
                .content_spacing(4.0)
                .separator_thickness(4.0),
            theme,
        );

        let body: Element<'_, Message> = match self.form_step {
            1 => shipping_address_form(theme, self),
            2 => shipping_method_form(theme, self),
            3 => payment_method_form(theme, self),
            4 => order_summary(theme, self),
            _ => shipping_address_form(theme, self),
        };

        let controls: Element<'_, Message> = if self.form_step < 4 {
            row![
                stepper_previous(
                    "Previous",
                    Some(Message::FormPrevious),
                    self.form_step,
                    4,
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
                stepper_next(
                    "Next",
                    if self.can_advance_form() {
                        Some(Message::FormNext)
                    } else {
                        None
                    },
                    self.form_step,
                    4,
                    ButtonProps::new()
                        .variant(ButtonVariant::Default)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        } else {
            row![
                stepper_previous(
                    "Previous",
                    Some(Message::FormPrevious),
                    self.form_step,
                    4,
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
                button(
                    "Complete Order",
                    Some(Message::CompleteOrder),
                    ButtonProps::new()
                        .variant(ButtonVariant::Default)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        };

        column![nav, body, controls]
            .spacing(20)
            .width(Length::Fill)
            .into()
    }

    fn has_address(&self) -> bool {
        !self.street.is_empty()
            && !self.city.is_empty()
            && !self.state.is_empty()
            && !self.zip.is_empty()
    }

    fn max_valid_step(&self) -> usize {
        if !self.has_address() {
            1
        } else if self.shipping_method.is_none() {
            2
        } else if self.payment_method.is_none() {
            3
        } else {
            4
        }
    }

    fn can_advance_form(&self) -> bool {
        self.form_step < self.max_valid_step()
    }
}

fn basic_step_items<'a>(theme: &'a Theme) -> Vec<StepperItem<'a, Message>> {
    vec![
        stepper_item(
            "basic-1",
            stepper_trigger(stepper_indicator(text("1")), stepper_title(" ", theme)),
        ),
        stepper_item(
            "basic-2",
            stepper_trigger(stepper_indicator(text("2")), stepper_title(" ", theme)),
        ),
        stepper_item(
            "basic-3",
            stepper_trigger(stepper_indicator(text("3")), stepper_title(" ", theme)),
        ),
        stepper_item(
            "basic-4",
            stepper_trigger(stepper_indicator(text("4")), stepper_title(" ", theme)),
        ),
    ]
}

fn icons_step_items<'a>(theme: &'a Theme) -> Vec<StepperItem<'a, Message>> {
    vec![
        stepper_item(
            "address",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::BookUser))),
                stepper_title("Address", theme),
            )
            .description(stepper_description("Add your address", theme)),
        ),
        stepper_item(
            "shipping",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::Truck))),
                stepper_title("Shipping", theme),
            )
            .description(stepper_description("Select your shipping method", theme)),
        ),
        stepper_item(
            "payment",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::CreditCard))),
                stepper_title("Payment", theme),
            )
            .description(stepper_description("Add your payment method", theme)),
        ),
        stepper_item(
            "checkout",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::ShoppingCart))),
                stepper_title("Checkout", theme),
            )
            .description(stepper_description("Confirm your order", theme)),
        ),
    ]
}

fn vertical_step_items<'a>(theme: &'a Theme) -> Vec<StepperItem<'a, Message>> {
    icons_step_items(theme)
}

fn form_step_items<'a>(max_valid_step: usize, theme: &'a Theme) -> Vec<StepperItem<'a, Message>> {
    vec![
        stepper_item(
            "form-address",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::BookUser))),
                stepper_title("Address", theme),
            )
            .description(stepper_description("Add your address", theme)),
        ),
        stepper_item(
            "form-shipping",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::Truck))),
                stepper_title("Shipping", theme),
            )
            .description(stepper_description("Select your shipping method", theme)),
        )
        .disabled(max_valid_step < 2),
        stepper_item(
            "form-payment",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::CreditCard))),
                stepper_title("Payment", theme),
            )
            .description(stepper_description("Add your payment method", theme)),
        )
        .disabled(max_valid_step < 3),
        stepper_item(
            "form-checkout",
            stepper_trigger(
                stepper_indicator(text(icon_text(Icon::ShoppingCart))),
                stepper_title("Checkout", theme),
            )
            .description(stepper_description("Confirm your order", theme)),
        )
        .disabled(max_valid_step < 4),
    ]
}

fn shipping_address_form<'a>(theme: &'a Theme, example: &'a Example) -> Element<'a, Message> {
    column![
        heading(
            theme,
            "Shipping Address",
            "Please enter your delivery address"
        ),
        column![
            field(
                theme,
                "Street Address",
                &example.street,
                "123 Main St",
                Some(Message::StreetChanged),
            ),
            row![
                field(
                    theme,
                    "City",
                    &example.city,
                    "New York",
                    Some(Message::CityChanged),
                ),
                field(
                    theme,
                    "State",
                    &example.state,
                    "NY",
                    Some(Message::StateChanged),
                ),
            ]
            .spacing(16)
            .width(Length::Fill),
            field(
                theme,
                "ZIP Code",
                &example.zip,
                "10001",
                Some(Message::ZipChanged)
            ),
        ]
        .spacing(16),
    ]
    .spacing(20)
    .width(Length::Fill)
    .into()
}

fn shipping_method_form<'a>(theme: &'a Theme, example: &'a Example) -> Element<'a, Message> {
    let options = [
        (
            ShippingMethod::Standard,
            "Standard Shipping",
            "Free",
            "5-7 business days",
        ),
        (
            ShippingMethod::Express,
            "Express Shipping",
            "$9.99",
            "2-3 business days",
        ),
        (
            ShippingMethod::Overnight,
            "Overnight Shipping",
            "$19.99",
            "Next business day",
        ),
    ];

    column![
        heading(
            theme,
            "Shipping Method",
            "Select your preferred shipping option",
        ),
        column(
            options
                .into_iter()
                .map(|(method, label_text, price, delivery)| {
                    shipping_option_card(
                        theme,
                        method,
                        label_text,
                        price,
                        delivery,
                        example.shipping_method,
                    )
                })
                .collect::<Vec<_>>(),
        )
        .spacing(12),
    ]
    .spacing(20)
    .width(Length::Fill)
    .into()
}

fn payment_method_form<'a>(theme: &'a Theme, example: &'a Example) -> Element<'a, Message> {
    column![
        heading(theme, "Payment Method", "Choose how you'd like to pay"),
        column![
            payment_card(
                theme,
                PaymentMethod::Card,
                "Pay with Card",
                Icon::CreditCard,
                example.payment_method,
                column![
                    field(theme, "Card Number", "", "1234 5678 9012 3456", None),
                    row![
                        field(theme, "Expiry Date", "", "MM/YY", None),
                        field(theme, "CVV", "", "123", None),
                    ]
                    .spacing(16)
                    .width(Length::Fill),
                    field(theme, "Cardholder Name", "", "John Doe", None),
                ]
                .into(),
                Some(Message::PaymentChanged(PaymentMethod::Card)),
            ),
            payment_card(
                theme,
                PaymentMethod::OpenSourceSpecial,
                "Open Source Special",
                Icon::Smile,
                example.payment_method,
                column![text("Nothing to pay just enjoy the free software!").size(14),].into(),
                Some(Message::PaymentChanged(PaymentMethod::OpenSourceSpecial)),
            ),
        ]
        .spacing(12),
    ]
    .spacing(20)
    .width(Length::Fill)
    .into()
}

fn order_summary<'a>(theme: &'a Theme, example: &'a Example) -> Element<'a, Message> {
    let shipping_label = match example.shipping_method {
        Some(ShippingMethod::Standard) => "Standard Shipping",
        Some(ShippingMethod::Express) => "Express Shipping",
        Some(ShippingMethod::Overnight) => "Overnight Shipping",
        None => "-",
    };
    let payment_label = match example.payment_method {
        Some(PaymentMethod::Card) => "Pay with Card",
        Some(PaymentMethod::OpenSourceSpecial) => "Open Source Special",
        None => "-",
    };

    container(
        column![
            heading(theme, "Order Summary", "Review your order details"),
            container(
                column![
                    summary_block(
                        theme,
                        "Shipping Address",
                        if example.has_address() {
                            format!(
                                "{}\n{}, {} {}",
                                example.street, example.city, example.state, example.zip
                            )
                        } else {
                            String::from("-")
                        },
                    ),
                    summary_block(theme, "Shipping Method", shipping_label.to_string()),
                    summary_block(theme, "Payment Method", payment_label.to_string()),
                    column![
                        separator(
                            SeparatorProps::new()
                                .orientation(SeparatorOrientation::Horizontal)
                                .thickness(1.0),
                            theme,
                        ),
                        row![text("Stepper Component").size(13), text("$0.00").size(13),]
                            .width(Length::Fill)
                            .spacing(8)
                            .align_y(Alignment::Center)
                            .padding([10, 0]),
                        separator(
                            SeparatorProps::new()
                                .orientation(SeparatorOrientation::Horizontal)
                                .thickness(1.0),
                            theme,
                        ),
                        row![text("Order Total").size(14), text("$0.00").size(18),]
                            .width(Length::Fill)
                            .spacing(8)
                            .align_y(Alignment::Center)
                            .padding([10, 0]),
                    ]
                    .spacing(0)
                    .width(Length::Fill),
                ]
                .spacing(18),
            )
            .padding(20)
            .width(Length::Fill)
            .style({
                let background = theme.palette.background;
                let border = theme.palette.border;
                let radius = theme.radius.md;
                move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(background)),
                    border: Border {
                        radius: radius.into(),
                        width: 1.0,
                        color: border,
                    },
                    ..iced::widget::container::Style::default()
                }
            }),
        ]
        .spacing(20),
    )
    .width(Length::Fill)
    .into()
}

fn shipping_option_card<'a>(
    theme: &'a Theme,
    method: ShippingMethod,
    label_text: &'a str,
    price: &'a str,
    delivery: &'a str,
    current: Option<ShippingMethod>,
) -> Element<'a, Message> {
    let selected = current == Some(method);
    let props = ButtonProps::new()
        .variant(if selected {
            ButtonVariant::Soft
        } else {
            ButtonVariant::Outline
        })
        .size(ButtonSize::Size4);

    let content = container(
        row![
            column![
                text(label_text).size(14),
                text(delivery)
                    .size(12)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
            ]
            .spacing(4)
            .width(Length::Fill),
            text(price).size(13),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .align_x(Alignment::Start);

    button_content(
        content,
        Some(Message::ShippingChanged(method)),
        props,
        theme,
    )
    .width(Length::Fill)
    .into()
}

fn payment_card<'a>(
    theme: &'a Theme,
    method: PaymentMethod,
    label_text: &'a str,
    icon: Icon,
    current: Option<PaymentMethod>,
    body: Element<'a, Message>,
    on_select: Option<Message>,
) -> Element<'a, Message> {
    let selected = current == Some(method);
    let border = if selected {
        theme.palette.primary
    } else {
        theme.palette.border
    };
    let background = if selected {
        theme.palette.accent
    } else {
        theme.palette.background
    };

    let header = button_content(
        container(
            row![
                radio_dot(theme, selected),
                text(icon_text(icon)).size(14),
                text(label_text).size(14),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .align_x(Alignment::Start),
        on_select,
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Size2),
        theme,
    )
    .width(Length::Fill);

    let mut content = column![header].spacing(0).width(Length::Fill);
    if selected {
        content = content.push(container(body).padding([0, 36]).width(Length::Fill));
    }

    container(content)
        .padding(0)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: 10.0.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn summary_block<'a>(theme: &'a Theme, label_text: &'a str, value: String) -> Element<'a, Message> {
    column![
        text(label_text).size(14),
        text(value)
            .size(13)
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(6)
    .into()
}

fn field<'a>(
    theme: &'a Theme,
    label_text: &'a str,
    value: &'a str,
    placeholder: &'a str,
    on_input: Option<fn(String) -> Message>,
) -> Element<'a, Message> {
    column![
        label(label_text, theme),
        input(
            value,
            placeholder,
            on_input,
            InputProps::new()
                .size(InputSize::Size2)
                .variant(InputVariant::Surface),
            theme,
        ),
    ]
    .spacing(8)
    .width(Length::Fill)
    .into()
}

fn heading<'a>(theme: &'a Theme, title: &'a str, description: &'a str) -> Element<'a, Message> {
    column![
        text(title).size(22),
        text(description)
            .size(13)
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(4)
    .into()
}

fn section_header<'a>(
    theme: &'a Theme,
    title: &'a str,
    description: &'a str,
) -> Element<'a, Message> {
    column![
        text(title).size(18),
        text(description)
            .size(13)
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(4)
    .into()
}

fn demo_card<'a>(
    theme: &'a Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
        .width(Length::Fill)
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

fn icon_text(icon: Icon) -> String {
    char::from(icon).to_string()
}

fn radio_dot<'a>(theme: &'a Theme, selected: bool) -> Element<'a, Message> {
    let background = if selected {
        theme.palette.primary
    } else {
        theme.palette.background
    };
    let border = if selected {
        theme.palette.primary
    } else {
        theme.palette.border
    };

    container(text(""))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: 999.0.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
        .into()
}
