//! Пример Textarea, содержащий все референсы shadcn/ui для Textarea.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{
    ControlSize, ControlVariant, Label, SeparatorProps, Textarea, TextareaSize, Theme, button,
    separator,
};

struct TextareaDemo {
    theme: Theme,
    message: String,
    disabled_message: String,
    labeled_message: String,
    text_message: String,
    bio: String,
}

impl TextareaDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            message: String::new(),
            disabled_message: String::new(),
            labeled_message: String::new(),
            text_message: String::new(),
            bio: String::new(),
        }
    }
}

impl App for TextareaDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;
                ui.set_max_width(420.0);

                // textarea-demo
                Textarea::new("textarea-demo")
                    .placeholder("Type your message here.")
                    .size(TextareaSize::Size2)
                    .show(ui, &self.theme, &mut self.message);

                // textarea-disabled
                Textarea::new("textarea-disabled")
                    .placeholder("Type your message here.")
                    .size(TextareaSize::Size2)
                    .enabled(false)
                    .show(ui, &self.theme, &mut self.disabled_message);

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                // textarea-with-label
                ui.vertical(|group| {
                    group.spacing_mut().item_spacing.y = 8.0;
                    let message_id = group.make_persistent_id("message");
                    Label::new("Your message")
                        .for_id(message_id)
                        .size(ControlSize::Sm)
                        .show(group, &self.theme);
                    Textarea::new(message_id)
                        .placeholder("Type your message here.")
                        .size(TextareaSize::Size2)
                        .show(group, &self.theme, &mut self.labeled_message);
                });

                // textarea-with-text
                ui.vertical(|group| {
                    group.spacing_mut().item_spacing.y = 8.0;
                    let message_id = group.make_persistent_id("message-2");
                    Label::new("Your Message")
                        .for_id(message_id)
                        .size(ControlSize::Sm)
                        .show(group, &self.theme);
                    Textarea::new(message_id)
                        .placeholder("Type your message here.")
                        .size(TextareaSize::Size2)
                        .show(group, &self.theme, &mut self.text_message);
                    group.label(
                        egui::RichText::new("Your message will be copied to the support team.")
                            .color(self.theme.palette.muted_foreground)
                            .size(12.0),
                    );
                });

                // textarea-with-button
                ui.vertical(|group| {
                    group.spacing_mut().item_spacing.y = 8.0;
                    let message_id = group.make_persistent_id("message-3");
                    Textarea::new(message_id)
                        .placeholder("Type your message here.")
                        .size(TextareaSize::Size2)
                        .show(group, &self.theme, &mut self.labeled_message);
                    let _ = button(
                        group,
                        &self.theme,
                        "Send message",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                // textarea-form (упрощённо)
                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 8.0;
                    let bio_id = form.make_persistent_id("bio");
                    Label::new("Bio")
                        .for_id(bio_id)
                        .size(ControlSize::Sm)
                        .show(form, &self.theme);
                    Textarea::new(bio_id)
                        .placeholder("Tell us a little bit about yourself")
                        .size(TextareaSize::Size2)
                        .show(form, &self.theme, &mut self.bio);
                    form.label(
                        egui::RichText::new("You can @mention other users and organizations.")
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
        "Textarea example",
        options,
        Box::new(|_cc| Ok(Box::new(TextareaDemo::new()))),
    )
}
