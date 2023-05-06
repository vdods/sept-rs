use crate::AddressedEdit;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

pub struct Model {
    /// This is the sept::dy::Value that's being viewed.
    pub root_value_la: Arc<RwLock<sept::dy::Value>>,
    /// This is the queue of edits that have been applied.  In other words, this represents the undo queue.
    pub applied_edit_v: VecDeque<AddressedEdit>,
    // TODO: A redo edit queue.
}
