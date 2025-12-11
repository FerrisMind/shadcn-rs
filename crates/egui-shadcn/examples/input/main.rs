use eframe::{App, Frame, NativeOptions, egui};
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Painter, Rect};
use egui_shadcn::{ColorPalette, ControlSize, ControlVariant, Input, InputRadius, InputSize, InputType, InputVariant as Variant, Theme, switch};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct InputDemo {
    theme: Theme,
    dark_mode: bool,

    basic_text: String,
    email_text: String,
    password_text: String,
    search_text: String,

    surface_text: String,
    classic_text: String,
    soft_text: String,

    size1_text: String,
    size2_text: String,
    size3_text: String,

    invalid_text: String,
    disabled_text: String,
    readonly_text: String,

    with_icon_text: String,
    with_both_icons_text: String,

    limited_text: String,
    accent_text: String,
}

impl InputDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            dark_mode: true,
            basic_text: String::new(),
            email_text: String::new(),
            password_text: String::new(),
            search_text: String::new(),
            surface_text: String::new(),
            classic_text: String::new(),
            soft_text: String::new(),
            size1_text: String::new(),
            size2_text: String::new(),
            size3_text: String::new(),
            invalid_text: String::new(),
            disabled_text: "Cannot edit this".into(),
            readonly_text: "Read-only value".into(),
            with_icon_text: String::new(),
            with_both_icons_text: String::new(),
            limited_text: String::new(),
            accent_text: String::new(),
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

fn apply_background(ctx: &egui::Context, dark_mode: bool) {
    let mut style = ctx.style().as_ref().clone();
    if dark_mode {
        let bg = egui::Color32::from_rgb(10, 10, 10);
        let input_bg = egui::Color32::from_rgb(21, 21, 21);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.extreme_bg_color = input_bg;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(249, 249, 249));
    } else {
        let bg = egui::Color32::from_rgb(255, 255, 255);
        let input_bg = egui::Color32::from_rgb(245, 245, 245);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.extreme_bg_color = input_bg;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(37, 37, 37));
    }
    ctx.set_style(style);
}

fn draw_lucide_icon(painter: &Painter, rect: Rect, color: Color32, icon: Icon) {
    let size = rect.width().min(rect.height());

    let font_id = FontId::new(size, FontFamily::Proportional);
    let galley = painter.layout_no_wrap(icon.unicode().to_string(), font_id, color);
    let pos = rect.center() - galley.rect.center().to_vec2();
    painter.galley(pos, galley, color);
}

fn draw_search_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::Search);
}

fn draw_mail_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::Mail);
}

fn draw_lock_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::Lock);
}

fn draw_user_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::User);
}

fn draw_x_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::X);
}

fn draw_eye_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::Eye);
}

fn draw_at_sign_icon(painter: &Painter, rect: Rect, color: Color32) {
    draw_lucide_icon(painter, rect, color, Icon::AtSign);
}

