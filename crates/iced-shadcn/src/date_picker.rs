use std::hash::Hash;

use chrono::{Datelike, NaiveDate};
use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::calendar::{
    CalendarAction, CalendarCaptionLayout, CalendarMode, CalendarProps, CalendarState, calendar,
};
use crate::combobox::ButtonJustify;
use crate::popover::{PopoverProps, PopoverSize, popover};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DatePickerIconPosition {
    #[default]
    Leading,
    Trailing,
    None,
}

pub struct DatePickerProps<'a, Id> {
    pub id_source: Id,
    pub value: &'a Option<NaiveDate>,
    pub placeholder: &'a str,
    pub disabled: bool,
    pub icon_position: DatePickerIconPosition,
    pub justify: ButtonJustify,
    pub size: ButtonSize,
    pub variant: ButtonVariant,
    pub trigger_width: f32,
    pub caption_layout: CalendarCaptionLayout,
    pub min_date: Option<NaiveDate>,
    pub max_date: Option<NaiveDate>,
}

impl<'a, Id: Hash> DatePickerProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a Option<NaiveDate>) -> Self {
        Self {
            id_source,
            value,
            placeholder: "Pick a date",
            disabled: false,
            icon_position: DatePickerIconPosition::Leading,
            justify: ButtonJustify::Start,
            size: ButtonSize::Size2,
            variant: ButtonVariant::Outline,
            trigger_width: 240.0,
            caption_layout: CalendarCaptionLayout::Label,
            min_date: None,
            max_date: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon_position(mut self, position: DatePickerIconPosition) -> Self {
        self.icon_position = position;
        self
    }

    pub fn justify(mut self, justify: ButtonJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn trigger_width(mut self, width: f32) -> Self {
        self.trigger_width = width;
        self
    }

    pub fn caption_layout(mut self, layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = layout;
        self
    }

    pub fn min_date(mut self, min_date: Option<NaiveDate>) -> Self {
        self.min_date = min_date;
        self
    }

    pub fn max_date(mut self, max_date: Option<NaiveDate>) -> Self {
        self.max_date = max_date;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

pub struct DateRangePickerProps<'a, Id> {
    pub id_source: Id,
    pub value: &'a DateRange,
    pub placeholder: &'a str,
    pub disabled: bool,
    pub number_of_months: usize,
    pub trigger_width: f32,
}

impl<'a, Id: Hash> DateRangePickerProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a DateRange) -> Self {
        Self {
            id_source,
            value,
            placeholder: "Pick a date",
            disabled: false,
            number_of_months: 2,
            trigger_width: 300.0,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn number_of_months(mut self, months: usize) -> Self {
        self.number_of_months = months.max(1);
        self
    }

    pub fn trigger_width(mut self, width: f32) -> Self {
        self.trigger_width = width;
        self
    }
}

pub fn date_picker<'a, Message: Clone + 'a, Id: Hash, F>(
    props: DatePickerProps<'a, Id>,
    calendar_state: CalendarState,
    on_action: Option<F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(CalendarAction) -> Message + 'a,
{
    let label = match props.value {
        Some(date) => text(format_ppp(*date)),
        None => text(props.placeholder).style(move |_t| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        }),
    };

    let icon = text("📅").size(12);
    let trigger_content: Element<'a, Message> = match props.icon_position {
        DatePickerIconPosition::Leading => build_trigger_content(icon, label, props.justify),
        DatePickerIconPosition::Trailing => build_trigger_content(label, icon, props.justify),
        DatePickerIconPosition::None => build_trigger_content(label, text(""), props.justify),
    };

    let trigger = button_content(
        trigger_content,
        None,
        ButtonProps::new()
            .variant(props.variant)
            .size(props.size)
            .disabled(props.disabled),
        theme,
    )
    .width(Length::Fixed(props.trigger_width));

    let calendar_props = CalendarProps::new(props.id_source)
        .selected(*props.value)
        .mode(CalendarMode::Single)
        .caption_layout(props.caption_layout)
        .min_date(props.min_date)
        .max_date(props.max_date);

    let calendar_element = calendar(calendar_props, calendar_state, on_action, theme);

    popover(
        container(trigger).width(Length::Fixed(props.trigger_width)),
        calendar_element,
        PopoverProps::new().size(PopoverSize::Size2).offset(6.0),
        theme,
    )
    .into()
}

pub fn date_range_picker<'a, Message: Clone + 'a, Id: Hash, F>(
    props: DateRangePickerProps<'a, Id>,
    calendar_state: CalendarState,
    on_action: Option<F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(CalendarAction) -> Message + 'a,
{
    let label = match (props.value.from, props.value.to) {
        (Some(from), Some(to)) => text(format!(
            "{} - {}",
            format_mmm_dd_y(from),
            format_mmm_dd_y(to)
        )),
        (Some(from), None) => text(format!("{} -", format_mmm_dd_y(from))),
        _ => text(props.placeholder).style(move |_t| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        }),
    };

    let trigger = button_content(
        label,
        None,
        ButtonProps::new()
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Size2)
            .disabled(props.disabled),
        theme,
    )
    .width(Length::Fixed(props.trigger_width));

    let calendar_props = CalendarProps::new(props.id_source)
        .mode(CalendarMode::Range)
        .number_of_months(props.number_of_months)
        .range_start(props.value.from)
        .range_end(props.value.to);

    let calendar_element = calendar(calendar_props, calendar_state, on_action, theme);

    popover(
        container(trigger).width(Length::Fixed(props.trigger_width)),
        calendar_element,
        PopoverProps::new().size(PopoverSize::Size2).offset(6.0),
        theme,
    )
    .into()
}

fn build_trigger_content<'a, Message: Clone + 'a>(
    left: impl Into<Element<'a, Message>>,
    right: impl Into<Element<'a, Message>>,
    justify: ButtonJustify,
) -> Element<'a, Message> {
    let content = match justify {
        ButtonJustify::Between => row![
            left.into(),
            iced::widget::space().width(Length::Fill),
            right.into()
        ]
        .align_y(Alignment::Center),
        ButtonJustify::Center => row![left.into(), right.into()]
            .spacing(6)
            .align_y(Alignment::Center),
        ButtonJustify::Start => row![left.into(), right.into()]
            .spacing(6)
            .align_y(Alignment::Center),
    };
    content.into()
}

fn ordinal_suffix(day: u32) -> &'static str {
    let rem_100 = day % 100;
    if (11..=13).contains(&rem_100) {
        return "th";
    }
    match day % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

fn format_ppp(date: NaiveDate) -> String {
    let day = date.day();
    format!(
        "{} {}{}, {}",
        date.format("%b"),
        day,
        ordinal_suffix(day),
        date.year()
    )
}

fn format_mmm_dd_y(date: NaiveDate) -> String {
    format!("{} {:02}, {}", date.format("%b"), date.day(), date.year())
}
