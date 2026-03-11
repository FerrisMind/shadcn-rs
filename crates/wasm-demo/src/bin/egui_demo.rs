use eframe::{App, Frame, egui};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::{
    Button, ButtonSize, ButtonVariant, CardProps, CardVariant, ControlSize,
    ControlVariant, Input, InputSize, InputType, Label, Theme, card,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

fn main() -> eframe::Result {
    let web_options = eframe::WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "egui-canvas",
                web_options,
                Box::new(|_cc| {
                    ensure_lucide_font(&_cc.egui_ctx);
                    Ok(Box::new(EguiApp::default()))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
    Ok(())
}

fn ensure_lucide_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
}

struct EguiApp {
    theme: Theme,
    email: String,
    password: String,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            theme: Theme::new(ColorPalette::dark()),
            email: String::new(),
            password: String::new(),
        }
    }
}

impl App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.set_max_width(350.0);
                render_login_card(ui, &self.theme, &mut self.email, &mut self.password);
            });
        });
    }
}

fn render_login_card(ui: &mut egui::Ui, theme: &Theme, email: &mut String, password: &mut String) {
    card(
        ui,
        theme,
        CardProps::default()
            .padding(egui::vec2(24.0, 24.0))
            .variant(CardVariant::Outline),
        |card_ui| {
            card_ui.spacing_mut().item_spacing.y = 20.0;

            card_ui.vertical(|header| {
                header.spacing_mut().item_spacing.y = 8.0;
                header.label(
                    egui::RichText::new("Login to your account")
                        .size(16.0)
                        .strong()
                        .color(theme.palette.foreground),
                );
                header.label(
                    egui::RichText::new("Enter your email below to login.")
                        .color(theme.palette.muted_foreground),
                );
            });

            card_ui.vertical(|content| {
                content.spacing_mut().item_spacing.y = 12.0;

                content.vertical(|field| {
                    field.spacing_mut().item_spacing.y = 8.0;
                    Label::new("Email").size(ControlSize::Sm).show(field, theme);
                    Input::new(field.make_persistent_id("email"))
                        .placeholder("m@example.com")
                        .width(field.available_width())
                        .show(field, theme, email);
                });

                content.vertical(|field| {
                    field.spacing_mut().item_spacing.y = 8.0;
                    Label::new("Password").size(ControlSize::Sm).show(field, theme);
                    Input::new(field.make_persistent_id("pass"))
                        .input_type(InputType::Password)
                        .width(field.available_width())
                        .show(field, theme, password);
                });
            });

            card_ui.vertical(|footer| {
                let full_width = footer.available_width();
                Button::new("Login")
                    .variant(ButtonVariant::Default)
                    .min_width(full_width)
                    .show(footer, theme);
            });
        },
    );
}
