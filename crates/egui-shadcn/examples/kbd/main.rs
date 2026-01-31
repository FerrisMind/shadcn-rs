#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::CornerRadius;
use egui_shadcn::{
    CardProps, CardVariant, KbdProps, ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize,
    ScrollAreaType, ScrollDirection, Theme, card, kbd, scroll_area,
};

struct KbdDemo {
    theme: Theme,
}

impl KbdDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let card_size = egui::vec2(760.0, 560.0);
        card(
            ui,
            &self.theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_padding(egui::vec2(16.0, 16.0))
                .with_rounding(CornerRadius::same(12))
                .with_shadow(true),
            |card_ui| {
                card_ui.set_min_size(card_size);
                card_ui.set_max_size(card_size);

                card_ui.vertical(|card_ui| {
                    scroll_area(
                        card_ui,
                        &self.theme,
                        ScrollAreaProps {
                            scroll_type: ScrollAreaType::Auto,
                            direction: ScrollDirection::Vertical,
                            size: ScrollAreaSize::Size2,
                            radius: ScrollAreaRadius::Small,
                            max_size: Some(card_size),
                            auto_shrink: [false; 2],
                            ..Default::default()
                        },
                        |ui| {
                            ui.set_width(ui.available_width());

                            // Header
                            ui.heading("Kbd Component");
                            ui.add_space(8.0);
                            ui.label(
                                "Keyboard shortcut indicators for displaying key combinations.",
                            );
                            ui.add_space(24.0);

                            // Section: Basic Usage
                            ui.label(egui::RichText::new("Basic Usage").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, KbdProps::new("Ctrl"));
                                ui.label("+");
                                kbd(ui, &self.theme, KbdProps::new("C"));
                            });
                            ui.add_space(16.0);

                            // Section: Modifier Keys
                            ui.label(egui::RichText::new("Modifier Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, KbdProps::new("⌘"));
                                kbd(ui, &self.theme, KbdProps::new("⇧"));
                                kbd(ui, &self.theme, KbdProps::new("⌥"));
                                kbd(ui, &self.theme, KbdProps::new("⌃"));
                            });
                            ui.add_space(16.0);

                            // Section: Different Sizes
                            ui.label(egui::RichText::new("Sizes").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, KbdProps::new("XS").size(10.0));
                                kbd(ui, &self.theme, KbdProps::new("S").size(11.0));
                                kbd(ui, &self.theme, KbdProps::new("M").size(12.0));
                                kbd(ui, &self.theme, KbdProps::new("L").size(14.0));
                                kbd(ui, &self.theme, KbdProps::new("XL").size(16.0));
                            });
                            ui.add_space(16.0);

                            // Section: Common Shortcuts
                            ui.label(egui::RichText::new("Common Shortcuts").strong());
                            ui.add_space(8.0);

                            let shortcuts = vec![
                                ("Copy", "Ctrl", "C"),
                                ("Paste", "Ctrl", "V"),
                                ("Cut", "Ctrl", "X"),
                                ("Undo", "Ctrl", "Z"),
                                ("Save", "Ctrl", "S"),
                                ("Find", "Ctrl", "K"),
                            ];

                            for (action, key1, key2) in shortcuts {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}:", action));
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            kbd(ui, &self.theme, KbdProps::new(key2));
                                            ui.label("+");
                                            kbd(ui, &self.theme, KbdProps::new(key1));
                                        },
                                    );
                                });
                                ui.add_space(4.0);
                            }
                            ui.add_space(16.0);

                            // Section: Function Keys
                            ui.label(egui::RichText::new("Function Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                for i in 1..=12 {
                                    kbd(ui, &self.theme, KbdProps::new(format!("F{}", i)));
                                }
                            });
                            ui.add_space(16.0);

                            // Section: Arrow Keys
                            ui.label(egui::RichText::new("Arrow Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, KbdProps::new("↑"));
                                kbd(ui, &self.theme, KbdProps::new("↓"));
                                kbd(ui, &self.theme, KbdProps::new("←"));
                                kbd(ui, &self.theme, KbdProps::new("→"));
                            });
                            ui.add_space(16.0);

                            // Section: Navigation
                            ui.label(egui::RichText::new("Navigation").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, KbdProps::new("Home"));
                                kbd(ui, &self.theme, KbdProps::new("End"));
                                kbd(ui, &self.theme, KbdProps::new("PgUp"));
                                kbd(ui, &self.theme, KbdProps::new("PgDn"));
                                kbd(ui, &self.theme, KbdProps::new("Ins"));
                                kbd(ui, &self.theme, KbdProps::new("Del"));
                            });
                            ui.add_space(16.0);

                            // Section: Special Keys
                            ui.label(egui::RichText::new("Special Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, KbdProps::new("Tab"));
                                kbd(ui, &self.theme, KbdProps::new("Esc"));
                                kbd(ui, &self.theme, KbdProps::new("Enter"));
                                kbd(ui, &self.theme, KbdProps::new("Space"));
                                kbd(ui, &self.theme, KbdProps::new("Backspace"));
                            });
                            ui.add_space(24.0);
                        },
                    );
                });
            },
        );
    }
}

impl App for KbdDemo {
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
    options.viewport = options.viewport.with_inner_size(egui::vec2(840.0, 640.0));
    eframe::run_native(
        "Kbd demo",
        options,
        Box::new(|_cc| Ok(Box::new(KbdDemo::new()))),
    )
}
