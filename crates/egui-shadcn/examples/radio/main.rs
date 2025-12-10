use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::radio::{GridLayout, RadioCardVariant, RadioDirection, RadioGroup, RadioOption};
use egui_shadcn::{ControlSize, ControlVariant, Theme};
use log::{error, info};

struct RadioDemo {
    theme: Theme,
    shipping: String,
    billing: String,
    plan: String,
    payment_method: String,
    card_selection: String,
}

impl RadioDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            shipping: "standard".to_string(),
            billing: "card".to_string(),
            plan: "pro".to_string(),
            payment_method: "credit".to_string(),
            card_selection: "option_1".to_string(),
        }
    }
}

impl App for RadioDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut style = ctx.style().as_ref().clone();
        let bg = egui::Color32::from_rgb(10, 10, 10);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.heading("Radio Group - Complete Demo");
                    ui.add_space(8.0);

                    ui.label("📦 Basic Vertical Group (with descriptions)");
                    ui.add_space(4.0);
                    let shipping_options = vec![
                        RadioOption::new("standard".to_string(), "Standard (3-5 days)")
                            .description("Included in plan"),
                        RadioOption::new("express".to_string(), "Express (1-2 days)")
                            .description("Adds $5 surcharge"),
                        RadioOption::new("pickup".to_string(), "Pickup in store").disabled(true),
                    ];

                    RadioGroup::new("shipping", &mut self.shipping, &shipping_options)
                        .variant(ControlVariant::Primary)
                        .size(ControlSize::Md)
                        .show_separators(false)
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label("💳 Horizontal Group with High Contrast");
                    ui.add_space(4.0);
                    let billing_options = vec![
                        RadioOption::new("card".to_string(), "Card").description("Visa/Mastercard"),
                        RadioOption::new("paypal".to_string(), "PayPal"),
                        RadioOption::new("wire".to_string(), "Wire transfer")
                            .description("Slower but secure"),
                    ];

                    RadioGroup::new("billing", &mut self.billing, &billing_options)
                        .variant(ControlVariant::Secondary)
                        .direction(RadioDirection::Horizontal)
                        .size(ControlSize::Sm)
                        .high_contrast(true)
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label("🎨 Custom Accent Color");
                    ui.add_space(4.0);
                    let plan_options = vec![
                        RadioOption::new("starter".to_string(), "Starter")
                            .description("Perfect for individuals"),
                        RadioOption::new("pro".to_string(), "Pro").description("Best for teams"),
                        RadioOption::new("enterprise".to_string(), "Enterprise")
                            .description("Custom solutions"),
                    ];

                    RadioGroup::new("plan", &mut self.plan, &plan_options)
                        .variant(ControlVariant::Primary)
                        .accent_color(egui::Color32::from_rgb(100, 200, 255))
                        .size(ControlSize::Md)
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label("🃏 Card Variant with 2-Column Grid");
                    ui.add_space(4.0);
                    let card_options = vec![
                        RadioOption::new("option_1".to_string(), "Option 1")
                            .description("First choice")
                            .icon("⭐"),
                        RadioOption::new("option_2".to_string(), "Option 2")
                            .description("Second choice")
                            .icon("💎"),
                        RadioOption::new("option_3".to_string(), "Option 3")
                            .description("Third choice")
                            .icon("🏆"),
                        RadioOption::new("option_4".to_string(), "Option 4")
                            .description("Fourth choice")
                            .icon("🎯"),
                    ];

                    RadioGroup::new("card_selection", &mut self.card_selection, &card_options)
                        .card_variant(RadioCardVariant::Card)
                        .grid_layout(GridLayout::new(2).with_spacing(12.0))
                        .variant(ControlVariant::Primary)
                        .size(ControlSize::Md)
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label("📏 Small Size with Secondary Variant");
                    ui.add_space(4.0);
                    let payment_options = vec![
                        RadioOption::new("credit".to_string(), "Credit Card"),
                        RadioOption::new("debit".to_string(), "Debit Card"),
                        RadioOption::new("crypto".to_string(), "Cryptocurrency"),
                    ];

                    RadioGroup::new("payment", &mut self.payment_method, &payment_options)
                        .variant(ControlVariant::Secondary)
                        .size(ControlSize::Sm)
                        .custom_spacing(6.0)
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.separator();
                    ui.label("Current Selections:");
                    ui.label(format!("  Shipping: {}", self.shipping));
                    ui.label(format!("  Billing: {}", self.billing));
                    ui.label(format!("  Plan: {}", self.plan));
                    ui.label(format!("  Card: {}", self.card_selection));
                    ui.label(format!("  Payment: {}", self.payment_method));
                });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting comprehensive radio group example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "Radio Group - Complete Demo",
        native_options,
        Box::new(|_cc| Ok(Box::new(RadioDemo::new()))),
    ) {
        error!("Failed to run example: {err}");
    }
}
