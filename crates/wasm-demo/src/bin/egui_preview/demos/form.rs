use super::super::app::EguiPreviewApp;
use eframe::egui::{RichText, Ui};
use egui_shadcn::{
    ControlSize, ControlVariant, FormControl, FormDescription, FormItem, FormLabel, FormMessage,
    FormState, Input, button, compose, email, required,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let form_width = if compact { 220.0 } else { 320.0 };
    let user_id = ui.make_persistent_id("preview-form-user");
    let email_id = ui.make_persistent_id("preview-form-email");
    let msg_id = ui.make_persistent_id("preview-form-msg");

    let mut form = FormState::default();
    let mut username = ui
        .data(|d| d.get_temp::<String>(user_id))
        .unwrap_or_default();
    let mut mail = ui
        .data(|d| d.get_temp::<String>(email_id))
        .unwrap_or_default();
    let mut message = ui
        .data(|d| d.get_temp::<Option<String>>(msg_id))
        .unwrap_or(None);

    form.field("username", required("Username is required."));
    form.field(
        "email",
        compose(vec![
            required("Email is required."),
            email("Please enter a valid email address."),
        ]),
    );

    ui.horizontal(|row| {
        row.add_space(((row.available_width() - form_width) * 0.5).max(0.0));
        row.vertical(|form_ui| {
            form_ui.set_width(form_width);

            let username_error = form.error("username").map(|e| e.to_string());
            FormItem::new(form_ui.make_persistent_id("preview-form-username-item")).show(
                form_ui,
                |ui, ctx| {
                    FormLabel::new("Username")
                        .required(true)
                        .error(username_error.is_some())
                        .show(ui, &app.theme, ctx);
                    let resp = FormControl::new().show(ui, ctx, |ui, id| {
                        Input::new(id)
                            .placeholder("shadcn_user")
                            .invalid(username_error.is_some())
                            .width(form_width)
                            .show(ui, &app.theme, &mut username)
                    });
                    form.set_text("username", username.clone());
                    if resp.lost_focus() {
                        form.blur("username");
                    }
                    FormDescription::new("This is your public display name.").show(ui, &app.theme);
                    FormMessage::from_error(username_error.as_deref()).show(ui, &app.theme);
                },
            );

            form_ui.add_space(10.0);

            let email_error = form.error("email").map(|e| e.to_string());
            FormItem::new(form_ui.make_persistent_id("preview-form-email-item")).show(
                form_ui,
                |ui, ctx| {
                    FormLabel::new("Email")
                        .required(true)
                        .error(email_error.is_some())
                        .show(ui, &app.theme, ctx);
                    let resp = FormControl::new().show(ui, ctx, |ui, id| {
                        Input::new(id)
                            .placeholder("name@example.com")
                            .invalid(email_error.is_some())
                            .width(form_width)
                            .show(ui, &app.theme, &mut mail)
                    });
                    form.set_text("email", mail.clone());
                    if resp.lost_focus() {
                        form.blur("email");
                    }
                    FormMessage::from_error(email_error.as_deref()).show(ui, &app.theme);
                },
            );

            form_ui.add_space(10.0);
            if button(
                form_ui,
                &app.theme,
                "Submit",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            )
            .clicked()
            {
                if form.validate() {
                    message = Some("Form submitted successfully.".to_owned());
                } else {
                    message = None;
                }
            }

            if let Some(text) = message.as_deref() {
                form_ui.label(
                    RichText::new(text)
                        .size(12.0)
                        .color(app.theme.palette.primary),
                );
            }
        });
    });

    ui.data_mut(|d| {
        d.insert_temp(user_id, username);
        d.insert_temp(email_id, mail);
        d.insert_temp(msg_id, message);
    });
}
