#[cfg(feature = "charts")]
use iced::border::Border;
#[cfg(feature = "charts")]
use iced::widget::{column, container, text as iced_text};
#[cfg(feature = "charts")]
use iced::{Background, Element, Length};
#[cfg(feature = "charts")]
use iced_shadcn::{BarChart, ChartProps, Theme, chart};

#[cfg(feature = "charts")]
pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[cfg(not(feature = "charts"))]
pub fn main() -> iced::Result {
    Ok(())
}

#[cfg(feature = "charts")]
#[derive(Default)]
struct Example {
    theme: Theme,
}

#[cfg(feature = "charts")]
impl Example {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let theme = &self.theme;
        let data_a = vec![(0.0, 6.0), (1.0, 5.0), (2.0, 8.0), (3.0, 3.0), (4.0, 7.0)];
        let data_b = vec![(0.0, 4.0), (1.0, 7.0), (2.0, 6.0), (3.0, 5.0), (4.0, 4.0)];

        let props = ChartProps::new().title("Legend demo").show_legend(true);
        let content = column![
            iced_text("Chart bar demo legend").size(20),
            chart(props, theme, |plot| {
                BarChart::new(data_a)
                    .label("Series A")
                    .color(theme.palette.primary)
                    .show(plot);
                BarChart::new(data_b)
                    .label("Series B")
                    .color(theme.palette.accent)
                    .show(plot);
            })
        ]
        .spacing(16);

        app(theme, preview(theme, content).into())
    }
}

#[cfg(feature = "charts")]
fn app<'a>(theme: &Theme, content: Element<'a, ()>) -> Element<'a, ()> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

#[cfg(feature = "charts")]
fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, ()>>,
) -> iced::widget::Container<'a, ()> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(24)
        .width(Length::Shrink)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
