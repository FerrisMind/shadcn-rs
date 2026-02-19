//! Empty state component - placeholder for empty lists or search results.
//!
//! # Example
//! ```ignore
//! empty(ui, &theme, EmptyProps::new("No results").description("Try adjusting your search."));
//! ```

use crate::theme::Theme;
use egui::{Align, Layout, RichText, Ui};

/// Properties for the Empty state component.
#[derive(Clone, Debug)]
pub struct EmptyProps<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> EmptyProps<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            icon: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Render an empty state placeholder.
pub fn empty(ui: &mut Ui, theme: &Theme, props: EmptyProps<'_>) {
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.add_space(32.0);
        ui.spacing_mut().item_spacing.y = 8.0;

        if let Some(icon) = props.icon {
            ui.label(
                RichText::new(icon)
                    .size(32.0)
                    .color(theme.palette.muted_foreground),
            );
        }

        ui.label(
            RichText::new(props.title)
                .size(16.0)
                .strong()
                .color(theme.palette.foreground),
        );

        if let Some(desc) = props.description {
            ui.label(
                RichText::new(desc)
                    .size(14.0)
                    .color(theme.palette.muted_foreground),
            );
        }

        ui.add_space(32.0);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_props_builder() {
        let props = EmptyProps::new("No results")
            .description("Try adjusting your search.")
            .icon("🔍");

        assert_eq!(props.title, "No results");
        assert_eq!(props.description, Some("Try adjusting your search."));
        assert_eq!(props.icon, Some("🔍"));
    }
}
