//! Пример Select, содержащий `select-demo`, `select-scrollable` и `select-form`.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{
    ControlSize, ControlVariant, Label, SelectItem, SelectProps, SeparatorProps, Theme, button,
    select_with_items, separator,
};

struct SelectDemo {
    theme: Theme,
    fruit: Option<String>,
    timezone: Option<String>,
    email: Option<String>,
}

impl SelectDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            fruit: None,
            timezone: None,
            email: None,
        }
    }
}

impl App for SelectDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;
                ui.set_max_width(360.0);

                // select-demo
                let fruits = vec![SelectItem::group(
                    "Fruits",
                    vec![
                        SelectItem::option("apple", "Apple"),
                        SelectItem::option("banana", "Banana"),
                        SelectItem::option("blueberry", "Blueberry"),
                        SelectItem::option("grapes", "Grapes"),
                        SelectItem::option("pineapple", "Pineapple"),
                    ],
                )];
                select_with_items(
                    ui,
                    &self.theme,
                    SelectProps::new("select-demo", &mut self.fruit)
                        .placeholder("Select a fruit")
                        .width(180.0),
                    &fruits,
                );

                // select-scrollable
                let timezones = vec![
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
                                "ist_indonesia",
                                "Indonesia Central Standard Time (WITA)",
                            ),
                        ],
                    ),
                    SelectItem::group(
                        "Australia & Pacific",
                        vec![
                            SelectItem::option("awst", "Australian Western Standard Time (AWST)"),
                            SelectItem::option("acst", "Australian Central Standard Time (ACST)"),
                            SelectItem::option("aest", "Australian Eastern Standard Time (AEST)"),
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
                    SelectProps::new("select-scrollable", &mut self.timezone)
                        .placeholder("Select a timezone")
                        .width(280.0),
                    &timezones,
                );

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                // select-form (упрощённо)
                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 8.0;
                    let email_id = form.make_persistent_id("select-form-email");
                    Label::new("Email")
                        .for_id(email_id)
                        .size(ControlSize::Sm)
                        .show(form, &self.theme);

                    let emails = vec![
                        SelectItem::option("m@example.com", "m@example.com"),
                        SelectItem::option("m@google.com", "m@google.com"),
                        SelectItem::option("m@support.com", "m@support.com"),
                    ];

                    select_with_items(
                        form,
                        &self.theme,
                        SelectProps::new(email_id, &mut self.email)
                            .placeholder("Select a verified email to display")
                            .width(form.available_width()),
                        &emails,
                    );

                    form.label(
                        egui::RichText::new(
                            "You can manage email addresses in your email settings.",
                        )
                        .color(self.theme.palette.muted_foreground)
                        .size(12.0),
                    );

                    let _ = button(
                        form,
                        &self.theme,
                        "Submit",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Select example",
        options,
        Box::new(|_cc| Ok(Box::new(SelectDemo::new()))),
    )
}
