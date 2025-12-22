#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use std::{cell::RefCell, rc::Rc};

use chrono::{Local, NaiveDate};
use eframe::{App, Frame, egui};
use egui::CornerRadius;
use egui_shadcn::{
    CalendarCaptionLayout, CalendarMode, CalendarProps, CardProps, CardVariant, Theme,
    calendar_with_props, card,
};

struct CalendarDemo {
    theme: Theme,
    selected: Rc<RefCell<Option<NaiveDate>>>,
}

impl CalendarDemo {
    fn new() -> Self {
        let today = Local::now().date_naive();
        Self {
            theme: Theme::default(),
            selected: Rc::new(RefCell::new(Some(today))),
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let selected = *self.selected.borrow();
        let selection = self.selected.clone();
        let card_size = egui::vec2(288.0, 360.0);

        card(
            ui,
            &self.theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_padding(egui::vec2(12.0, 12.0))
                .with_rounding(CornerRadius::same(12))
                .with_shadow(true),
            |card_ui| {
                card_ui.horizontal_centered(|ui| {
                    ui.set_min_size(card_size);
                    ui.set_max_size(card_size);
                    calendar_with_props(
                        ui,
                        &self.theme,
                        CalendarProps::new("calendar-demo")
                            .mode(CalendarMode::Single)
                            .selected(selected)
                            .on_select(move |date| {
                                *selection.borrow_mut() = date;
                            })
                            .caption_layout(CalendarCaptionLayout::Dropdown),
                    );
                });
            },
        );
    }
}

impl App for CalendarDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    self.render(ui);
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let mut options = icon::native_options();
    options.viewport = options.viewport.with_inner_size(egui::vec2(360.0, 380.0));
    eframe::run_native(
        "Calendar demo",
        options,
        Box::new(|_cc| Ok(Box::new(CalendarDemo::new()))),
    )
}
