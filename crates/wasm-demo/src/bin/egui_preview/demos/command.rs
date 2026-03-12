use super::super::app::EguiPreviewApp;
use eframe::egui::{Id, Ui};
use egui_shadcn::{
    CommandDialogProps, CommandGroupProps, CommandInputProps, CommandItemProps, ControlSize,
    ControlVariant, button, command_dialog, command_empty, command_group, command_input,
    command_item, command_list, command_separator,
};
use lucide_icons::Icon;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let open_id = ui.make_persistent_id("preview-command-open");
    let mut open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);

    if button(
        ui,
        &app.theme,
        if compact {
            "Open Command"
        } else {
            "Open Command Palette"
        },
        ControlVariant::Outline,
        ControlSize::Md,
        true,
    )
    .clicked()
    {
        open = true;
    }

    command_dialog(
        ui,
        &app.theme,
        CommandDialogProps::new(Id::new("preview-command-dialog"), &mut open),
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
                });
                command_separator(ui, cmd);
                command_group(ui, cmd, CommandGroupProps::new("Settings"), |ui, cmd| {
                    command_item(
                        ui,
                        cmd,
                        CommandItemProps::new("profile", "Profile")
                            .icon(Icon::User)
                            .shortcut("Ctrl+P"),
                    );
                    command_item(
                        ui,
                        cmd,
                        CommandItemProps::new("billing", "Billing")
                            .icon(Icon::CreditCard)
                            .shortcut("Ctrl+B"),
                    );
                });
            });
        },
    );

    ui.data_mut(|d| d.insert_temp(open_id, open));
}
