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
    TableCellProps, TableProps, TableRowProps, Theme, table, table_body, table_caption, table_cell,
    table_footer, table_head, table_header, table_row,
};

struct Invoice {
    invoice: &'static str,
    payment_status: &'static str,
    total_amount: &'static str,
    payment_method: &'static str,
}

const INVOICES: [Invoice; 7] = [
    Invoice {
        invoice: "INV001",
        payment_status: "Paid",
        total_amount: "$250.00",
        payment_method: "Credit Card",
    },
    Invoice {
        invoice: "INV002",
        payment_status: "Pending",
        total_amount: "$150.00",
        payment_method: "PayPal",
    },
    Invoice {
        invoice: "INV003",
        payment_status: "Unpaid",
        total_amount: "$350.00",
        payment_method: "Bank Transfer",
    },
    Invoice {
        invoice: "INV004",
        payment_status: "Paid",
        total_amount: "$450.00",
        payment_method: "Credit Card",
    },
    Invoice {
        invoice: "INV005",
        payment_status: "Paid",
        total_amount: "$550.00",
        payment_method: "PayPal",
    },
    Invoice {
        invoice: "INV006",
        payment_status: "Pending",
        total_amount: "$200.00",
        payment_method: "Bank Transfer",
    },
    Invoice {
        invoice: "INV007",
        payment_status: "Unpaid",
        total_amount: "$300.00",
        payment_method: "Credit Card",
    },
];

struct TableExample {
    theme: Theme,
}

impl TableExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for TableExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Table");
            ui.label("A responsive table component.");
            ui.add_space(16.0);

            table(ui, &self.theme, TableProps::new(), |ui, ctx| {
                table_header(ui, ctx, |ui| {
                    table_row(
                        ui,
                        ctx,
                        TableRowProps::new("header").hoverable(false),
                        |ui| {
                            table_head(ui, ctx, TableCellProps::new(), |ui| {
                                ui.set_min_width(100.0);
                                ui.label(RichText::new("Invoice").strong());
                            });
                            table_head(ui, ctx, TableCellProps::new(), |ui| {
                                ui.label(RichText::new("Status").strong());
                            });
                            table_head(ui, ctx, TableCellProps::new(), |ui| {
                                ui.label(RichText::new("Method").strong());
                            });
                            table_head(ui, ctx, TableCellProps::new().fill(true), |ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new("Amount").strong());
                                });
                            });
                        },
                    );
                });

                table_body(ui, ctx, |ui| {
                    for invoice in INVOICES.iter() {
                        table_row(ui, ctx, TableRowProps::new(invoice.invoice), |ui| {
                            table_cell(ui, ctx, TableCellProps::new(), |ui| {
                                ui.set_min_width(100.0);
                                ui.label(RichText::new(invoice.invoice).strong());
                            });
                            table_cell(ui, ctx, TableCellProps::new(), |ui| {
                                ui.label(invoice.payment_status);
                            });
                            table_cell(ui, ctx, TableCellProps::new(), |ui| {
                                ui.label(invoice.payment_method);
                            });
                            table_cell(ui, ctx, TableCellProps::new().fill(true), |ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(invoice.total_amount);
                                });
                            });
                        });
                    }
                });

                table_footer(ui, ctx, |ui| {
                    table_row(
                        ui,
                        ctx,
                        TableRowProps::new("footer").hoverable(false),
                        |ui| {
                            table_cell(ui, ctx, TableCellProps::new(), |ui| {
                                ui.set_min_width(100.0);
                                ui.label(RichText::new("Total").strong());
                            });
                            table_cell(ui, ctx, TableCellProps::new(), |ui| {
                                ui.label("");
                            });
                            table_cell(ui, ctx, TableCellProps::new(), |ui| {
                                ui.label("");
                            });
                            table_cell(ui, ctx, TableCellProps::new().fill(true), |ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new("$2,500.00").strong());
                                });
                            });
                        },
                    );
                });

                table_caption(ui, ctx, "A list of your recent invoices.");
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Table example",
        options,
        Box::new(|_cc| Ok(Box::new(TableExample::new()))),
    )
}
