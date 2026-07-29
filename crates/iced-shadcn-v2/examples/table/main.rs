//! Visual playground for the compositional table component.
//!
//! The layout mirrors shadcn-svelte's table demo: caption, header, seven body
//! rows, a footer, a fixed leading column, right-aligned amounts, and live row
//! hover/selected surfaces.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example table`.

use iced::widget::{column, container, scrollable, text};
use iced::{Background, Element, Length, Task};

use iced_shadcn_v2::{
    FontId, FontWeight, StyleId, Table, TableBody, TableCaption, TableCell, TableFooter, TableHead,
    TableHeader, TableRow, Theme, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Nova),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Table".to_owned()
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let invoices = [
            ("INV001", "Paid", "Credit Card", "$250.00"),
            ("INV002", "Pending", "PayPal", "$150.00"),
            ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
            ("INV004", "Paid", "Credit Card", "$450.00"),
            ("INV005", "Paid", "PayPal", "$550.00"),
            ("INV006", "Pending", "Bank Transfer", "$200.00"),
            ("INV007", "Unpaid", "Credit Card", "$300.00"),
        ];

        let body = invoices
            .into_iter()
            .map(|(invoice, status, method, amount)| {
                let invoice_cell = TableCell::text(invoice, theme).font_weight(FontWeight::Medium);

                TableRow::new(theme)
                    .cell(invoice_cell)
                    .cell(TableCell::text(status, theme))
                    .cell(TableCell::text(method, theme))
                    .cell(
                        TableCell::text(amount, theme).align_x(iced::alignment::Horizontal::Right),
                    )
            });

        let table = Table::new(theme)
            .column_widths([
                Length::Fixed(100.0),
                Length::Fill,
                Length::Fill,
                Length::Fill,
            ])
            .caption(TableCaption::text("A list of your recent invoices.", theme))
            .header(
                TableHeader::new(theme).push(
                    TableRow::new(theme)
                        .head(TableHead::text("Invoice", theme))
                        .head(TableHead::text("Status", theme))
                        .head(TableHead::text("Method", theme))
                        .head(
                            TableHead::text("Amount", theme)
                                .align_x(iced::alignment::Horizontal::Right),
                        ),
                ),
            )
            .body(TableBody::new(theme).extend(body))
            .footer(
                TableFooter::new(theme).push(
                    TableRow::new(theme)
                        .cell(TableCell::text("Total", theme).span(3))
                        .cell(
                            TableCell::text("$2,500.00", theme)
                                .align_x(iced::alignment::Horizontal::Right),
                        ),
                ),
            );

        container(scrollable(column![
            text("Table").size(28),
            text("A responsive compositional table for iced-shadcn-v2."),
            table,
        ]))
        .padding(32)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.palette.background)),
            text_color: Some(theme.palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}
