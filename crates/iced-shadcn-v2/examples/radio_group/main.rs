//! Interactive playground for `iced-shadcn-v2::RadioGroup`.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example radio_group`

use iced::keyboard::{self, key};
use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Background, Element, Event, Length, Subscription, Task};

use iced_shadcn_v2::{
    RadioGroup, RadioGroupItem, RadioGroupOrientation, RadioGroupSize, Theme, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    spacing: String,
    notify: String,
    plan: String,
}

#[derive(Debug, Clone)]
enum Message {
    SpacingChanged(String),
    NotifyChanged(String),
    PlanChanged(String),
    NotifyStepped(bool),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            spacing: "comfortable".to_owned(),
            notify: "all".to_owned(),
            plan: "yearly".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Radio Group".to_owned()
    }

    /// Arrow keys drive the `notify` group, the way bits-ui's roving tabindex
    /// does on the web. iced has no focus tree, so the app asks the group which
    /// value comes next.
    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key.as_ref() {
                keyboard::Key::Named(key::Named::ArrowDown | key::Named::ArrowRight) => {
                    Some(Message::NotifyStepped(true))
                }
                keyboard::Key::Named(key::Named::ArrowUp | key::Named::ArrowLeft) => {
                    Some(Message::NotifyStepped(false))
                }
                _ => None,
            },
            _ => None,
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SpacingChanged(value) => self.spacing = value,
            Message::NotifyChanged(value) => self.notify = value,
            Message::PlanChanged(value) => self.plan = value,
            Message::NotifyStepped(forward) => {
                let stepped = {
                    let group = self.notify_group();
                    let value = if forward {
                        group.next_value()
                    } else {
                        group.previous_value()
                    };

                    value.map(str::to_owned)
                };

                if let Some(value) = stepped {
                    self.notify = value;
                }
            }
        }

        Task::none()
    }

    /// The `notify` group is built twice — once to answer arrow keys in
    /// `update`, once to render in `view` — because a builder is consumed when
    /// it becomes an `Element`.
    fn notify_group(&self) -> RadioGroup<'_, Message> {
        RadioGroup::new(&self.theme)
            .value(self.notify.clone())
            .focused(self.notify.clone())
            .name("notify")
            .required(true)
            .push(RadioGroupItem::text("all", "All new messages"))
            .push(RadioGroupItem::text(
                "mentions",
                "Direct messages and mentions",
            ))
            .push(RadioGroupItem::text("none", "Nothing"))
            .on_change(Message::NotifyChanged)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let basic = RadioGroup::new(theme)
            .value(self.spacing.clone())
            .push(RadioGroupItem::text("default", "Default"))
            .push(RadioGroupItem::text("comfortable", "Comfortable"))
            .push(RadioGroupItem::text("compact", "Compact"))
            .on_change(Message::SpacingChanged);

        let horizontal = RadioGroup::new(theme)
            .value(self.spacing.clone())
            .orientation(RadioGroupOrientation::Horizontal)
            .spacing(6.0)
            .push(RadioGroupItem::text("default", "Default"))
            .push(RadioGroupItem::text("comfortable", "Comfortable"))
            .push(RadioGroupItem::text("compact", "Compact"))
            .on_change(Message::SpacingChanged);

        let sizes = RadioGroup::new(theme)
            .value(self.spacing.clone())
            .orientation(RadioGroupOrientation::Horizontal)
            .spacing(6.0)
            .size(RadioGroupSize::Lg)
            .push(RadioGroupItem::text("default", "Large"))
            .push(RadioGroupItem::text("comfortable", "indicators"))
            .on_change(Message::SpacingChanged);

        let descriptions = RadioGroup::new(theme)
            .value(self.plan.clone())
            .full_width()
            .push(
                RadioGroupItem::text("monthly", "Monthly ($9.99/month)")
                    .description("Cancel any time, billed every month."),
            )
            .push(
                RadioGroupItem::text("yearly", "Yearly ($99.99/year)")
                    .description("Save 17% compared to monthly billing."),
            )
            .push(
                RadioGroupItem::text("lifetime", "Lifetime ($299.99)")
                    .description("One payment, no renewals — sold out for now.")
                    .disabled(true),
            )
            .on_change(Message::PlanChanged);

        let invalid = RadioGroup::new(theme)
            .value(self.spacing.clone())
            .invalid(true)
            .orientation(RadioGroupOrientation::Horizontal)
            .spacing(6.0)
            .push(RadioGroupItem::text("default", "Default"))
            .push(RadioGroupItem::text("compact", "Compact"))
            .on_change(Message::SpacingChanged);

        let disabled = RadioGroup::new(theme)
            .value("comfortable")
            .disabled(true)
            .push(RadioGroupItem::text("comfortable", "Comfortable"))
            .push(RadioGroupItem::text("compact", "Compact"));

        let content = column![
            text("iced-shadcn-v2 Radio Group")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text(
                "shadcn-svelte parity: controlled value, orientation, focus ring, invalid, disabled"
            )
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(palette.muted_foreground),
            section("Vertical (pack gap)", basic, theme),
            section("Horizontal", horizontal, theme),
            section("Large indicators", sizes, theme),
            section("Arrow keys + focus ring", self.notify_group(), theme),
            section("Labels with descriptions", descriptions, theme),
            section("aria-invalid", invalid, theme),
            section("Disabled group", disabled, theme),
            text(format!(
                "spacing: {}; notify: {}; plan: {}",
                self.spacing, self.notify, self.plan
            ))
            .size(13)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(18)
        .max_width(900)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn section<'a>(
    label: &'static str,
    group: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(17)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        group.into(),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .into()
}
