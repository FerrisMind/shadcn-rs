//! Public value types used by the table component.

use crate::iced_compat::alignment::Horizontal;

/// Horizontal alignment for table cell content.
///
/// The variants use table terminology while converting to iced's native
/// [`Horizontal`] alignment at the rendering boundary.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableCellAlignment {
    /// Align content at the leading edge of the cell.
    #[default]
    Start,
    /// Center content inside the cell.
    Center,
    /// Align content at the trailing edge of the cell.
    End,
}

impl From<TableCellAlignment> for Horizontal {
    fn from(alignment: TableCellAlignment) -> Self {
        match alignment {
            TableCellAlignment::Start => Self::Left,
            TableCellAlignment::Center => Self::Center,
            TableCellAlignment::End => Self::Right,
        }
    }
}
