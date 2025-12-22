use crate::button::{Button, ButtonJustify, ButtonSize, ButtonVariant};
use crate::calendar::{CalendarCaptionLayout, CalendarMode, CalendarProps, calendar_with_props};
use crate::icons::icon_calendar;
use crate::popover::{PopoverAlign, PopoverProps, PopoverSide, popover};
use crate::theme::Theme;
use chrono::{Datelike, NaiveDate};
use egui::{Margin, Response, RichText, Ui};
use log::trace;
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DatePickerIconPosition {
    #[default]
    Leading,
    Trailing,
    None,
}

pub struct DatePickerProps<'a, Id> {
    pub id_source: Id,
    pub value: &'a mut Option<NaiveDate>,
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
    pub close_on_select: bool,
    pub on_value_change: Option<Box<dyn FnMut(Option<NaiveDate>) + 'a>>,
}

impl<'a, Id: Hash + Debug> DatePickerProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a mut Option<NaiveDate>) -> Self {
        Self {
            id_source,
            value,
            placeholder: "Pick a date",
            disabled: false,
            icon_position: DatePickerIconPosition::Leading,
            justify: ButtonJustify::Start,
            size: ButtonSize::Default,
            variant: ButtonVariant::Outline,
            trigger_width: 240.0,
            caption_layout: CalendarCaptionLayout::Label,
            min_date: None,
            max_date: None,
            close_on_select: false,
            on_value_change: None,
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

    pub fn close_on_select(mut self, close_on_select: bool) -> Self {
        self.close_on_select = close_on_select;
        self
    }

    pub fn on_value_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<NaiveDate>) + 'a,
    {
        self.on_value_change = Some(Box::new(callback));
        self
    }
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

pub fn date_picker_with_props<'a, Id>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: DatePickerProps<'a, Id>,
) -> Response
where
    Id: Hash + Debug,
{
    trace!("Rendering date picker value={:?}", props.value);

    let id = ui.make_persistent_id(&props.id_source);
    let open_id = id.with("open");
    let mut open_state = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<bool>(open_id).unwrap_or(false));
    let selection_storage = Rc::new(RefCell::new(None));

    let label_widget: egui::WidgetText = if let Some(date) = *props.value {
        format_ppp(date).into()
    } else {
        RichText::new(props.placeholder)
            .color(theme.palette.muted_foreground)
            .into()
    };

    let trailing_calendar_icon =
        |p: &egui::Painter, center: egui::Pos2, size: f32, color: egui::Color32| {
            let muted = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 128);
            icon_calendar(p, center, size, muted);
        };

    let (trigger_resp, calendar_result) = popover(
        ui,
        theme,
        PopoverProps::new(id.with("popover"), &mut open_state)
            .side(PopoverSide::Bottom)
            .align(PopoverAlign::Start)
            .with_width(280.0)
            .with_max_height(360.0)
            .with_content_padding(Margin::same(0))
            .with_animation(true),
        |ui| {
            let mut button = Button::new(label_widget)
                .variant(props.variant)
                .size(props.size)
                .justify(props.justify)
                .min_width(props.trigger_width)
                .enabled(!props.disabled);

            button = match props.icon_position {
                DatePickerIconPosition::Leading => button.icon(&icon_calendar),
                DatePickerIconPosition::Trailing => button.trailing_icon(&trailing_calendar_icon),
                DatePickerIconPosition::None => button,
            };

            button.show(ui, theme)
        },
        |ui| {
            selection_storage.borrow_mut().take();
            let callback_storage = selection_storage.clone();
            calendar_with_props(
                ui,
                theme,
                CalendarProps::new(id.with("calendar"))
                    .selected(*props.value)
                    .mode(CalendarMode::Single)
                    .caption_layout(props.caption_layout)
                    .min_date(props.min_date)
                    .max_date(props.max_date)
                    .on_select(move |date| {
                        *callback_storage.borrow_mut() = date;
                    }),
            );
            *selection_storage.borrow()
        },
    );

    ui.memory_mut(|m| m.data.insert_persisted(open_id, open_state));

    if let Some(new_date) = calendar_result {
        if new_date != *props.value {
            *props.value = new_date;
            if let Some(ref mut cb) = props.on_value_change {
                cb(new_date);
            }
        }
        if props.close_on_select {
            ui.memory_mut(|m| m.data.insert_persisted(open_id, false));
        }
    }

    trigger_resp
}

