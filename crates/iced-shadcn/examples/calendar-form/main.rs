#[cfg(feature = "date-components")]
use chrono::NaiveDate;
#[cfg(feature = "date-components")]
use iced::border::Border;
#[cfg(feature = "date-components")]
use iced::widget::{column, container, text as iced_text};
#[cfg(feature = "date-components")]
use iced::{Background, Element, Length};
#[cfg(feature = "date-components")]
use iced_shadcn::{
    CalendarAction, CalendarCaptionLayout, CalendarMode, CalendarProps, CalendarState, Theme,
    calendar,
};

#[cfg(feature = "date-components")]
pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[cfg(not(feature = "date-components"))]
pub fn main() -> iced::Result {
    Ok(())
}

#[cfg(feature = "date-components")]
struct Example {
    theme: Theme,
    selected: Option<NaiveDate>,
    calendar_state: CalendarState,
}

#[cfg(feature = "date-components")]
impl Default for Example {
    fn default() -> Self {
        let initial = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        Self {
            theme: Theme::default(),
            selected: Some(initial),
            calendar_state: CalendarState::new(initial),
        }
    }
}

#[cfg(feature = "date-components")]
impl Example {
    fn update(&mut self, message: CalendarAction) {
        match message {
            CalendarAction::MonthChanged(month) => {
                self.calendar_state.current_month = month;
            }
            CalendarAction::Selected(date) => {
                self.selected = date;
            }
            CalendarAction::RangeSelected(_, _) => {}
        }
    }

    fn view(&self) -> Element<'_, CalendarAction> {
        let theme = &self.theme;
        let selected_text = self
            .selected
            .map(|date| date.format("%b %d, %Y").to_string())
            .unwrap_or_else(|| "None".to_string());

        let content = column![
            iced_text("Calendar form").size(20),
            iced_text(format!("Selected: {selected_text}")).size(13),
            calendar(
                CalendarProps::new("calendar-form")
                    .mode(CalendarMode::Single)
                    .caption_layout(CalendarCaptionLayout::Dropdown)
                    .number_of_months(2)
                    .selected(self.selected),
                self.calendar_state,
                Some(|action| action),
                theme
            )
        ]
        .spacing(16);

        app(theme, preview(theme, content).into())
    }
}

#[cfg(feature = "date-components")]
fn app<'a>(theme: &Theme, content: Element<'a, CalendarAction>) -> Element<'a, CalendarAction> {
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

#[cfg(feature = "date-components")]
fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, CalendarAction>>,
) -> iced::widget::Container<'a, CalendarAction> {
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
