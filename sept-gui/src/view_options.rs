use crate::ANSIColor;

/// Provides control over how things are rendered.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewOptions {
    /// This defines the font to use when rendering text.
    pub font_id: egui::FontId,
    /// String to use when indenting with no indent item indicator.
    pub invisible_indent: String,
    /// String to use when indenting with indent item indicator.
    pub visible_indent: String,
    /// Indicates if there should be a visible indicator to the left of items in expanded view.
    pub show_expanded_item_indicator: bool,
    /// Nesting depth at which elements are in-lined.
    pub inline_at_nesting_depth: u32,
    /// Indicates if type annotations should be shown.
    pub show_type_annotations: bool,
    /// Indicates if the names of StructTerm fields should be shown before their values in StructTermTerm.
    pub show_struct_field_name_hints: bool,
    /// The number of elements to skip using PageUp/PageDown.
    // TODO: This is not a view option, but some sort of config option.
    pub page_up_down_delta: u32,
    // TODO: Add color config
}

impl ViewOptions {
    pub fn indent_str(&self) -> &str {
        if self.show_expanded_item_indicator {
            self.visible_indent.as_str()
        } else {
            self.invisible_indent.as_str()
        }
    }
    pub fn color_for_cursor_background(&self) -> egui::Color32 {
        if false {
            // TEMP HACK
            egui::Color32::TRANSPARENT
        } else {
            let opaque_color = ANSIColor::DARK_MAGENTA;
            egui::Color32::from_rgba_unmultiplied(
                opaque_color.r(),
                opaque_color.g(),
                opaque_color.b(),
                0x0C,
                // 0x18,
            )
        }
    }
    pub fn color_for_type_annotation(&self) -> egui::Color32 {
        ANSIColor::BRIGHT_BLACK
    }
    pub fn color_for_utf8string_quotes(&self) -> egui::Color32 {
        ANSIColor::DARK_YELLOW
    }
    pub fn color_for_utf8string_escape_chars(&self) -> egui::Color32 {
        ANSIColor::BRIGHT_RED
    }
    pub fn color_for_global_sym_ref_quotes(&self) -> egui::Color32 {
        ANSIColor::DARK_GREEN
    }
    pub fn color_for_global_sym_ref_escape_chars(&self) -> egui::Color32 {
        ANSIColor::DARK_GREEN
    }
    pub fn color_for_local_sym_ref_quotes(&self) -> egui::Color32 {
        ANSIColor::DARK_CYAN
    }
    pub fn color_for_local_sym_ref_escape_chars(&self) -> egui::Color32 {
        ANSIColor::DARK_CYAN
    }
    pub fn color_for_indentation_for<T: 'static>(&self) -> egui::Color32 {
        use std::any::TypeId;
        let type_id = TypeId::of::<T>();
        // TODO: More efficient lookup
        if type_id == TypeId::of::<sept::st::Utf8StringTerm>() {
            ANSIColor::DARK_YELLOW
        } else if type_id == TypeId::of::<sept::dy::ArrayTerm>() {
            ANSIColor::DARK_RED
        } else if type_id == TypeId::of::<sept::dy::OrderedMapTerm>() {
            ANSIColor::DARK_MAGENTA
        } else if type_id == TypeId::of::<sept::dy::StructTerm>()
            || type_id == TypeId::of::<sept::dy::StructTermTerm>()
        {
            ANSIColor::DARK_BLUE
        } else if type_id == TypeId::of::<sept::dy::TupleTerm>() {
            ANSIColor::DARK_GREEN
        } else {
            ANSIColor::BRIGHT_BLACK
        }
    }
    pub fn color_for<T: 'static>(&self) -> egui::Color32 {
        use std::any::TypeId;
        let type_id = TypeId::of::<T>();
        // TODO: More efficient lookup
        if type_id == TypeId::of::<sept::st::Utf8StringTerm>() {
            ANSIColor::BRIGHT_YELLOW
        } else if type_id == TypeId::of::<sept::st::Sint8Term>()
            || type_id == TypeId::of::<sept::st::Sint16Term>()
            || type_id == TypeId::of::<sept::st::Sint32Term>()
            || type_id == TypeId::of::<sept::st::Sint64Term>()
        {
            ANSIColor::DARK_CYAN
        } else if type_id == TypeId::of::<sept::st::Uint8Term>()
            || type_id == TypeId::of::<sept::st::Uint16Term>()
            || type_id == TypeId::of::<sept::st::Uint32Term>()
            || type_id == TypeId::of::<sept::st::Uint64Term>()
        {
            ANSIColor::BRIGHT_CYAN
        } else if type_id == TypeId::of::<sept::st::Float32Term>() {
            ANSIColor::DARK_MAGENTA
        } else if type_id == TypeId::of::<sept::st::Float64Term>() {
            ANSIColor::BRIGHT_MAGENTA
        } else if type_id == TypeId::of::<sept::st::Void>()
            || type_id == TypeId::of::<sept::st::VoidType>()
            || type_id == TypeId::of::<sept::st::Placeholder>()
            || type_id == TypeId::of::<sept::st::PlaceholderType>()
            || type_id == TypeId::of::<sept::st::True>()
            || type_id == TypeId::of::<sept::st::TrueType>()
            || type_id == TypeId::of::<sept::st::False>()
            || type_id == TypeId::of::<sept::st::FalseType>()
            || type_id == TypeId::of::<sept::st::EmptyType>()
            || type_id == TypeId::of::<sept::st::Bool>()
            || type_id == TypeId::of::<sept::st::BoolType>()
            || type_id == TypeId::of::<sept::st::Sint8>()
            || type_id == TypeId::of::<sept::st::Sint16>()
            || type_id == TypeId::of::<sept::st::Sint32>()
            || type_id == TypeId::of::<sept::st::Sint64>()
            || type_id == TypeId::of::<sept::st::Uint8>()
            || type_id == TypeId::of::<sept::st::Uint16>()
            || type_id == TypeId::of::<sept::st::Uint32>()
            || type_id == TypeId::of::<sept::st::Uint64>()
            || type_id == TypeId::of::<sept::st::Float32>()
            || type_id == TypeId::of::<sept::st::Float64>()
            || type_id == TypeId::of::<sept::st::Sint8Type>()
            || type_id == TypeId::of::<sept::st::Sint16Type>()
            || type_id == TypeId::of::<sept::st::Sint32Type>()
            || type_id == TypeId::of::<sept::st::Sint64Type>()
            || type_id == TypeId::of::<sept::st::Uint8Type>()
            || type_id == TypeId::of::<sept::st::Uint16Type>()
            || type_id == TypeId::of::<sept::st::Uint32Type>()
            || type_id == TypeId::of::<sept::st::Uint64Type>()
            || type_id == TypeId::of::<sept::st::Float32Type>()
            || type_id == TypeId::of::<sept::st::Float64Type>()
            || type_id == TypeId::of::<sept::st::Utf8String>()
            || type_id == TypeId::of::<sept::st::Utf8StringType>()
            || type_id == TypeId::of::<sept::st::Array>()
            || type_id == TypeId::of::<sept::st::ArrayType>()
            || type_id == TypeId::of::<sept::st::Struct>()
            || type_id == TypeId::of::<sept::st::StructType>()
            || type_id == TypeId::of::<sept::st::Tuple>()
            || type_id == TypeId::of::<sept::st::TupleType>()
        {
            ANSIColor::BRIGHT_WHITE
        } else if type_id == TypeId::of::<sept::st::BoolTerm>() {
            ANSIColor::BRIGHT_YELLOW
        } else if type_id == TypeId::of::<sept::dy::ArrayTerm>() {
            ANSIColor::BRIGHT_RED
        } else if type_id == TypeId::of::<sept::dy::OrderedMapTerm>() {
            ANSIColor::BRIGHT_MAGENTA
        } else if type_id == TypeId::of::<sept::dy::StructTerm>()
            || type_id == TypeId::of::<sept::dy::StructTermTerm>()
        {
            ANSIColor::BRIGHT_BLUE
        } else if type_id == TypeId::of::<sept::dy::TupleTerm>() {
            ANSIColor::BRIGHT_GREEN
        } else if type_id == TypeId::of::<sept::dy::GlobalSymRefTerm>() {
            ANSIColor::BRIGHT_GREEN
        } else if type_id == TypeId::of::<sept::dy::LocalSymRefTerm>() {
            ANSIColor::BRIGHT_CYAN
        } else {
            // TODO: Use some default from the style
            ANSIColor::DARK_WHITE
        }
    }
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self {
            font_id: egui::FontId::new(12.0, egui::FontFamily::Monospace),
            invisible_indent: "    ".to_string(),
            visible_indent: "-   ".to_string(),
            show_expanded_item_indicator: true,
            inline_at_nesting_depth: 2,
            show_type_annotations: true,
            show_struct_field_name_hints: true,
            page_up_down_delta: 4u32,
        }
    }
}
