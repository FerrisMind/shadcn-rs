#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui::RichText;
use egui_shadcn::{
    ControlSize, ControlVariant, Label, PopupPosition, SelectItem, SelectProps, Theme, button,
    select_with_items,
};

struct SelectDemo {
    theme: Theme,
    fruit: Option<String>,
    timezone: Option<String>,
    department: Option<String>,
    email: Option<String>,
    language_rhf: Option<String>,
    language_rhf_error: Option<String>,
    language_tanstack: Option<String>,
    language_tanstack_error: Option<String>,
}

impl SelectDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            fruit: None,
            timezone: None,
            department: None,
            email: None,
            language_rhf: None,
            language_rhf_error: None,
            language_tanstack: None,
            language_tanstack_error: None,
        }
    }

    fn fruit_items() -> Vec<SelectItem> {
        vec![SelectItem::group(
            "Fruits",
            vec![
                SelectItem::option("apple", "Apple"),
                SelectItem::option("banana", "Banana"),
                SelectItem::option("blueberry", "Blueberry"),
                SelectItem::option("grapes", "Grapes"),
                SelectItem::option("pineapple", "Pineapple"),
            ],
        )]
    }

    fn timezone_items() -> Vec<SelectItem> {
        vec![
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
                    SelectItem::option("ist_indonesia", "Indonesia Central Standard Time (WITA)"),
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
        ]
    }

    fn department_items() -> Vec<SelectItem> {
        vec![
            SelectItem::option("engineering", "Engineering"),
            SelectItem::option("design", "Design"),
            SelectItem::option("marketing", "Marketing"),
            SelectItem::option("sales", "Sales"),
            SelectItem::option("support", "Customer Support"),
            SelectItem::option("hr", "Human Resources"),
            SelectItem::option("finance", "Finance"),
            SelectItem::option("operations", "Operations"),
        ]
    }

    fn language_items() -> Vec<SelectItem> {
        vec![
            SelectItem::option("auto", "Auto"),
            SelectItem::separator(),
            SelectItem::option("en", "English"),
            SelectItem::option("es", "Spanish"),
            SelectItem::option("fr", "French"),
            SelectItem::option("de", "German"),
            SelectItem::option("it", "Italian"),
            SelectItem::option("zh", "Chinese"),
            SelectItem::option("ja", "Japanese"),
        ]
    }
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

fn muted_text(theme: &Theme, text: &str) -> RichText {
    RichText::new(text)
        .color(theme.palette.muted_foreground)
        .size(12.0)
}

fn error_text(theme: &Theme, text: &str) -> RichText {
    RichText::new(text)
        .color(theme.palette.destructive)
        .size(12.0)
}

