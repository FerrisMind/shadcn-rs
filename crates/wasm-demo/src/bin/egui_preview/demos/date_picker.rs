use super::super::app::EguiPreviewApp;
use chrono::NaiveDate;
use eframe::egui::Ui;
use egui_shadcn::{
    ButtonJustify, DatePickerIconPosition, DatePickerProps, DateRange, DateRangePickerProps,
    date_picker_with_props, date_range_picker_with_props,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let date_id = ui.make_persistent_id("preview-date-picker-single");
    let range_id = ui.make_persistent_id("preview-date-picker-range");

    let mut date = ui
        .data(|d| d.get_temp::<Option<NaiveDate>>(date_id))
        .unwrap_or(None);
    let mut range = ui
        .data(|d| d.get_temp::<DateRange>(range_id))
        .unwrap_or_default();

    let _ = date_picker_with_props(
        ui,
        &app.theme,
        DatePickerProps::new("preview-date-picker", &mut date)
            .placeholder("Pick a date")
            .trigger_width(if compact { 220.0 } else { 260.0 })
            .icon_position(DatePickerIconPosition::Leading)
            .justify(ButtonJustify::Start),
    );

    if !compact {
        ui.add_space(10.0);
        let _ = date_range_picker_with_props(
            ui,
            &app.theme,
            DateRangePickerProps::new("preview-date-range-picker", &mut range)
                .placeholder("Pick a date range")
                .trigger_width(320.0)
                .number_of_months(2),
        );
    }

    ui.data_mut(|d| {
        d.insert_temp(date_id, date);
        d.insert_temp(range_id, range);
    });
}
