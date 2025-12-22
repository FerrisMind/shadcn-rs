use crate::select::TriggerVariant;
use crate::theme::Theme;
use crate::tokens::mix;
use crate::{SelectProps, select_with_items};
use chrono::{Datelike, Months};
use egui::{
    Color32, CornerRadius, Direction, FontId, Layout, Rect, Response, Sense, Ui, UiBuilder, pos2,
    vec2,
};
use log::trace;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarMode {
    Single,
    Range,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarView {
    Month,
    Year,
    Decade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CalendarCaptionLayout {
    #[default]
    Label,

    Dropdown,
}

const MONTH_LABELS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub struct CalendarProps<Id> {
    pub id_source: Id,
    pub selected: Option<chrono::NaiveDate>,
    pub range_start: Option<chrono::NaiveDate>,
    pub range_end: Option<chrono::NaiveDate>,
    pub mode: CalendarMode,
    pub caption_layout: CalendarCaptionLayout,
    pub number_of_months: usize,
    pub default_month: Option<chrono::NaiveDate>,
    pub min_date: Option<chrono::NaiveDate>,
    pub max_date: Option<chrono::NaiveDate>,
    pub disabled_dates: Vec<chrono::NaiveDate>,
    pub on_select: Option<Box<dyn FnMut(Option<chrono::NaiveDate>)>>,
    pub on_range_select: Option<CalendarRangeSelectCallback>,
}

type CalendarRangeSelectCallback =
    Box<dyn FnMut(Option<chrono::NaiveDate>, Option<chrono::NaiveDate>)>;

impl<Id: Hash + Debug> CalendarProps<Id> {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            selected: None,
            range_start: None,
            range_end: None,
            mode: CalendarMode::Single,
            caption_layout: CalendarCaptionLayout::Label,
            number_of_months: 1,
            default_month: None,
            min_date: None,
            max_date: None,
            disabled_dates: Vec::new(),
            on_select: None,
            on_range_select: None,
        }
    }

    pub fn caption_layout(mut self, layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = layout;
        self
    }

    pub fn selected(mut self, date: Option<chrono::NaiveDate>) -> Self {
        self.selected = date;
        self
    }

    pub fn range_start(mut self, date: Option<chrono::NaiveDate>) -> Self {
        self.range_start = date;
        self
    }

    pub fn range_end(mut self, date: Option<chrono::NaiveDate>) -> Self {
        self.range_end = date;
        self
    }

    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn number_of_months(mut self, months: usize) -> Self {
        self.number_of_months = months.max(1);
        self
    }

    pub fn default_month(mut self, date: chrono::NaiveDate) -> Self {
        self.default_month = Some(date);
        self
    }

    pub fn min_date(mut self, date: Option<chrono::NaiveDate>) -> Self {
        self.min_date = date;
        self
    }

    pub fn max_date(mut self, date: Option<chrono::NaiveDate>) -> Self {
        self.max_date = date;
        self
    }

    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<chrono::NaiveDate>) + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    pub fn on_range_select<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<chrono::NaiveDate>, Option<chrono::NaiveDate>) + 'static,
    {
        self.on_range_select = Some(Box::new(callback));
        self
    }
}

fn is_date_disabled(
    date: chrono::NaiveDate,
    min_date: Option<chrono::NaiveDate>,
    max_date: Option<chrono::NaiveDate>,
    disabled_dates: &[chrono::NaiveDate],
) -> bool {
    if let Some(min) = min_date
        && date < min
    {
        return true;
    }
    if let Some(max) = max_date
        && date > max
    {
        return true;
    }
    disabled_dates.contains(&date)
}

fn toggle_single_selection(
    current: Option<chrono::NaiveDate>,
    clicked: chrono::NaiveDate,
) -> Option<chrono::NaiveDate> {
    if current == Some(clicked) {
        None
    } else {
        Some(clicked)
    }
}

