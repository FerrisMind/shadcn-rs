//! Interactive playground for `iced-shadcn-v2::Tabs`.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example tabs`.

use iced::widget::{column, container, scrollable, text};
use iced::{Background, Element, Length, Task};

use iced_shadcn_v2::{
    Tabs, TabsActivationMode, TabsContent, TabsList, TabsListLoop, TabsListVariant,
    TabsOrientation, TabsTrigger, Theme, fonts, iced_font,
};

fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    active: String,
    vertical_active: String,
    line_active: String,
}

#[derive(Debug, Clone)]
enum Message {
    Active(String),
    VerticalActive(String),
    LineActive(String),
    ToggleMode,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            active: "account".to_owned(),
            vertical_active: "account".to_owned(),
            line_active: "overview".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Tabs".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Active(value) => self.active = value,
            Message::VerticalActive(value) => self.vertical_active = value,
            Message::LineActive(value) => self.line_active = value,
            Message::ToggleMode => {
                self.theme = if self.theme.is_dark() {
                    self.theme
                        .clone()
                        .with_mode(iced_shadcn_v2::ThemeMode::Light)
                } else {
                    self.theme
                        .clone()
                        .with_mode(iced_shadcn_v2::ThemeMode::Dark)
                };
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let default_tabs = Tabs::new(theme)
            .value(&self.active)
            .list(
                TabsList::new(theme)
                    .push(TabsTrigger::text("account", "Account", theme))
                    .push(TabsTrigger::text("password", "Password", theme))
                    .push(TabsTrigger::text("disabled", "Disabled", theme).disabled(true)),
            )
            .push(TabsContent::text(
                "account",
                "Manage your account preferences and profile information.",
                theme,
            ))
            .push(TabsContent::text(
                "password",
                "Update your password to keep your account secure.",
                theme,
            ))
            .on_value_change(Message::Active);

        let vertical_tabs = Tabs::new(theme)
            .value(&self.vertical_active)
            .orientation(TabsOrientation::Vertical)
            .activation_mode(TabsActivationMode::Manual)
            .list_loop(TabsListLoop::Disabled)
            .list(
                TabsList::new(theme)
                    .variant(TabsListVariant::Line)
                    .push(TabsTrigger::text("account", "Account", theme))
                    .push(TabsTrigger::text("password", "Password", theme))
                    .push(TabsTrigger::text("notifications", "Notifications", theme)),
            )
            .push(TabsContent::text(
                "account",
                "Manage your account preferences and profile information.",
                theme,
            ))
            .push(TabsContent::text(
                "password",
                "Use a strong password with a mix of letters, numbers, and symbols.",
                theme,
            ))
            .push(TabsContent::text(
                "notifications",
                "Configure how you receive notifications and alerts.",
                theme,
            ))
            .on_value_change(Message::VerticalActive);

        let line_tabs = Tabs::new(theme)
            .value(&self.line_active)
            .list(
                TabsList::new(theme)
                    .variant(TabsListVariant::Line)
                    .push(TabsTrigger::text("overview", "Overview", theme))
                    .push(TabsTrigger::text("analytics", "Analytics", theme))
                    .push(TabsTrigger::text("reports", "Reports", theme)),
            )
            .push(TabsContent::text(
                "overview",
                "A transparent list variant with a selected underline.",
                theme,
            ))
            .push(TabsContent::text(
                "analytics",
                "The active panel follows the controlled value.",
                theme,
            ))
            .push(TabsContent::text(
                "reports",
                "Disabled triggers stay out of keyboard navigation.",
                theme,
            ))
            .on_value_change(Message::LineActive);

        let content =
            column![
            text("Tabs")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Controlled values, line indicators, vertical orientation, and disabled triggers.")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            iced_shadcn_v2::Button::text(
                if theme.is_dark() { "Use light mode" } else { "Use dark mode" },
                theme,
            )
            .on_press(Message::ToggleMode),
            section("Default", default_tabs),
            section("Vertical + manual activation", vertical_tabs),
            section("Line variant", line_tabs),
        ]
            .spacing(20)
            .max_width(760)
            .padding(24);

        container(scrollable(
            container(content)
                .width(Length::Fill)
                .center_x(Length::Fill),
        ))
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

fn section<'a>(title: &'static str, tabs: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![text(title).size(16), tabs.into()].spacing(8).into()
}
