#[cfg(feature = "date-components")]
use chrono::NaiveDate;
#[cfg(feature = "date-components")]
use iced::border::Border;
#[cfg(feature = "date-components")]
use iced::widget::{column, container, row, text as iced_text};
#[cfg(feature = "date-components")]
use iced::{Background, Element, Length};
#[cfg(feature = "date-components")]
use iced_shadcn::{
    ButtonProps, ButtonVariant, CalendarAction, CalendarState, DatePickerProps, Theme, button,
    date_picker,
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
#[derive(Clone, Debug)]
enum Message {
    Calendar(CalendarAction),
    Preset(Option<NaiveDate>),
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
        let initial = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
        Self {
            theme: Theme::default(),
            selected: None,
            calendar_state: CalendarState::new(initial),
        }
    }
}

#[cfg(feature = "date-components")]
impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Calendar(action) => match action {
                CalendarAction::MonthChanged(month) => {
                    self.calendar_state.current_month = month;
                }
                CalendarAction::Selected(date) => {
                    self.selected = date;
                }
                CalendarAction::RangeSelected(_, _) => {}
            },
            Message::Preset(value) => {
                self.selected = value;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let today = NaiveDate::from_ymd_opt(2024, 5, 12).unwrap();
        let tomorrow = NaiveDate::from_ymd_opt(2024, 5, 13).unwrap();
        let next_week = NaiveDate::from_ymd_opt(2024, 5, 19).unwrap();

        let presets = row![
            button(
                "Today",
                Some(Message::Preset(Some(today))),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Tomorrow",
                Some(Message::Preset(Some(tomorrow))),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Next week",
                Some(Message::Preset(Some(next_week))),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
        ]
        .spacing(8);

        let picker = date_picker(
            DatePickerProps::new("date-picker-presets", &self.selected),
            self.calendar_state,
            Some(Message::Calendar),
            theme,
        );

        let content = column![
            iced_text("Date picker with presets").size(20),
            presets,
            picker
        ]
        .spacing(16);

        app(theme, preview(theme, content).into())
    }
}

#[cfg(feature = "date-components")]
fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
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
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
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