pub fn calendar_with_props<Id>(ui: &mut Ui, theme: &Theme, mut props: CalendarProps<Id>) -> Response
where
    Id: Hash + Debug,
{
    trace!("Rendering calendar mode={:?}", props.mode);

    let id = ui.make_persistent_id(&props.id_source);
    let view_id = id.with("view");
    let current_month_id = id.with("current_month");

    let _view = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<CalendarView>(view_id))
        .unwrap_or(CalendarView::Month);

    let today = chrono::Local::now().date_naive();
    let initial_month = props.default_month.unwrap_or(today);
    let initial_month = initial_month.with_day(1).unwrap_or(initial_month);
    let current_month = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<chrono::NaiveDate>(current_month_id))
        .unwrap_or(initial_month);
    let mut current_month = current_month.with_day(1).unwrap_or(current_month);

    let cell_size = 32.0;
    let cell_padding = 4.0;
    let header_height = 28.0;
    let weekdays_height = 24.0;

    let month_width = 7.0 * cell_size + 6.0 * cell_padding;
    let month_gap = 16.0;
    let months_count = props.number_of_months.max(1);
    let total_width =
        month_width * months_count as f32 + month_gap * (months_count.saturating_sub(1) as f32);

    let weekday_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let min_month = props.min_date.map(|min| min.with_day(1).unwrap_or(min));
    let max_month = props.max_date.map(|max| max.with_day(1).unwrap_or(max));

    ui.vertical(|ui| {
        ui.set_width(total_width);

        let (_header_id, header_rect) = ui.allocate_space(vec2(total_width, header_height));

        let nav_vertical_padding = 1.0;
        let nav_height = header_height - nav_vertical_padding * 2.0;

        let prev_rect = Rect::from_min_size(
            pos2(
                header_rect.left() + 8.0,
                header_rect.top() + nav_vertical_padding,
            ),
            vec2(32.0, nav_height),
        );
        let next_rect = Rect::from_min_size(
            pos2(
                header_rect.right() - 40.0,
                header_rect.top() + nav_vertical_padding,
            ),
            vec2(32.0, nav_height),
        );

        let prev_id = id.with("nav_prev");
        let next_id = id.with("nav_next");

        let prev_response = ui.interact(prev_rect, prev_id, Sense::click());
        let next_response = ui.interact(next_rect, next_id, Sense::click());

        let prev_disabled = min_month
            .map(|month| current_month <= month)
            .unwrap_or(false);
        let last_visible_month = if months_count <= 1 {
            current_month
        } else {
            current_month
                .checked_add_months(Months::new((months_count - 1) as u32))
                .unwrap_or(current_month)
        };
        let next_disabled = max_month
            .map(|month| last_visible_month >= month)
            .unwrap_or(false);

        let min_year_option = props.min_date.map(|date| date.year());
        let max_year_option = props.max_date.map(|date| date.year());

        let select_height = 28.0;
        let select_bottom_margin = 2.0;
        let caption_height = select_height + select_bottom_margin;
        let caption_rect =
            if props.caption_layout == CalendarCaptionLayout::Dropdown && months_count == 1 {
                let (_caption_id, rect) = ui.allocate_space(vec2(total_width, caption_height));
                Some(rect)
            } else {
                None
            };

        if let Some(caption_rect) = caption_rect {
            let month_dropdown_id = id.with("caption_month");
            let year_dropdown_id = id.with("caption_year");
            let mut selected_month = current_month.month();
            let mut selected_year = current_month.year();
            let min_date = props.min_date;
            let max_date = props.max_date;

            let caption_left_padding = 32.0;
            let caption_right_padding = 48.0;
            let caption_inner_rect = Rect::from_min_max(
                pos2(
                    caption_rect.left() + caption_left_padding,
                    caption_rect.top(),
                ),
                pos2(
                    caption_rect.right() - caption_right_padding,
                    caption_rect.top() + select_height,
                ),
            );

            ui.scope_builder(
                UiBuilder::new()
                    .max_rect(caption_inner_rect)
                    .layout(Layout::centered_and_justified(Direction::LeftToRight)),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    ui.horizontal(|ui| {
                        let month_width = 72.0;
                        let year_width = 72.0;

                        let mut month_selected = Some(month_label(selected_month).to_string());
                        let before_month = month_selected.clone();

                        let mut month_options: Vec<String> = Vec::new();
                        for month in 1..=12 {
                            if month_has_valid_day(selected_year, month, min_date, max_date) {
                                month_options.push(month_label(month).to_string());
                            }
                        }

                        let month_items: Vec<String> = month_options.clone();

                        let _ = select_with_items(
                            ui,
                            theme,
                            SelectProps::new(month_dropdown_id, &mut month_selected)
                                .placeholder("Month")
                                .size(crate::select::SelectSize::Size2)
                                .trigger_variant(TriggerVariant::Classic)
                                .width(month_width),
                            &month_items
                                .iter()
                                .map(|m| crate::select::SelectItem::option(m.clone(), m.clone()))
                                .collect::<Vec<_>>(),
                        );

                        if month_selected != before_month
                            && let Some(value) = &month_selected
                            && let Some(idx) = month_options.iter().position(|s| s == value)
                        {
                            selected_month = (idx + 1) as u32;
                        }

                        let mut year_start = min_year_option.unwrap_or(current_month.year() - 5);
                        let mut year_end = max_year_option.unwrap_or(current_month.year() + 5);
                        if year_start > year_end {
                            std::mem::swap(&mut year_start, &mut year_end);
                        }
                        year_start = year_start.min(current_month.year());
                        year_end = year_end.max(current_month.year());

                        let mut year_options: Vec<String> = Vec::new();
                        for year in year_start..=year_end {
                            let disabled = min_year_option
                                .map(|min_year| year < min_year)
                                .unwrap_or(false)
                                || max_year_option
                                    .map(|max_year| year > max_year)
                                    .unwrap_or(false);
                            if !disabled {
                                year_options.push(year.to_string());
                            }
                        }

                        let mut year_selected = Some(selected_year.to_string());
                        let before_year = year_selected.clone();

                        let year_items: Vec<String> = year_options.clone();

                        let _ = select_with_items(
                            ui,
                            theme,
                            SelectProps::new(year_dropdown_id, &mut year_selected)
                                .placeholder("Year")
                                .size(crate::select::SelectSize::Size2)
                                .trigger_variant(TriggerVariant::Classic)
                                .width(year_width),
                            &year_items
                                .iter()
                                .map(|y| crate::select::SelectItem::option(y.clone(), y.clone()))
                                .collect::<Vec<_>>(),
                        );

                        if year_selected != before_year
                            && let Some(value) = &year_selected
                            && let Ok(year) = value.parse::<i32>()
                        {
                            selected_year = year;
                        }
                    });
                },
            );

            if (selected_month != current_month.month() || selected_year != current_month.year())
                && let Some(new_month) =
                    chrono::NaiveDate::from_ymd_opt(selected_year, selected_month, 1)
            {
                current_month = new_month;
                ui.memory_mut(|m| m.data.insert_persisted(current_month_id, current_month));
            }
        }

        {
            let painter = ui.painter();
            if props.caption_layout == CalendarCaptionLayout::Label || months_count > 1 {
                for idx in 0..months_count {
                    let month = current_month
                        .checked_add_months(Months::new(idx as u32))
                        .unwrap_or(current_month);
                    let header_text = format!("{} {}", month.format("%B"), month.format("%Y"));
                    let header_galley = painter.layout_no_wrap(
                        header_text,
                        FontId::proportional(14.0),
                        theme.palette.foreground,
                    );
                    let col_center_x = header_rect.left()
                        + idx as f32 * (month_width + month_gap)
                        + month_width * 0.5;
                    let text_pos = pos2(
                        col_center_x - header_galley.size().x * 0.5,
                        header_rect.center().y - header_galley.size().y * 0.5,
                    );
                    painter.galley(text_pos, header_galley, theme.palette.foreground);
                }
            }

            let arrow_font = FontId::proportional(16.0);

            let prev_fill = if prev_response.hovered() && !prev_disabled {
                mix(theme.palette.background, theme.palette.accent, 0.12)
            } else {
                Color32::TRANSPARENT
            };
            painter.rect_filled(prev_rect, CornerRadius::same(6), prev_fill);
            let prev_color = if prev_disabled {
                theme.palette.muted_foreground
            } else {
                theme.palette.foreground
            };
            let prev_arrow =
                painter.layout_no_wrap("<".to_string(), arrow_font.clone(), prev_color);
            let prev_pos = pos2(
                prev_rect.center().x - prev_arrow.size().x * 0.5,
                prev_rect.center().y - prev_arrow.size().y * 0.5,
            );
            painter.galley(prev_pos, prev_arrow, prev_color);

            let next_fill = if next_response.hovered() && !next_disabled {
                mix(theme.palette.background, theme.palette.accent, 0.12)
            } else {
                Color32::TRANSPARENT
            };
            painter.rect_filled(next_rect, CornerRadius::same(6), next_fill);
            let next_color = if next_disabled {
                theme.palette.muted_foreground
            } else {
                theme.palette.foreground
            };
            let next_arrow =
                painter.layout_no_wrap(">".to_string(), arrow_font.clone(), next_color);
            let next_pos = pos2(
                next_rect.center().x - next_arrow.size().x * 0.5,
                next_rect.center().y - next_arrow.size().y * 0.5,
            );
            painter.galley(next_pos, next_arrow, next_color);
        }

        if prev_response.clicked() && !prev_disabled {
            let first_day = current_month.with_day(1).unwrap();
            let new_month = first_day
                .checked_sub_months(chrono::Months::new(1))
                .unwrap_or(first_day);
            ui.memory_mut(|m| m.data.insert_persisted(current_month_id, new_month));
        }

        if next_response.clicked() && !next_disabled {
            let first_day = current_month.with_day(1).unwrap();
            let new_month = first_day
                .checked_add_months(chrono::Months::new(1))
                .unwrap_or(first_day);
            ui.memory_mut(|m| m.data.insert_persisted(current_month_id, new_month));
        }

        ui.spacing_mut().item_spacing.x = month_gap;
        ui.spacing_mut().item_spacing.y = 2.0;
        ui.horizontal(|months_ui| {
            for month_idx in 0..months_count {
                months_ui.vertical(|month_ui| {
                    month_ui.set_width(month_width);

                    let month = current_month
                        .checked_add_months(Months::new(month_idx as u32))
                        .unwrap_or(current_month);

                    let (_weekdays_id, _weekdays_rect) =
                        month_ui.allocate_space(vec2(month_width, weekdays_height));

                    {
                        month_ui.spacing_mut().item_spacing.x = cell_padding;
                        month_ui.horizontal(|weekdays_ui| {
                            for weekday in weekday_names.iter() {
                                let (_id, cell_rect) =
                                    weekdays_ui.allocate_space(vec2(cell_size, weekdays_height));
                                let weekdays_painter = weekdays_ui.painter();
                                let galley = weekdays_painter.layout_no_wrap(
                                    weekday.to_string(),
                                    FontId::proportional(12.0),
                                    theme.palette.muted_foreground,
                                );
                                let text_pos = pos2(
                                    cell_rect.center().x - galley.size().x * 0.5,
                                    cell_rect.center().y - galley.size().y * 0.5,
                                );
                                weekdays_painter.galley(
                                    text_pos,
                                    galley,
                                    theme.palette.muted_foreground,
                                );
                            }
                        });
                    }

                    let first_day = month.with_day(1).unwrap();
                    let first_weekday = first_day.weekday();
                    let days_in_month = if let Some(next_month) =
                        first_day.checked_add_months(chrono::Months::new(1))
                    {
                        next_month.pred_opt().unwrap_or(first_day).day()
                    } else {
                        31
                    };

                    let start_offset = match first_weekday {
                        chrono::Weekday::Mon => 0,
                        chrono::Weekday::Tue => 1,
                        chrono::Weekday::Wed => 2,
                        chrono::Weekday::Thu => 3,
                        chrono::Weekday::Fri => 4,
                        chrono::Weekday::Sat => 5,
                        chrono::Weekday::Sun => 6,
                    };

                    let rows =
                        ((start_offset + days_in_month as usize) as f32 / 7.0).ceil() as usize;

                    month_ui.spacing_mut().item_spacing.x = cell_padding;

                    for row in 0..rows {
                        month_ui.horizontal(|ui| {
                            for col in 0..7 {
                                let day_num = (row * 7 + col) as u32;
                                if day_num < start_offset as u32
                                    || day_num >= start_offset as u32 + days_in_month
                                {
                                    ui.allocate_space(vec2(cell_size, cell_size));
                                    continue;
                                }

                                let day = day_num - start_offset as u32 + 1;
                                let date = first_day.with_day(day).unwrap();

                                let is_today = date == today;
                                let is_selected = match props.mode {
                                    CalendarMode::Single => props.selected == Some(date),
                                    CalendarMode::Range => {
                                        props.range_start == Some(date)
                                            || props.range_end == Some(date)
                                    }
                                };

                                let is_in_range = match props.mode {
                                    CalendarMode::Single => false,
                                    CalendarMode::Range => {
                                        if let (Some(start), Some(end)) =
                                            (props.range_start, props.range_end)
                                        {
                                            date >= start && date <= end
                                        } else if let Some(start) = props.range_start {
                                            date == start
                                        } else {
                                            false
                                        }
                                    }
                                };

                                let is_disabled = is_date_disabled(
                                    date,
                                    props.min_date,
                                    props.max_date,
                                    &props.disabled_dates,
                                );

                                let (cell_id, cell_rect) =
                                    ui.allocate_space(vec2(cell_size, cell_size));
                                let cell_response = ui.interact(cell_rect, cell_id, Sense::click());

                                let bg_color = if is_disabled {
                                    Color32::TRANSPARENT
                                } else if is_selected {
                                    theme.palette.primary
                                } else if is_in_range {
                                    theme.palette.accent
                                } else if is_today {
                                    theme.palette.muted
                                } else {
                                    Color32::TRANSPARENT
                                };

                                let text_color = if is_disabled {
                                    theme.palette.muted_foreground
                                } else if is_selected {
                                    theme.palette.primary_foreground
                                } else if is_in_range {
                                    theme.palette.accent_foreground
                                } else {
                                    theme.palette.foreground
                                };

                                let cell_painter = ui.painter();
                                cell_painter.rect_filled(
                                    cell_rect,
                                    CornerRadius::same(4),
                                    bg_color,
                                );

                                let galley = cell_painter.layout_no_wrap(
                                    day.to_string(),
                                    FontId::proportional(13.0),
                                    text_color,
                                );
                                let text_pos = pos2(
                                    cell_rect.center().x - galley.size().x * 0.5,
                                    cell_rect.center().y - galley.size().y * 0.5,
                                );
                                cell_painter.galley(text_pos, galley, text_color);

                                if !is_disabled && cell_response.clicked() {
                                    match props.mode {
                                        CalendarMode::Single => {
                                            let next =
                                                toggle_single_selection(props.selected, date);
                                            props.selected = next;
                                            if let Some(ref mut cb) = props.on_select {
                                                cb(next);
                                            }
                                        }
                                        CalendarMode::Range => {
                                            if props.range_start.is_none()
                                                || props.range_end.is_some()
                                            {
                                                props.range_start = Some(date);
                                                props.range_end = None;
                                            } else if let Some(start) = props.range_start {
                                                if date < start {
                                                    props.range_start = Some(date);
                                                    props.range_end = Some(start);
                                                } else {
                                                    props.range_end = Some(date);
                                                }
                                            }
                                            if let Some(ref mut cb) = props.on_range_select {
                                                cb(props.range_start, props.range_end);
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }
                });
            }
        });
    })
    .response
}

fn month_label(month: u32) -> &'static str {
    MONTH_LABELS
        .get((month.saturating_sub(1) as usize).min(MONTH_LABELS.len() - 1))
        .copied()
        .unwrap_or("Jan")
}

fn month_has_valid_day(
    year: i32,
    month: u32,
    min_date: Option<chrono::NaiveDate>,
    max_date: Option<chrono::NaiveDate>,
) -> bool {
    let start = match chrono::NaiveDate::from_ymd_opt(year, month, 1) {
        Some(date) => date,
        None => return false,
    };

    let end = start
        .checked_add_months(Months::new(1))
        .and_then(|next| next.pred_opt())
        .unwrap_or(start);

    if let Some(min) = min_date
        && end < min
    {
        return false;
    }

    if let Some(max) = max_date
        && start > max
    {
        return false;
    }

    true
}

pub fn calendar<Id>(ui: &mut Ui, theme: &Theme, id_source: Id) -> Response
where
    Id: Hash + Debug,
{
    calendar_with_props(ui, theme, CalendarProps::new(id_source))
}

#[cfg(test)]
mod tests {
    use super::toggle_single_selection;
    use chrono::NaiveDate;

    #[test]
    fn single_selection_toggles_on_same_day() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert_eq!(toggle_single_selection(None, day), Some(day));
        assert_eq!(toggle_single_selection(Some(day), day), None);
    }
}
