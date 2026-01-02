#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{Align, CentralPanel, Layout, RichText};
use egui_shadcn::{
    CheckboxProps, CheckboxState, ControlSize, ControlVariant, FormControl, FormDescription,
    FormItem, FormLabel, FormMessage, FormState, Input, SelectItem, SelectProps, Textarea, Theme,
    ValidationMode, button, checkbox_with_props, compose, email, max_length, min_length, pattern,
    required, select_with_items,
};

struct FormExample {
    theme: Theme,
    submit_form: FormState,
    change_form: FormState,
    blur_form: FormState,
    submit_message: Option<String>,
    username: String,
    email: String,
    bio: String,
    department: Option<String>,
    accept_terms: bool,
    change_name: String,
    change_note: String,
    blur_name: String,
    blur_note: String,
}

impl FormExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            submit_form: FormState::new(ValidationMode::OnSubmit),
            change_form: FormState::new(ValidationMode::OnChange),
            blur_form: FormState::new(ValidationMode::OnBlur),
            submit_message: None,
            username: String::new(),
            email: String::new(),
            bio: String::new(),
            department: None,
            accept_terms: false,
            change_name: String::new(),
            change_note: String::new(),
            blur_name: String::new(),
            blur_note: String::new(),
        }
    }

    fn department_items() -> Vec<SelectItem> {
        vec![
            SelectItem::option("design", "Design"),
            SelectItem::option("engineering", "Engineering"),
            SelectItem::option("marketing", "Marketing"),
            SelectItem::option("product", "Product"),
            SelectItem::option("support", "Support"),
        ]
    }
}

impl App for FormExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Form");
            ui.label("Validation helpers with Input, Textarea, Checkbox, and Select.");
            ui.add_space(16.0);

            render_section_title(
                ui,
                &self.theme,
                "Form demo",
                "OnSubmit validation with multiple field types.",
            );
            render_submit_form(
                ui,
                &self.theme,
                &mut self.submit_form,
                &mut self.username,
                &mut self.email,
                &mut self.bio,
                &mut self.department,
                &mut self.accept_terms,
                &mut self.submit_message,
            );

            ui.add_space(28.0);

            render_section_title(
                ui,
                &self.theme,
                "Validation modes",
                "OnChange validates as you type, OnBlur validates on focus loss.",
            );
            render_validation_modes(
                ui,
                &self.theme,
                &mut self.change_form,
                &mut self.blur_form,
                &mut self.change_name,
                &mut self.change_note,
                &mut self.blur_name,
                &mut self.blur_note,
            );
        });
    }
}

fn render_section_title(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.label(
            RichText::new(description)
                .color(theme.palette.muted_foreground)
                .size(12.0),
        );
        ui.add_space(12.0);
    });
}

