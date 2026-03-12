use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{SelectItem, SelectProps, select_with_items};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let items = vec![
        SelectItem::group(
            "Languages",
            vec![
                SelectItem::option("rust", "Rust"),
                SelectItem::option("ts", "TypeScript"),
                SelectItem::option("go", "Go"),
                SelectItem::option("python", "Python"),
            ],
        ),
        SelectItem::separator(),
        SelectItem::option("other", "Other"),
    ];

    select_with_items(
        ui,
        &app.theme,
        SelectProps::new("preview-select", &mut app.select_value)
            .placeholder("Select language")
            .width(if compact { 220.0 } else { 320.0 }),
        &items,
    );
}
