use iced::border::Border;
use iced::widget::{column, container};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{SelectProps, SelectSize, Theme, select};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Blueberry,
    Grapes,
    Pineapple,
}

impl Fruit {
    const ALL: [Fruit; 5] = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Blueberry,
        Fruit::Grapes,
        Fruit::Pineapple,
    ];
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Apple => write!(f, "Apple"),
            Fruit::Banana => write!(f, "Banana"),
            Fruit::Blueberry => write!(f, "Blueberry"),
            Fruit::Grapes => write!(f, "Grapes"),
            Fruit::Pineapple => write!(f, "Pineapple"),
        }
    }
}

const TIMEZONES: [&str; 27] = [
    "Eastern Standard Time (EST)",
    "Central Standard Time (CST)",
    "Mountain Standard Time (MST)",
    "Pacific Standard Time (PST)",
    "Alaska Standard Time (AKST)",
    "Hawaii Standard Time (HST)",
    "Greenwich Mean Time (GMT)",
    "Central European Time (CET)",
    "Eastern European Time (EET)",
    "Western European Summer Time (WEST)",
    "Central Africa Time (CAT)",
    "East Africa Time (EAT)",
    "Moscow Time (MSK)",
    "India Standard Time (IST)",
    "China Standard Time (CST)",
    "Japan Standard Time (JST)",
    "Korea Standard Time (KST)",
    "Indonesia Central Standard Time (WITA)",
    "Australian Western Standard Time (AWST)",
    "Australian Central Standard Time (ACST)",
    "Australian Eastern Standard Time (AEST)",
    "New Zealand Standard Time (NZST)",
    "Fiji Time (FJT)",
    "Argentina Time (ART)",
    "Bolivia Time (BOT)",
    "Brasilia Time (BRT)",
    "Chile Standard Time (CLT)",
];

#[derive(Default)]
struct Example {
    theme: Theme,
    selected_fruit: Option<Fruit>,
    selected_timezone: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    SelectedFruit(Fruit),
    SelectedTimezone(&'static str),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SelectedFruit(fruit) => self.selected_fruit = Some(fruit),
            Message::SelectedTimezone(timezone) => self.selected_timezone = Some(timezone),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let content = column![
            select(
                &Fruit::ALL,
                self.selected_fruit,
                "Select a fruit",
                Message::SelectedFruit,
                SelectProps::new().size(SelectSize::Two),
                theme,
            )
            .width(Length::Fixed(180.0)),
            select(
                &TIMEZONES,
                self.selected_timezone,
                "Select a timezone",
                Message::SelectedTimezone,
                SelectProps::new().size(SelectSize::Two),
                theme,
            )
            .menu_height(Length::Fixed(200.0))
            .width(Length::Fixed(280.0)),
        ]
        .spacing(24)
        .align_x(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .padding(32)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: border,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}
