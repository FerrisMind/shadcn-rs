use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, ContentVariant, SelectEntry,
    SelectGroup, SelectItem, SelectProps, SelectSize, Theme, TriggerVariant, button, label, select,
    select_entries,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced_shadcn::profiling::init_runtime();

    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

const FRUITS: [&str; 5] = ["Apple", "Banana", "Blueberry", "Grapes", "Pineapple"];
const EMAILS: [&str; 5] = [
    "m@example.com",
    "m@google.com",
    "m@support.com",
    "m@icloud.com",
    "m@radix-ui.com",
];
const VARIANT_OPTIONS: [&str; 4] = ["Classic", "Surface", "Soft", "Ghost"];
const SIZE_OPTIONS: [&str; 5] = ["Small", "Medium", "Large", "XL", "2XL"];
const COLOR_OPTIONS: [&str; 4] = ["Overview", "Analytics", "Reports", "Settings"];
const HIGH_CONTRAST_OPTIONS: [&str; 4] = ["Basic", "Pro", "Team", "Enterprise"];
const RADIUS_OPTIONS: [&str; 4] = ["North", "South", "East", "West"];
const CONTENT_OPTIONS: [&str; 4] = ["Low", "Medium", "High", "Critical"];
const STATUS_OPTIONS: [&str; 3] = ["Pending", "Active", "Archived"];
const DISABLED_OPTIONS: [&str; 4] = ["Account", "Billing", "Team", "Support"];

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

