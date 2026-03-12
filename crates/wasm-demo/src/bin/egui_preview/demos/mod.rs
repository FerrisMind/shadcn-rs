mod accordion;
mod alert;
mod alert_dialog;
mod aspect_ratio;
mod avatar;
mod badge;
mod breadcrumb;
mod button;
mod button_group;
mod calendar;
mod card;
mod carousel;
mod chart;
mod checkbox;
mod collapsible;
mod combobox;
mod command;
mod context_menu;
mod data_table;
mod date_picker;
mod dialog;
mod dropdown_menu;
mod fallback;
mod form;
mod hover_card;
mod input;
mod input_otp_preview;
mod kbd_preview;
mod label;
mod navigation_menu;
mod pagination;
mod popover;
mod progress;
mod radio_preview;
mod resizable;
mod scroll_area_preview;
mod select_preview;
mod separator;
mod sheet;
mod sidebar;
mod skeleton;
mod slider;
mod spinner;
mod switch_preview;
mod table_preview;
mod tabs;
mod textarea;
mod toast;
mod toggle;
mod toggle_group;
mod tooltip;
mod typography_preview;

use super::app::EguiPreviewApp;
use eframe::egui::Ui;

pub fn render_component_preview(app: &mut EguiPreviewApp, ui: &mut Ui, slug: &str, compact: bool) {
    match slug {
        "accordion" => accordion::render(app, ui, compact),
        "alert" => alert::render(app, ui, compact),
        "alert_dialog" => alert_dialog::render(app, ui),
        "aspect_ratio" => aspect_ratio::render(app, ui, compact),
        "avatar" => avatar::render(app, ui, compact),
        "badge" => badge::render(app, ui, slug),
        "breadcrumb" => breadcrumb::render(app, ui, compact),
        "button" => button::render(app, ui),
        "button_group" => button_group::render(app, ui, compact),
        "calendar" => calendar::render(app, ui),
        "card" => card::render(app, ui, compact),
        "carousel" => carousel::render(app, ui, compact),
        "chart" => chart::render(app, ui, compact),
        "checkbox" => checkbox::render(app, ui),
        "collapsible" => collapsible::render(app, ui, compact),
        "combobox" => combobox::render(app, ui, compact),
        "command" => command::render(app, ui, compact),
        "context_menu" => context_menu::render(app, ui, compact),
        "data_table" => data_table::render(app, ui, compact),
        "date_picker" => date_picker::render(app, ui, compact),
        "dialog" => dialog::render(app, ui, compact),
        "dropdown_menu" => dropdown_menu::render(app, ui, compact),
        "form" => form::render(app, ui, compact),
        "hover_card" => hover_card::render(app, ui, compact),
        "input" => input::render(app, ui, compact),
        "input_otp" => input_otp_preview::render(app, ui, compact),
        "kbd" => kbd_preview::render(app, ui, compact),
        "label" => label::render(app, ui),
        "navigation_menu" => navigation_menu::render(app, ui, compact),
        "pagination" => pagination::render(app, ui),
        "popover" => popover::render(app, ui, compact),
        "progress" => progress::render(app, ui, compact),
        "radio" => radio_preview::render(app, ui, compact),
        "resizable" => resizable::render(app, ui, compact),
        "scroll_area" => scroll_area_preview::render(app, ui, compact),
        "select" => select_preview::render(app, ui, compact),
        "separator" => separator::render(app, ui),
        "sheet" => sheet::render(app, ui, compact),
        "sidebar" => sidebar::render(app, ui, compact),
        "skeleton" => skeleton::render(app, ui, compact),
        "slider" => slider::render(app, ui),
        "spinner" => spinner::render(app, ui, compact),
        "switch" => switch_preview::render(app, ui),
        "table" => table_preview::render(app, ui, compact),
        "tabs" => tabs::render(app, ui),
        "textarea" => textarea::render(app, ui, compact),
        "toast" => toast::render(app, ui, compact),
        "toggle" => toggle::render(app, ui, compact),
        "toggle_group" => toggle_group::render(app, ui, compact),
        "tooltip" => tooltip::render(app, ui, compact),
        "typography" => typography_preview::render(app, ui, compact),
        _ => fallback::render(app, ui, slug, compact),
    }
}
