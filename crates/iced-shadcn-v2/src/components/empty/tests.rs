use crate::iced_compat::widget::text;
use crate::{StyleId, Theme};

use super::geometry;
use super::{
    Empty, EmptyBorderStyle, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle,
};

#[test]
fn source_style_metrics_are_resolved() {
    let nova = Theme::light().with_style(StyleId::Nova);
    let nova_metrics = geometry::metrics(&nova);
    assert_eq!(nova_metrics.root_padding_px, 24.0);
    assert_eq!(nova_metrics.root_gap_px, 16.0);
    assert_eq!(nova_metrics.media_size_px, 32.0);
    assert_eq!(nova_metrics.content_gap_px, 10.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    let sera_title = geometry::title_metrics(&sera);
    assert!(sera_title.uppercase);
    assert_eq!(sera_title.size_px, 18.0);
    assert_eq!(geometry::description_metrics(&sera).top_padding_px, 2.0);
}

#[test]
fn typed_description_matches_source_balance_break() {
    let description =
        "You haven't created any projects yet. Get started by creating your first project.";
    assert_eq!(
        super::render::balance_text(description, 384.0, 14.0),
        "You haven't created any projects yet.\nGet started by creating your first project."
    );
}

#[test]
fn builder_keeps_typed_and_arbitrary_slots_composable() {
    let theme = Theme::light();
    let element: crate::iced_compat::Element<'_, ()> = Empty::new(&theme)
        .outline()
        .border(EmptyBorderStyle::Dashed)
        .header(
            EmptyHeader::new(&theme)
                .media(EmptyMedia::icon(text("□"), &theme))
                .title(EmptyTitle::text("Nothing here", &theme))
                .description(EmptyDescription::text("Add your first item.", &theme)),
        )
        .content(EmptyContent::new(&theme).push(text("Action")))
        .into();

    let _ = element;
}