struct Example {
    theme: Theme,
    group_entries: Vec<SelectEntry<&'static str>>,
    selected_demo: Option<&'static str>,
    selected_fruit: Option<Fruit>,
    selected_timezone: Option<&'static str>,
    selected_email: Option<&'static str>,
    selected_group: Option<&'static str>,
    selected_disabled: Option<&'static str>,
    selected_variant: Option<&'static str>,
    selected_size: Option<&'static str>,
    selected_color: Option<&'static str>,
    selected_high_contrast: Option<&'static str>,
    selected_radius: Option<&'static str>,
    selected_content_variant: Option<&'static str>,
    selected_status: Option<&'static str>,
    selected_button_group: Option<&'static str>,
    selected_content_color: Option<&'static str>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            group_entries: build_entries(),
            selected_demo: Some(FRUITS[1]),
            selected_fruit: Some(Fruit::Banana),
            selected_timezone: Some(TIMEZONES[0]),
            selected_email: Some(EMAILS[0]),
            selected_group: Some("orange"),
            selected_disabled: None,
            selected_variant: Some(VARIANT_OPTIONS[0]),
            selected_size: Some(SIZE_OPTIONS[1]),
            selected_color: Some(COLOR_OPTIONS[1]),
            selected_high_contrast: Some(HIGH_CONTRAST_OPTIONS[1]),
            selected_radius: Some(RADIUS_OPTIONS[0]),
            selected_content_variant: Some(CONTENT_OPTIONS[1]),
            selected_status: Some(STATUS_OPTIONS[1]),
            selected_button_group: Some(STATUS_OPTIONS[1]),
            selected_content_color: Some(COLOR_OPTIONS[2]),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SelectedDemo(&'static str),
    SelectedFruit(Fruit),
    SelectedTimezone(&'static str),
    SelectedEmail(&'static str),
    SelectedGroup(&'static str),
    SelectedDisabled(&'static str),
    SelectedVariant(&'static str),
    SelectedSize(&'static str),
    SelectedColor(&'static str),
    SelectedHighContrast(&'static str),
    SelectedRadius(&'static str),
    SelectedContentVariant(&'static str),
    SelectedStatus(&'static str),
    SelectedButtonGroup(&'static str),
    SelectedContentColor(&'static str),
    Submit,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SelectedDemo(value) => self.selected_demo = Some(value),
            Message::SelectedFruit(fruit) => self.selected_fruit = Some(fruit),
            Message::SelectedTimezone(timezone) => self.selected_timezone = Some(timezone),
            Message::SelectedEmail(value) => self.selected_email = Some(value),
            Message::SelectedGroup(value) => self.selected_group = Some(value),
            Message::SelectedDisabled(value) => self.selected_disabled = Some(value),
            Message::SelectedVariant(value) => self.selected_variant = Some(value),
            Message::SelectedSize(value) => self.selected_size = Some(value),
            Message::SelectedColor(value) => self.selected_color = Some(value),
            Message::SelectedHighContrast(value) => self.selected_high_contrast = Some(value),
            Message::SelectedRadius(value) => self.selected_radius = Some(value),
            Message::SelectedContentVariant(value) => self.selected_content_variant = Some(value),
            Message::SelectedStatus(value) => self.selected_status = Some(value),
            Message::SelectedButtonGroup(value) => self.selected_button_group = Some(value),
            Message::SelectedContentColor(value) => self.selected_content_color = Some(value),
            Message::Submit => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;
        let helper_color = theme.palette.muted_foreground;

        let demo_section = column![
            iced_text("Demo").size(16),
            select(
                &FRUITS,
                self.selected_demo,
                "Select a fruit",
                Message::SelectedDemo,
                SelectProps::new(),
                theme,
            )
            .width(Length::Fixed(200.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let typed_section = column![
            iced_text("Typed options").size(16),
            select(
                &Fruit::ALL,
                self.selected_fruit,
                "Select a fruit",
                Message::SelectedFruit,
                SelectProps::new().size(SelectSize::Size2),
                theme,
            )
            .width(Length::Fixed(180.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let scrollable_section = column![
            iced_text("Scrollable content").size(16),
            select(
                &TIMEZONES,
                self.selected_timezone,
                "Select a timezone",
                Message::SelectedTimezone,
                SelectProps::new(),
                theme,
            )
            .menu_height(Length::Fixed(200.0))
            .width(Length::Fixed(280.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let form_section = column![
            iced_text("Form").size(16),
            column![
                label("Email", theme),
                select(
                    &EMAILS,
                    self.selected_email,
                    "Select an email",
                    Message::SelectedEmail,
                    SelectProps::new(),
                    theme,
                )
                .width(Length::Fixed(260.0)),
                iced_text("We will only use this for account notifications.")
                    .size(12)
                    .style(move |_| iced::widget::text::Style {
                        color: Some(helper_color),
                    }),
            ]
            .spacing(8),
            row![button(
                "Submit",
                Some(Message::Submit),
                ButtonProps::new().size(ButtonSize::Size2),
                theme,
            )]
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let groups_section = column![
            iced_text("Groups").size(16),
            select_entries(
                &self.group_entries,
                self.selected_group,
                "Select an item",
                Message::SelectedGroup,
                SelectProps::new(),
                theme,
            )
            .width(Length::Fixed(220.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let disabled_section = column![
            iced_text("Disabled").size(16),
            select(
                &DISABLED_OPTIONS,
                self.selected_disabled,
                "Select a section",
                Message::SelectedDisabled,
                SelectProps::new().disabled(true),
                theme,
            )
            .width(Length::Fixed(200.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let variants_section = column![
            iced_text("Trigger variants").size(16),
            column![
                select(
                    &VARIANT_OPTIONS,
                    self.selected_variant,
                    "Classic",
                    Message::SelectedVariant,
                    SelectProps::new().variant(TriggerVariant::Classic),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &VARIANT_OPTIONS,
                    self.selected_variant,
                    "Surface",
                    Message::SelectedVariant,
                    SelectProps::new().variant(TriggerVariant::Surface),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &VARIANT_OPTIONS,
                    self.selected_variant,
                    "Soft",
                    Message::SelectedVariant,
                    SelectProps::new().variant(TriggerVariant::Soft),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &VARIANT_OPTIONS,
                    self.selected_variant,
                    "Ghost",
                    Message::SelectedVariant,
                    SelectProps::new().variant(TriggerVariant::Ghost),
                    theme,
                )
                .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_x(Alignment::Start),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let sizes_section = column![
            iced_text("Sizes").size(16),
            column![
                select(
                    &SIZE_OPTIONS,
                    self.selected_size,
                    "Size 1",
                    Message::SelectedSize,
                    SelectProps::new().size(SelectSize::Size1),
                    theme,
                )
                .width(Length::Fixed(200.0)),
                select(
                    &SIZE_OPTIONS,
                    self.selected_size,
                    "Size 2",
                    Message::SelectedSize,
                    SelectProps::new().size(SelectSize::Size2),
                    theme,
                )
                .width(Length::Fixed(200.0)),
                select(
                    &SIZE_OPTIONS,
                    self.selected_size,
                    "Size 3",
                    Message::SelectedSize,
                    SelectProps::new().size(SelectSize::Size3),
                    theme,
                )
                .width(Length::Fixed(200.0)),
            ]
            .spacing(12)
            .align_x(Alignment::Start),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let colors_section = column![
            iced_text("Accent colors").size(16),
            column![
                select(
                    &COLOR_OPTIONS,
                    self.selected_color,
                    "Gray",
                    Message::SelectedColor,
                    SelectProps::new()
                        .variant(TriggerVariant::Soft)
                        .color(AccentColor::Gray),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &COLOR_OPTIONS,
                    self.selected_color,
                    "Blue",
                    Message::SelectedColor,
                    SelectProps::new()
                        .variant(TriggerVariant::Soft)
                        .color(AccentColor::Blue),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &COLOR_OPTIONS,
                    self.selected_color,
                    "Green",
                    Message::SelectedColor,
                    SelectProps::new()
                        .variant(TriggerVariant::Soft)
                        .color(AccentColor::Green),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &COLOR_OPTIONS,
                    self.selected_color,
                    "Orange",
                    Message::SelectedColor,
                    SelectProps::new()
                        .variant(TriggerVariant::Soft)
                        .color(AccentColor::Orange),
                    theme,
                )
                .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_x(Alignment::Start),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let high_contrast_section = column![
            iced_text("High contrast").size(16),
            select(
                &HIGH_CONTRAST_OPTIONS,
                self.selected_high_contrast,
                "Select a plan",
                Message::SelectedHighContrast,
                SelectProps::new()
                    .variant(TriggerVariant::Soft)
                    .color(AccentColor::Indigo)
                    .high_contrast(true),
                theme,
            )
            .width(Length::Fixed(240.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let radius_section = column![
            iced_text("Radius").size(16),
            column![
                select(
                    &RADIUS_OPTIONS,
                    self.selected_radius,
                    "Radius none",
                    Message::SelectedRadius,
                    SelectProps::new().radius(ButtonRadius::None),
                    theme,
                )
                .width(Length::Fixed(200.0)),
                select(
                    &RADIUS_OPTIONS,
                    self.selected_radius,
                    "Radius small",
                    Message::SelectedRadius,
                    SelectProps::new().radius(ButtonRadius::Small),
                    theme,
                )
                .width(Length::Fixed(200.0)),
                select(
                    &RADIUS_OPTIONS,
                    self.selected_radius,
                    "Radius large",
                    Message::SelectedRadius,
                    SelectProps::new().radius(ButtonRadius::Large),
                    theme,
                )
                .width(Length::Fixed(200.0)),
                select(
                    &RADIUS_OPTIONS,
                    self.selected_radius,
                    "Radius full",
                    Message::SelectedRadius,
                    SelectProps::new().radius(ButtonRadius::Full),
                    theme,
                )
                .width(Length::Fixed(200.0)),
            ]
            .spacing(12)
            .align_x(Alignment::Start),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content_variants_section = column![
            iced_text("Content variants").size(16),
            column![
                select(
                    &CONTENT_OPTIONS,
                    self.selected_content_variant,
                    "Solid content",
                    Message::SelectedContentVariant,
                    SelectProps::new().content_variant(ContentVariant::Solid),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &CONTENT_OPTIONS,
                    self.selected_content_variant,
                    "Soft content",
                    Message::SelectedContentVariant,
                    SelectProps::new().content_variant(ContentVariant::Soft),
                    theme,
                )
                .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_x(Alignment::Start),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content_color_section = column![
            iced_text("Content color override").size(16),
            select(
                &COLOR_OPTIONS,
                self.selected_content_color,
                "Trigger: Blue / Content: Orange",
                Message::SelectedContentColor,
                SelectProps::new()
                    .variant(TriggerVariant::Soft)
                    .color(AccentColor::Blue)
                    .content_color(AccentColor::Orange),
                theme,
            )
            .width(Length::Fixed(260.0)),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let placeholder_section = column![
            iced_text("Placeholder").size(16),
            column![
                select(
                    &STATUS_OPTIONS,
                    self.selected_status,
                    "Choose a status",
                    Message::SelectedStatus,
                    SelectProps::new(),
                    theme,
                )
                .width(Length::Fixed(220.0)),
                select(
                    &STATUS_OPTIONS,
                    None,
                    "No selection",
                    Message::SelectedStatus,
                    SelectProps::new(),
                    theme,
                )
                .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_x(Alignment::Start),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let button_group_section = column![
            iced_text("Button group").size(16),
            row![
                button(
                    "Previous",
                    Some(Message::Submit),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
                select(
                    &STATUS_OPTIONS,
                    self.selected_button_group,
                    "Status",
                    Message::SelectedButtonGroup,
                    SelectProps::new().size(SelectSize::Size2),
                    theme,
                )
                .width(Length::Fixed(160.0)),
                button(
                    "Next",
                    Some(Message::Submit),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content = column![
            demo_section,
            typed_section,
            scrollable_section,
            form_section,
            groups_section,
            disabled_section,
            variants_section,
            sizes_section,
            colors_section,
            high_contrast_section,
            radius_section,
            content_variants_section,
            content_color_section,
            placeholder_section,
            button_group_section,
        ]
        .spacing(24)
        .align_x(Alignment::Start);

        let content = scrollable(content).width(Length::Fill).height(Length::Fill);

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

fn build_entries() -> Vec<SelectEntry<&'static str>> {
    vec![
        SelectEntry::Group(SelectGroup::new(
            "Fruits",
            vec![
                SelectItem::new("orange", "Orange"),
                SelectItem::new("apple", "Apple"),
                SelectItem::new("grapes", "Grapes").disabled(true),
            ],
        )),
        SelectEntry::separator(),
        SelectEntry::Group(SelectGroup::new(
            "Vegetables",
            vec![
                SelectItem::new("carrot", "Carrot"),
                SelectItem::new("potato", "Potato"),
            ],
        )),
    ]
}
