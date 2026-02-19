#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CornerRadius, FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    CardProps, CardVariant, KbdProps, KbdSize, ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize,
    ScrollAreaType, ScrollDirection, Theme, card, kbd, scroll_area,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded");
    let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
    if already_set {
        return;
    }
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

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
                .variant(CardVariant::Outline)
                .padding(egui::vec2(16.0, 16.0))
                .rounding(CornerRadius::same(12))
                .shadow(true),
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
                                kbd(ui, &self.theme, "Ctrl", KbdProps::new());
                                ui.label("+");
                                kbd(ui, &self.theme, "C", KbdProps::new());
                            });
                            ui.add_space(16.0);

                            // Section: Modifier Keys
                            ui.label(egui::RichText::new("Modifier Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::Command.unicode().to_string(),
                                    KbdProps::new(),
                                );
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::ArrowBigUp.unicode().to_string(),
                                    KbdProps::new(),
                                );
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::Option.unicode().to_string(),
                                    KbdProps::new(),
                                );
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::ChevronUp.unicode().to_string(),
                                    KbdProps::new(),
                                );
                            });
                            ui.add_space(16.0);

                            // Section: Different Sizes
                            ui.label(egui::RichText::new("Sizes").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, "XS", KbdProps::new().size(KbdSize::Size1));
                                kbd(ui, &self.theme, "S", KbdProps::new().size(KbdSize::Size2));
                                kbd(ui, &self.theme, "M", KbdProps::new().size(KbdSize::Size3));
                                kbd(ui, &self.theme, "L", KbdProps::new().size(KbdSize::Five));
                                kbd(ui, &self.theme, "XL", KbdProps::new().size(KbdSize::Six));
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
                                            kbd(ui, &self.theme, key2, KbdProps::new());
                                            ui.label("+");
                                            kbd(ui, &self.theme, key1, KbdProps::new());
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
                                    kbd(ui, &self.theme, &format!("F{}", i), KbdProps::new());
                                }
                            });
                            ui.add_space(16.0);

                            // Section: Arrow Keys
                            ui.label(egui::RichText::new("Arrow Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::ArrowUp.unicode().to_string(),
                                    KbdProps::new(),
                                );
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::ArrowDown.unicode().to_string(),
                                    KbdProps::new(),
                                );
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::ArrowLeft.unicode().to_string(),
                                    KbdProps::new(),
                                );
                                kbd(
                                    ui,
                                    &self.theme,
                                    &Icon::ArrowRight.unicode().to_string(),
                                    KbdProps::new(),
                                );
                            });
                            ui.add_space(16.0);

                            // Section: Navigation
                            ui.label(egui::RichText::new("Navigation").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, "Home", KbdProps::new());
                                kbd(ui, &self.theme, "End", KbdProps::new());
                                kbd(ui, &self.theme, "PgUp", KbdProps::new());
                                kbd(ui, &self.theme, "PgDn", KbdProps::new());
                                kbd(ui, &self.theme, "Ins", KbdProps::new());
                                kbd(ui, &self.theme, "Del", KbdProps::new());
                            });
                            ui.add_space(16.0);

                            // Section: Special Keys
                            ui.label(egui::RichText::new("Special Keys").strong());
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                kbd(ui, &self.theme, "Tab", KbdProps::new());
                                kbd(ui, &self.theme, "Esc", KbdProps::new());
                                kbd(ui, &self.theme, "Enter", KbdProps::new());
                                kbd(ui, &self.theme, "Space", KbdProps::new());
                                kbd(ui, &self.theme, "Backspace", KbdProps::new());
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
        ensure_lucide_font(ctx);
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
