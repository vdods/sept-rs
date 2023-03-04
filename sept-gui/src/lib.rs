#![warn(clippy::all, rust_2018_idioms)]

mod ansi_color;
mod app;
mod view;
mod view_ctx;
mod view_ctx_nesting_guard;
mod view_ctx_ta_guard;

pub use ansi_color::ANSIColor;
pub use app::App;
pub use view::View;
pub use view_ctx::{LayoutMode, ViewCtx};
pub use view_ctx_nesting_guard::ViewCtxNestingGuard;
pub use view_ctx_ta_guard::ViewCtxTAGuard;
