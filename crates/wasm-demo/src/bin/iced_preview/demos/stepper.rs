use super::super::app::preview_card;
use super::super::app::{Message, PreviewApp};
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, InputProps, InputSize, InputVariant, StepperItem,
    StepperOrientation, StepperProps, Theme, input, label, stepper, stepper_description,
    stepper_indicator, stepper_item, stepper_next, stepper_previous, stepper_title,
    stepper_trigger,
};
use lucide_icons::Icon;

pub fn render<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();

    scrollable(
        column![
            basic_section(theme, app.stepper_step()),
            section_header(
                theme,
                "Icons",
                "Add an icon to the Indicator component to display an icon.",
            ),
            preview_card(theme, "Icons", icons_section(theme)),
            section_header(
                theme,
                "Vertical",
                "Add orientation=\"vertical\" to the Nav component to make the stepper vertical.",
            ),
            preview_card(theme, "Vertical", vertical_section(theme)),
            section_header(
                theme,
                "Form",
                "Create a multi-step form with the Stepper component.",
            ),
            preview_card(theme, "Form", form_section(theme)),
        ]
        .spacing(24)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn basic_section<'a>(theme: &'a Theme, step: usize) -> Element<'a, Message> {
    let nav = stepper(
        basic_step_items(theme),
        step,
        Some(Message::StepperStepChanged),
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
            Some(Message::StepperPrevious),
            step,
            4,
            ButtonProps::new()
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Size2),
            theme,
        ),
        container(stepper_next(
            "Next",
            Some(Message::StepperNext),
            step,
            4,
            ButtonProps::new()
                .variant(ButtonVariant::Default)
                .size(ButtonSize::Size2),
            theme,
        ))
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let content = column![nav, controls].spacing(20).width(Length::Fill);

    preview_card(
        theme,
        "Horizontal",
        row![
            container(text("")).width(Length::FillPortion(1)),
            container(content).width(Length::FillPortion(10)),
            container(text("")).width(Length::FillPortion(1)),
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center),
    )
    .into()
}

fn icons_section<'a>(theme: &'a Theme) -> Element<'a, Message> {
    stepper(
        icons_step_items(theme),
        2,
        Some(noop_step),
        StepperProps::new()
            .orientation(StepperOrientation::Horizontal)
            .item_spacing(4.0)
            .content_spacing(4.0)
            .separator_thickness(4.0),
        theme,
    )
}

fn vertical_section<'a>(theme: &'a Theme) -> Element<'a, Message> {
    container(stepper(
        vertical_step_items(theme),
        2,
        Some(noop_step),
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

fn form_section<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let nav = stepper(
        form_step_items(1, theme),
        1,
        Some(noop_step),
        StepperProps::new()
            .orientation(StepperOrientation::Horizontal)
            .item_spacing(4.0)
            .content_spacing(4.0)
            .separator_thickness(4.0),
        theme,
    );

    let body = shipping_address_form(theme);

    let controls = row![
        stepper_previous(
            "Previous",
            None::<Message>,
            1,
            4,
            ButtonProps::new()
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Size2),
            theme,
        ),
        stepper_next(
            "Next",
            None::<Message>,
            1,
            4,
            ButtonProps::new()
                .variant(ButtonVariant::Default)
                .size(ButtonSize::Size2),
            theme,
        ),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    column![nav, body, controls]
        .spacing(20)
        .width(Length::Fill)
        .into()
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
                stepper_indicator(text(icon(Icon::BookUser))),
                stepper_title("Address", theme),
            )
            .description(stepper_description("Add your address", theme)),
        ),
        stepper_item(
            "shipping",
            stepper_trigger(
                stepper_indicator(text(icon(Icon::Truck))),
                stepper_title("Shipping", theme),
            )
            .description(stepper_description("Select your shipping method", theme)),
        ),
        stepper_item(
            "payment",
            stepper_trigger(
                stepper_indicator(text(icon(Icon::CreditCard))),
                stepper_title("Payment", theme),
            )
            .description(stepper_description("Add your payment method", theme)),
        ),
        stepper_item(
            "checkout",
            stepper_trigger(
                stepper_indicator(text(icon(Icon::ShoppingCart))),
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
                stepper_indicator(text(icon(Icon::BookUser))),
                stepper_title("Address", theme),
            )
            .description(stepper_description("Add your address", theme)),
        ),
        stepper_item(
            "form-shipping",
            stepper_trigger(
                stepper_indicator(text(icon(Icon::Truck))),
                stepper_title("Shipping", theme),
            )
            .description(stepper_description("Select your shipping method", theme)),
        )
        .disabled(max_valid_step < 2),
        stepper_item(
            "form-payment",
            stepper_trigger(
                stepper_indicator(text(icon(Icon::CreditCard))),
                stepper_title("Payment", theme),
            )
            .description(stepper_description("Add your payment method", theme)),
        )
        .disabled(max_valid_step < 3),
        stepper_item(
            "form-checkout",
            stepper_trigger(
                stepper_indicator(text(icon(Icon::ShoppingCart))),
                stepper_title("Checkout", theme),
            )
            .description(stepper_description("Confirm your order", theme)),
        )
        .disabled(max_valid_step < 4),
    ]
}

fn shipping_address_form<'a>(theme: &'a Theme) -> Element<'a, Message> {
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
                "",
                "123 Main St",
                None::<fn(String) -> Message>,
            ),
            row![
                field(theme, "City", "", "New York", None::<fn(String) -> Message>),
                field(theme, "State", "", "NY", None::<fn(String) -> Message>),
            ]
            .spacing(16)
            .width(Length::Fill),
            field(
                theme,
                "ZIP Code",
                "",
                "10001",
                None::<fn(String) -> Message>
            ),
        ]
        .spacing(16),
    ]
    .spacing(20)
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

fn noop_step(_: usize) -> Message {
    Message::Noop
}

fn icon(icon: Icon) -> String {
    char::from(icon).to_string()
}
