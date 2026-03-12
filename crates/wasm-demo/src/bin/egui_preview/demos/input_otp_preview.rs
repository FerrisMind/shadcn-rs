use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    InputOTPProps, input_otp, input_otp_group, input_otp_separator, input_otp_slot,
    input_otp_slot_last,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let slots = if compact { 4 } else { 6 };
    let row_width = if compact { 170.0 } else { 250.0 };
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let _ = input_otp(
            row,
            &app.theme,
            &mut app.otp_value,
            InputOTPProps::new(slots),
            |ui, ctx| {
                if slots == 4 {
                    input_otp_group(ui, |ui| {
                        input_otp_slot(ui, ctx, 0);
                        input_otp_slot(ui, ctx, 1);
                        input_otp_slot(ui, ctx, 2);
                        input_otp_slot_last(ui, ctx, 3);
                    });
                } else {
                    input_otp_group(ui, |ui| {
                        input_otp_slot(ui, ctx, 0);
                        input_otp_slot(ui, ctx, 1);
                        input_otp_slot_last(ui, ctx, 2);
                    });
                    input_otp_separator(ui, &app.theme);
                    input_otp_group(ui, |ui| {
                        input_otp_slot(ui, ctx, 3);
                        input_otp_slot(ui, ctx, 4);
                        input_otp_slot_last(ui, ctx, 5);
                    });
                }
            },
        );
    });
}
