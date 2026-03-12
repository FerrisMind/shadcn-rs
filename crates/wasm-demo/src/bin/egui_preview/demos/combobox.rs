use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{ComboboxProps, SelectItem, combobox_with_props};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let value_id = ui.make_persistent_id("preview-combobox-value");
    let search_id = ui.make_persistent_id("preview-combobox-search");
    let mut value = ui
        .data(|d| d.get_temp::<Option<String>>(value_id))
        .unwrap_or(None);
    let mut search = ui
        .data(|d| d.get_temp::<String>(search_id))
        .unwrap_or_default();

    let items = vec![
        SelectItem::option("next.js", "Next.js"),
        SelectItem::option("sveltekit", "SvelteKit"),
        SelectItem::option("nuxt.js", "Nuxt.js"),
        SelectItem::option("remix", "Remix"),
        SelectItem::option("astro", "Astro"),
    ];

    let _ = combobox_with_props(
        ui,
        &app.theme,
        ComboboxProps::new("preview-combobox", &mut value, &items, &mut search)
            .placeholder("Select framework...")
            .search_placeholder("Search framework...")
            .width(if compact { 220.0 } else { 300.0 }),
    );

    ui.data_mut(|d| {
        d.insert_temp(value_id, value);
        d.insert_temp(search_id, search);
    });
}
