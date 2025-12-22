#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CornerRadius, FontId};
use egui_shadcn::{
    CardProps, CardVariant, ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize, ScrollAreaType,
    ScrollDirection, ShadcnTypographyVariant, TextAlign, Theme, TypographyProps, blockquote, card,
    code, link, scroll_area, text, typography,
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
        let rows = [
            [("Empty", false), ("Overflowing", false)],
            [("Modest", false), ("Satisfied", false)],
            [("Full", false), ("Ecstatic", false)],
        ];

        let available = ui.available_width();
        let col_w = (available / 2.0).floor();
        let row_h = 28.0;
        let border = egui::Stroke::new(1.0, self.theme.palette.border);

        let mut row_index = 0usize;
        let mut paint_row = |ui: &mut egui::Ui, cells: [(&str, bool); 2]| {
            let bg = if row_index % 2 == 1 {
                self.theme.palette.muted
            } else {
                egui::Color32::TRANSPARENT
            };
            row_index += 1;

            ui.horizontal(|ui| {
                for (text_value, bold) in cells {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(col_w, row_h), egui::Sense::hover());
                    ui.painter().rect_filled(rect, CornerRadius::same(0), bg);
                    ui.painter().rect_stroke(
                        rect,
                        CornerRadius::same(0),
                        border,
                        egui::StrokeKind::Inside,
                    );
                    let mut rich = egui::RichText::new(text_value).font(FontId::proportional(14.0));
                    if bold {
                        rich = rich.strong();
                    }
                    rich = rich.color(self.theme.palette.foreground);
                    let inner = rect.shrink2(egui::vec2(8.0, 6.0));
                    ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
                        ui.add(egui::Label::new(rich).wrap());
                    });
                }
            });
        };

        paint_row(ui, header);
        for row in rows {
            paint_row(ui, row);
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let card_size = egui::vec2(760.0, 560.0);
        card(
            ui,
            &self.theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_padding(egui::vec2(16.0, 16.0))
                .with_rounding(CornerRadius::same(12))
                .with_shadow(true),
            |card_ui| {
                card_ui.set_min_size(card_size);
                card_ui.set_max_size(card_size);

                scroll_area(
                    card_ui,
                    &self.theme,
                    ScrollAreaProps {
                        scroll_type: ScrollAreaType::Auto,
                        direction: ScrollDirection::Vertical,
                        size: ScrollAreaSize::Size2,
                        radius: ScrollAreaRadius::Small,
                        max_size: Some(card_size),
                        ..Default::default()
                    },
                    |ui| {
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Taxing Laughter: The Joke Tax Chronicles")
                                .variant(ShadcnTypographyVariant::H1)
                                .align(TextAlign::Center),
                        );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "Once upon a time, in a far-off land, there was a very lazy king who\nspent all day lounging on his throne. One day, his advisors came to him\nwith a problem: the kingdom was running out of money.",
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
                                "\"After all,\" he said, \"everyone enjoys a good joke, so\nit's only fair that they should pay for the privilege.\"",
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
                                "The king's subjects were not amused. They grumbled and complained,\nbut the king was firm:",
                            ),
                        );

                        ui.add_space(24.0);
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.add_space(24.0);
                            ui.vertical(|ui| {
                                for item in [
                                    "1st level of puns: 5 gold coins",
                                    "2nd level of jokes: 10 gold coins",
                                    "3rd level of one-liners : 20 gold coins",
                                ] {
                                    ui.horizontal(|ui| {
                                        ui.label("•");
                                        typography(ui, &self.theme, TypographyProps::new(item));
                                    });
                                    ui.add_space(8.0);
                                }
                            });
                        });

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "As a result, people stopped telling jokes, and the kingdom fell into a\ngloom. But there was one person who refused to let the king's\nfoolishness get him down: a court jester named Jokester.",
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
                                "Jokester began sneaking into the castle in the middle of the night and\nleaving jokes all over the place: under the king's pillow, in his\nsoup, even in the royal toilet. The king was furious, but he\ncouldn't seem to stop Jokester.",
                            ),
                        );

                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "And then, one day, the people of the kingdom discovered that the jokes\nleft by Jokester were so funny that they couldn't help but laugh.\nAnd once they started laughing, they couldn't stop.",
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
                                "The people of the kingdom, feeling uplifted by the laughter, started to\ntell jokes and puns again, and soon the entire kingdom was in on the\njoke.",
                            ),
                        );

                        ui.add_space(24.0);
                        ui.add_space(8.0);
                        self.render_table(ui);
                        ui.add_space(24.0);

                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "The king, seeing how much happier his subjects were, realized the error\nof his ways and repealed the joke tax. Jokester was declared a hero, and\nthe kingdom lived happily ever after.",
                            ),
                        );
                        ui.add_space(24.0);
                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new(
                                "The moral of the story is: never underestimate the power of a good laugh\nand always be careful of bad ideas.",
                            ),
                        );

                        ui.add_space(32.0);
                        ui.separator();
                        ui.add_space(24.0);

                        typography(
                            ui,
                            &self.theme,
                            TypographyProps::new("Inline code")
                                .variant(ShadcnTypographyVariant::H4),
                        );
                        ui.add_space(16.0);
                        let _ = code(
                            ui,
                            &self.theme,
                            egui_shadcn::CodeProps::new("@radix-ui/react-alert-dialog")
                                .variant(egui_shadcn::CodeVariant::Soft),
                        );
                    },
                );
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
