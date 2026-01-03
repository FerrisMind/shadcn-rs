#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui::{FontData, FontDefinitions, FontFamily, FontId, RichText, vec2};
use egui_shadcn::{
    ControlSize, Theme, ToggleGroupProps, ToggleVariant, toggle_group, toggle_group_item,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InlineChoice {
    Bold,
    Italic,
    Underline,
}

struct ToggleGroupDemo {
    theme: Theme,

    // Multiple selection states
    multi_bold: bool,
    multi_italic: bool,
    multi_underline: bool,

    // Single selection state
    single_value: Option<InlineChoice>,

    // Outline state
    outline_bold: bool,
    outline_italic: bool,
    outline_underline: bool,

    // Disabled state
    disabled_bold: bool,
    disabled_italic: bool,
    disabled_underline: bool,

    // Sizes
    sm_bold: bool,
    sm_italic: bool,
    sm_underline: bool,

    lg_bold: bool,
    lg_italic: bool,
    lg_underline: bool,
}

impl ToggleGroupDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            multi_bold: false,
            multi_italic: true,
            multi_underline: false,
            single_value: Some(InlineChoice::Bold),
            outline_bold: false,
            outline_italic: true,
            outline_underline: false,
            disabled_bold: false,
            disabled_italic: false,
            disabled_underline: false,
            sm_bold: false,
            sm_italic: true,
            sm_underline: false,
            lg_bold: false,
            lg_italic: true,
            lg_underline: false,
        }
    }
}

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
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn lucide_icon(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string()).font(FontId::new(size, FontFamily::Proportional))
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

impl App for ToggleGroupDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(16.0, 16.0);

            ui.heading("Toggle Group");
            ui.add_space(12.0);

            egui::Grid::new("toggle_groups_grid")
                .num_columns(3)
                .spacing(vec2(40.0, 40.0))
                .show(ui, |grid| {
                    // Demo 1: Default (Multiple)
                    example_card(grid, "Default (Multiple)", |ui| {
                        toggle_group(ui, ToggleGroupProps::default(), |ui, ctx| {
                            toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut self.multi_bold,
                                lucide_icon(Icon::Bold, 16.0),
                            );
                            toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut self.multi_italic,
                                lucide_icon(Icon::Italic, 16.0),
                            );
                            toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut self.multi_underline,
                                lucide_icon(Icon::Underline, 16.0),
                            );
                        });
                    });

                    // Demo 2: Outline
                    example_card(grid, "Outline", |ui| {
                        toggle_group(
                            ui,
                            ToggleGroupProps {
                                variant: ToggleVariant::Outline,
                                ..Default::default()
                            },
                            |ui, ctx| {
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.outline_bold,
                                    lucide_icon(Icon::Bold, 16.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.outline_italic,
                                    lucide_icon(Icon::Italic, 16.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.outline_underline,
                                    lucide_icon(Icon::Underline, 16.0),
                                );
                            },
                        );
                    });

                    // Demo 3: Single Selection
                    example_card(grid, "Single Selection", |ui| {
                        toggle_group(ui, ToggleGroupProps::default(), |ui, ctx| {
                            let mut bold_on = self.single_value == Some(InlineChoice::Bold);
                            if toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut bold_on,
                                lucide_icon(Icon::Bold, 16.0),
                            )
                            .clicked()
                            {
                                self.single_value = if bold_on {
                                    Some(InlineChoice::Bold)
                                } else {
                                    None
                                };
                            }

                            let mut italic_on = self.single_value == Some(InlineChoice::Italic);
                            if toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut italic_on,
                                lucide_icon(Icon::Italic, 16.0),
                            )
                            .clicked()
                            {
                                self.single_value = if italic_on {
                                    Some(InlineChoice::Italic)
                                } else {
                                    None
                                };
                            }

                            let mut underline_on =
                                self.single_value == Some(InlineChoice::Underline);
                            if toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut underline_on,
                                lucide_icon(Icon::Underline, 16.0),
                            )
                            .clicked()
                            {
                                self.single_value = if underline_on {
                                    Some(InlineChoice::Underline)
                                } else {
                                    None
                                };
                            }
                        });
                    });

                    grid.end_row();

                    // Demo 4: Small
                    example_card(grid, "Small", |ui| {
                        toggle_group(
                            ui,
                            ToggleGroupProps {
                                size: ControlSize::Sm,
                                ..Default::default()
                            },
                            |ui, ctx| {
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.sm_bold,
                                    lucide_icon(Icon::Bold, 14.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.sm_italic,
                                    lucide_icon(Icon::Italic, 14.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.sm_underline,
                                    lucide_icon(Icon::Underline, 14.0),
                                );
                            },
                        );
                    });

                    // Demo 5: Large
                    example_card(grid, "Large", |ui| {
                        toggle_group(
                            ui,
                            ToggleGroupProps {
                                size: ControlSize::Lg,
                                ..Default::default()
                            },
                            |ui, ctx| {
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.lg_bold,
                                    lucide_icon(Icon::Bold, 20.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.lg_italic,
                                    lucide_icon(Icon::Italic, 20.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.lg_underline,
                                    lucide_icon(Icon::Underline, 20.0),
                                );
                            },
                        );
                    });

                    // Demo 6: Disabled
                    example_card(grid, "Disabled (Simulated)", |ui| {
                        ui.add_enabled_ui(false, |ui| {
                            toggle_group(ui, ToggleGroupProps::default(), |ui, ctx| {
                                // Note: toggle_group_item enables explicitly via true in impl
                                // I need to update toggle_group_item to take enabled or use ui.is_enabled()
                                // For now, I'll rely on ui.add_enabled_ui wrapper if the component respects it.
                                // But toggle implementation hardcodes `true` for enabled.
                                // I should fix toggle_group_item to respect ui.is_enabled() or take a parameter.
                                // Let's modify toggle_group_item to check ui.is_enabled().

                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.disabled_bold,
                                    lucide_icon(Icon::Bold, 16.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.disabled_italic,
                                    lucide_icon(Icon::Italic, 16.0),
                                );
                                toggle_group_item(
                                    ui,
                                    &self.theme,
                                    ctx,
                                    &mut self.disabled_underline,
                                    lucide_icon(Icon::Underline, 16.0),
                                );
                            });
                        });
                    });

                    // Demo 7: Custom Spacing
                    example_card(grid, "Custom Spacing", |ui| {
                        toggle_group(ui, ToggleGroupProps::default(), |ui, ctx| {
                            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
                            toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut self.multi_bold,
                                lucide_icon(Icon::Bold, 16.0),
                            );
                            toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut self.multi_italic,
                                lucide_icon(Icon::Italic, 16.0),
                            );
                            toggle_group_item(
                                ui,
                                &self.theme,
                                ctx,
                                &mut self.multi_underline,
                                lucide_icon(Icon::Underline, 16.0),
                            );
                        });
                    });

                    grid.end_row();
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Toggle Group example",
        options,
        Box::new(|_cc| Ok(Box::new(ToggleGroupDemo::new()))),
    )
}
