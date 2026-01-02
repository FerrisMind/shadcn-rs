#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, FontData, FontDefinitions, FontFamily, Id, Key};
use egui_shadcn::{
    CommandDialogProps, CommandGroupProps, CommandInputProps, CommandItemProps, KbdProps, Theme,
    command_dialog, command_empty, command_group, command_input, command_item, command_list,
    command_separator, kbd,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct CommandDialogExample {
    theme: Theme,
    open: bool,
}

impl CommandDialogExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            open: false,
        }
    }
}

impl App for CommandDialogExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        ensure_lucide_font(ctx);

        let toggle =
            ctx.input(|i| i.key_pressed(Key::J) && (i.modifiers.command || i.modifiers.ctrl));
        if toggle {
            self.open = !self.open;
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Press");
                kbd(ui, &self.theme, KbdProps::new("⌘J"));
            });

            command_dialog(
                ui,
                &self.theme,
                CommandDialogProps::new(Id::new("command-dialog"), &mut self.open),
                |ui, cmd| {
                    command_input(
                        ui,
                        cmd,
                        CommandInputProps::new("Type a command or search..."),
                    );
                    command_list(ui, cmd, Default::default(), |ui, cmd| {
                        command_empty(ui, cmd, "No results found.");
                        command_group(ui, cmd, CommandGroupProps::new("Suggestions"), |ui, cmd| {
                            command_item(
                                ui,
                                cmd,
                                CommandItemProps::new("calendar", "Calendar").icon(Icon::Calendar),
                            );
                            command_item(
                                ui,
                                cmd,
                                CommandItemProps::new("emoji", "Search Emoji").icon(Icon::Smile),
                            );
                            command_item(
                                ui,
                                cmd,
                                CommandItemProps::new("calculator", "Calculator")
                                    .icon(Icon::Calculator),
                            );
                        });
                        command_separator(ui, cmd);
                        command_group(ui, cmd, CommandGroupProps::new("Settings"), |ui, cmd| {
                            command_item(
                                ui,
                                cmd,
                                CommandItemProps::new("profile", "Profile")
                                    .icon(Icon::User)
                                    .shortcut("⌘P"),
                            );
                            command_item(
                                ui,
                                cmd,
                                CommandItemProps::new("billing", "Billing")
                                    .icon(Icon::CreditCard)
                                    .shortcut("⌘B"),
                            );
                            command_item(
                                ui,
                                cmd,
                                CommandItemProps::new("settings", "Settings")
                                    .icon(Icon::Settings)
                                    .shortcut("⌘S"),
                            );
                        });
                    });
                },
            );
        });
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
        "Command dialog example",
        options,
        Box::new(|_cc| Ok(Box::new(CommandDialogExample::new()))),
    )
}
