#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, FontData, FontDefinitions, FontFamily, RichText, vec2};
use egui_shadcn::{
    ControlSize, ControlVariant, Input, InputSize, Label, SheetProps, SheetSide, Theme, button,
    sheet, sheet_content, sheet_description, sheet_footer, sheet_header, sheet_title,
    sheet_trigger,
};
use lucide_icons::LUCIDE_FONT_BYTES;

struct SheetExample {
    theme: Theme,
    demo_open: bool,
    demo_name: String,
    demo_username: String,
    side_open: [bool; 4],
    side_names: [String; 4],
    side_usernames: [String; 4],
}

impl SheetExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            demo_open: false,
            demo_name: "Pedro Duarte".to_string(),
            demo_username: "@peduarte".to_string(),
            side_open: [false; 4],
            side_names: std::array::from_fn(|_| "Pedro Duarte".to_string()),
            side_usernames: std::array::from_fn(|_| "@peduarte".to_string()),
        }
    }
}

impl App for SheetExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        ensure_lucide_font(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sheet");
            ui.label("Slide-in panels from each side of the screen.");
            ui.add_space(16.0);

            ui.spacing_mut().item_spacing.y = 20.0;

            render_section(
                ui,
                &self.theme,
                "Sheet demo",
                "Basic sheet with form fields.",
            );
            render_sheet_demo(
                ui,
                &self.theme,
                &mut self.demo_open,
                &mut self.demo_name,
                &mut self.demo_username,
            );

            render_section(ui, &self.theme, "Sheet side", "Open sheet from each side.");
            render_sheet_side(
                ui,
                &self.theme,
                &mut self.side_open,
                &mut self.side_names,
                &mut self.side_usernames,
            );
        });
    }
}

fn render_section(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.label(
            RichText::new(description)
                .color(theme.palette.muted_foreground)
                .size(12.0),
        );
        ui.add_space(8.0);
    });
}

fn render_sheet_demo(
    ui: &mut egui::Ui,
    theme: &Theme,
    open: &mut bool,
    name: &mut String,
    username: &mut String,
) {
    sheet(
        ui,
        SheetProps::new(ui.make_persistent_id("sheet-demo"), open),
        |ui, ctx| {
            let _ = sheet_trigger(ui, ctx, |ui| {
                button(
                    ui,
                    theme,
                    "Open",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                )
            });

            let mut should_close = false;
            let _ = sheet_content(ui, theme, ctx, |content_ui| {
                sheet_header(content_ui, |header_ui| {
                    sheet_title(header_ui, theme, "Edit profile");
                    sheet_description(
                        header_ui,
                        theme,
                        "Make changes to your profile here. Click save when you're done.",
                    );
                });

                content_ui
                    .vertical(|body_ui| {
                        body_ui.spacing_mut().item_spacing.y = 16.0;
                        body_ui.horizontal(|row| {
                            let label_width = 80.0;
                            let input_size = InputSize::Size2;
                            let name_id = row.make_persistent_id("sheet-demo-name");
                            row.allocate_ui_with_layout(
                                vec2(label_width, input_size.height()),
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
                                .show(row, theme, name);
                        });
                        body_ui.horizontal(|row| {
                            let label_width = 80.0;
                            let input_size = InputSize::Size2;
                            let username_id = row.make_persistent_id("sheet-demo-username");
                            row.allocate_ui_with_layout(
                                vec2(label_width, input_size.height()),
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
                                .show(row, theme, username);
                        });
                    });

                sheet_footer(content_ui, |footer_ui| {
                    footer_ui.horizontal(|row| {
                        if button(
                            row,
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
                        if button(
                            row,
                            theme,
                            "Close",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            true,
                        )
                        .clicked()
                        {
                            should_close = true;
                        }
                    });
                });
            });

            if should_close {
                *ctx.open = false;
            }
        },
    );
}

fn render_sheet_side(
    ui: &mut egui::Ui,
    theme: &Theme,
    open: &mut [bool; 4],
    names: &mut [String; 4],
    usernames: &mut [String; 4],
) {
    let sides = [
        (SheetSide::Top, "top"),
        (SheetSide::Right, "right"),
        (SheetSide::Bottom, "bottom"),
        (SheetSide::Left, "left"),
    ];

    egui::Grid::new("sheet-side-grid")
        .num_columns(2)
        .spacing(vec2(8.0, 8.0))
        .show(ui, |grid| {
            for (idx, (side, label)) in sides.iter().enumerate() {
                sheet(
                    grid,
                    SheetProps::new(grid.make_persistent_id(("sheet-side", label)), &mut open[idx])
                        .side(*side),
                    |ui, ctx| {
                        let _ = sheet_trigger(ui, ctx, |ui| {
                            button(
                                ui,
                                theme,
                                *label,
                                ControlVariant::Outline,
                                ControlSize::Md,
                                true,
                            )
                        });

                        let mut should_close = false;
                        let _ = sheet_content(ui, theme, ctx, |content_ui| {
                            sheet_header(content_ui, |header_ui| {
                                sheet_title(header_ui, theme, "Edit profile");
                                sheet_description(
                                    header_ui,
                                    theme,
                                    "Make changes to your profile here. Click save when you're done.",
                                );
                            });

                            content_ui
                                .vertical(|body_ui| {
                                    body_ui.spacing_mut().item_spacing.y = 12.0;
                                    let input_size = InputSize::Size2;
                                    let name_id =
                                        body_ui.make_persistent_id(("sheet-side-name", idx));
                                    Label::new("Name")
                                        .for_id(name_id)
                                        .size(ControlSize::Sm)
                                        .show(body_ui, theme);
                                    Input::new(name_id)
                                        .size(input_size)
                                        .width(body_ui.available_width())
                                        .show(body_ui, theme, &mut names[idx]);

                                    let username_id =
                                        body_ui.make_persistent_id(("sheet-side-username", idx));
                                    Label::new("Username")
                                        .for_id(username_id)
                                        .size(ControlSize::Sm)
                                        .show(body_ui, theme);
                                    Input::new(username_id)
                                        .size(input_size)
                                        .width(body_ui.available_width())
                                        .show(body_ui, theme, &mut usernames[idx]);
                                });

                            sheet_footer(content_ui, |footer_ui| {
                                if button(
                                    footer_ui,
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
                            });
                        });

                        if should_close {
                            *ctx.open = false;
                        }
                    },
                );

                if idx % 2 == 1 {
                    grid.end_row();
                }
            }
        });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Sheet example",
        options,
        Box::new(|_cc| Ok(Box::new(SheetExample::new()))),
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
