// use crate::{dy, st, Result};
// use std::sync::{Arc, RwLock};

//
//
// Work-in-progress below.
//
//

// /// This is the minimal, dy-module conceptualization of EditTrait, using dy::Value for everything.
// pub trait EditTrait {
//     /// Convert into the inverse edit.
//     fn into_inverse(self) -> Box<dyn EditTrait>;
//     // /// Return the inverse edit.  The default implementation simply calls `self.clone().into_inverse()`,
//     // /// but an impl of EditTrait may want to specialize this.
//     // fn inverse(&self) -> Box<dyn EditTrait> {
//     //     self.clone().into_inverse()
//     // }
// }
