use crate::Error;
use toasty_core::{schema::ModelId, stmt};

pub trait Model: Sized {
    /// Unique identifier for this model within the schema.
    ///
    /// Identifiers are *not* unique across schemas.
    const ID: ModelId;

    /// Number of fields used to represent the model
    const FIELD_COUNT: usize;

    /// Model key type
    type Key;

    /// Load an instance of the model, populating fields using the given row.
    fn load(row: stmt::Record<'_>) -> Result<Self, Error>;
}
