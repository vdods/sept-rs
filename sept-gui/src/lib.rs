//#![warn(clippy::all, rust_2018_idioms)]

mod action;
mod addressed_edit;
mod ansi_color;
mod app;
mod array_term_event_handler;
mod array_term_value_ui;
mod cursor_edit;
mod edit;
mod event_handler;
mod event_handler_ctx;
mod event_handler_ctx_nesting_guard;
mod global_sym_ref_term_value_ui;
mod layout_discriminant;
mod layout_mode;
mod local_sym_ref_term_value_ui;
mod model;
mod ordered_map_term_value_ui;
mod placeholder_event_handler;
mod placeholder_value_ui;
mod root_value_edit;
mod struct_term_event_handler;
mod struct_term_term_value_ui;
mod struct_term_value_ui;
mod tuple_term_value_ui;
mod utf8_string_term_event_handler;
mod utf8_string_term_value_ui;
mod value_event_handler;
mod value_ui;
mod value_value_ui;
mod view_ctx;
mod view_ctx_nesting_guard;
mod view_ctx_render_address_guard;
mod view_ctx_ta_guard;
mod view_options;

pub use action::Action;
pub use addressed_edit::AddressedEdit;
pub use ansi_color::ANSIColor;
pub use app::App;
pub use cursor_edit::CursorEdit;
pub use edit::Edit;
pub use event_handler::EventHandler;
pub use event_handler_ctx::EventHandlerCtx;
pub use event_handler_ctx_nesting_guard::EventHandlerCtxNestingGuard;
pub use layout_discriminant::LayoutDiscriminant;
pub use layout_mode::LayoutMode;
pub use model::{Model, SaveBehavior};
pub use placeholder_event_handler::placeholder_event_handler_impl;
pub use root_value_edit::RootValueEdit;
pub use value_ui::{
    extract_text_prefix_from_front_text, first_char_stripped_string, ValueUI,
    END_OF_TRANSMISSION_CHAR, END_OF_TRANSMISSION_STR,
};
pub use view_ctx::ViewCtx;
pub use view_ctx_nesting_guard::ViewCtxNestingGuard;
pub use view_ctx_render_address_guard::ViewCtxRenderAddressGuard;
pub use view_ctx_ta_guard::ViewCtxTAGuard;
pub use view_options::ViewOptions;

pub(crate) use value_ui::{
    indentation_for, layout_job_append, render_postfix_annotation,
    render_str_as_literal_without_quotes, render_type_annotation_for,
    render_type_annotation_for_str,
};
