//! Typography and icon selections exposed by [`super::Theme`].

use shadcn_common::{FontHeading, FontId, FontPack};

use super::tokens::Theme;

impl Theme {
    /// Resolved font pack (sans/mono/heading).
    pub fn font_pack(&self) -> FontPack {
        self.resolved.font_pack()
    }

    /// Resolved body font.
    pub fn font_id(&self) -> FontId {
        self.resolved.font_id()
    }

    /// Resolved heading font.
    pub fn font_heading(&self) -> FontHeading {
        self.resolved.font_heading()
    }
}
