#[cfg(feature = "chrono")]
use chrono::NaiveDate;
#[cfg(feature = "chrono")]
use iced::border::Border;
#[cfg(feature = "chrono")]
use iced::widget::{column, container, text as iced_text};
#[cfg(feature = "chrono")]
use iced::{Background, Element, Length};
#[cfg(feature = "chrono")]
use iced_shadcn::{CalendarAction, CalendarState, DatePickerProps, Theme, date_picker};

#[cfg(feature = "chrono")]
pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[cfg(not(feature = "chrono"))]
pub fn main() -> iced::Result {
    Ok(())
}

#[cfg(feature = "chrono")]
struct Example {
    theme: Theme,
    selected: Option<NaiveDate>,
    calendar_state: CalendarState,
}

#[cfg(feature = "chrono")]
impl Default for Example {
    fn default() -> Self {
        let initial = NaiveDate::from_ymd_opt(2024, 6, 12).unwrap();
        Self {
            theme: Theme::default(),
            selected: None,
            calendar_state: CalendarState::new(initial),
        }
    }
}

#[cfg(feature = "chrono")]
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
        let content = column![
            iced_text("Date picker demo").size(20),
            date_picker(
                DatePickerProps::new("date-picker-demo", &self.selected),
                self.calendar_state,
                Some(|action| action),
                theme
            )
        ]
        .spacing(16);

        app(theme, preview(theme, content).into())
    }
}

#[cfg(feature = "chrono")]
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

#[cfg(feature = "chrono")]
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
