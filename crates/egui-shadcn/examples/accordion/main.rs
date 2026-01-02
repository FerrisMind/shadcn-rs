#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/logging.rs"]
mod logging;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, FontId, RichText, vec2};
use egui_shadcn::{AccordionItemProps, AccordionProps, Theme, accordion, accordion_item};

struct AccordionDemo {
    theme: Theme,
    value: Option<String>,
    collapsible_value: Option<String>,
}

impl AccordionDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value: Some("item-1".to_string()),
            collapsible_value: None,
        }
    }
}

impl App for AccordionDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        let theme = self.theme.clone();

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_main_align(egui::Align::Min),
                |ui| {
                    ui.add_space(40.0);

                    ui.heading("Accordion Demo");
                    ui.add_space(24.0);

                    // Main accordion example (matches shadcn accordion-demo.tsx)
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(450.0);

                            ui.label(RichText::new("Default (Single Mode)").size(16.0).strong());
                            ui.add_space(8.0);

                            accordion(
                                ui,
                                &theme,
                                AccordionProps::new("accordion-demo", &mut self.value)
                                    .collapsible(true)
                                    .default_value("item-1"),
                                |ui, acc_ctx| {
                                    // Item 1: Product Information
                                    accordion_item(
                                        ui,
                                        &theme,
                                        acc_ctx,
                                        AccordionItemProps::new("item-1"),
                                        |ui, item_ctx| {
                                            accordion_trigger(ui, &theme, "Product Information", item_ctx.is_open)
                                        },
                                        |ui| {
                                            ui.spacing_mut().item_spacing.y = 8.0;
                                            ui.label(RichText::new(
                                                "Our flagship product combines cutting-edge technology with sleek \
                                                design. Built with premium materials, it offers unparalleled \
                                                performance and reliability."
                                            ).size(14.0));
                                            ui.label(RichText::new(
                                                "Key features include advanced processing capabilities, and an \
                                                intuitive user interface designed for both beginners and experts."
                                            ).size(14.0));
                                        },
                                    );

                                    // Item 2: Shipping Details
                                    accordion_item(
                                        ui,
                                        &theme,
                                        acc_ctx,
                                        AccordionItemProps::new("item-2"),
                                        |ui, item_ctx| {
                                            accordion_trigger(ui, &theme, "Shipping Details", item_ctx.is_open)
                                        },
                                        |ui| {
                                            ui.spacing_mut().item_spacing.y = 8.0;
                                            ui.label(RichText::new(
                                                "We offer worldwide shipping through trusted courier partners. \
                                                Standard delivery takes 3-5 business days, while express shipping \
                                                ensures delivery within 1-2 business days."
                                            ).size(14.0));
                                            ui.label(RichText::new(
                                                "All orders are carefully packaged and fully insured. Track your \
                                                shipment in real-time through our dedicated tracking portal."
                                            ).size(14.0));
                                        },
                                    );

                                    // Item 3: Return Policy
                                    accordion_item(
                                        ui,
                                        &theme,
                                        acc_ctx,
                                        AccordionItemProps::new("item-3"),
                                        |ui, item_ctx| {
                                            accordion_trigger(ui, &theme, "Return Policy", item_ctx.is_open)
                                        },
                                        |ui| {
                                            ui.spacing_mut().item_spacing.y = 8.0;
                                            ui.label(RichText::new(
                                                "We stand behind our products with a comprehensive 30-day return \
                                                policy. If you're not completely satisfied, simply return the \
                                                item in its original condition."
                                            ).size(14.0));
                                            ui.label(RichText::new(
                                                "Our hassle-free return process includes free return shipping and \
                                                full refunds processed within 48 hours of receiving the returned \
                                                item."
                                            ).size(14.0));
                                        },
                                    );
                                },
                            );

                            ui.add_space(32.0);

                            // Collapsible mode example
                            ui.label(RichText::new("Collapsible (can close all)").size(16.0).strong());
                            ui.add_space(8.0);

                            accordion(
                                ui,
                                &theme,
                                AccordionProps::new("accordion-collapsible", &mut self.collapsible_value)
                                    .collapsible(true),
                                |ui, acc_ctx| {
                                    accordion_item(
                                        ui,
                                        &theme,
                                        acc_ctx,
                                        AccordionItemProps::new("faq-1"),
                                        |ui, item_ctx| {
                                            accordion_trigger(ui, &theme, "Is it accessible?", item_ctx.is_open)
                                        },
                                        |ui| {
                                            ui.label(RichText::new(
                                                "Yes. It adheres to the WAI-ARIA design pattern."
                                            ).size(14.0));
                                        },
                                    );

                                    accordion_item(
                                        ui,
                                        &theme,
                                        acc_ctx,
                                        AccordionItemProps::new("faq-2"),
                                        |ui, item_ctx| {
                                            accordion_trigger(ui, &theme, "Is it styled?", item_ctx.is_open)
                                        },
                                        |ui| {
                                            ui.label(RichText::new(
                                                "Yes. It comes with default styles that matches the other components' aesthetic."
                                            ).size(14.0));
                                        },
                                    );

                                    accordion_item(
                                        ui,
                                        &theme,
                                        acc_ctx,
                                        AccordionItemProps::new("faq-3"),
                                        |ui, item_ctx| {
                                            accordion_trigger(ui, &theme, "Is it animated?", item_ctx.is_open)
                                        },
                                        |ui| {
                                            ui.label(RichText::new(
                                                "Yes. It's animated by default, but you can disable it if you prefer."
                                            ).size(14.0));
                                        },
                                    );
                                },
                            );

                            ui.add_space(24.0);
                            ui.label(format!("Current value: {:?}", self.value));
                            ui.label(format!("Collapsible value: {:?}", self.collapsible_value));
                        },
                    );
                },
            );
        });
    }
}

/// Helper function to render accordion trigger with chevron icon
fn accordion_trigger(
    ui: &mut egui::Ui,
    theme: &Theme,
    title: &str,
    is_open: bool,
) -> egui::Response {
    let response = ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true),
        |ui| {
            ui.add_space(0.0);

            let text = RichText::new(title)
                .font(FontId::proportional(14.0))
                .strong();

            ui.label(text);

            // Chevron icon that rotates when open
            let icon_color = theme.palette.muted_foreground;

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(4.0);

                // Simple chevron using text
                let chevron = if is_open { "▲" } else { "▼" };
                ui.label(RichText::new(chevron).size(10.0).color(icon_color));
            });
        },
    );

    // Make the whole area clickable
    let rect = response.response.rect.expand2(vec2(0.0, 8.0));
    ui.allocate_rect(rect, egui::Sense::click())
}

fn main() -> eframe::Result<()> {
    let log_path = logging::init_file_logger("accordion")
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<failed to open log file>".to_string());
    let options = icon::native_options();
    eframe::run_native(
        "Accordion example",
        options,
        Box::new(move |_cc| {
            log::info!("logging to {log_path}");
            Ok(Box::new(AccordionDemo::new()))
        }),
    )
}
