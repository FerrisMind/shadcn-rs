use eframe::{App, Frame, NativeOptions, egui};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    ControlSize, SelectItem, SelectProps, SelectPropsSimple, SelectSize, SelectStyle, Theme,
    select, select_with_items,
};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct SelectDemo {
    theme: Theme,
    legacy_selected: Option<String>,
    legacy_options: Vec<String>,
    grouped_selected: Option<String>,
    grouped_items: Vec<SelectItem>,
    sm_selected: Option<String>,
    sm_items: Vec<SelectItem>,
    invalid_selected: Option<String>,
    disabled_selected: Option<String>,
    custom_selected: Option<String>,
    custom_items: Vec<SelectItem>,
    scroll_selected: Option<String>,
}

impl SelectDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            legacy_selected: Some("Option A".to_string()),
            legacy_options: vec!["Option A".into(), "Option B".into(), "Option C".into()],
            grouped_selected: None,
            grouped_items: vec![
                SelectItem::group(
                    "Fruits",
                    vec![
                        SelectItem::option("apple", icon_label(Icon::Apple, "Apple")),
                        SelectItem::option("banana", icon_label(Icon::Banana, "Banana")),
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
                        SelectItem::option("pepper", icon_label(Icon::Flame, "Pepper")),
                    ],
                ),
            ],
            sm_selected: None,
            sm_items: vec![
                SelectItem::label("Timezones"),
                SelectItem::option("est", "Eastern (EST)"),
                SelectItem::option("pst", "Pacific (PST)"),
                SelectItem::option("cet", "Central Europe (CET)"),
                SelectItem::separator(),
                SelectItem::option("ist", "India (IST)"),
                SelectItem::option("jst", "Japan (JST)"),
            ],
            invalid_selected: None,
            disabled_selected: Some("locked".to_string()),
            custom_selected: None,
            custom_items: vec![
                SelectItem::option("rust", icon_label(Icon::Code, "Rust")),
                SelectItem::option("go", icon_label(Icon::Code, "Go")),
                SelectItem::option("ts", icon_label(Icon::Braces, "TypeScript")),
                SelectItem::option("python", icon_label(Icon::Braces, "Python")),
            ],
            scroll_selected: None,
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

fn apply_dark_background(ctx: &egui::Context) {
    let mut style = ctx.style().as_ref().clone();
    let bg = egui::Color32::from_rgb(10, 10, 10);
    let input_bg = egui::Color32::from_rgb(21, 21, 21);
    style.visuals.window_fill = bg;
    style.visuals.panel_fill = bg;
    style.visuals.extreme_bg_color = input_bg;
    ctx.set_style(style);
}

fn custom_style(theme: &Theme) -> SelectStyle {
    let mut style = SelectStyle::from_palette(&theme.palette);
    style.trigger_bg = egui::Color32::from_rgb(24, 24, 32);
    style.trigger_bg_hover = egui::Color32::from_rgb(34, 34, 44);
    style.trigger_border = theme.palette.accent;
    style.trigger_icon = theme.palette.accent_foreground;
    style.content_bg = egui::Color32::from_rgb(16, 16, 24);
    style.item_bg_hover = egui::Color32::from_rgb(40, 40, 52);
    style
}

