use super::super::app::EguiPreviewApp;
use chrono::NaiveDate;
use eframe::egui::Ui;
use egui_shadcn::{
    ButtonJustify, DatePickerIconPosition, DatePickerProps, DateRange, DateRangePickerProps,
    date_picker_with_props, date_range_picker_with_props,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let scope_id = ui.make_persistent_id("preview-date-picker-scope");
    let date_id = scope_id.with("single-value");
    let range_id = scope_id.with("range-value");

    let mut date = ui
        .data(|d| d.get_temp::<Option<NaiveDate>>(date_id))
        .unwrap_or(None);
    let mut range = ui
        .data(|d| d.get_temp::<DateRange>(range_id))
        .unwrap_or_default();

    let _ = date_picker_with_props(
        ui,
        &app.theme,
        DatePickerProps::new(scope_id.with("single"), &mut date)
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
            DateRangePickerProps::new(scope_id.with("range"), &mut range)
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
