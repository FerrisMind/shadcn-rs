#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{
    FontData, FontDefinitions, FontFamily, FontId, RichText, text::LayoutJob, text::TextFormat,
};
use egui_shadcn::{
    Button, ButtonRadius, ButtonSize, ButtonVariant, ControlSize, ControlVariant, Label,
    ScrollAreaProps, ScrollAreaType, ScrollDirection, SeparatorProps, Theme, button, scroll_area,
    separator,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct ButtonDemo {
    theme: Theme,
}

impl ButtonDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded");
    let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
    if already_set {
        return;
    }

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn lucide_icon(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string()).font(FontId::new(size, FontFamily::Proportional))
}

fn icon_with_text(icon: Icon, icon_size: f32, text: &str) -> egui::WidgetText {
    let mut job = LayoutJob::default();
    job.append(
        &icon.unicode().to_string(),
        0.0,
        TextFormat {
            font_id: FontId::new(icon_size, FontFamily::Proportional),
            ..Default::default()
        },
    );
    job.append(
        &format!(" {}", text),
        0.0,
        TextFormat {
            font_id: FontId::new(14.0, FontFamily::Proportional),
            ..Default::default()
        },
    );
    job.into()
}

#[derive(Clone, Copy)]
enum DemoKind {
    Primary,
    OutlineText,
    Secondary,
    Ghost,
    Destructive,
    Link,
    OutlineIcon,
    SizeSmText,
    SizeSmIcon,
    SizeMdText,
    SizeMdIcon,
    SizeLgText,
    SizeLgIcon,
    IconCircular,
    OutlineLeadingIcon,
    OutlineIconRounded,
    Loading,
    DefaultSolid,
}

fn demo_tile(ui: &mut egui::Ui, theme: &Theme, title: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|col| {
        col.spacing_mut().item_spacing.y = 8.0;
        Label::new(title).size(ControlSize::Sm).show(col, theme);
        add(col);
    });
}