pub fn date_picker<Id>(
    ui: &mut Ui,
    theme: &Theme,
    id_source: Id,
    value: &mut Option<NaiveDate>,
) -> Response
where
    Id: Hash + Debug,
{
    date_picker_with_props(ui, theme, DatePickerProps::new(id_source, value))
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

pub struct DateRangePickerProps<'a, Id> {
    pub id_source: Id,
    pub value: &'a mut DateRange,
    pub placeholder: &'a str,
    pub disabled: bool,
    pub number_of_months: usize,
    pub trigger_width: f32,
    pub on_value_change: Option<Box<dyn FnMut(DateRange) + 'a>>,
}

impl<'a, Id: Hash + Debug> DateRangePickerProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a mut DateRange) -> Self {
        Self {
            id_source,
            value,
            placeholder: "Pick a date",
            disabled: false,
            number_of_months: 2,
            trigger_width: 300.0,
            on_value_change: None,
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

    pub fn on_value_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(DateRange) + 'a,
    {
        self.on_value_change = Some(Box::new(callback));
        self
    }
}

pub fn date_range_picker_with_props<'a, Id>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: DateRangePickerProps<'a, Id>,
) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering date range picker from={:?} to={:?}",
        props.value.from, props.value.to
    );

    let id = ui.make_persistent_id(&props.id_source);
    let open_id = id.with("open");
    let mut open_state = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<bool>(open_id).unwrap_or(false));

    let label_text = match (props.value.from, props.value.to) {
        (Some(from), Some(to)) => format!("{} - {}", format_mmm_dd_y(from), format_mmm_dd_y(to)),
        (Some(from), None) => format_mmm_dd_y(from),
        _ => props.placeholder.to_string(),
    };

    let label_widget: egui::WidgetText = if props.value.from.is_some() {
        label_text.into()
    } else {
        RichText::new(label_text)
            .color(theme.palette.muted_foreground)
            .into()
    };

    let selection_storage: Rc<RefCell<(Option<NaiveDate>, Option<NaiveDate>)>> =
        Rc::new(RefCell::new((props.value.from, props.value.to)));

    let (trigger_resp, range_result) = popover(
        ui,
        theme,
        PopoverProps::new(id.with("popover"), &mut open_state)
            .side(PopoverSide::Bottom)
            .align(PopoverAlign::Start)
            .with_width(280.0 * (props.number_of_months as f32).min(2.0) + 16.0)
            .with_max_height(420.0)
            .with_content_padding(Margin::same(0))
            .with_animation(true),
        |ui| {
            Button::new(label_widget)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Default)
                .justify(ButtonJustify::Start)
                .icon(&icon_calendar)
                .min_width(props.trigger_width)
                .enabled(!props.disabled)
                .show(ui, theme)
        },
        |ui| {
            let callback_storage = selection_storage.clone();
            calendar_with_props(
                ui,
                theme,
                CalendarProps::new(id.with("calendar"))
                    .mode(CalendarMode::Range)
                    .range_start(props.value.from)
                    .range_end(props.value.to)
                    .default_month(
                        props
                            .value
                            .from
                            .unwrap_or_else(|| chrono::Local::now().date_naive()),
                    )
                    .number_of_months(props.number_of_months)
                    .on_range_select(move |start, end| {
                        *callback_storage.borrow_mut() = (start, end);
                    }),
            );

            Some(*selection_storage.borrow())
        },
    );

    ui.memory_mut(|m| m.data.insert_persisted(open_id, open_state));

    if let Some((from, to)) = range_result.flatten() {
        let next = DateRange { from, to };
        if next != *props.value {
            *props.value = next;
            if let Some(ref mut cb) = props.on_value_change {
                cb(next);
            }
        }
    }

    trigger_resp
}

pub fn date_range_picker<Id>(
    ui: &mut Ui,
    theme: &Theme,
    id_source: Id,
    value: &mut DateRange,
) -> Response
where
    Id: Hash + Debug,
{
    date_range_picker_with_props(ui, theme, DateRangePickerProps::new(id_source, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_suffix_handles_teens() {
        assert_eq!(ordinal_suffix(11), "th");
        assert_eq!(ordinal_suffix(12), "th");
        assert_eq!(ordinal_suffix(13), "th");
        assert_eq!(ordinal_suffix(21), "st");
    }

    #[test]
    fn format_ppp_matches_expected_shape() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        assert_eq!(format_ppp(date), "Jan 15th, 2024");
    }
}
