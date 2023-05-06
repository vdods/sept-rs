use crate::{dy, Result};

/// QueryViewTrait represents a view object resulting from a query action, and that view object
/// can be evaluated to produce a value (via the queried_value method).
pub trait QueryViewTrait {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>>;
}
