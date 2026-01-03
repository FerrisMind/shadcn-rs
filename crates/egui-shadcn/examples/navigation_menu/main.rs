#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CornerRadius, Margin, RichText, vec2};
use egui_shadcn::{
    NavigationMenuContentProps, NavigationMenuLinkProps, NavigationMenuProps, Theme,
    navigation_menu, navigation_menu_content, navigation_menu_item, navigation_menu_link,
    navigation_menu_list, navigation_menu_trigger,
};

struct NavigationMenuDemo {
    theme: Theme,
}

impl NavigationMenuDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

struct ComponentItem {
    title: &'static str,
    description: &'static str,
}

const COMPONENTS: [ComponentItem; 6] = [
    ComponentItem {
        title: "Alert Dialog",
        description: "A modal dialog that interrupts the user with important content and expects a response.",
    },
    ComponentItem {
        title: "Hover Card",
        description: "For sighted users to preview content available behind a link.",
    },
    ComponentItem {
        title: "Progress",
        description: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
    },
    ComponentItem {
        title: "Scroll Area",
        description: "Visually or semantically separates content.",
    },
    ComponentItem {
        title: "Tabs",
        description: "A set of layered sections of content that are displayed one at a time.",
    },
    ComponentItem {
        title: "Tooltip",
        description: "A popup that displays information related to an element when it receives focus.",
    },
];

impl App for NavigationMenuDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(40.0);

                navigation_menu(
                    ui,
                    &self.theme,
                    NavigationMenuProps::new(ui.make_persistent_id("nav-menu-demo")),
                    |ui, nav_ctx| {
                        navigation_menu_list(ui, nav_ctx, |ui, nav_ctx| {
                            navigation_menu_item(ui, nav_ctx, "getting-started", |ui, item_ctx| {
                                navigation_menu_trigger(
                                    ui,
                                    &self.theme,
                                    nav_ctx,
                                    item_ctx,
                                    "Getting started",
                                );
                                let _ = navigation_menu_content(
                                    ui,
                                    &self.theme,
                                    nav_ctx,
                                    item_ctx,
                                    NavigationMenuContentProps::new().width(520.0),
                                    |content_ui| {
                                        content_ui.spacing_mut().item_spacing = vec2(16.0, 0.0);
                                        content_ui.horizontal(|row| {
                                            nav_card(
                                                row,
                                                &self.theme,
                                                "shadcn/ui",
                                                "Beautifully designed components built with Rust and egui.",
                                            );
                                            row.vertical(|list| {
                                                list.spacing_mut().item_spacing.y = 6.0;
                                                nav_list_item(
                                                    list,
                                                    &self.theme,
                                                    "Introduction",
                                                    "Reusable components built with egui.",
                                                );
                                                nav_list_item(
                                                    list,
                                                    &self.theme,
                                                    "Installation",
                                                    "How to install dependencies and structure your app.",
                                                );
                                                nav_list_item(
                                                    list,
                                                    &self.theme,
                                                    "Typography",
                                                    "Styles for headings, paragraphs, lists.",
                                                );
                                            });
                                        });
                                    },
                                );
                            });

                            navigation_menu_item(ui, nav_ctx, "components", |ui, item_ctx| {
                                navigation_menu_trigger(
                                    ui,
                                    &self.theme,
                                    nav_ctx,
                                    item_ctx,
                                    "Components",
                                );
                                let _ = navigation_menu_content(
                                    ui,
                                    &self.theme,
                                    nav_ctx,
                                    item_ctx,
                                    NavigationMenuContentProps::new().width(600.0),
                                    |content_ui| {
                                        content_ui.columns(2, |cols| {
                                            cols[0].spacing_mut().item_spacing.y = 6.0;
                                            cols[1].spacing_mut().item_spacing.y = 6.0;
                                            cols[0].set_min_width(260.0);
                                            cols[1].set_min_width(260.0);

                                            for (index, item) in COMPONENTS.iter().enumerate() {
                                                let col = if index % 2 == 0 {
                                                    &mut cols[0]
                                                } else {
                                                    &mut cols[1]
                                                };
                                                nav_list_item(
                                                    col,
                                                    &self.theme,
                                                    item.title,
                                                    item.description,
                                                );
                                            }
                                        });
                                    },
                                );
                            });

                            let _ = navigation_menu_link(
                                ui,
                                &self.theme,
                                NavigationMenuLinkProps::new()
                                    .min_width(120.0)
                                    .min_height(32.0)
                                    .padding(Margin::symmetric(12, 6))
                                    .rounding(CornerRadius::same(6)),
                                |link_ui, state| {
                                    let text_color = if state.hovered {
                                        self.theme.palette.accent_foreground
                                    } else {
                                        self.theme.palette.foreground
                                    };
                                    link_ui.label(
                                        RichText::new("Documentation")
                                            .size(13.0)
                                            .color(text_color),
                                    );
                                },
                            );
                        })
                    },
                );
            });
        });
    }
}

fn nav_list_item(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    let _ = navigation_menu_link(
        ui,
        theme,
        NavigationMenuLinkProps::new()
            .min_width(ui.available_width())
            .min_height(60.0)
            .padding(Margin::symmetric(12, 8)),
        |content_ui, state| {
            let title_color = if state.hovered {
                theme.palette.accent_foreground
            } else {
                theme.palette.foreground
            };
            let desc_color = if state.hovered {
                theme.palette.accent_foreground.gamma_multiply(0.85)
            } else {
                theme.palette.muted_foreground
            };
            content_ui.label(RichText::new(title).size(13.0).strong().color(title_color));
            content_ui.label(RichText::new(description).size(12.0).color(desc_color));
        },
    );
}

fn nav_card(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    let _ = navigation_menu_link(
        ui,
        theme,
        NavigationMenuLinkProps::new()
            .min_width(200.0)
            .min_height(160.0)
            .padding(Margin::same(12))
            .rounding(CornerRadius::same(8))
            .active(true),
        |content_ui, state| {
            let title_color = if state.hovered {
                theme.palette.accent_foreground
            } else {
                theme.palette.foreground
            };
            content_ui.add_space(8.0);
            content_ui.label(RichText::new(title).size(15.0).strong().color(title_color));
            content_ui.label(
                RichText::new(description)
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
        },
    );
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Navigation Menu example",
        options,
        Box::new(|_cc| Ok(Box::new(NavigationMenuDemo::new()))),
    )
}