fn render_submit_form(
    ui: &mut egui::Ui,
    theme: &Theme,
    form: &mut FormState,
    username: &mut String,
    email_value: &mut String,
    bio: &mut String,
    department: &mut Option<String>,
    accept_terms: &mut bool,
    submit_message: &mut Option<String>,
) {
    form.field(
        "username",
        compose(vec![
            required("Username is required."),
            min_length(2, "Username must be at least 2 characters."),
            max_length(16, "Username must be 16 characters or less."),
            pattern(
                r"^[a-zA-Z0-9_]+$",
                "Only letters, numbers, and underscores are allowed.",
            ),
        ]),
    );
    form.field(
        "email",
        compose(vec![
            required("Email is required."),
            email("Please enter a valid email address."),
        ]),
    );
    form.field(
        "bio",
        compose(vec![
            required("Bio is required."),
            max_length(160, "Bio must be 160 characters or less."),
        ]),
    );
    form.field("department", required("Pick a department."));
    form.field("accept_terms", required("You must accept the terms."));

    ui.spacing_mut().item_spacing.y = 14.0;
    ui.set_max_width(420.0);

    let username_error = form.error("username").map(|err| err.to_string());
    FormItem::new(ui.make_persistent_id("form-demo-username")).show(ui, |ui, ctx| {
        FormLabel::new("Username")
            .required(true)
            .error(username_error.is_some())
            .show(ui, theme, ctx);
        let response = FormControl::new().show(ui, ctx, |ui, id| {
            Input::new(id)
                .placeholder("shadcn_user")
                .invalid(username_error.is_some())
                .width(320.0)
                .show(ui, theme, username)
        });
        form.set_text("username", username.clone());
        if response.lost_focus() {
            form.blur("username");
        }
        FormDescription::new("This is your public display name.").show(ui, theme);
        FormMessage::from_error(username_error.as_deref()).show(ui, theme);
    });

    let email_error = form.error("email").map(|err| err.to_string());
    FormItem::new(ui.make_persistent_id("form-demo-email")).show(ui, |ui, ctx| {
        FormLabel::new("Email")
            .required(true)
            .error(email_error.is_some())
            .show(ui, theme, ctx);
        let response = FormControl::new().show(ui, ctx, |ui, id| {
            Input::new(id)
                .placeholder("name@example.com")
                .invalid(email_error.is_some())
                .width(320.0)
                .show(ui, theme, email_value)
        });
        form.set_text("email", email_value.clone());
        if response.lost_focus() {
            form.blur("email");
        }
        FormMessage::from_error(email_error.as_deref()).show(ui, theme);
    });

    let bio_error = form.error("bio").map(|err| err.to_string());
    FormItem::new(ui.make_persistent_id("form-demo-bio")).show(ui, |ui, ctx| {
        FormLabel::new("Bio")
            .required(true)
            .error(bio_error.is_some())
            .show(ui, theme, ctx);
        let response = FormControl::new().show(ui, ctx, |ui, id| {
            Textarea::new(id)
                .placeholder("Tell us about yourself.")
                .invalid(bio_error.is_some())
                .width(320.0)
                .show(ui, theme, bio)
        });
        form.set_text("bio", bio.clone());
        if response.lost_focus() {
            form.blur("bio");
        }
        FormDescription::new("A short bio keeps your profile personal.").show(ui, theme);
        FormMessage::from_error(bio_error.as_deref()).show(ui, theme);
    });

    let department_error = form.error("department").map(|err| err.to_string());
    FormItem::new(ui.make_persistent_id("form-demo-department")).show(ui, |ui, ctx| {
        FormLabel::new("Department")
            .required(true)
            .error(department_error.is_some())
            .show(ui, theme, ctx);
        let items = FormExample::department_items();
        let response = FormControl::new().show(ui, ctx, |ui, id| {
            select_with_items(
                ui,
                theme,
                SelectProps::new(id, department)
                    .placeholder("Select a department")
                    .invalid(department_error.is_some())
                    .width(320.0),
                &items,
            )
        });
        form.set_select("department", department.clone());
        if response.lost_focus() {
            form.blur("department");
        }
        FormMessage::from_error(department_error.as_deref()).show(ui, theme);
    });

    let terms_error = form.error("accept_terms").map(|err| err.to_string());
    FormItem::new(ui.make_persistent_id("form-demo-terms")).show(ui, |ui, ctx| {
        FormLabel::new("Terms")
            .required(true)
            .error(terms_error.is_some())
            .show(ui, theme, ctx);
        let mut state = CheckboxState::from(*accept_terms);
        let response = FormControl::new().show(ui, ctx, |ui, _| {
            checkbox_with_props(
                ui,
                theme,
                &mut state,
                "I agree to the terms and conditions",
                CheckboxProps::default().with_invalid(terms_error.is_some()),
            )
        });
        *accept_terms = bool::from(state);
        form.set_bool("accept_terms", *accept_terms);
        if response.clicked() {
            form.blur("accept_terms");
        }
        FormMessage::from_error(terms_error.as_deref()).show(ui, theme);
    });

    ui.add_space(4.0);
    let submit_clicked = button(
        ui,
        theme,
        "Submit",
        ControlVariant::Primary,
        ControlSize::Md,
        true,
    )
    .clicked();

    if submit_clicked {
        let is_valid = form.validate();
        *submit_message = if is_valid {
            Some("Form submitted successfully.".to_string())
        } else {
            None
        };
    }

    if let Some(message) = submit_message.as_ref() {
        ui.add_space(8.0);
        ui.label(
            RichText::new(message)
                .color(theme.palette.primary)
                .size(12.0),
        );
    }
}

