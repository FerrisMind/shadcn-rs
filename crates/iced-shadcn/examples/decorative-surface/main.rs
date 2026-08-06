use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length, Task};

use iced_shadcn::new_api::{Button, ButtonVariant, Theme as ApiTheme};
use iced_shadcn::{DecorativeSurfaceProps, Theme, decorative_surface};
use twill::prelude::{DynamicSemanticTheme, SemanticThemeVars, ThemeVariant};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .run()
}

struct Example {
    use_dynamic: bool,
    variant: ThemeVariant,
    pressed: u32,
    api_theme: ApiTheme,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleSource,
    ToggleVariant,
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            use_dynamic: false,
            variant: ThemeVariant::Light,
            pressed: 0,
            api_theme: ApiTheme::light(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn DecorativeSurface".to_owned()
    }

    fn theme(&self) -> Theme {
        if self.use_dynamic {
            let dynamic = DynamicSemanticTheme::from_brand_oklch(0.628, 0.258, 29.234);
            Theme::from_dynamic_semantic_theme(&dynamic, self.variant)
        } else {
            Theme::from_semantic_theme(SemanticThemeVars::shadcn_neutral(), self.variant)
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleSource => {
                self.use_dynamic = !self.use_dynamic;
            }
            Message::ToggleVariant => {
                self.variant = if self.variant.is_dark() {
                    ThemeVariant::Light
                } else {
                    ThemeVariant::Dark
                };
                self.api_theme = if self.variant.is_dark() {
                    ApiTheme::dark()
                } else {
                    ApiTheme::light()
                };
            }
            Message::Pressed => {
                self.pressed += 1;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = self.theme();
        let api_theme = &self.api_theme;
        let background = theme.semantic_color(twill::prelude::SemanticColor::Background);
        let foreground = theme.semantic_color(twill::prelude::SemanticColor::Foreground);
        let primary = theme.semantic_color(twill::prelude::SemanticColor::Primary);
        let card_foreground = theme.semantic_foreground(twill::prelude::SemanticColor::Card);

        let controls = row![
            Button::text(
                if self.use_dynamic {
                    "Use shadcn neutral"
                } else {
                    "Use dynamic brand"
                },
                api_theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleSource),
            Button::text(
                if self.variant.is_dark() {
                    "Switch to light"
                } else {
                    "Switch to dark"
                },
                api_theme,
            )
            .variant(ButtonVariant::Secondary)
            .on_press(Message::ToggleVariant),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let action = Button::text("Decorated action", api_theme)
            .variant(ButtonVariant::Default)
            .on_press(Message::Pressed);

        let underlay = container("")
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(iced::Color { a: 0.12, ..primary })),
                ..iced::widget::container::Style::default()
            });

        let overlay = container(
            text(if self.use_dynamic {
                "dynamic semantic theme"
            } else {
                "shadcn semantic theme"
            })
            .size(12),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(12)
        .align_x(iced::alignment::Horizontal::Right)
        .align_y(iced::alignment::Vertical::Top)
        .style(move |_| iced::widget::container::Style {
            text_color: Some(card_foreground),
            ..iced::widget::container::Style::default()
        });

        let surface = decorative_surface(
            action,
            vec![underlay.into()],
            vec![overlay.into()],
            DecorativeSurfaceProps::new()
                .themed()
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fixed(112.0)),
            &theme,
        );

        container(
            column![
                text("DecorativeSurface").size(32),
                text("Generic decorative host with semantic theme defaults").size(16),
                text(format!("Pressed: {}", self.pressed)).size(14),
                controls,
                surface,
            ]
            .spacing(20)
            .max_width(860),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            text_color: Some(foreground),
            ..iced::widget::container::Style::default()
        })
        .into()
    }
}
