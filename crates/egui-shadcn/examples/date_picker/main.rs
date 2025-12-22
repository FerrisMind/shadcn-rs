#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use chrono::{Datelike, Duration, NaiveDate};
use eframe::{App, Frame, egui};
use egui::{CornerRadius, Margin, Stroke, vec2};
use egui_shadcn::{
    Button, ButtonJustify, ButtonSize, ButtonVariant, CalendarCaptionLayout, CalendarMode,
    CalendarProps, DatePickerIconPosition, DatePickerProps, DateRange, DateRangePickerProps,
    SelectPropsSimple, Theme, calendar_with_props, date_picker_with_props,
    date_range_picker_with_props, icon_calendar, popover, select,
};
use std::cell::RefCell;
use std::rc::Rc;

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

struct DatePickerExample {
    theme: Theme,
    date: Option<NaiveDate>,
    range: DateRange,
    presets_open: bool,
    presets_date: Option<NaiveDate>,
    presets_value: Option<String>,
    presets_options: Vec<String>,
    dob: Option<NaiveDate>,
    dob_error: Option<String>,
    submitted: Option<NaiveDate>,
}

impl DatePickerExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            date: None,
            range: DateRange::default(),
            presets_open: false,
            presets_date: None,
            presets_value: None,
            presets_options: vec![
                "Today".to_string(),
                "Tomorrow".to_string(),
                "In 3 days".to_string(),
                "In a week".to_string(),
            ],
            dob: None,
            dob_error: None,
            submitted: None,
        }
    }
}

impl App for DatePickerExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(16.0, 16.0);
            ui.heading("Date Picker");
            ui.add_space(12.0);

            ui.label(egui::RichText::new("Demo").strong());
            let _ = date_picker_with_props(
                ui,
                &self.theme,
                DatePickerProps::new("date-picker-demo", &mut self.date)
                    .placeholder("Pick a date")
                    .trigger_width(240.0)
                    .icon_position(DatePickerIconPosition::Leading)
                    .justify(ButtonJustify::Start),
            );

            ui.add_space(8.0);
            ui.label(egui::RichText::new("With range").strong());
            let _ = date_range_picker_with_props(
                ui,
                &self.theme,
                DateRangePickerProps::new("date-picker-range", &mut self.range)
                    .placeholder("Pick a date")
                    .trigger_width(300.0)
                    .number_of_months(2),
            );

            ui.add_space(8.0);
            ui.label(egui::RichText::new("With presets").strong());

            let today = chrono::Local::now().date_naive();
            let preset_selection = Rc::new(RefCell::new(None));
            let preset_callback = preset_selection.clone();
            let (preset_trigger, preset_date) = popover(
                ui,
                &self.theme,
                egui_shadcn::PopoverProps::new(
                    ui.make_persistent_id("date-picker-presets").with("popover"),
                    &mut self.presets_open,
                )
                .align(egui_shadcn::PopoverAlign::Start)
                .side(egui_shadcn::PopoverSide::Bottom)
                .with_width(280.0)
                .with_max_height(420.0)
                .with_content_padding(Margin::same(8))
                .with_animation(true),
                |ui| {
                    let label = if let Some(date) = self.presets_date {
                        egui::WidgetText::from(format_ppp(date))
                    } else {
                        egui::RichText::new("Pick a date")
                            .color(self.theme.palette.muted_foreground)
                            .into()
                    };
                    Button::new(label)
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Default)
                        .justify(ButtonJustify::Start)
                        .icon(&icon_calendar)
                        .min_width(240.0)
                        .enabled(true)
                        .show(ui, &self.theme)
                },
                |ui| {
                    let before = self.presets_value.clone();
                    let _ = select(
                        ui,
                        &self.theme,
                        SelectPropsSimple {
                            id_source: ui.make_persistent_id("date-picker-presets").with("select"),
                            selected: &mut self.presets_value,
                            options: &self.presets_options,
                            placeholder: "Select",
                            size: egui_shadcn::ControlSize::Md,
                            enabled: true,
                            is_invalid: false,
                        },
                    );

                    if self.presets_value != before
                        && let Some(label) = self.presets_value.as_deref()
                    {
                        let offset = match label {
                            "Today" => 0,
                            "Tomorrow" => 1,
                            "In 3 days" => 3,
                            "In a week" => 7,
                            _ => 0,
                        };
                        *preset_callback.borrow_mut() = Some(today + Duration::days(offset));
                    }

                    egui::Frame::NONE
                        .stroke(Stroke::new(1.0, self.theme.palette.border))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            let cb = preset_callback.clone();
                            calendar_with_props(
                                ui,
                                &self.theme,
                                CalendarProps::new(
                                    ui.make_persistent_id("date-picker-presets")
                                        .with("calendar"),
                                )
                                .mode(CalendarMode::Single)
                                .selected(self.presets_date)
                                .on_select(move |date| {
                                    *cb.borrow_mut() = date;
                                }),
                            );
                        });

                    *preset_selection.borrow()
                },
            );
            let _ = preset_trigger;
            if let Some(date) = preset_date.flatten() {
                self.presets_date = Some(date);
            }

            ui.add_space(8.0);
            ui.label(egui::RichText::new("Form").strong());
            ui.vertical(|form| {
                form.spacing_mut().item_spacing = vec2(8.0, 8.0);
                form.label(egui::RichText::new("Date of birth").size(13.0));

                let max = chrono::Local::now().date_naive();
                let min = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

                let _ = date_picker_with_props(
                    form,
                    &self.theme,
                    DatePickerProps::new("date-picker-form", &mut self.dob)
                        .placeholder("Pick a date")
                        .trigger_width(240.0)
                        .icon_position(DatePickerIconPosition::Trailing)
                        .justify(ButtonJustify::Between)
                        .caption_layout(CalendarCaptionLayout::Dropdown)
                        .min_date(Some(min))
                        .max_date(Some(max)),
                );

                form.label(
                    egui::RichText::new("Your date of birth is used to calculate your age.")
                        .color(self.theme.palette.muted_foreground)
                        .size(12.0),
                );

                if let Some(err) = &self.dob_error {
                    form.label(egui::RichText::new(err).color(self.theme.palette.destructive));
                }

                if Button::new("Submit")
                    .variant(ButtonVariant::Default)
                    .size(ButtonSize::Default)
                    .enabled(true)
                    .show(form, &self.theme)
                    .clicked()
                {
                    if self.dob.is_none() {
                        self.dob_error = Some("A date of birth is required.".to_string());
                        self.submitted = None;
                    } else {
                        self.dob_error = None;
                        self.submitted = self.dob;
                    }
                }
            });

            if let Some(date) = self.submitted {
                egui::Window::new("Submitted values")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(format!("dob: {date:?}"));
                    });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Date picker example",
        options,
        Box::new(|_cc| Ok(Box::new(DatePickerExample::new()))),
    )
}
