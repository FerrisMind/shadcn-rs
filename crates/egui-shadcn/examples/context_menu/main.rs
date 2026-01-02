#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, Sense};
use egui_shadcn::{
    ContextMenuCheckboxItemProps, ContextMenuItemProps, ContextMenuItemVariant,
    ContextMenuLabelProps, ContextMenuRadioItemProps, ContextMenuSubProps, Theme, context_menu,
    context_menu_checkbox_item, context_menu_item, context_menu_label, context_menu_radio_item,
    context_menu_separator, context_menu_sub,
};

struct ContextMenuExample {
    theme: Theme,
    show_bookmarks: bool,
    show_full_urls: bool,
    selected_person: String,
}

impl ContextMenuExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            show_bookmarks: true,
            show_full_urls: false,
            selected_person: "pedro".to_string(),
        }
    }
}

impl App for ContextMenuExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Context Menu Component");
            ui.add_space(16.0);

            ui.label("Right-click on the box below to see the context menu:");
            ui.add_space(8.0);

            // Trigger area for context menu
            let _trigger_rect = ui.available_rect_before_wrap();
            let desired_size = egui::vec2(300.0, 150.0);
            let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

            // Draw the trigger area
            ui.painter().rect_stroke(
                rect,
                4.0,
                egui::Stroke::new(1.0, self.theme.palette.border),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Right click here",
                egui::FontId::proportional(14.0),
                self.theme.palette.muted_foreground,
            );

            // Show context menu on right-click
            context_menu(&response, &self.theme, |ui| {
                // Basic items with shortcuts
                if context_menu_item(
                    ui,
                    &self.theme,
                    ContextMenuItemProps::new("Back")
                        .with_shortcut("⌘[")
                        .inset(true),
                )
                .clicked()
                {
                    println!("Back clicked");
                    ui.close();
                }

                if context_menu_item(
                    ui,
                    &self.theme,
                    ContextMenuItemProps::new("Forward")
                        .with_shortcut("⌘]")
                        .inset(true)
                        .disabled(true),
                )
                .clicked()
                {
                    println!("Forward clicked");
                    ui.close();
                }

                if context_menu_item(
                    ui,
                    &self.theme,
                    ContextMenuItemProps::new("Reload")
                        .with_shortcut("⌘R")
                        .inset(true),
                )
                .clicked()
                {
                    println!("Reload clicked");
                    ui.close();
                }

                // Submenu
                context_menu_sub(
                    ui,
                    &self.theme,
                    ContextMenuSubProps::new("More Tools").inset(true),
                    |ui| {
                        if context_menu_item(
                            ui,
                            &self.theme,
                            ContextMenuItemProps::new("Save Page..."),
                        )
                        .clicked()
                        {
                            println!("Save Page clicked");
                            ui.close();
                        }
                        if context_menu_item(
                            ui,
                            &self.theme,
                            ContextMenuItemProps::new("Create Shortcut..."),
                        )
                        .clicked()
                        {
                            println!("Create Shortcut clicked");
                            ui.close();
                        }
                        if context_menu_item(
                            ui,
                            &self.theme,
                            ContextMenuItemProps::new("Name Window..."),
                        )
                        .clicked()
                        {
                            println!("Name Window clicked");
                            ui.close();
                        }
                        context_menu_separator(ui, &self.theme);
                        if context_menu_item(
                            ui,
                            &self.theme,
                            ContextMenuItemProps::new("Developer Tools"),
                        )
                        .clicked()
                        {
                            println!("Developer Tools clicked");
                            ui.close();
                        }
                        context_menu_separator(ui, &self.theme);
                        if context_menu_item(
                            ui,
                            &self.theme,
                            ContextMenuItemProps::new("Delete")
                                .with_variant(ContextMenuItemVariant::Destructive),
                        )
                        .clicked()
                        {
                            println!("Delete clicked");
                            ui.close();
                        }
                    },
                );

                context_menu_separator(ui, &self.theme);

                // Checkbox items
                if context_menu_checkbox_item(
                    ui,
                    &self.theme,
                    ContextMenuCheckboxItemProps::new("Show Bookmarks", self.show_bookmarks),
                )
                .clicked()
                {
                    self.show_bookmarks = !self.show_bookmarks;
                }

                if context_menu_checkbox_item(
                    ui,
                    &self.theme,
                    ContextMenuCheckboxItemProps::new("Show Full URLs", self.show_full_urls),
                )
                .clicked()
                {
                    self.show_full_urls = !self.show_full_urls;
                }

                context_menu_separator(ui, &self.theme);

                // Radio group
                context_menu_label(
                    ui,
                    &self.theme,
                    ContextMenuLabelProps::new("People").inset(true),
                );

                if context_menu_radio_item(
                    ui,
                    &self.theme,
                    ContextMenuRadioItemProps::new("Pedro Duarte", "pedro", &self.selected_person),
                )
                .clicked()
                {
                    self.selected_person = "pedro".to_string();
                }

                if context_menu_radio_item(
                    ui,
                    &self.theme,
                    ContextMenuRadioItemProps::new("Colm Tuite", "colm", &self.selected_person),
                )
                .clicked()
                {
                    self.selected_person = "colm".to_string();
                }
            });

            ui.add_space(24.0);

            // State display
            ui.separator();
            ui.add_space(8.0);
            ui.label("Current State:");
            ui.label(format!("• Show Bookmarks: {}", self.show_bookmarks));
            ui.label(format!("• Show Full URLs: {}", self.show_full_urls));
            ui.label(format!("• Selected Person: {}", self.selected_person));
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Context Menu example",
        options,
        Box::new(|_cc| Ok(Box::new(ContextMenuExample::new()))),
    )
}
