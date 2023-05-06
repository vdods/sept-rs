#![warn(clippy::all, rust_2018_idioms)]

mod addressed_edit;
mod ansi_color;
mod app;
mod layout_discriminant;
mod layout_mode;
mod model;
mod view;
mod view_ctx;
mod view_ctx_nesting_guard;
mod view_ctx_render_address_guard;
mod view_ctx_ta_guard;
mod view_options;

pub use addressed_edit::AddressedEdit;
pub use ansi_color::ANSIColor;
pub use app::App;
pub use layout_discriminant::LayoutDiscriminant;
pub use layout_mode::LayoutMode;
pub use model::Model;
pub use view::View;
pub use view_ctx::ViewCtx;
pub use view_ctx_nesting_guard::ViewCtxNestingGuard;
pub use view_ctx_render_address_guard::ViewCtxRenderAddressGuard;
pub use view_ctx_ta_guard::ViewCtxTAGuard;
pub use view_options::ViewOptions;
