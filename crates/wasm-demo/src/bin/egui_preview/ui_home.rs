use super::app::{ComponentTab, EguiPreviewApp, InstallTab, Screen, ThemeMode};
use super::catalog::{COMPONENT_SLUGS, component_title};
use super::demos::render_component_preview;
use eframe::egui::{self, Align2, Color32, FontFamily, FontId, RichText, Stroke, Ui};
use egui_shadcn::{
    Button, ButtonSize, ButtonVariant, HeadingProps, Input as ShadInput, InputSize,
    ScrollAreaProps, ScrollAreaSize, ScrollDirection, SeparatorOrientation, SeparatorProps,
    TextAlign, TextProps, Theme, heading, scroll_area, separator, text,
};
use lucide_icons::Icon;

const BRAND_ICON_DARK: egui::ImageSource<'static> =
    egui::include_image!("../../../../egui-shadcn/assets/icons/shadcn-egui/icon-white.svg");
const BRAND_ICON_LIGHT: egui::ImageSource<'static> =
    egui::include_image!("../../../../egui-shadcn/assets/icons/shadcn-egui/icon-black.svg");
const HERO_ICON_DARK: egui::ImageSource<'static> =
    egui::include_image!("../../../../../.github/assets/icon-white.svg");
const HERO_ICON_LIGHT: egui::ImageSource<'static> =
    egui::include_image!("../../../../../.github/assets/icon-black.svg");

pub fn icon_text(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string())
        .font(FontId::new(size, FontFamily::Name("lucide".into())))
}

pub fn render_topbar(app: &mut EguiPreviewApp, ui: &mut Ui) {
    ui.add_space(12.0);
    ui.horizontal(|row| {
        row.add_space(16.0);
        row.spacing_mut().item_spacing.x = 12.0;
        let brand_icon = match app.theme_mode {
            ThemeMode::Dark => BRAND_ICON_DARK,
            ThemeMode::Light => BRAND_ICON_LIGHT,
        };
        let _ = row.add_sized(
            egui::vec2(36.0, 36.0),
            egui::Image::new(brand_icon).fit_to_exact_size(egui::vec2(36.0, 36.0)),
        );
        if matches!(app.screen, Screen::Component(_))
            && Button::new(icon_text(Icon::ArrowLeft, 16.0))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon)
                .show(row, &app.theme)
                .clicked()
        {
            app.screen = Screen::Home;
            app.tab = ComponentTab::Demo;
            app.install_tab = InstallTab::Automatic;
        }
        row.with_layout(egui::Layout::right_to_left(egui::Align::Center), |right| {
            right.add_space(16.0);
            let theme_icon = match app.theme_mode {
                ThemeMode::Dark => Icon::Sun,
                ThemeMode::Light => Icon::Moon,
            };
            if Button::new(icon_text(theme_icon, 16.0))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon)
                .show(right, &app.theme)
                .clicked()
            {
                app.toggle_theme();
            }
            if Button::new(icon_text(Icon::Github, 16.0))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon)
                .show(right, &app.theme)
                .clicked()
            {
                right.ctx().open_url(egui::OpenUrl {
                    url: "https://github.com/FerrisMind/shadcn-rs".to_owned(),
                    new_tab: true,
                });
            }
        });
    });
    ui.add_space(8.0);
}

