//! Errors returned by the [`super::TreeView`] builder.

use std::fmt;

use shadcn_common::{TreeNodeId, TreeValidationError};

use super::TreeViewMeasurement;

/// Error returned when a tree cannot satisfy its render invariants.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TreeViewBuildError {
    /// Two nodes use the same stable ID.
    DuplicateId(TreeNodeId),
    /// An unloaded folder contains children that are not available yet.
    UnloadedFolderHasChildren(TreeNodeId),
    /// A numeric measurement is zero, negative, or non-finite.
    InvalidMeasurement(TreeViewMeasurement),
}

impl From<TreeValidationError> for TreeViewBuildError {
    fn from(error: TreeValidationError) -> Self {
        match error {
            TreeValidationError::DuplicateId(id) => Self::DuplicateId(id),
            TreeValidationError::UnloadedFolderHasChildren(id) => {
                Self::UnloadedFolderHasChildren(id)
            }
        }
    }
}

impl fmt::Display for TreeViewBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate tree node id: {id}"),
            Self::UnloadedFolderHasChildren(id) => {
                write!(formatter, "unloaded folder has children: {id}")
            }
            Self::InvalidMeasurement(measurement) => {
                write!(formatter, "invalid tree measurement: {measurement:?}")
            }
        }
    }
}

impl std::error::Error for TreeViewBuildError {}
