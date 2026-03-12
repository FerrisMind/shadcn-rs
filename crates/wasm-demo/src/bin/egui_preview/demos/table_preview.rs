use super::super::app::EguiPreviewApp;
use eframe::egui::{Align, Layout, RichText, Ui};
use egui_shadcn::{
    TableCellProps, TableProps, TableRowProps, table, table_body, table_cell, table_head,
    table_header, table_row,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let rows = if compact {
        vec![
            ("INV001", "Paid", "$250.00"),
            ("INV002", "Pending", "$150.00"),
            ("INV003", "Unpaid", "$350.00"),
        ]
    } else {
        vec![
            ("INV001", "Paid", "$250.00"),
            ("INV002", "Pending", "$150.00"),
            ("INV003", "Unpaid", "$350.00"),
            ("INV004", "Paid", "$450.00"),
            ("INV005", "Paid", "$550.00"),
        ]
    };

    table(ui, &app.theme, TableProps::new(), |ui, ctx| {
        table_header(ui, ctx, |ui| {
            table_row(ui, ctx, TableRowProps::new("preview-table-head"), |ui| {
                table_head(ui, ctx, TableCellProps::new(), |ui| {
                    ui.label(RichText::new("Invoice").strong());
                });
                table_head(ui, ctx, TableCellProps::new(), |ui| {
                    ui.label(RichText::new("Status").strong());
                });
                table_head(ui, ctx, TableCellProps::new().fill(true), |ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Amount").strong());
                    });
                });
            });
        });

        table_body(ui, ctx, |ui| {
            for (invoice, status, amount) in rows {
                table_row(ui, ctx, TableRowProps::new(invoice), |ui| {
                    table_cell(ui, ctx, TableCellProps::new(), |ui| {
                        ui.label(invoice);
                    });
                    table_cell(ui, ctx, TableCellProps::new(), |ui| {
                        ui.label(status);
                    });
                    table_cell(ui, ctx, TableCellProps::new().fill(true), |ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(amount);
                        });
                    });
                });
            }
        });
    });
}