pub fn render_home(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let theme = app.theme.clone();
    scroll_area(
        ui,
        &theme,
        ScrollAreaProps::default()
            .direction(ScrollDirection::Vertical)
            .size(ScrollAreaSize::Size2),
        |ui| {
            let content_width = ui.available_width();
            let target_card_width = 360.0;
            let base_card_height = 300.0;
            let card_gap = 16.0;
            let grid_side_padding = 16.0;

            ui.add_space(68.0);
            ui.vertical(|content| {
                content.set_width(content_width);
                content.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
                let input_width = (content_width * 0.92).min(1200.0);

                let hero_icon_size = 128.0;
                let hero_text_width = 720.0;
                let hero_block_width = hero_icon_size + 14.0 + 1.0 + 16.0 + hero_text_width;
                content.with_layout(egui::Layout::top_down(egui::Align::Center), |center| {
                    let _ = center.allocate_ui_with_layout(
                        egui::vec2(hero_block_width, 0.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |hero_content| {
                            let hero_icon = match app.theme_mode {
                                ThemeMode::Dark => HERO_ICON_DARK,
                                ThemeMode::Light => HERO_ICON_LIGHT,
                            };
                            let _ = hero_content.add_sized(
                                egui::vec2(hero_icon_size, hero_icon_size),
                                egui::Image::new(hero_icon)
                                    .fit_to_exact_size(egui::vec2(hero_icon_size, hero_icon_size)),
                            );
                            hero_content.add_space(14.0);
                            let _ = separator(
                                hero_content,
                                &theme,
                                SeparatorProps::default()
                                    .orientation(SeparatorOrientation::Vertical)
                                    .length(hero_icon_size)
                                    .thickness(1.0),
                            );
                            hero_content.add_space(16.0);
                            hero_content.vertical(|text_col| {
                                text_col.set_min_width(hero_text_width);
                                text_col.set_max_width(hero_text_width);
                                let _ = heading(
                                    text_col,
                                    &theme,
                                    HeadingProps::new("shadcn-rs Components").size(56.0),
                                );
                                text_col.add_space(8.0);
                                let _ = heading(
                                    text_col,
                                    &theme,
                                    HeadingProps::new(
                                        "Accessible, customizable components for Rust GUI",
                                    )
                                    .size(20.0)
                                    .align(TextAlign::Left),
                                );
                                text_col.add_space(8.0);
                                let _ = text(
                                    text_col,
                                    &theme,
                                    TextProps::new(
                                        "Egui preview built with shadcn-rs components and Lucide icons",
                                    )
                                    .size(14.0)
                                    .align(TextAlign::Left)
                                    .color(egui_shadcn::TypographyColor::Muted),
                                );
                            });
                        },
                    );
                });
                content.add_space(64.0);

                let search_id = content.make_persistent_id("preview-search");
                let _ = content.allocate_ui_with_layout(
                    egui::vec2(content_width, 0.0),
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |row| {
                        let _ = ShadInput::new(search_id)
                            .placeholder("Search components...")
                            .size(InputSize::Size2)
                            .width(input_width)
                            .left_slot(|painter, rect, color| {
                                let icon_size = 14.0;
                                painter.text(
                                    egui::pos2(rect.left() + 4.0 + icon_size * 0.5, rect.center().y),
                                    Align2::CENTER_CENTER,
                                    Icon::Search.unicode().to_string(),
                                    FontId::new(icon_size, FontFamily::Name("lucide".into())),
                                    color,
                                );
                            })
                            .show(row, &theme, &mut app.search);
                    },
                );
                content.add_space(32.0);

                let search_lc = app.search.to_lowercase();
                let mut filtered = Vec::new();
                for (index, slug) in COMPONENT_SLUGS.iter().enumerate() {
                    let title = component_title(slug);
                    if !search_lc.is_empty() && !title.to_lowercase().contains(&search_lc) {
                        continue;
                    }
                    filtered.push((index, *slug, title));
                }

                let viewport_width = content.clip_rect().width();
                let left_padding = grid_side_padding;
                let right_padding = grid_side_padding * 2.0;
                let grid_width = (viewport_width - left_padding - right_padding).max(0.0);
                let columns =
                    (((grid_width + card_gap) / (target_card_width + card_gap)).floor() as usize)
                        .max(1);
                let gaps_width = card_gap * (columns.saturating_sub(1) as f32);
                let card_width = ((grid_width - gaps_width) / columns as f32).max(1.0);
                let _ = content.allocate_ui_with_layout(
                    egui::vec2(viewport_width, 0.0),
                    egui::Layout::left_to_right(egui::Align::Min),
                    |row| {
                        row.spacing_mut().item_spacing.x = 0.0;
                        row.add_space(left_padding);
                        let _ = row.allocate_ui_with_layout(
                            egui::vec2(grid_width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |host| {
                                for row_items in filtered.chunks(columns) {
                                    let row_height = row_items
                                        .iter()
                                        .map(|(_, slug, _)| {
                                            preview_card_height(app, slug, base_card_height)
                                        })
                                        .fold(base_card_height, f32::max);

                                    host.horizontal(|grid_row| {
                                        grid_row.spacing_mut().item_spacing.x = card_gap;

                                        for (index, slug, title) in row_items {
                                            let current_card_height =
                                                preview_card_height(app, slug, base_card_height);
                                            let mut open_component = false;
                                            let _ = grid_row.allocate_ui_with_layout(
                                                egui::vec2(card_width, row_height),
                                                egui::Layout::top_down(egui::Align::Min),
                                                |cell| {
                                                    let _ = cell.allocate_ui_with_layout(
                                                        egui::vec2(card_width, current_card_height),
                                                        egui::Layout::top_down(egui::Align::Min),
                                                        |slot| {
                                                            egui_shadcn::card(
                                                                slot,
                                                                &theme,
                                                                egui_shadcn::CardProps::default()
                                                                    .variant(egui_shadcn::CardVariant::Classic)
                                                                    .padding(egui::vec2(16.0, 16.0))
                                                                    .tokens(preview_card_tokens(&theme))
                                                                    .shadow(false)
                                                                    .interactive(false),
                                                                |card_ui| {
                                                                    card_ui.set_width(card_width - 32.0);
                                                                    card_ui.set_height(current_card_height - 32.0);
                                                                    card_ui.horizontal(|top| {
                                                                        top.with_layout(
                                                                            egui::Layout::right_to_left(
                                                                                egui::Align::Center,
                                                                            ),
                                                                            |right| {
                                                                                let icon_btn = egui::Button::new(
                                                                                    icon_text(
                                                                                        Icon::SquareArrowOutUpRight,
                                                                                        14.0,
                                                                                    )
                                                                                    .color(
                                                                                        theme.palette
                                                                                            .muted_foreground,
                                                                                    ),
                                                                                )
                                                                                .frame(false)
                                                                                .min_size(egui::vec2(24.0, 24.0));
                                                                                if right.add(icon_btn).clicked()
                                                                                {
                                                                                    open_component = true;
                                                                                }
                                                                            },
                                                                        );
                                                                    });
                                                                    card_ui.add_space(6.0);
                                                                    card_ui.vertical_centered(|preview| {
                                                                        let _ = heading(
                                                                            preview,
                                                                            &theme,
                                                                            HeadingProps::new(title.clone())
                                                                                .size(28.0),
                                                                        );
                                                                        preview.add_space(14.0);
                                                                        preview.with_layout(
                                                                            egui::Layout::top_down(
                                                                                egui::Align::Center,
                                                                            ),
                                                                            |centered| {
                                                                                render_component_preview(
                                                                                    app, centered, slug, true,
                                                                                );
                                                                            },
                                                                        );
                                                                    });
                                                                },
                                                            );
                                                        },
                                                    );
                                                },
                                            );

                                            if open_component {
                                                app.screen = Screen::Component(*index);
                                                app.tab = ComponentTab::Demo;
                                                app.install_tab = InstallTab::Automatic;
                                            }
                                        }
                                    });

                                    host.add_space(card_gap);
                                }
                            },
                        );
                        row.add_space(right_padding);
                    },
                );
            });
            ui.add_space(card_gap);
        },
    );
}

fn preview_card_tokens(theme: &Theme) -> egui_shadcn::CardTokens {
    let p = &theme.palette;
    let transparent_shadow = egui::epaint::Shadow {
        offset: [0, 0],
        blur: 0,
        spread: 0,
        color: Color32::TRANSPARENT,
    };
    egui_shadcn::CardTokens {
        background: p.card,
        background_hover: p.accent,
        background_active: Color32::from_rgb(0x1A, 0x1A, 0x1A),
        stroke: Stroke::new(1.0_f32, p.border),
        stroke_hover: Stroke::new(1.0_f32, p.border),
        stroke_active: Stroke::new(1.0_f32, p.border),
        shadow_idle: transparent_shadow,
        shadow_hover: transparent_shadow,
        shadow_active: transparent_shadow,
        shadow_alpha: 0,
    }
}

fn preview_card_height(app: &EguiPreviewApp, slug: &str, base_card_height: f32) -> f32 {
    if slug == "accordion" && app.accordion_value.is_some() {
        return base_card_height + 76.0;
    }
    base_card_height
}
