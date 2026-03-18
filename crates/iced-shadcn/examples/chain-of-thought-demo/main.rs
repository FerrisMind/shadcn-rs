use std::time::{Duration, Instant};

use iced::time;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Subscription, Task};
use lucide_icons::{Icon as LucideIcon, LUCIDE_FONT_BYTES};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, ChainOfThoughtContentProps, ChainOfThoughtHeaderProps,
    ChainOfThoughtProps, ChainOfThoughtSearchResultProps, ChainOfThoughtSearchResultsProps,
    ChainOfThoughtState, ChainOfThoughtStepProps, ChainOfThoughtStepStatus, ChainOfThoughtUpdate,
    Theme, badge, button_content, chain_of_thought, chain_of_thought_header_default,
    chain_of_thought_reduce, chain_of_thought_search_result, chain_of_thought_search_results,
    chain_of_thought_step, chain_of_thought_step_is_visible,
};

const FRAME_MS: u64 = 32;

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Start,
    Stop,
    ToggleOpen,
}

struct Example {
    theme: Theme,
    chain_state: ChainOfThoughtState,
}

impl Default for Example {
    fn default() -> Self {
        let props = ChainOfThoughtProps::new().default_open(true);
        Self {
            theme: Theme::dark(),
            chain_state: ChainOfThoughtState::from_props(props),
        }
    }
}

impl Example {
    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(FRAME_MS)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {}
            Message::Start => {
                let now = Instant::now();
                let _ = chain_of_thought_reduce(
                    &mut self.chain_state,
                    ChainOfThoughtUpdate::OpenChanged { open: true, now },
                    ChainOfThoughtProps::new(),
                );
            }
            Message::Stop => {
                let now = Instant::now();
                let _ = chain_of_thought_reduce(
                    &mut self.chain_state,
                    ChainOfThoughtUpdate::OpenChanged { open: false, now },
                    ChainOfThoughtProps::new(),
                );
            }
            Message::ToggleOpen => {
                let now = Instant::now();
                let next_open = !self.chain_state.open;
                let _ = chain_of_thought_reduce(
                    &mut self.chain_state,
                    ChainOfThoughtUpdate::OpenChanged {
                        open: next_open,
                        now,
                    },
                    ChainOfThoughtProps::new(),
                );
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let now = Instant::now();
        let theme = &self.theme;

        let header = chain_of_thought_header_default(
            self.chain_state.open,
            ChainOfThoughtHeaderProps::new().label_override(if self.chain_state.open {
                "Reasoning in progress..."
            } else {
                "Chain of Thought"
            }),
            theme,
        );

        let step1 = chain_of_thought_step(
            ChainOfThoughtStepProps::new("Searching for relevant profiles")
                .icon(LucideIcon::Search)
                .status(ChainOfThoughtStepStatus::Complete),
            Some(chain_of_thought_search_results(
                vec![
                    chain_of_thought_search_result(
                        "x.com",
                        ChainOfThoughtSearchResultProps::new(),
                        theme,
                    ),
                    chain_of_thought_search_result(
                        "github.com",
                        ChainOfThoughtSearchResultProps::new(),
                        theme,
                    ),
                    chain_of_thought_search_result(
                        "dribbble.com",
                        ChainOfThoughtSearchResultProps::new(),
                        theme,
                    ),
                ],
                ChainOfThoughtSearchResultsProps::new(),
            )),
            chain_of_thought_step_is_visible(&self.chain_state, now, 0, None),
            false,
            theme,
        );

        let image_card = container(
            text("Image preview")
                .size(12)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb8(140, 140, 140)),
                }),
        )
        .width(Length::Fixed(220.0))
        .height(Length::Fixed(120.0))
        .center_x(Length::Fixed(220.0))
        .center_y(Length::Fixed(120.0));

        let step2 = chain_of_thought_step(
            ChainOfThoughtStepProps::new("Found a candidate profile image")
                .icon(LucideIcon::Image)
                .status(ChainOfThoughtStepStatus::Active),
            Some(iced_shadcn::chain_of_thought_image(
                image_card,
                iced_shadcn::ChainOfThoughtImageProps::new()
                    .caption("Profile photo candidate from x.com"),
                theme,
            )),
            chain_of_thought_step_is_visible(&self.chain_state, now, 1, None),
            false,
            theme,
        );

        let step3 = chain_of_thought_step(
            ChainOfThoughtStepProps::new(
                "Hayden is an Australian product designer and software engineer.",
            )
            .status(ChainOfThoughtStepStatus::Pending),
            None::<Element<'_, Message>>,
            chain_of_thought_step_is_visible(&self.chain_state, now, 2, None),
            true,
            theme,
        );

        let content = column![step1, step2, step3].spacing(12);

        let chain = chain_of_thought(
            self.chain_state.open,
            header,
            content,
            Some(|_| Message::ToggleOpen),
            ChainOfThoughtProps::new(),
            ChainOfThoughtContentProps::new(),
            theme,
        );

        let hint = badge(
            "Click header to toggle",
            iced_shadcn::BadgeProps::new().variant(iced_shadcn::BadgeVariant::Secondary),
            theme,
        );

        let controls = row![
            button_content(
                text("Start"),
                Some(Message::Start),
                ButtonProps::new()
                    .size(ButtonSize::Size1)
                    .variant(ButtonVariant::Default),
                theme
            ),
            button_content(
                text("Stop"),
                Some(Message::Stop),
                ButtonProps::new()
                    .size(ButtonSize::Size1)
                    .variant(ButtonVariant::Outline),
                theme
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(
            column![row![hint].align_y(Alignment::Center), controls, chain]
                .spacing(16)
                .width(Length::Fill),
        )
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(theme.palette.background)),
            text_color: Some(theme.palette.foreground),
            ..iced::widget::container::Style::default()
        })
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .subscription(Example::subscription)
        .run()
}
