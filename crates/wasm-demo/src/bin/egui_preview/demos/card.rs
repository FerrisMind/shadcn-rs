use super::super::app::EguiPreviewApp;
use eframe::egui::{RichText, Ui};
use egui_shadcn::{
    CardProps, CardVariant, ControlSize, ControlVariant, Input, InputSize, InputType, Label,
    SelectItem, SelectProps, button, card, select_with_items,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let width = if compact { 260.0 } else { 360.0 };
    let framework_items = vec![
        SelectItem::option("next", "Next.js"),
        SelectItem::option("remix", "Remix"),
        SelectItem::option("astro", "Astro"),
    ];

    card(
        ui,
        &app.theme,
        CardProps::default().variant(CardVariant::Outline),
        |ui| {
            ui.set_max_width(width);
            ui.vertical(|ui| {
                ui.label(RichText::new("Create project").strong());
                ui.label(
                    RichText::new("Deploy your new project in one click.")
                        .color(app.theme.palette.muted_foreground),
                );
                ui.add_space(8.0);

                let name_id = ui.make_persistent_id("preview-card-project");
                Label::new("Name").for_id(name_id).show(ui, &app.theme);
                Input::new(name_id)
                    .size(InputSize::Size2)
                    .input_type(InputType::Text)
                    .placeholder("my-app")
                    .width(width - 32.0)
                    .show(ui, &app.theme, &mut app.project_name);

                ui.add_space(8.0);
                let mail_id = ui.make_persistent_id("preview-card-email");
                Label::new("Email").for_id(mail_id).show(ui, &app.theme);
                Input::new(mail_id)
                    .size(InputSize::Size2)
                    .input_type(InputType::Email)
                    .placeholder("you@example.com")
                    .width(width - 32.0)
                    .show(ui, &app.theme, &mut app.email);

                ui.add_space(8.0);
                let fw_id = ui.make_persistent_id("preview-card-framework");
                Label::new("Framework").for_id(fw_id).show(ui, &app.theme);
                select_with_items(
                    ui,
                    &app.theme,
                    SelectProps::new(fw_id, &mut app.framework)
                        .placeholder("Select")
                        .width(width - 32.0),
                    &framework_items,
                );

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    let _ = button(
                        ui,
                        &app.theme,
                        "Cancel",
                        ControlVariant::Outline,
                        ControlSize::Md,
                        true,
                    );
                    let _ = button(
                        ui,
                        &app.theme,
                        "Deploy",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });
            });
        },
    );
}
