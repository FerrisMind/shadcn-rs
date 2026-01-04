use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Size, Task};

use iced_shadcn::{
    CarouselContentProps, CarouselOptions, CarouselOrientation, CarouselState, Theme,
    carousel_content, carousel_next, carousel_previous,
};

const SLIDES: [&str; 4] = ["Dashboard", "Templates", "Insights", "Settings"];

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    carousel: CarouselState,
}

#[derive(Debug, Clone)]
enum Message {
    Prev,
    Next,
    Scrolled(scrollable::Viewport),
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        let props = carousel_props();
        let count = SLIDES.len();
        let opts = CarouselOptions::default();

        match message {
            Message::Prev => {
                self.carousel
                    .scroll_prev(count, opts, props, CarouselOrientation::Horizontal)
            }
            Message::Next => {
                self.carousel
                    .scroll_next(count, opts, props, CarouselOrientation::Horizontal)
            }
            Message::Scrolled(viewport) => {
                self.carousel.sync_from_viewport(
                    viewport,
                    props,
                    CarouselOrientation::Horizontal,
                    count,
                );
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let props = carousel_props();
        let count = SLIDES.len();
        let opts = CarouselOptions::default();

        let slides = SLIDES
            .iter()
            .map(|label| slide(label, &self.theme))
            .collect::<Vec<_>>();

        let content = carousel_content(
            &self.carousel,
            CarouselOrientation::Horizontal,
            props,
            slides,
        )
        .on_scroll(Message::Scrolled);

        let controls = row![
            carousel_previous(
                Some(Message::Prev),
                self.carousel.can_scroll_prev(count, opts),
                CarouselOrientation::Horizontal,
                &self.theme
            ),
            content,
            carousel_next(
                Some(Message::Next),
                self.carousel.can_scroll_next(count, opts),
                CarouselOrientation::Horizontal,
                &self.theme
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let status = format!("Slide {} of {}", self.carousel.index + 1, count.max(1));

        let content = column![text("Carousel").size(16), controls, text(status).size(12),]
            .spacing(12)
            .width(Length::Fill);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

fn carousel_props() -> CarouselContentProps {
    CarouselContentProps::new()
        .size(Size::new(360.0, 160.0))
        .item_extent(360.0)
}

fn slide<'a, Message: 'a>(label: &'a str, theme: &Theme) -> Element<'a, Message> {
    let palette = theme.palette;
    let radius = theme.radius.md;

    container(text(label).size(16))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| container::Style {
            text_color: Some(palette.foreground),
            background: Some(Background::Color(palette.muted)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: palette.border,
            },
            ..container::Style::default()
        })
        .into()
}
