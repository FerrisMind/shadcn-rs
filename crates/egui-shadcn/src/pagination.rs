//! Pagination component - page navigation controls.

use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::theme::Theme;
use egui::{Color32, Painter, Pos2, Response, Sense, Stroke, Ui, Vec2, WidgetText, pos2, vec2};
use std::fmt::{self, Debug};

pub struct OnPageChange<'a>(pub Box<dyn FnMut(usize) + 'a>);

impl<'a> Debug for OnPageChange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnPageChange").finish()
    }
}

#[derive(Debug)]
pub struct PaginationProps<'a> {
    pub total_pages: usize,
    pub current_page: &'a mut usize,
    pub on_page_change: Option<OnPageChange<'a>>,
}

impl<'a> PaginationProps<'a> {
    pub fn new(total_pages: usize, current_page: &'a mut usize) -> Self {
        Self {
            total_pages,
            current_page,
            on_page_change: None,
        }
    }

    pub fn with_on_page_change(mut self, callback: impl FnMut(usize) + 'a) -> Self {
        self.on_page_change = Some(OnPageChange(Box::new(callback)));
        self
    }
}

#[derive(Clone, Debug)]
pub struct PaginationLinkProps {
    pub page: usize,
    pub label: WidgetText,
    pub size: ButtonSize,
    pub enabled: bool,
    pub is_active: bool,
}

impl PaginationLinkProps {
    pub fn new(page: usize, label: impl Into<WidgetText>) -> Self {
        Self {
            page,
            label: label.into(),
            size: ButtonSize::Icon,
            enabled: true,
            is_active: false,
        }
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }
}

pub fn pagination<R>(
    ui: &mut Ui,
    mut props: PaginationProps<'_>,
    add_contents: impl FnOnce(&mut Ui, &mut PaginationProps<'_>) -> R,
) -> R {
    let total_pages = props.total_pages.max(1);
    if *props.current_page < 1 {
        *props.current_page = 1;
    } else if *props.current_page > total_pages {
        *props.current_page = total_pages;
    }

    ui.horizontal_centered(|ui| add_contents(ui, &mut props))
        .inner
}

pub fn pagination_content<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
        add_contents(ui)
    })
    .inner
}

pub fn pagination_item<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    add_contents(ui)
}

pub fn pagination_link<'a>(
    ui: &mut Ui,
    theme: &Theme,
    props: &mut PaginationProps<'a>,
    link: PaginationLinkProps,
) -> Response {
    let total_pages = props.total_pages.max(1);
    let page = link.page.clamp(1, total_pages);
    let is_active = link.is_active || page == *props.current_page;
    let variant = if is_active {
        ButtonVariant::Outline
    } else {
        ButtonVariant::Ghost
    };

    let response = Button::new(link.label)
        .variant(variant)
        .size(link.size)
        .enabled(link.enabled)
        .show(ui, theme);

    if response.clicked() && link.enabled {
        set_page(props, page);
    }

    response
}

pub fn pagination_previous<'a>(
    ui: &mut Ui,
    theme: &Theme,
    props: &mut PaginationProps<'a>,
) -> Response {
    let total_pages = props.total_pages.max(1);
    let current = (*props.current_page).clamp(1, total_pages);
    let enabled = current > 1;
    let target = current.saturating_sub(1).max(1);

    let response = Button::new("Previous")
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Default)
        .icon(&icon_chevron_left)
        .enabled(enabled)
        .show(ui, theme);

    if response.clicked() && enabled {
        set_page(props, target);
    }

    response
}

pub fn pagination_next<'a>(
    ui: &mut Ui,
    theme: &Theme,
    props: &mut PaginationProps<'a>,
) -> Response {
    let total_pages = props.total_pages.max(1);
    let current = (*props.current_page).clamp(1, total_pages);
    let enabled = current < total_pages;
    let target = (current + 1).min(total_pages);

    let response = Button::new("Next")
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Default)
        .trailing_icon(&icon_chevron_right)
        .enabled(enabled)
        .show(ui, theme);

    if response.clicked() && enabled {
        set_page(props, target);
    }

    response
}

pub fn pagination_ellipsis(ui: &mut Ui, theme: &Theme) -> Response {
    let size = ButtonSize::Icon.height();
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let center = rect.center();
    icon_more_horizontal(
        ui.painter(),
        center,
        ButtonSize::Icon.icon_size(),
        theme.palette.muted_foreground,
    );
    response
}

fn set_page(props: &mut PaginationProps<'_>, page: usize) {
    let total_pages = props.total_pages.max(1);
    let clamped = page.clamp(1, total_pages);
    if clamped == *props.current_page {
        return;
    }
    *props.current_page = clamped;
    if let Some(callback) = props.on_page_change.as_mut() {
        (callback.0)(clamped);
    }
}

fn stroke_width(size: f32) -> f32 {
    (size * 0.12).clamp(1.5, 2.5)
}

fn icon_chevron_left(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new(stroke_width(size), color);
    let half_x = size * 0.18;
    let half_y = size * 0.22;
    painter.line_segment(
        [
            pos2(center.x + half_x, center.y - half_y),
            pos2(center.x - half_x, center.y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x - half_x, center.y),
            pos2(center.x + half_x, center.y + half_y),
        ],
        stroke,
    );
}

fn icon_chevron_right(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new(stroke_width(size), color);
    let half_x = size * 0.18;
    let half_y = size * 0.22;
    painter.line_segment(
        [
            pos2(center.x - half_x, center.y - half_y),
            pos2(center.x + half_x, center.y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x + half_x, center.y),
            pos2(center.x - half_x, center.y + half_y),
        ],
        stroke,
    );
}

fn icon_more_horizontal(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let radius = (size * 0.12).max(1.0);
    let gap = size * 0.28;
    painter.circle_filled(pos2(center.x - gap, center.y), radius, color);
    painter.circle_filled(center, radius, color);
    painter.circle_filled(pos2(center.x + gap, center.y), radius, color);
}