impl App for SelectDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Select");
            ui.add_space(12.0);

            egui::Grid::new("select_examples_grid")
                .num_columns(2)
                .spacing(egui::vec2(24.0, 20.0))
                .show(ui, |grid| {
                    let compact_width = 220.0;
                    let wide_width = 300.0;
                    let form_width = 320.0;

                    example_card(grid, "Select", |ui| {
                        ui.set_min_width(compact_width);
                        ui.set_max_width(compact_width);
                        let fruits = SelectDemo::fruit_items();
                        select_with_items(
                            ui,
                            &self.theme,
                            SelectProps::new("select-demo", &mut self.fruit)
                                .placeholder("Select a fruit")
                                .width(180.0),
                            &fruits,
                        );
                    });

                    example_card(grid, "Scrollable", |ui| {
                        ui.set_min_width(wide_width);
                        ui.set_max_width(wide_width);
                        let timezones = SelectDemo::timezone_items();
                        select_with_items(
                            ui,
                            &self.theme,
                            SelectProps::new("select-scrollable", &mut self.timezone)
                                .placeholder("Select a timezone")
                                .width(280.0),
                            &timezones,
                        );
                    });
                    grid.end_row();

                    example_card(grid, "Field", |ui| {
                        ui.spacing_mut().item_spacing.y = 10.0;
                        ui.set_min_width(form_width);
                        ui.set_max_width(form_width);

                        let department_id = ui.make_persistent_id("select-department");
                        Label::new("Department")
                            .for_id(department_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);

                        let departments = SelectDemo::department_items();
                        select_with_items(
                            ui,
                            &self.theme,
                            SelectProps::new(department_id, &mut self.department)
                                .placeholder("Choose department")
                                .width(form_width),
                            &departments,
                        );

                        ui.label(muted_text(
                            &self.theme,
                            "Select your department or area of work.",
                        ));
                    });

                    example_card(grid, "Form", |ui| {
                        ui.spacing_mut().item_spacing.y = 12.0;
                        ui.set_min_width(form_width);
                        ui.set_max_width(form_width);

                        let email_id = ui.make_persistent_id("select-form-email");
                        Label::new("Email")
                            .for_id(email_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);

                        let emails = vec![
                            SelectItem::option("m@example.com", "m@example.com"),
                            SelectItem::option("m@google.com", "m@google.com"),
                            SelectItem::option("m@support.com", "m@support.com"),
                        ];

                        select_with_items(
                            ui,
                            &self.theme,
                            SelectProps::new(email_id, &mut self.email)
                                .placeholder("Select a verified email to display")
                                .width(form_width),
                            &emails,
                        );

                        ui.label(muted_text(
                            &self.theme,
                            "You can manage email addresses in your email settings.",
                        ));
                        ui.add_space(4.0);

                        let _ = button(
                            ui,
                            &self.theme,
                            "Submit",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                    });
                    grid.end_row();

                    example_card(grid, "React Hook Form", |ui| {
                        ui.spacing_mut().item_spacing.y = 10.0;
                        ui.set_min_width(form_width);
                        ui.set_max_width(form_width);

                        ui.label(RichText::new("Spoken Language").size(13.0));
                        ui.label(muted_text(
                            &self.theme,
                            "For best results, select the language you speak.",
                        ));

                        let invalid = self.language_rhf_error.is_some();
                        let languages = SelectDemo::language_items();
                        select_with_items(
                            ui,
                            &self.theme,
                            SelectProps::new("select-rhf-language", &mut self.language_rhf)
                                .placeholder("Select")
                                .width(form_width)
                                .invalid(invalid)
                                .position(PopupPosition::ItemAligned),
                            &languages,
                        );

                        if let Some(error) = &self.language_rhf_error {
                            ui.label(error_text(&self.theme, error));
                        }

                        ui.add_space(8.0);
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;

                            let save = button(
                                row,
                                &self.theme,
                                "Save",
                                ControlVariant::Primary,
                                ControlSize::Md,
                                true,
                            );
                            if save.clicked() {
                                self.language_rhf_error =
                                    match self.language_rhf.as_deref() {
                                        None | Some("") => {
                                            Some("Please select your spoken language.".to_string())
                                        }
                                        Some("auto") => Some(
                                            "Auto-detection is not allowed. Please select a specific language."
                                                .to_string(),
                                        ),
                                        _ => None,
                                    };
                            }

                            let reset = button(
                                row,
                                &self.theme,
                                "Reset",
                                ControlVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                            if reset.clicked() {
                                self.language_rhf = None;
                                self.language_rhf_error = None;
                            }
                        });
                    });

                    example_card(grid, "TanStack Form", |ui| {
                        ui.spacing_mut().item_spacing.y = 10.0;
                        ui.set_min_width(form_width);
                        ui.set_max_width(form_width);

                        ui.label(RichText::new("Spoken Language").size(13.0));
                        ui.label(muted_text(
                            &self.theme,
                            "For best results, select the language you speak.",
                        ));

                        let invalid = self.language_tanstack_error.is_some();
                        let languages = SelectDemo::language_items();
                        select_with_items(
                            ui,
                            &self.theme,
                            SelectProps::new(
                                "select-tanstack-language",
                                &mut self.language_tanstack,
                            )
                            .placeholder("Select")
                            .width(form_width)
                            .invalid(invalid)
                            .position(PopupPosition::ItemAligned),
                            &languages,
                        );

                        if let Some(error) = &self.language_tanstack_error {
                            ui.label(error_text(&self.theme, error));
                        }

                        ui.add_space(8.0);
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;

                            let save = button(
                                row,
                                &self.theme,
                                "Save",
                                ControlVariant::Primary,
                                ControlSize::Md,
                                true,
                            );
                            if save.clicked() {
                                self.language_tanstack_error =
                                    match self.language_tanstack.as_deref() {
                                        None | Some("") => {
                                            Some("Please select your spoken language.".to_string())
                                        }
                                        Some("auto") => Some(
                                            "Auto-detection is not allowed. Please select a specific language."
                                                .to_string(),
                                        ),
                                        _ => None,
                                    };
                            }

                            let reset = button(
                                row,
                                &self.theme,
                                "Reset",
                                ControlVariant::Outline,
                                ControlSize::Md,
                                true,
                            );
                            if reset.clicked() {
                                self.language_tanstack = None;
                                self.language_tanstack_error = None;
                            }
                        });
                    });
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Select example",
        options,
        Box::new(|_cc| Ok(Box::new(SelectDemo::new()))),
    )
}
