use eframe::{App, Frame, NativeOptions, egui};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    ControlSize, ControlVariant, SelectItem, SelectProps, SelectPropsSimple, SelectSize, Textarea,
    TextareaSize, Theme, ToggleVariant, button, checkbox, select, select_with_items, switch,
    toggle,
};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct DemoApp {
    theme: Theme,
    value: String,

    selected_legacy: Option<String>,
    options_legacy: Vec<String>,

    selected_fruit: Option<String>,
    selected_timezone: Option<String>,
    checked: bool,
    switch_on: bool,
    toggle_on: bool,
}

impl DemoApp {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value: "Hello".to_string(),
            selected_legacy: Some("Option A".to_string()),
            options_legacy: vec!["Option A".to_string(), "Option B".to_string()],
            selected_fruit: None,
            selected_timezone: None,
            checked: false,
            switch_on: true,
            toggle_on: false,
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

fn icon_label(icon: Icon, label: &str) -> String {
    format!("{} {}", icon.unicode(), label)
}

impl App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);

        let mut style = ctx.style().as_ref().clone();
        let bg = egui::Color32::from_rgb(10, 10, 10);

        let input_bg = egui::Color32::from_rgb(21, 21, 21);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.extreme_bg_color = input_bg;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui-shadcn Components Demo");
            ui.add_space(16.0);

            ui.label("Button:");
            button(
                ui,
                &self.theme,
                "Button",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
            ui.add_space(12.0);

            ui.label("Text Input:");
            egui_shadcn::Input::new("basic_input")
                .placeholder("Enter text")
                .size(egui_shadcn::InputSize::Size2)
                .invalid(false)
                .enabled(true)
                .show(ui, &self.theme, &mut self.value);
            ui.add_space(12.0);

            ui.label("Select (legacy API):");
            select(
                ui,
                &self.theme,
                SelectPropsSimple {
                    id_source: "legacy_select",
                    selected: &mut self.selected_legacy,
                    options: &self.options_legacy,
                    placeholder: "Select",
                    size: ControlSize::Md,
                    enabled: true,
                    is_invalid: false,
                },
            );
            ui.add_space(12.0);

            ui.label("Select with Groups (shadcn API):");
            let fruit_items = vec![
                SelectItem::group(
                    "Fruits",
                    vec![
                        SelectItem::option("apple", icon_label(Icon::Apple, "Apple")),
                        SelectItem::option("banana", icon_label(Icon::Banana, "Banana")),
                        SelectItem::option("grape", icon_label(Icon::Grape, "Orange")),
                        SelectItem::option_disabled(
                            "mango",
                            icon_label(Icon::Salad, "Mango (out of stock)"),
                        ),
                    ],
                ),
                SelectItem::separator(),
                SelectItem::group(
                    "Vegetables",
                    vec![
                        SelectItem::option("carrot", icon_label(Icon::Carrot, "Carrot")),
                        SelectItem::option("broccoli", icon_label(Icon::LeafyGreen, "Broccoli")),
                    ],
                ),
            ];
            select_with_items(
                ui,
                &self.theme,
                SelectProps::new("fruit_select", &mut self.selected_fruit)
                    .placeholder("Select food...")
                    .size(SelectSize::Default)
                    .width(220.0),
                &fruit_items,
            );
            if let Some(fruit) = &self.selected_fruit {
                ui.label(format!("Selected: {}", fruit));
            }
            ui.add_space(12.0);

            ui.label("Select (size: Sm):");
            let timezone_items = vec![
                SelectItem::label("North America"),
                SelectItem::option("est", "Eastern Standard Time (EST)"),
                SelectItem::option("cst", "Central Standard Time (CST)"),
                SelectItem::option("pst", "Pacific Standard Time (PST)"),
                SelectItem::separator(),
                SelectItem::label("Europe"),
                SelectItem::option("gmt", "Greenwich Mean Time (GMT)"),
                SelectItem::option("cet", "Central European Time (CET)"),
                SelectItem::separator(),
                SelectItem::label("Asia"),
                SelectItem::option("ist", "India Standard Time (IST)"),
                SelectItem::option("jst", "Japan Standard Time (JST)"),
            ];
            select_with_items(
                ui,
                &self.theme,
                SelectProps::new("timezone_select", &mut self.selected_timezone)
                    .placeholder("Select timezone...")
                    .size(SelectSize::Sm)
                    .width(280.0),
                &timezone_items,
            );
            ui.add_space(12.0);

            ui.label("Select (invalid state):");
            let mut invalid_selected: Option<String> = None;
            select_with_items(
                ui,
                &self.theme,
                SelectProps::new("invalid_select", &mut invalid_selected)
                    .placeholder("Required field")
                    .invalid(true)
                    .width(180.0),
                &vec![
                    SelectItem::option("one", "One"),
                    SelectItem::option("two", "Two"),
                ],
            );
            ui.add_space(12.0);

            ui.label("Select (disabled):");
            let mut disabled_selected = Some("locked".to_string());
            select_with_items(
                ui,
                &self.theme,
                SelectProps::new("disabled_select", &mut disabled_selected)
                    .placeholder("Disabled")
                    .enabled(false)
                    .width(180.0),
                &vec![SelectItem::option("locked", "Locked Value")],
            );
            ui.add_space(12.0);

            ui.label("Checkbox:");
            checkbox(
                ui,
                &self.theme,
                &mut self.checked,
                "I agree",
                ControlVariant::Secondary,
                ControlSize::Sm,
                true,
            );
            ui.add_space(12.0);

            ui.label("Toggle:");
            toggle(
                ui,
                &self.theme,
                &mut self.toggle_on,
                "Toggle (button)",
                ToggleVariant::Outline,
                ControlSize::Md,
                true,
            );
            ui.add_space(12.0);

            ui.label("Switch:");
            switch(
                ui,
                &self.theme,
                &mut self.switch_on,
                "Switch (shadcn size)",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            );
            ui.add_space(12.0);

            ui.label("Textarea:");
            Textarea::new("basic-textarea")
                .placeholder("Multiline input")
                .size(TextareaSize::from(ControlSize::Lg))
                .show_counter(true)
                .max_len(120)
                .show(ui, &self.theme, &mut self.value);
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting egui-shadcn example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn demo",
        native_options,
        Box::new(|_cc| Ok(Box::new(DemoApp::new()))),
    ) {
        error!("Failed to run example: {err}");
    }
}
