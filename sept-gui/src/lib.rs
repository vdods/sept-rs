#![warn(clippy::all, rust_2018_idioms)]

mod ansi_color;
mod app;

pub use ansi_color::ANSIColor;
pub use app::App;