fn render_demo(ui: &mut egui::Ui, theme: &Theme, kind: DemoKind) {
    match kind {
        DemoKind::Primary => {
            let _ = button(
                ui,
                theme,
                "Button",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::OutlineText => {
            let _ = button(
                ui,
                theme,
                "Outline",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::Secondary => {
            let _ = button(
                ui,
                theme,
                "Secondary",
                ControlVariant::Secondary,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::Ghost => {
            let _ = button(
                ui,
                theme,
                "Ghost",
                ControlVariant::Ghost,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::Destructive => {
            let _ = button(
                ui,
                theme,
                "Destructive",
                ControlVariant::Destructive,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::Link => {
            let _ = button(
                ui,
                theme,
                "Link",
                ControlVariant::Link,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::OutlineIcon => {
            let _ = button(
                ui,
                theme,
                lucide_icon(Icon::ArrowUp, 16.0),
                ControlVariant::Outline,
                ControlSize::Icon,
                true,
            )
            .on_hover_text("Submit");
        }
        DemoKind::SizeSmText => {
            let _ = button(
                ui,
                theme,
                "Small",
                ControlVariant::Outline,
                ControlSize::Sm,
                true,
            );
        }
        DemoKind::SizeSmIcon => {
            let _ = button(
                ui,
                theme,
                lucide_icon(Icon::ArrowUpRight, 16.0),
                ControlVariant::Outline,
                ControlSize::IconSm,
                true,
            )
            .on_hover_text("Submit");
        }
        DemoKind::SizeMdText => {
            let _ = button(
                ui,
                theme,
                "Default",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            );
        }
        DemoKind::SizeMdIcon => {
            let _ = button(
                ui,
                theme,
                lucide_icon(Icon::ArrowUpRight, 16.0),
                ControlVariant::Outline,
                ControlSize::Icon,
                true,
            )
            .on_hover_text("Submit");
        }
        DemoKind::SizeLgText => {
            let _ = button(
                ui,
                theme,
                "Large",
                ControlVariant::Outline,
                ControlSize::Lg,
                true,
            );
        }
        DemoKind::SizeLgIcon => {
            let _ = button(
                ui,
                theme,
                lucide_icon(Icon::ArrowUpRight, 16.0),
                ControlVariant::Outline,
                ControlSize::IconLg,
                true,
            )
            .on_hover_text("Submit");
        }
        DemoKind::IconCircular => {
            let _ = button(
                ui,
                theme,
                lucide_icon(Icon::CircleFadingArrowUp, 16.0),
                ControlVariant::Outline,
                ControlSize::Icon,
                true,
            );
        }
        DemoKind::OutlineLeadingIcon => {
            let _ = Button::new(icon_with_text(Icon::GitBranch, 16.0, "New Branch"))
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Sm)
                .show(ui, theme);
        }
        DemoKind::OutlineIconRounded => {
            let _ = Button::new(lucide_icon(Icon::ArrowUp, 16.0))
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Icon)
                .radius(ButtonRadius::Full)
                .show(ui, theme);
        }
        DemoKind::Loading => {
            let _ = Button::new("Submit")
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Sm)
                .loading(true)
                .enabled(false)
                .show(ui, theme);
        }
        DemoKind::DefaultSolid => {
            let _ = Button::new("Login")
                .variant(ButtonVariant::Default)
                .size(ButtonSize::Default)
                .show(ui, theme);
        }
    }
}

impl App for ButtonDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            let center = panel_rect.center();
            let max_width = panel_rect.width().min(960.0);

            // Center content both horizontally and vertically in the window.
            egui::Area::new("button_demo_center".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .fixed_pos(center)
                .show(ui.ctx(), |area_ui| {
                    area_ui.set_max_width(max_width);

                    area_ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 16.0;

                        let sections: &[(&str, &[(&str, DemoKind)])] = &[
                            (
                                "Variants",
                                &[
                                    ("Primary (default)", DemoKind::Primary),
                                    ("Outline (text)", DemoKind::OutlineText),
                                    ("Secondary", DemoKind::Secondary),
                                    ("Ghost", DemoKind::Ghost),
                                    ("Destructive", DemoKind::Destructive),
                                    ("Link style", DemoKind::Link),
                                ],
                            ),
                            (
                                "Icons",
                                &[
                                    ("Outline (icon-only)", DemoKind::OutlineIcon),
                                    ("Icon button (outline, circular)", DemoKind::IconCircular),
                                    ("Outline with leading icon", DemoKind::OutlineLeadingIcon),
                                    ("Outline icon (rounded full)", DemoKind::OutlineIconRounded),
                                ],
                            ),
                            (
                                "Sizes",
                                &[
                                    ("Size Small (text)", DemoKind::SizeSmText),
                                    ("Size Small (icon)", DemoKind::SizeSmIcon),
                                    ("Size Default (text)", DemoKind::SizeMdText),
                                    ("Size Default (icon)", DemoKind::SizeMdIcon),
                                    ("Size Large (text)", DemoKind::SizeLgText),
                                    ("Size Large (icon)", DemoKind::SizeLgIcon),
                                ],
                            ),
                            (
                                "States",
                                &[
                                    ("Loading state (disabled)", DemoKind::Loading),
                                    ("Default variant (solid)", DemoKind::DefaultSolid),
                                ],
                            ),
                        ];

                        for (idx, (section_title, demos)) in sections.iter().enumerate() {
                            Label::new(*section_title)
                                .size(ControlSize::Sm)
                                .show(ui, &self.theme);

                            ui.horizontal_wrapped(|row| {
                                row.spacing_mut().item_spacing = egui::Vec2::new(16.0, 24.0);
                                for (title, kind) in *demos {
                                    row.allocate_ui(egui::Vec2::new(220.0, 96.0), |tile| {
                                        demo_tile(tile, &self.theme, title, |inner| {
                                            render_demo(inner, &self.theme, *kind);
                                        });
                                    });
                                }
                            });

                            if idx + 1 < sections.len() {
                                ui.add_space(8.0);
                                separator(ui, &self.theme, SeparatorProps::default());
                                ui.add_space(8.0);
                            }
                        }
                    });
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Button example",
        options,
        Box::new(|_cc| Ok(Box::new(ButtonDemo::new()))),
    )
}
