#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, RichText, vec2};
use egui_shadcn::{ControlSize, SeparatorProps, Theme, ToggleVariant, separator, toggle};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InlineChoice {
    Bold,
    Italic,
    Strikethrough,
}

struct ToggleDemo {
    theme: Theme,
    bookmark_on: bool,
    italic_on: bool,
    italic_lg_on: bool,
    italic_outline_on: bool,
    italic_with_text_on: bool,
    italic_disabled_on: bool,
    group_multi_bold_on: bool,
    group_multi_italic_on: bool,
    group_multi_strike_on: bool,
    group_disabled_bold_on: bool,
    group_disabled_italic_on: bool,
    group_disabled_strike_on: bool,
    group_outline_bold_on: bool,
    group_outline_italic_on: bool,
    group_outline_strike_on: bool,
    group_lg_bold_on: bool,
    group_lg_italic_on: bool,
    group_lg_strike_on: bool,
    group_single: Option<InlineChoice>,
    group_single_sm: Option<InlineChoice>,
    group_spacing_star_on: bool,
    group_spacing_heart_on: bool,
    group_spacing_bookmark_on: bool,
}

impl ToggleDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            bookmark_on: false,
            italic_on: false,
            italic_lg_on: false,
            italic_outline_on: false,
            italic_with_text_on: false,
            italic_disabled_on: true,
            group_multi_bold_on: false,
            group_multi_italic_on: false,
            group_multi_strike_on: false,
            group_disabled_bold_on: false,
            group_disabled_italic_on: false,
            group_disabled_strike_on: false,
            group_outline_bold_on: false,
            group_outline_italic_on: false,
            group_outline_strike_on: false,
            group_lg_bold_on: false,
            group_lg_italic_on: false,
            group_lg_strike_on: false,
            group_single: None,
            group_single_sm: None,
            group_spacing_star_on: false,
            group_spacing_heart_on: false,
            group_spacing_bookmark_on: false,
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