fn render_validation_modes(
    ui: &mut egui::Ui,
    theme: &Theme,
    change_form: &mut FormState,
    blur_form: &mut FormState,
    change_name: &mut String,
    change_note: &mut String,
    blur_name: &mut String,
    blur_note: &mut String,
) {
    change_form.field(
        "change_name",
        compose(vec![
            required("Display name is required."),
            min_length(3, "Display name must be at least 3 characters."),
        ]),
    );
    change_form.field(
        "change_note",
        compose(vec![
            required("Note is required."),
            max_length(80, "Keep it short."),
        ]),
    );

    blur_form.field(
        "blur_name",
        compose(vec![
            required("Project name is required."),
            min_length(2, "Project name must be at least 2 characters."),
        ]),
    );
    blur_form.field(
        "blur_note",
        compose(vec![
            required("Summary is required."),
            max_length(120, "Summary must be 120 characters or less."),
        ]),
    );

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(24.0, 12.0);
        ui.vertical(|ui| {
            ui.set_max_width(360.0);
            ui.label(RichText::new("OnChange").strong());
            ui.add_space(8.0);

            let name_error = change_form.error("change_name").map(|err| err.to_string());
            FormItem::new(ui.make_persistent_id("form-change-name")).show(ui, |ui, ctx| {
                FormLabel::new("Display name")
                    .required(true)
                    .error(name_error.is_some())
                    .show(ui, theme, ctx);
                let response = FormControl::new().show(ui, ctx, |ui, id| {
                    Input::new(id)
                        .placeholder("Acme Inc.")
                        .invalid(name_error.is_some())
                        .width(300.0)
                        .show(ui, theme, change_name)
                });
                change_form.set_text("change_name", change_name.clone());
                if response.lost_focus() {
                    change_form.blur("change_name");
                }
                FormMessage::from_error(name_error.as_deref()).show(ui, theme);
            });

            let note_error = change_form.error("change_note").map(|err| err.to_string());
            FormItem::new(ui.make_persistent_id("form-change-note")).show(ui, |ui, ctx| {
                FormLabel::new("Note")
                    .required(true)
                    .error(note_error.is_some())
                    .show(ui, theme, ctx);
                let response = FormControl::new().show(ui, ctx, |ui, id| {
                    Textarea::new(id)
                        .placeholder("Short note for the team.")
                        .invalid(note_error.is_some())
                        .width(300.0)
                        .show(ui, theme, change_note)
                });
                change_form.set_text("change_note", change_note.clone());
                if response.lost_focus() {
                    change_form.blur("change_note");
                }
                FormMessage::from_error(note_error.as_deref()).show(ui, theme);
            });
        });

        ui.vertical(|ui| {
            ui.set_max_width(360.0);
            ui.label(RichText::new("OnBlur").strong());
            ui.add_space(8.0);

            let name_error = blur_form.error("blur_name").map(|err| err.to_string());
            FormItem::new(ui.make_persistent_id("form-blur-name")).show(ui, |ui, ctx| {
                FormLabel::new("Project name")
                    .required(true)
                    .error(name_error.is_some())
                    .show(ui, theme, ctx);
                let response = FormControl::new().show(ui, ctx, |ui, id| {
                    Input::new(id)
                        .placeholder("Launch plan")
                        .invalid(name_error.is_some())
                        .width(300.0)
                        .show(ui, theme, blur_name)
                });
                blur_form.set_text("blur_name", blur_name.clone());
                if response.lost_focus() {
                    blur_form.blur("blur_name");
                }
                FormMessage::from_error(name_error.as_deref()).show(ui, theme);
            });

            let note_error = blur_form.error("blur_note").map(|err| err.to_string());
            FormItem::new(ui.make_persistent_id("form-blur-note")).show(ui, |ui, ctx| {
                FormLabel::new("Summary")
                    .required(true)
                    .error(note_error.is_some())
                    .show(ui, theme, ctx);
                let response = FormControl::new().show(ui, ctx, |ui, id| {
                    Textarea::new(id)
                        .placeholder("Add a short summary for review.")
                        .invalid(note_error.is_some())
                        .width(300.0)
                        .show(ui, theme, blur_note)
                });
                blur_form.set_text("blur_note", blur_note.clone());
                if response.lost_focus() {
                    blur_form.blur("blur_note");
                }
                FormMessage::from_error(note_error.as_deref()).show(ui, theme);
            });
        });
    });

    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.label(
            RichText::new("Tip: try switching focus to see OnBlur in action.")
                .color(theme.palette.muted_foreground)
                .size(12.0),
        );
    });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Form example",
        options,
        Box::new(|_cc| Ok(Box::new(FormExample::new()))),
    )
}
