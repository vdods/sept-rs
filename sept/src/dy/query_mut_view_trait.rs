use crate::{dy, Result};

// TODO: Figure out if QueryViewTrait supertrait is really needed.  Probably not, unless there's some definitional
// reason why it's needed for QueryMutViewTrait.  Actually, a given mutation might alter a QueryViewTrait
// such that it's no longer valid, so maybe QueryViewTrait is not even well-defined here.
// TODO: Rename this to EditableTrait or something
pub trait QueryMutViewTrait: dy::QueryViewTrait {
    /// This operation must be atomic -- if an error is to be returned, it must not alter the queried data,
    /// and if the queried data is altered, then this method call must succeed.
    // TODO: Figure out how this can take &dy::ValueGuts.
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()>;
}
