#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::CornerRadius;
use egui_shadcn::{
    CardProps, CardVariant, ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize, ScrollAreaType,
    ScrollDirection, ShadcnTypographyVariant, Theme, TypographyProps, blockquote, card, link,
    scroll_area, text, typography,
};

struct TypographyDemo {
    theme: Theme,
}

impl TypographyDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    fn render_table(&self, ui: &mut egui::Ui) {
        let header = [("King's Treasury", true), ("People's happiness", true)];
        let rows: &[[(&str, bool); 2]] = &[
            [("Empty", false), ("Overflowing", false)],
            [("Modest", false), ("Satisfied", false)],
            [("Full", false), ("Ecstatic", false)],
        ];

        let available = ui.available_width();
        let col_w = (available / 2.0).floor();
        let row_h = 28.0;
        let total_rows = 1 + rows.len();
        let table_size = egui::vec2(col_w * 2.0, row_h * total_rows as f32);
        let border = egui::Stroke::new(1.0_f32, self.theme.palette.border);

        let (table_rect, _) = ui.allocate_exact_size(table_size, egui::Sense::hover());
        let painter = ui.painter();

        let all_rows: Vec<([(&str, bool); 2], egui::Color32)> =
            std::iter::once((header, egui::Color32::TRANSPARENT))
                .chain(rows.iter().enumerate().map(|(i, r)| {
                    let bg = if i % 2 == 1 {
                        self.theme.palette.muted
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    (*r, bg)
                }))
                .collect();

        let radius = 6.0;
        let last_row = total_rows - 1;
        let last_col = 1;

        for (row_idx, (cells, bg)) in all_rows.iter().enumerate() {
            for (col_idx, (text_value, bold)) in cells.iter().enumerate() {
                let x = table_rect.left() + col_w * col_idx as f32;
                let y = table_rect.top() + row_h * row_idx as f32;
                let cell_rect =
                    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(col_w, row_h));

                let corner = CornerRadius {
                    nw: if row_idx == 0 && col_idx == 0 {
                        radius as u8
                    } else {
                        0
                    },
                    ne: if row_idx == 0 && col_idx == last_col {
                        radius as u8
                    } else {
                        0
                    },
                    sw: if row_idx == last_row && col_idx == 0 {
                        radius as u8
                    } else {
                        0
                    },
                    se: if row_idx == last_row && col_idx == last_col {
                        radius as u8
                    } else {
                        0
                    },
                };

                painter.rect_filled(cell_rect, corner, *bg);
                painter.rect_stroke(cell_rect, corner, border, egui::StrokeKind::Inside);

                let text_pos = egui::pos2(cell_rect.left() + 8.0, cell_rect.center().y);
                let rich = if *bold {
                    egui::RichText::new(*text_value)
                        .font(egui::FontId::proportional(15.0))
                        .strong()
                } else {
                    egui::RichText::new(*text_value)
                        .font(egui::FontId::proportional(14.0))
                        .color(self.theme.palette.foreground)
                };
                let galley = egui::WidgetText::from(rich).into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    col_w - 16.0,
                    egui::TextStyle::Body,
                );
                painter.galley(
                    text_pos - egui::vec2(0.0, galley.size().y / 2.0),
                    galley,
                    self.theme.palette.foreground,
                );
            }
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let card_size = egui::vec2(760.0, 560.0);
        card(
            ui,
            &self.theme,
            CardProps::default()
                .variant(CardVariant::Outline)
                .padding(egui::vec2(16.0, 16.0))
                .rounding(CornerRadius::same(12))
                .shadow(true),
            |card_ui| {
                card_ui.set_min_size(card_size);
                card_ui.set_max_size(card_size);

                card_ui.vertical(|card_ui| {
                    scroll_area(
                        card_ui,
                        &self.theme,
                        ScrollAreaProps {
                            scroll_type: ScrollAreaType::Auto,
                            direction: ScrollDirection::Vertical,
                            size: ScrollAreaSize::Size2,
                            radius: ScrollAreaRadius::Small,
                            max_size: Some(card_size),
                            auto_shrink: [false; 2],
                            ..Default::default()
                        },
                        |ui| {
                            ui.set_width(ui.available_width());
                            typography(
                                ui,
                                &self.theme,
                                TypographyProps::new("Taxing Laughter: The Joke Tax Chronicles")
                                    .variant(ShadcnTypographyVariant::H1),
                            );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne. One day, his advisors came to him with a problem: the kingdom was running out of money.",
                            )
                            .variant(ShadcnTypographyVariant::Lead),
                        );

                        ui.add_space(40.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("The King's Plan")
                                .variant(ShadcnTypographyVariant::H2),
                        );

                        ui.add_space(24.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            text(
                                ui,
                                &self.theme,
                                egui_shadcn::TextProps::new(
                                    "The king thought long and hard, and finally came up with ",
                                )
                                .as_tag(egui_shadcn::TextAs::P),
                            );
                            let _ = link(
                                ui,
                                &self.theme,
                                egui_shadcn::LinkProps::new("a brilliant plan")
                                    .weight(egui_shadcn::TextWeight::Medium)
                                    .underline(egui_shadcn::LinkUnderline::Always),
                            );
                            text(
                                ui,
                                &self.theme,
                                egui_shadcn::TextProps::new(
                                    ": he would tax the jokes in the kingdom.",
                                )
                                .as_tag(egui_shadcn::TextAs::P),
                            );
                        });

                        ui.add_space(24.0);
                        blockquote(
                            ui,
                            &self.theme,
                            egui_shadcn::BlockquoteProps::new(
                                "\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that they should pay for the privilege.\"",
                            ),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("The Joke Tax")
                                .variant(ShadcnTypographyVariant::H3),
                        );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "The king's subjects were not amused. They grumbled and complained, but the king was firm:",
                            ),
                        );

                        ui.add_space(24.0);
                        ui.horizontal(|ui| {
                            ui.add_space(24.0);
                            ui.vertical(|ui| {
                                for (index, item) in [
                                    "1st level of puns: 5 gold coins",
                                    "2nd level of jokes: 10 gold coins",
                                    "3rd level of one-liners : 20 gold coins",
                                ]
                                .iter()
                                .enumerate()
                                {
                                    ui.horizontal(|ui| {
                                        ui.label("•");
                                        typography(ui, &self.theme, TypographyProps::new(*item));
                                    });
                                    if index + 1 < 3 {
                                        ui.add_space(8.0);
                                    }
                                }
                            });
                        });

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king's foolishness get him down: a court jester named Jokester.",
                            ),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Jokester's Revolt")
                                .variant(ShadcnTypographyVariant::H3),
                        );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "Jokester began sneaking into the castle in the middle of the night and leaving jokes all over the place: under the king's pillow, in his soup, even in the royal toilet. The king was furious, but he couldn't seem to stop Jokester.",
                            ),
                        );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "And then, one day, the people of the kingdom discovered that the jokes left by Jokester were so funny that they couldn't help but laugh. And once they started laughing, they couldn't stop.",
                            ),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("The People's Rebellion")
                                .variant(ShadcnTypographyVariant::H3),
                        );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke.",
                            ),
                        );

                        ui.add_space(24.0);
                        self.render_table(ui);
                        ui.add_space(24.0);

                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after.",
                            ),
                        );
                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "The moral of the story is: never underestimate the power of a good laugh and always be careful of bad ideas.",
                            ),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Inline Code")
                                .variant(ShadcnTypographyVariant::H3),
                        );
                        ui.add_space(16.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("@radix-ui/react-alert-dialog")
                                .variant(ShadcnTypographyVariant::InlineCode),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Lead")
                                .variant(ShadcnTypographyVariant::H3),
                        );
                        ui.add_space(16.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "A modal dialog that interrupts the user with important content and expects a response.",
                            )
                            .variant(ShadcnTypographyVariant::Lead),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Large")
                                .variant(ShadcnTypographyVariant::H3),
                        );
                        ui.add_space(16.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Are you absolutely sure?")
                                .variant(ShadcnTypographyVariant::Large),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Small")
                                .variant(ShadcnTypographyVariant::H3),
                        );
                        ui.add_space(16.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Email address")
                                .variant(ShadcnTypographyVariant::Small),
                        );

                        ui.add_space(32.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Muted")
                                .variant(ShadcnTypographyVariant::H3),
                        );
                        ui.add_space(16.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Enter your email address.")
                                .variant(ShadcnTypographyVariant::Muted),
                        );

                            ui.add_space(24.0);
                        },
                    );
                });
            },
        );
    }
}

impl App for TypographyDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    self.render(ui);
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let mut options = icon::native_options();
    options.viewport = options.viewport.with_inner_size(egui::vec2(840.0, 640.0));
    eframe::run_native(
        "Typography demo",
        options,
        Box::new(|_cc| Ok(Box::new(TypographyDemo::new()))),
    )
}
