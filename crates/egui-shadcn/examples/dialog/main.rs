#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    ControlSize, ControlVariant, DialogAlign, DialogProps, Input, InputSize, Label, Theme, button,
    dialog,
};
use lucide_icons::LUCIDE_FONT_BYTES;

struct DialogDemo {
    theme: Theme,
    dialog_open: bool,
    name_text: String,
    username_text: String,
    share_open: bool,
    share_link: String,
}

impl DialogDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            dialog_open: false,
            name_text: "Pedro Duarte".to_string(),
            username_text: "@peduarte".to_string(),
            share_open: false,
            share_link: "https://ui.shadcn.com/docs/installation".to_string(),
        }
    }
}

impl App for DialogDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        ensure_lucide_font(ctx);
        let theme = &self.theme;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;

                if button(
                    ui,
                    theme,
                    "Open Dialog",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                )
                .clicked()
                {
                    self.dialog_open = true;
                }

                if button(
                    ui,
                    theme,
                    "Share",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                )
                .clicked()
                {
                    self.share_open = true;
                }
            });

            let mut open = self.dialog_open;
            let mut should_close = false;
            let name_text = &mut self.name_text;
            let username_text = &mut self.username_text;

            let _dialog_result = dialog(
                ui,
                theme,
                DialogProps::new(ui.make_persistent_id("edit-profile-dialog"), &mut open)
                    .with_title("Edit profile")
                    .with_description(
                        "Make changes to your profile here. Click save when you're done.",
                    )
                    .with_align(DialogAlign::Center)
                    .with_max_width(425.0)
                    .with_height(280.0)
                    .scrollable(false)
                    .with_scrim_opacity(160)
                    .with_close_on_background(true)
                    .with_close_on_escape(true)
                    .with_animation(true),
                |body_ui| {
                    let input_size = InputSize::Size2;
                    let row_height = input_size.height();
                    let label_width = 80.0;

                    body_ui.add_space(16.0);

                    body_ui.horizontal(|row| {
                        let name_id = row.make_persistent_id("name_input");
                        row.allocate_ui_with_layout(
                            egui::vec2(label_width, row_height),
                            egui::Layout::right_to_left(egui::Align::Center),
                            |label_ui| {
                                Label::new("Name")
                                    .for_id(name_id)
                                    .size(ControlSize::Sm)
                                    .show(label_ui, theme);
                            },
                        );
                        row.add_space(12.0);
                        Input::new(name_id)
                            .size(input_size)
                            .width(row.available_width())
                            .show(row, theme, name_text);
                    });

                    body_ui.add_space(16.0);

                    body_ui.horizontal(|row| {
                        let username_id = row.make_persistent_id("username_input");
                        row.allocate_ui_with_layout(
                            egui::vec2(label_width, row_height),
                            egui::Layout::right_to_left(egui::Align::Center),
                            |label_ui| {
                                Label::new("Username")
                                    .for_id(username_id)
                                    .size(ControlSize::Sm)
                                    .show(label_ui, theme);
                            },
                        );
                        row.add_space(12.0);
                        Input::new(username_id)
                            .size(input_size)
                            .width(row.available_width())
                            .show(row, theme, username_text);
                    });

                    body_ui.add_space(16.0);
                    body_ui.horizontal(|footer_ui| {
                        footer_ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |right_ui| {
                                if button(
                                    right_ui,
                                    theme,
                                    "Save changes",
                                    ControlVariant::Primary,
                                    ControlSize::Md,
                                    true,
                                )
                                .clicked()
                                {
                                    should_close = true;
                                }

                                right_ui.add_space(8.0);

                                if button(
                                    right_ui,
                                    theme,
                                    "Cancel",
                                    ControlVariant::Outline,
                                    ControlSize::Md,
                                    true,
                                )
                                .clicked()
                                {
                                    should_close = true;
                                }
                            },
                        );
                    });
                },
            );

            if should_close {
                open = false;
            }
            self.dialog_open = open;

            let mut share_open = self.share_open;
            let mut share_should_close = false;
            let share_link = &mut self.share_link;

            let _ = dialog(
                ui,
                theme,
                DialogProps::new(ui.make_persistent_id("share-link-dialog"), &mut share_open)
                    .with_title("Share link")
                    .with_description("Anyone who has this link will be able to view this.")
                    .with_align(DialogAlign::Center)
                    .with_max_width(448.0)
                    .with_height(220.0)
                    .scrollable(false)
                    .with_scrim_opacity(160)
                    .with_close_on_background(true)
                    .with_close_on_escape(true)
                    .with_animation(true),
                |body_ui| {
                    body_ui.add_space(12.0);

                    body_ui.horizontal(|row| {
                        let link_id = row.make_persistent_id("share-link-input");
                        Input::new(link_id)
                            .size(InputSize::Size2)
                            .read_only(true)
                            .width(row.available_width())
                            .show(row, theme, share_link);
                    });

                    body_ui.add_space(16.0);

                    body_ui.horizontal(|footer_ui| {
                        let close = button(
                            footer_ui,
                            theme,
                            "Close",
                            ControlVariant::Secondary,
                            ControlSize::Md,
                            true,
                        );
                        if close.clicked() {
                            share_should_close = true;
                        }
                    });
                },
            );

            if share_should_close {
                share_open = false;
            }
            self.share_open = share_open;
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Dialog example",
        options,
        Box::new(|_cc| Ok(Box::new(DialogDemo::new()))),
    )
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
        .entry(FontFamily::Name("lucide".into()))
        .or_default()
        .insert(0, "lucide".into());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}
