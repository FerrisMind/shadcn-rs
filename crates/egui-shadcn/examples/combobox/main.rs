#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui_shadcn::{ComboboxProps, SelectItem, Theme, combobox_with_props};

struct ComboboxExample {
    theme: Theme,
    value: Option<String>,
    search: String,
    items: Vec<SelectItem>,
}

impl ComboboxExample {
    fn new() -> Self {
        let items = vec![
            SelectItem::option("next.js", "Next.js"),
            SelectItem::option("sveltekit", "SvelteKit"),
            SelectItem::option("nuxt.js", "Nuxt.js"),
            SelectItem::option("remix", "Remix"),
            SelectItem::option("astro", "Astro"),
        ];

        Self {
            theme: Theme::default(),
            value: None,
            search: String::new(),
            items,
        }
    }
}

impl App for ComboboxExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Combobox");
            ui.add_space(12.0);

            ui.vertical_centered(|ui| {
                let _ = combobox_with_props(
                    ui,
                    &self.theme,
                    ComboboxProps::new(
                        "combobox-demo",
                        &mut self.value,
                        &self.items,
                        &mut self.search,
                    )
                    .placeholder("Select framework...")
                    .search_placeholder("Search framework...")
                    .empty_text("No framework found.")
                    .width(200.0),
                );
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Combobox example",
        options,
        Box::new(|_cc| Ok(Box::new(ComboboxExample::new()))),
    )
}
