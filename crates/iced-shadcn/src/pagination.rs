use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button};
use crate::theme::Theme;

#[derive(Clone, Debug)]
pub struct PaginationProps {
    pub total_pages: usize,
    pub current_page: usize,
}

impl PaginationProps {
    pub fn new(total_pages: usize, current_page: usize) -> Self {
        Self {
            total_pages,
            current_page,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PaginationLinkProps {
    pub page: usize,
    pub label: String,
    pub size: ButtonSize,
    pub enabled: bool,
    pub is_active: bool,
}

impl PaginationLinkProps {
    pub fn new(page: usize, label: impl Into<String>) -> Self {
        Self {
            page,
            label: label.into(),
            size: ButtonSize::Size1,
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

    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }
}

pub enum PaginationItem {
    Link(PaginationLinkProps),
    Previous,
    Next,
    Ellipsis,
}

pub fn pagination_link(page: usize, label: impl Into<String>) -> PaginationItem {
    PaginationItem::Link(PaginationLinkProps::new(page, label))
}

pub fn pagination_previous() -> PaginationItem {
    PaginationItem::Previous
}

pub fn pagination_next() -> PaginationItem {
    PaginationItem::Next
}

pub fn pagination_ellipsis() -> PaginationItem {
    PaginationItem::Ellipsis
}

pub fn pagination_content<'a, Message: Clone + 'a>(
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    row(items).spacing(4).align_y(Alignment::Center).into()
}

pub fn pagination_item<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    content.into()
}

pub fn pagination<'a, Message: Clone + 'a, F>(
    items: Vec<PaginationItem>,
    props: PaginationProps,
    on_page_change: Option<F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(usize) -> Message + 'a,
{
    let total_pages = props.total_pages.max(1);
    let current_page = props.current_page.clamp(1, total_pages);
    let on_page_change = on_page_change.as_ref();

    let mut children: Vec<Element<'a, Message>> = Vec::new();

    for item in items {
        let element = match item {
            PaginationItem::Link(link) => {
                let page = link.page.clamp(1, total_pages);
                let active = link.is_active || page == current_page;
                let variant = if active {
                    ButtonVariant::Outline
                } else {
                    ButtonVariant::Ghost
                };
                let enabled = link.enabled && on_page_change.is_some();
                let on_press = on_page_change.map(|f| f(page)).filter(|_| enabled);

                button(
                    link.label,
                    on_press,
                    ButtonProps::new()
                        .variant(variant)
                        .size(link.size)
                        .disabled(!enabled),
                    theme,
                )
                .into()
            }
            PaginationItem::Previous => {
                let enabled = current_page > 1 && on_page_change.is_some();
                let target = current_page.saturating_sub(1).max(1);
                let on_press = on_page_change.map(|f| f(target)).filter(|_| enabled);

                button(
                    "Previous",
                    on_press,
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size2)
                        .disabled(!enabled),
                    theme,
                )
                .into()
            }
            PaginationItem::Next => {
                let enabled = current_page < total_pages && on_page_change.is_some();
                let target = (current_page + 1).min(total_pages);
                let on_press = on_page_change.map(|f| f(target)).filter(|_| enabled);

                button(
                    "Next",
                    on_press,
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size2)
                        .disabled(!enabled),
                    theme,
                )
                .into()
            }
            PaginationItem::Ellipsis => {
                let ellipsis = text("…")
                    .size(12)
                    .style(move |_t| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    });
                container(ellipsis)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .center_x(Length::Fixed(24.0))
                    .center_y(Length::Fixed(24.0))
                    .into()
            }
        };

        children.push(element);
    }

    container(row(children).spacing(4).align_y(Alignment::Center))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