impl App for ToggleDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(16.0, 16.0);

            let blue = Color32::from_rgb(59, 130, 246);
            let yellow = Color32::from_rgb(234, 179, 8);
            let red = Color32::from_rgb(239, 68, 68);
            let foreground = self.theme.palette.foreground;

            ui.heading("Toggle");
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing = vec2(24.0, 18.0);

                example_card(ui, "Bookmark (outline, sm, text)", |ui| {
                    let label = RichText::new(format!("{} Bookmark", Icon::Bookmark.unicode()))
                        .font(FontId::proportional(14.0))
                        .color(if self.bookmark_on { blue } else { foreground });
                    let _ = toggle(
                        ui,
                        &self.theme,
                        &mut self.bookmark_on,
                        label,
                        ToggleVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .on_hover_text("Toggle bookmark");
                });

                example_card(ui, "Italic (sm)", |ui| {
                    let _ = toggle(
                        ui,
                        &self.theme,
                        &mut self.italic_on,
                        lucide_icon(Icon::Italic, 16.0),
                        ToggleVariant::Default,
                        ControlSize::IconSm,
                        true,
                    )
                    .on_hover_text("Toggle italic");
                });

                example_card(ui, "Italic (lg)", |ui| {
                    let _ = toggle(
                        ui,
                        &self.theme,
                        &mut self.italic_lg_on,
                        lucide_icon(Icon::Italic, 18.0),
                        ToggleVariant::Default,
                        ControlSize::IconLg,
                        true,
                    )
                    .on_hover_text("Toggle italic");
                });

                example_card(ui, "Italic (outline)", |ui| {
                    let _ = toggle(
                        ui,
                        &self.theme,
                        &mut self.italic_outline_on,
                        lucide_icon(Icon::Italic, 16.0),
                        ToggleVariant::Outline,
                        ControlSize::Icon,
                        true,
                    )
                    .on_hover_text("Toggle italic");
                });

                example_card(ui, "Italic with text", |ui| {
                    let with_text = RichText::new(format!("{} Italic", Icon::Italic.unicode()))
                        .font(FontId::proportional(14.0));
                    let _ = toggle(
                        ui,
                        &self.theme,
                        &mut self.italic_with_text_on,
                        with_text,
                        ToggleVariant::Default,
                        ControlSize::Md,
                        true,
                    )
                    .on_hover_text("Toggle italic");
                });

                example_card(ui, "Disabled", |ui| {
                    let _ = toggle(
                        ui,
                        &self.theme,
                        &mut self.italic_disabled_on,
                        lucide_icon(Icon::Underline, 16.0),
                        ToggleVariant::Default,
                        ControlSize::Icon,
                        false,
                    )
                    .on_hover_text("Toggle italic");
                });
            });

            ui.add_space(12.0);
            separator(ui, &self.theme, SeparatorProps::default());
            ui.add_space(12.0);

            ui.heading("Toggle groups");
            ui.add_space(12.0);
            egui::Grid::new("toggle_groups_grid")
                .num_columns(4)
                .spacing(vec2(24.0, 18.0))
                .show(ui, |grid| {
                    example_card(grid, "Group: multiple, outline", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_multi_bold_on,
                                lucide_icon(Icon::Bold, 16.0),
                                ToggleVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_multi_italic_on,
                                lucide_icon(Icon::Italic, 16.0),
                                ToggleVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_multi_strike_on,
                                lucide_icon(Icon::Underline, 16.0),
                                ToggleVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                        });
                    });

                    example_card(grid, "Group: disabled", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_disabled_bold_on,
                                lucide_icon(Icon::Bold, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Md,
                                false,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_disabled_italic_on,
                                lucide_icon(Icon::Italic, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Md,
                                false,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_disabled_strike_on,
                                lucide_icon(Icon::Underline, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Md,
                                false,
                            );
                        });
                    });

                    example_card(grid, "Group: size lg", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_lg_bold_on,
                                lucide_icon(Icon::Bold, 18.0),
                                ToggleVariant::Default,
                                ControlSize::Lg,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_lg_italic_on,
                                lucide_icon(Icon::Italic, 18.0),
                                ToggleVariant::Default,
                                ControlSize::Lg,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_lg_strike_on,
                                lucide_icon(Icon::Underline, 18.0),
                                ToggleVariant::Default,
                                ControlSize::Lg,
                                true,
                            );
                        });
                    });

                    example_card(grid, "Group: outline (multiple)", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_outline_bold_on,
                                lucide_icon(Icon::Bold, 16.0),
                                ToggleVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_outline_italic_on,
                                lucide_icon(Icon::Italic, 16.0),
                                ToggleVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_outline_strike_on,
                                lucide_icon(Icon::Underline, 16.0),
                                ToggleVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                        });
                    });
                    grid.end_row();

                    example_card(grid, "Group: single", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;

                            let mut bold_on = self.group_single == Some(InlineChoice::Bold);
                            let resp = toggle(
                                row,
                                &self.theme,
                                &mut bold_on,
                                lucide_icon(Icon::Bold, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Md,
                                true,
                            );
                            if resp.clicked() {
                                self.group_single = if bold_on {
                                    Some(InlineChoice::Bold)
                                } else {
                                    None
                                };
                            }

                            let mut italic_on = self.group_single == Some(InlineChoice::Italic);
                            let resp = toggle(
                                row,
                                &self.theme,
                                &mut italic_on,
                                lucide_icon(Icon::Italic, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Md,
                                true,
                            );
                            if resp.clicked() {
                                self.group_single = if italic_on {
                                    Some(InlineChoice::Italic)
                                } else {
                                    None
                                };
                            }

                            let mut strike_on = self.group_single == Some(InlineChoice::Strikethrough);
                            let resp = toggle(
                                row,
                                &self.theme,
                                &mut strike_on,
                                lucide_icon(Icon::Underline, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Md,
                                true,
                            );
                            if resp.clicked() {
                                self.group_single = if strike_on {
                                    Some(InlineChoice::Strikethrough)
                                } else {
                                    None
                                };
                            }
                        });
                    });

                    example_card(grid, "Group: single, sm", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;

                            let mut bold_on = self.group_single_sm == Some(InlineChoice::Bold);
                            let resp = toggle(
                                row,
                                &self.theme,
                                &mut bold_on,
                                lucide_icon(Icon::Bold, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Sm,
                                true,
                            );
                            if resp.clicked() {
                                self.group_single_sm = if bold_on {
                                    Some(InlineChoice::Bold)
                                } else {
                                    None
                                };
                            }

                            let mut italic_on = self.group_single_sm == Some(InlineChoice::Italic);
                            let resp = toggle(
                                row,
                                &self.theme,
                                &mut italic_on,
                                lucide_icon(Icon::Italic, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Sm,
                                true,
                            );
                            if resp.clicked() {
                                self.group_single_sm = if italic_on {
                                    Some(InlineChoice::Italic)
                                } else {
                                    None
                                };
                            }

                            let mut strike_on = self.group_single_sm == Some(InlineChoice::Strikethrough);
                            let resp = toggle(
                                row,
                                &self.theme,
                                &mut strike_on,
                                lucide_icon(Icon::Underline, 16.0),
                                ToggleVariant::Default,
                                ControlSize::Sm,
                                true,
                            );
                            if resp.clicked() {
                                self.group_single_sm = if strike_on {
                                    Some(InlineChoice::Strikethrough)
                                } else {
                                    None
                                };
                            }
                        });
                    });

                    example_card(grid, "Group: spacing, outline, sm", |ui| {
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            let star_color = if self.group_spacing_star_on {
                                yellow
                            } else {
                                foreground
                            };
                            let heart_color = if self.group_spacing_heart_on {
                                red
                            } else {
                                foreground
                            };
                            let bookmark_color = if self.group_spacing_bookmark_on {
                                blue
                            } else {
                                foreground
                            };

                            let star_text = RichText::new(format!("{} Star", Icon::Star.unicode()))
                                .color(star_color)
                                .font(FontId::proportional(14.0));
                            let heart_text = RichText::new(format!("{} Heart", Icon::Heart.unicode()))
                                .color(heart_color)
                                .font(FontId::proportional(14.0));
                            let bookmark_text =
                                RichText::new(format!("{} Bookmark", Icon::Bookmark.unicode()))
                                    .color(bookmark_color)
                                    .font(FontId::proportional(14.0));

                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_spacing_star_on,
                                star_text,
                                ToggleVariant::Outline,
                                ControlSize::Sm,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_spacing_heart_on,
                                heart_text,
                                ToggleVariant::Outline,
                                ControlSize::Sm,
                                true,
                            );
                            let _ = toggle(
                                row,
                                &self.theme,
                                &mut self.group_spacing_bookmark_on,
                                bookmark_text,
                                ToggleVariant::Outline,
                                ControlSize::Sm,
                                true,
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
        "Toggle example",
        options,
        Box::new(|_cc| Ok(Box::new(ToggleDemo::new()))),
    )
}
