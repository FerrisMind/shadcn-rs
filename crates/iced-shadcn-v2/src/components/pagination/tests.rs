//! Behavioral tests for the pagination component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::{Element, Length};

use super::*;
use crate::components::button::{ButtonSize, ButtonVariant};
use crate::theme::Theme;

use PaginationItem::{Ellipsis, Page};

#[derive(Clone, Debug)]
enum Message {
    PageChanged(usize),
}

#[test]
fn total_pages_rounds_up_and_never_hits_zero() {
    assert_eq!(total_pages(95, 10), 10);
    assert_eq!(total_pages(100, 10), 10);
    assert_eq!(total_pages(101, 10), 11);
    assert_eq!(total_pages(1, 10), 1);
    assert_eq!(total_pages(0, 10), 1);
    assert_eq!(total_pages(7, 0), 7);
}

#[test]
fn page_items_shows_every_page_when_the_range_is_small() {
    assert_eq!(
        page_items(1, 5, 1),
        [Page(1), Page(2), Page(3), Page(4), Page(5)],
    );
    assert_eq!(page_items(1, 1, 1), [Page(1)]);
}

#[test]
fn page_items_matches_bits_ui_windows() {
    // Leading window: 10 pages, page 1, one sibling.
    assert_eq!(
        page_items(1, 10, 1),
        [Page(1), Page(2), Page(3), Page(4), Ellipsis, Page(10)],
    );

    // Centered window.
    assert_eq!(
        page_items(5, 10, 1),
        [
            Page(1),
            Ellipsis,
            Page(4),
            Page(5),
            Page(6),
            Ellipsis,
            Page(10)
        ],
    );

    // Trailing window.
    assert_eq!(
        page_items(10, 10, 1),
        [Page(1), Ellipsis, Page(7), Page(8), Page(9), Page(10)],
    );
}

#[test]
fn page_items_respects_the_sibling_count() {
    assert_eq!(
        page_items(10, 20, 2),
        [
            Page(1),
            Ellipsis,
            Page(8),
            Page(9),
            Page(10),
            Page(11),
            Page(12),
            Ellipsis,
            Page(20),
        ],
    );
}

#[test]
fn page_items_normalizes_degenerate_inputs() {
    assert_eq!(page_items(7, 0, 1), [Page(1)]);
    assert_eq!(page_items(0, 3, 1), [Page(1), Page(2), Page(3)]);
    assert_eq!(
        page_items(99, 10, usize::MAX),
        [
            Page(1),
            Page(2),
            Page(3),
            Page(4),
            Page(5),
            Page(6),
            Page(7),
            Page(8),
            Page(9),
            Page(10),
        ],
    );
}

#[test]
fn pagination_item_reports_its_kind() {
    assert_eq!(Page(3).page(), Some(3));
    assert_eq!(Ellipsis.page(), None);
    assert!(Ellipsis.is_ellipsis());
    assert!(!Page(3).is_ellipsis());
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let root: Pagination<'_, Message> = Pagination::new(&theme)
        .count(95)
        .per_page(20)
        .page(3)
        .sibling_count(2)
        .link_size(ButtonSize::IconSm)
        .controls_size(ButtonSize::Sm)
        .active_variant(ButtonVariant::Secondary)
        .inactive_variant(ButtonVariant::Outline)
        .spacing(2.0)
        .show_controls(false)
        .show_links(false)
        .show_labels(false)
        .previous_label("Back")
        .next_label("Forward")
        .disabled(true)
        .width(Length::Fill)
        .on_page_change(Message::PageChanged);

    assert_eq!(root.count, 95);
    assert_eq!(root.per_page, 20);
    assert_eq!(root.page, 3);
    assert_eq!(root.sibling_count, 2);
    assert_eq!(root.link_size, ButtonSize::IconSm);
    assert_eq!(root.controls_size, ButtonSize::Sm);
    assert_eq!(root.active_variant, ButtonVariant::Secondary);
    assert_eq!(root.inactive_variant, ButtonVariant::Outline);
    assert_eq!(root.spacing, Some(2.0));
    assert!(!root.show_controls);
    assert!(!root.show_links);
    assert!(!root.show_labels);
    assert_eq!(root.previous_label.as_ref(), "Back");
    assert_eq!(root.next_label.as_ref(), "Forward");
    assert!(root.disabled);
    assert_eq!(root.width, Length::Fill);
    assert!(std::ptr::eq(root.theme, &theme));

    // The stored callback maps a page press to the app message.
    let callback = root.on_page_change.as_ref().expect("callback was set");
    assert!(matches!(callback(7), Message::PageChanged(7)));
}

