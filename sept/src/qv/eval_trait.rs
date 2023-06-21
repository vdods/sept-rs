use crate::{dy, Result};

/// EvalTrait represents a view object resulting from a query action, and that view object
/// can be evaluated to produce a value (via the eval method).
pub trait EvalTrait {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>>;
}
