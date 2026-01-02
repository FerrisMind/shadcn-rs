#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    ControlSize, ControlVariant, DialogAlign, DialogProps, DropdownMenuCheckboxItemProps,
    DropdownMenuItemProps, DropdownMenuItemVariant, DropdownMenuLabelProps, DropdownMenuProps,
    DropdownMenuRadioItemProps, DropdownMenuSubProps, DropdownMenuTriggerProps, Theme, button,
    dialog, dropdown_menu, dropdown_menu_checkbox_item, dropdown_menu_item, dropdown_menu_label,
    dropdown_menu_radio_group, dropdown_menu_radio_item, dropdown_menu_separator,
    dropdown_menu_sub, dropdown_menu_trigger,
};
use lucide_icons::LUCIDE_FONT_BYTES;

struct DropdownMenuExample {
    theme: Theme,
    show_status_bar: bool,
    show_activity_bar: bool,
    show_panel: bool,
    selected_profile: String,
    dialog_open: bool,
}

impl DropdownMenuExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            show_status_bar: true,
            show_activity_bar: false,
            show_panel: true,
            selected_profile: "profile".to_string(),
            dialog_open: false,
        }
    }
}

impl App for DropdownMenuExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        ensure_lucide_font(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dropdown Menu");
            ui.add_space(16.0);

            ui.spacing_mut().item_spacing.y = 24.0;

            ui.vertical(|ui| {
                render_section_title(ui, &self.theme, "Dropdown menu demo", "Basic menu items.");
                render_demo_menu(ui, &self.theme);
            });

            ui.vertical(|ui| {
                render_section_title(
                    ui,
                    &self.theme,
                    "Dropdown menu checkboxes",
                    "Toggle options from the menu.",
                );
                render_checkbox_menu(
                    ui,
                    &self.theme,
                    &mut self.show_status_bar,
                    &mut self.show_activity_bar,
                    &mut self.show_panel,
                );
            });

            ui.vertical(|ui| {
                render_section_title(
                    ui,
                    &self.theme,
                    "Dropdown menu radio group",
                    "Pick a single profile.",
                );
                render_radio_menu(ui, &self.theme, &mut self.selected_profile);
            });

            ui.vertical(|ui| {
                render_section_title(
                    ui,
                    &self.theme,
                    "Dropdown menu dialog",
                    "Open a dialog from a menu item.",
                );
                render_dialog_menu(ui, &self.theme, &mut self.dialog_open);
            });

            let mut dialog_open = self.dialog_open;
            let mut should_close = false;
            let _dialog_response = dialog(
                ui,
                &self.theme,
                DialogProps::new(ui.make_persistent_id("dropdown-dialog"), &mut dialog_open)
                    .with_title("Menu dialog")
                    .with_description("Opened from a dropdown menu item.")
                    .with_align(DialogAlign::Center)
                    .with_max_width(420.0)
                    .scrollable(false),
                |body| {
                    body.label("This dialog was opened from the menu.");
                    body.add_space(12.0);
                    if button(
                        body,
                        &self.theme,
                        "Close",
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        should_close = true;
                        body.close();
                    }
                },
            );
            if should_close {
                dialog_open = false;
            }
            self.dialog_open = dialog_open;
        });
    }
}

fn render_section_title(ui: &mut egui::Ui, theme: &Theme, title: &str, subtitle: &str) {
    ui.label(egui::RichText::new(title).size(14.0).strong());
    ui.label(
        egui::RichText::new(subtitle)
            .size(12.0)
            .color(theme.palette.muted_foreground),
    );
    ui.add_space(8.0);
}

fn render_demo_menu(ui: &mut egui::Ui, theme: &Theme) {
    let trigger = dropdown_menu_trigger(
        ui,
        DropdownMenuTriggerProps::new(ui.make_persistent_id("dropdown-demo-trigger")),
        |ui| {
            button(
                ui,
                theme,
                "Open menu",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            )
        },
    );

    let _ = dropdown_menu(
        ui,
        theme,
        DropdownMenuProps::new(&trigger.response),
        |menu_ui| {
            if dropdown_menu_item(
                menu_ui,
                theme,
                DropdownMenuItemProps::new("Back")
                    .with_shortcut("Ctrl+[")
                    .inset(true),
            )
            .clicked()
            {
                menu_ui.close();
            }

            if dropdown_menu_item(
                menu_ui,
                theme,
                DropdownMenuItemProps::new("Forward")
                    .with_shortcut("Ctrl+]")
                    .inset(true)
                    .disabled(true),
            )
            .clicked()
            {
                menu_ui.close();
            }

            if dropdown_menu_item(
                menu_ui,
                theme,
                DropdownMenuItemProps::new("Reload")
                    .with_shortcut("Ctrl+R")
                    .inset(true),
            )
            .clicked()
            {
                menu_ui.close();
            }

            dropdown_menu_separator(menu_ui, theme);

            dropdown_menu_label(
                menu_ui,
                theme,
                DropdownMenuLabelProps::new("More tools").inset(true),
            );
            dropdown_menu_sub(
                menu_ui,
                theme,
                DropdownMenuSubProps::new("Developer Tools").inset(true),
                |submenu_ui| {
                    if dropdown_menu_item(
                        submenu_ui,
                        theme,
                        DropdownMenuItemProps::new("View source"),
                    )
                    .clicked()
                    {
                        submenu_ui.close();
                    }
                    if dropdown_menu_item(
                        submenu_ui,
                        theme,
                        DropdownMenuItemProps::new("Extensions"),
                    )
                    .clicked()
                    {
                        submenu_ui.close();
                    }
                    dropdown_menu_separator(submenu_ui, theme);
                    if dropdown_menu_item(
                        submenu_ui,
                        theme,
                        DropdownMenuItemProps::new("Delete")
                            .with_variant(DropdownMenuItemVariant::Destructive),
                    )
                    .clicked()
                    {
                        submenu_ui.close();
                    }
                },
            );

            dropdown_menu_separator(menu_ui, theme);
            if dropdown_menu_item(
                menu_ui,
                theme,
                DropdownMenuItemProps::new("Delete")
                    .with_variant(DropdownMenuItemVariant::Destructive),
            )
            .clicked()
            {
                menu_ui.close();
            }
        },
    );
}