#[test]
fn builder_clamps_invalid_inputs() {
    let theme = Theme::light();
    let root: Pagination<'_, Message> = Pagination::new(&theme)
        .per_page(0)
        .page(0)
        .spacing(f32::NAN);

    assert_eq!(root.per_page, 1);
    assert_eq!(root.page, 1);
    assert_eq!(root.spacing, Some(0.0));
}

#[test]
fn free_function_seeds_count_and_page() {
    let theme = Theme::light();
    let root: Pagination<'_, Message> = pagination(95, 4, &theme);

    assert_eq!(root.count, 95);
    assert_eq!(root.page, 4);
    assert_eq!(root.per_page, DEFAULT_PER_PAGE);
    assert_eq!(root.sibling_count, DEFAULT_SIBLING_COUNT);
}

#[test]
fn derived_state_tracks_the_current_page() {
    let theme = Theme::light();
    let root: Pagination<'_, Message> = pagination(95, 10, &theme);

    assert_eq!(root.total_pages(), 10);
    assert!(root.has_previous());
    assert!(!root.has_next());
    assert_eq!(root.item_range(), Some((91, 95)));
    assert_eq!(
        root.items(),
        [Page(1), Ellipsis, Page(7), Page(8), Page(9), Page(10)],
    );

    let first: Pagination<'_, Message> = pagination(95, 1, &theme);
    assert!(!first.has_previous());
    assert!(first.has_next());
    assert_eq!(first.item_range(), Some((1, 10)));
}

#[test]
fn item_range_is_none_without_items() {
    let theme = Theme::light();
    let root: Pagination<'_, Message> = Pagination::new(&theme);

    assert_eq!(root.item_range(), None);
    assert_eq!(root.total_pages(), 1);
    assert!(!root.has_previous());
    assert!(!root.has_next());
}

#[test]
fn out_of_range_pages_are_clamped_in_derived_state() {
    let theme = Theme::light();
    let root: Pagination<'_, Message> = pagination(30, 99, &theme);

    assert_eq!(root.total_pages(), 3);
    assert_eq!(root.item_range(), Some((21, 30)));
    assert!(!root.has_next());
}

#[test]
fn roots_convert_to_elements() {
    let theme = Theme::light();

    // Full bar with links, ellipsis, and boundary controls.
    let _: Element<'_, Message> = pagination(95, 5, &theme)
        .on_page_change(Message::PageChanged)
        .into();

    // Compact previous/next layout.
    let _: Element<'_, Message> = pagination(95, 1, &theme)
        .show_links(false)
        .show_labels(false)
        .on_page_change(Message::PageChanged)
        .into();

    // Disabled bar without a callback.
    let _: Element<'_, Message> = pagination(95, 5, &theme).disabled(true).into();

    // Empty bar builds without panicking.
    let _: Element<'_, Message> = Pagination::new(&theme).into();
}

#[test]
fn subcomponents_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = PaginationLink::new(2, &theme)
        .active(true)
        .on_press(Message::PageChanged(2))
        .into();
    let _: Element<'_, Message> = PaginationLink::new(3, &theme)
        .content(crate::iced_compat::widget::text("III"))
        .into();
    let _: Element<'_, Message> = PaginationPrevious::new(&theme)
        .on_press(Message::PageChanged(1))
        .into();
    let _: Element<'_, Message> = PaginationNext::new(&theme)
        .icon_only()
        .disabled(true)
        .into();
    let _: Element<'_, Message> = PaginationEllipsis::new(&theme)
        .size(ButtonSize::IconSm)
        .into();
}

#[test]
fn link_clamps_page_and_reports_it() {
    let theme = Theme::light();
    let link: PaginationLink<'_, Message> = PaginationLink::new(0, &theme);

    assert_eq!(link.page(), 1);
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();

    let root = Pagination::<NoDebugMessage>::new(&theme);
    assert!(format!("{root:?}").contains("Pagination"));

    let link = PaginationLink::<NoDebugMessage>::new(2, &theme);
    assert!(format!("{link:?}").contains("PaginationLink"));

    let previous = PaginationPrevious::<NoDebugMessage>::new(&theme);
    assert!(format!("{previous:?}").contains("PaginationPrevious"));

    let next = PaginationNext::<NoDebugMessage>::new(&theme);
    assert!(format!("{next:?}").contains("PaginationNext"));

    assert!(format!("{:?}", PaginationEllipsis::new(&theme)).contains("PaginationEllipsis"));
}

#[test]
fn item_supports_hashing_and_display_tokens() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    assert_ne!(hash(&Page(1)), hash(&Page(2)));
    assert_eq!(Page(7).to_string(), "7");
    assert_eq!(Ellipsis.to_string(), "ellipsis");
}
