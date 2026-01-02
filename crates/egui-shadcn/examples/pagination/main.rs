#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::CentralPanel;
use egui_shadcn::{
    PaginationLinkProps, PaginationProps, Theme, pagination, pagination_content,
    pagination_ellipsis, pagination_item, pagination_link, pagination_next, pagination_previous,
};

struct PaginationExample {
    theme: Theme,
    current_page: usize,
}

impl PaginationExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            current_page: 2,
        }
    }
}

impl App for PaginationExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pagination");
            ui.add_space(16.0);

            pagination(
                ui,
                PaginationProps::new(10, &mut self.current_page),
                |ui, props| {
                    pagination_content(ui, |ui| {
                        pagination_item(ui, |ui| {
                            pagination_previous(ui, &self.theme, props);
                        });
                        pagination_item(ui, |ui| {
                            pagination_link(
                                ui,
                                &self.theme,
                                props,
                                PaginationLinkProps::new(1, "1"),
                            );
                        });
                        pagination_item(ui, |ui| {
                            pagination_link(
                                ui,
                                &self.theme,
                                props,
                                PaginationLinkProps::new(2, "2"),
                            );
                        });
                        pagination_item(ui, |ui| {
                            pagination_link(
                                ui,
                                &self.theme,
                                props,
                                PaginationLinkProps::new(3, "3"),
                            );
                        });
                        pagination_item(ui, |ui| {
                            pagination_ellipsis(ui, &self.theme);
                        });
                        pagination_item(ui, |ui| {
                            pagination_next(ui, &self.theme, props);
                        });
                    });
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Pagination example",
        options,
        Box::new(|_cc| Ok(Box::new(PaginationExample::new()))),
    )
}