impl App for SelectDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        apply_dark_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.heading("Select — legacy API");
                    select(
                        ui,
                        &self.theme,
                        SelectPropsSimple {
                            id_source: "legacy",
                            selected: &mut self.legacy_selected,
                            options: &self.legacy_options,
                            placeholder: "Select an option",
                            size: ControlSize::Md,
                            enabled: true,
                            is_invalid: false,
                        },
                    );
                    if let Some(value) = &self.legacy_selected {
                        ui.label(format!("legacy selected: {value}"));
                    }
                    ui.add_space(12.0);

                    ui.heading("Select — Groups, Disabled and Icons");
                    select_with_items(
                        ui,
                        &self.theme,
                        SelectProps::new("grouped", &mut self.grouped_selected)
                            .placeholder("Categories with groups")
                            .width(240.0),
                        &self.grouped_items,
                    );
                    if let Some(value) = &self.grouped_selected {
                        ui.label(format!("grouped selected: {value}"));
                    }
                    ui.add_space(12.0);

                    ui.heading("Select — Size Sm");
                    select_with_items(
                        ui,
                        &self.theme,
                        SelectProps::new("small", &mut self.sm_selected)
                            .placeholder("Compact select")
                            .size(SelectSize::Sm)
                            .width(260.0),
                        &self.sm_items,
                    );
                    ui.add_space(12.0);

                    ui.heading("Select — Invalid and Disabled");
                    select_with_items(
                        ui,
                        &self.theme,
                        SelectProps::new("invalid", &mut self.invalid_selected)
                            .placeholder("Required field")
                            .invalid(true)
                            .width(200.0),
                        &[
                            SelectItem::option("one", "One"),
                            SelectItem::option("two", "Two"),
                        ],
                    );
                    select_with_items(
                        ui,
                        &self.theme,
                        SelectProps::new("disabled", &mut self.disabled_selected)
                            .placeholder("Disabled select")
                            .enabled(false)
                            .width(200.0),
                        &[SelectItem::option("locked", "Locked")],
                    );
                    ui.add_space(12.0);

                    ui.heading("Select — Custom Style");
                    let style = custom_style(&self.theme);
                    select_with_items(
                        ui,
                        &self.theme,
                        SelectProps::new("custom", &mut self.custom_selected)
                            .placeholder("Custom bg and border")
                            .width(260.0)
                            .style(style),
                        &self.custom_items,
                    );
                    if let Some(value) = &self.custom_selected {
                        ui.label(format!("custom selected: {value}"));
                    }
                    ui.add_space(12.0);

                    ui.heading("Select — scrollable (timezones)");
                    let scroll_items = vec![
                        SelectItem::group(
                            "North America",
                            vec![
                                SelectItem::option("est", "Eastern Standard Time (EST)"),
                                SelectItem::option("cst", "Central Standard Time (CST)"),
                                SelectItem::option("mst", "Mountain Standard Time (MST)"),
                                SelectItem::option("pst", "Pacific Standard Time (PST)"),
                                SelectItem::option("akst", "Alaska Standard Time (AKST)"),
                                SelectItem::option("hst", "Hawaii Standard Time (HST)"),
                            ],
                        ),
                        SelectItem::group(
                            "Europe & Africa",
                            vec![
                                SelectItem::option("gmt", "Greenwich Mean Time (GMT)"),
                                SelectItem::option("cet", "Central European Time (CET)"),
                                SelectItem::option("eet", "Eastern European Time (EET)"),
                                SelectItem::option("west", "Western European Summer Time (WEST)"),
                                SelectItem::option("cat", "Central Africa Time (CAT)"),
                                SelectItem::option("eat", "East Africa Time (EAT)"),
                            ],
                        ),
                        SelectItem::group(
                            "Asia",
                            vec![
                                SelectItem::option("msk", "Moscow Time (MSK)"),
                                SelectItem::option("ist", "India Standard Time (IST)"),
                                SelectItem::option("cst_china", "China Standard Time (CST)"),
                                SelectItem::option("jst", "Japan Standard Time (JST)"),
                                SelectItem::option("kst", "Korea Standard Time (KST)"),
                                SelectItem::option(
                                    "wita",
                                    "Indonesia Central Standard Time (WITA)",
                                ),
                            ],
                        ),
                        SelectItem::group(
                            "Australia & Pacific",
                            vec![
                                SelectItem::option(
                                    "awst",
                                    "Australian Western Standard Time (AWST)",
                                ),
                                SelectItem::option(
                                    "acst",
                                    "Australian Central Standard Time (ACST)",
                                ),
                                SelectItem::option(
                                    "aest",
                                    "Australian Eastern Standard Time (AEST)",
                                ),
                                SelectItem::option("nzst", "New Zealand Standard Time (NZST)"),
                                SelectItem::option("fjt", "Fiji Time (FJT)"),
                            ],
                        ),
                        SelectItem::group(
                            "South America",
                            vec![
                                SelectItem::option("art", "Argentina Time (ART)"),
                                SelectItem::option("bot", "Bolivia Time (BOT)"),
                                SelectItem::option("brt", "Brasilia Time (BRT)"),
                                SelectItem::option("clt", "Chile Standard Time (CLT)"),
                            ],
                        ),
                    ];
                    select_with_items(
                        ui,
                        &self.theme,
                        SelectProps::new("scrollable_select", &mut self.scroll_selected)
                            .placeholder("Select a timezone")
                            .width(280.0),
                        &scroll_items,
                    );
                    if let Some(val) = &self.scroll_selected {
                        ui.label(format!("scrollable selected: {val}"));
                    }
                });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting select example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — select",
        native_options,
        Box::new(|_cc| Ok(Box::new(SelectDemo::new()))),
    ) {
        error!("Failed to run select example: {err}");
    }
}