fn render_checkbox_menu(
    ui: &mut egui::Ui,
    theme: &Theme,
    show_status_bar: &mut bool,
    show_activity_bar: &mut bool,
    show_panel: &mut bool,
) {
    let trigger = dropdown_menu_trigger(
        ui,
        DropdownMenuTriggerProps::new(ui.make_persistent_id("dropdown-checkbox-trigger")),
        |ui| {
            button(
                ui,
                theme,
                "View",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            )
        },
    );

    let _ = dropdown_menu(
        ui,
        theme,
        DropdownMenuProps::new(&trigger.response),
        |menu_ui| {
            dropdown_menu_label(
                menu_ui,
                theme,
                DropdownMenuLabelProps::new("Appearance").inset(true),
            );

            if dropdown_menu_checkbox_item(
                menu_ui,
                theme,
                DropdownMenuCheckboxItemProps::new("Show status bar", *show_status_bar),
            )
            .clicked()
            {
                *show_status_bar = !*show_status_bar;
            }

            if dropdown_menu_checkbox_item(
                menu_ui,
                theme,
                DropdownMenuCheckboxItemProps::new("Show activity bar", *show_activity_bar),
            )
            .clicked()
            {
                *show_activity_bar = !*show_activity_bar;
            }

            if dropdown_menu_checkbox_item(
                menu_ui,
                theme,
                DropdownMenuCheckboxItemProps::new("Show panel", *show_panel),
            )
            .clicked()
            {
                *show_panel = !*show_panel;
            }
        },
    );
}

fn render_radio_menu(ui: &mut egui::Ui, theme: &Theme, selected_profile: &mut String) {
    let trigger = dropdown_menu_trigger(
        ui,
        DropdownMenuTriggerProps::new(ui.make_persistent_id("dropdown-radio-trigger")),
        |ui| {
            button(
                ui,
                theme,
                "Profile",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            )
        },
    );

    let current = selected_profile.clone();
    let _ = dropdown_menu(
        ui,
        theme,
        DropdownMenuProps::new(&trigger.response),
        |menu_ui| {
            dropdown_menu_label(
                menu_ui,
                theme,
                DropdownMenuLabelProps::new("Profile").inset(true),
            );
            dropdown_menu_radio_group(menu_ui, theme, current.as_str(), |menu_ui, current| {
                if dropdown_menu_radio_item(
                    menu_ui,
                    theme,
                    DropdownMenuRadioItemProps::new("Personal", "profile", current),
                )
                .clicked()
                {
                    *selected_profile = "profile".to_string();
                }
                if dropdown_menu_radio_item(
                    menu_ui,
                    theme,
                    DropdownMenuRadioItemProps::new("Work", "work", current),
                )
                .clicked()
                {
                    *selected_profile = "work".to_string();
                }
                if dropdown_menu_radio_item(
                    menu_ui,
                    theme,
                    DropdownMenuRadioItemProps::new("Team", "team", current),
                )
                .clicked()
                {
                    *selected_profile = "team".to_string();
                }
            });
        },
    );
}

fn render_dialog_menu(ui: &mut egui::Ui, theme: &Theme, dialog_open: &mut bool) {
    let trigger = dropdown_menu_trigger(
        ui,
        DropdownMenuTriggerProps::new(ui.make_persistent_id("dropdown-dialog-trigger")),
        |ui| {
            button(
                ui,
                theme,
                "Actions",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            )
        },
    );

    let _ = dropdown_menu(
        ui,
        theme,
        DropdownMenuProps::new(&trigger.response),
        |menu_ui| {
            if dropdown_menu_item(menu_ui, theme, DropdownMenuItemProps::new("Open dialog"))
                .clicked()
            {
                *dialog_open = true;
                menu_ui.close();
            }

            dropdown_menu_separator(menu_ui, theme);

            if dropdown_menu_item(
                menu_ui,
                theme,
                DropdownMenuItemProps::new("Delete")
                    .with_variant(DropdownMenuItemVariant::Destructive),
            )
            .clicked()
            {
                menu_ui.close();
            }
        },
    );
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

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Dropdown Menu example",
        options,
        Box::new(|_cc| Ok(Box::new(DropdownMenuExample::new()))),
    )
}
