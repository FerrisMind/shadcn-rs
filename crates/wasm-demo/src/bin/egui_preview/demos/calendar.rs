use super::super::app::EguiPreviewApp;
use chrono::NaiveDate;
use eframe::egui::Ui;
use egui_shadcn::{CalendarCaptionLayout, CalendarProps, calendar_with_props};
use std::cell::RefCell;
use std::rc::Rc;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let row_width = 300.0;
    let calendar_id = ui.make_persistent_id("preview-calendar");
    let selected_id = calendar_id.with("selected");
    let mut selected = ui
        .data(|d| d.get_temp::<Option<NaiveDate>>(selected_id))
        .unwrap_or(None);
    let selection_storage = Rc::new(RefCell::new(selected));
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let callback_storage = selection_storage.clone();
        calendar_with_props(
            row,
            &app.theme,
            CalendarProps::new(calendar_id)
                .caption_layout(CalendarCaptionLayout::Dropdown)
                .selected(selected)
                .on_select(move |date| {
                    *callback_storage.borrow_mut() = date;
                }),
        );
    });
    selected = *selection_storage.borrow();
    ui.data_mut(|d| d.insert_temp(selected_id, selected));
}