impl App for InputDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        apply_background(ctx, self.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 16.0;

                ui.horizontal(|ui| {
                    ui.heading("Theme:");
                    let prev_dark = self.dark_mode;
                    let icon = if self.dark_mode { Icon::Moon } else { Icon::Sun };
                    let label = icon.unicode().to_string();
                    switch(
                        ui,
                        &self.theme,
                        &mut self.dark_mode,
                        label,
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    );
                    if prev_dark != self.dark_mode {
                        let palette = if self.dark_mode {
                            ColorPalette::dark()
                        } else {
                            ColorPalette::light()
                        };
                        self.theme = Theme::new(palette);
                    }
                });
                ui.add_space(8.0);

                ui.heading("Basic Input Types");
                ui.horizontal(|ui| {
                    ui.label("Text:");
                    Input::new("basic_input")
                        .placeholder("Enter text...")
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.basic_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Email:");
                    Input::new("email_input")
                        .placeholder("email@example.com")
                        .input_type(InputType::Email)
                        .width(200.0)
                        .left_slot(draw_mail_icon)
                        .show(ui, &self.theme, &mut self.email_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Password:");
                    Input::new("password_input")
                        .placeholder("Enter password...")
                        .input_type(InputType::Password)
                        .width(200.0)
                        .left_slot(draw_lock_icon)
                        .right_slot(draw_eye_icon)
                        .show(ui, &self.theme, &mut self.password_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Search:");
                    Input::new("search_input")
                        .placeholder("Search...")
                        .input_type(InputType::Search)
                        .width(200.0)
                        .radius(InputRadius::Full)
                        .left_slot(draw_search_icon)
                        .show(ui, &self.theme, &mut self.search_text);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.heading("Variants (matching radix-ui-themes)");

                ui.horizontal(|ui| {
                    ui.label("Surface (default):");
                    Input::new("surface_input")
                        .placeholder("Surface variant...")
                        .variant(Variant::Surface)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.surface_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Classic:");
                    Input::new("classic_input")
                        .placeholder("Classic variant...")
                        .variant(Variant::Classic)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.classic_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Soft:");
                    Input::new("soft_input")
                        .placeholder("Soft variant...")
                        .variant(Variant::Soft)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.soft_text);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.heading("Sizes (matching radix-ui-themes)");

                ui.horizontal(|ui| {
                    ui.label("Size 1 (24px):");
                    Input::new("size1_input")
                        .placeholder("Compact...")
                        .size(InputSize::Size1)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.size1_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Size 2 (32px):");
                    Input::new("size2_input")
                        .placeholder("Default...")
                        .size(InputSize::Size2)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.size2_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Size 3 (40px):");
                    Input::new("size3_input")
                        .placeholder("Large...")
                        .size(InputSize::Size3)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.size3_text);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.heading("States (matching shadcn-ui)");

                ui.horizontal(|ui| {
                    ui.label("Invalid:");
                    Input::new("invalid_input")
                        .placeholder("This field has an error")
                        .invalid(true)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.invalid_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Disabled:");
                    Input::new("disabled_input")
                        .placeholder("Disabled")
                        .enabled(false)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.disabled_text);
                });

                ui.horizontal(|ui| {
                    ui.label("Read-only:");
                    Input::new("readonly_input")
                        .placeholder("Read-only")
                        .read_only(true)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.readonly_text);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.heading("Slots (matching radix-ui TextField.Slot)");

                ui.horizontal(|ui| {
                    ui.label("With left icon:");
                    Input::new("left_icon_input")
                        .placeholder("Search...")
                        .left_slot(draw_search_icon)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.with_icon_text);
                });

                ui.horizontal(|ui| {
                    ui.label("With both icons:");
                    Input::new("both_icons_input")
                        .placeholder("Username")
                        .left_slot(draw_user_icon)
                        .right_slot(draw_x_icon)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.with_both_icons_text);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.heading("Advanced Features");

                ui.horizontal(|ui| {
                    ui.label("Character limit:");
                    Input::new("limited_input")
                        .placeholder("Max 20 chars")
                        .max_len(20)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.limited_text);
                    ui.label(format!("{}/20", self.limited_text.len()));
                });

                ui.horizontal(|ui| {
                    ui.label("Accent color:");
                    Input::new("accent_input")
                        .placeholder("Blue accent...")
                        .variant(Variant::Soft)
                        .accent_color(Color32::from_rgb(59, 130, 246))
                        .left_slot(draw_at_sign_icon)
                        .width(200.0)
                        .show(ui, &self.theme, &mut self.accent_text);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.heading("Radius Options");

                ui.horizontal(|ui| {
                    let mut temp = String::new();

                    Input::new("radius_none")
                        .placeholder("None")
                        .radius(InputRadius::None)
                        .width(100.0)
                        .show(ui, &self.theme, &mut temp);

                    Input::new("radius_small")
                        .placeholder("Small")
                        .radius(InputRadius::Small)
                        .width(100.0)
                        .show(ui, &self.theme, &mut temp);

                    Input::new("radius_medium")
                        .placeholder("Medium")
                        .radius(InputRadius::Medium)
                        .width(100.0)
                        .show(ui, &self.theme, &mut temp);

                    Input::new("radius_large")
                        .placeholder("Large")
                        .radius(InputRadius::Large)
                        .width(100.0)
                        .show(ui, &self.theme, &mut temp);

                    Input::new("radius_full")
                        .placeholder("Full")
                        .radius(InputRadius::Full)
                        .width(100.0)
                        .show(ui, &self.theme, &mut temp);
                });
            });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting input example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — Input Component",
        native_options,
        Box::new(|_cc| Ok(Box::new(InputDemo::new()))),
    ) {
        error!("Failed to run input example: {err}");
    }
}
