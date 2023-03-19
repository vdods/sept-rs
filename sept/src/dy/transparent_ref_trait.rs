use crate::{dy, st, Result};
use std::sync::{Arc, RwLock};

pub trait TransparentRefTrait: st::TermTrait {
    /// This should dereference this ref one time.  It should return Err if there's a problem
    /// during the dereferencing operation.
    fn dereferenced_once(&self) -> Result<Arc<RwLock<dy::Value>>>;
}
