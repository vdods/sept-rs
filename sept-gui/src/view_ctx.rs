use crate::{ANSIColor, ViewCtxNestingGuard, ViewCtxTAGuard};

/// Provides control over how things are rendered.
pub struct ViewCtx {
    /// This defines the font to use when rendering text.
    pub font_id: egui::FontId,
    /// String to use when indenting with no indent item indicator.
    pub invisible_indent: String,
    /// String to use when indenting with indent item indicator.
    pub visible_indent: String,
    /// Indicates if there should be a visible indicator to the left of items in expanded view.
    pub show_expanded_item_indicator: bool,
    /// Current nesting depth.
    pub current_nesting_depth: u32,
    /// Nesting depth at which elements are in-lined.
    pub inline_at_nesting_depth: u32,
    /// Indicates if type annotations should be shown.
    pub show_type_annotations: bool,
    /// Indicates if the names of StructTerm fields should be shown before their values in StructTermTerm.
    pub show_struct_field_name_hints: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutMode {
    Expanded,
    BoundaryLevelInline,
    InteriorLevelInline,
}

impl ViewCtx {
    pub fn new() -> Self {
        Self {
            font_id: egui::FontId::new(12.0, egui::FontFamily::Monospace),
            invisible_indent: "    ".to_string(),
            visible_indent: "-   ".to_string(),
            show_expanded_item_indicator: true,
            current_nesting_depth: 0,
            inline_at_nesting_depth: 0,
            show_type_annotations: true,
            show_struct_field_name_hints: true,
        }
    }
    pub fn push_nesting_depth<'a>(&'a mut self) -> ViewCtxNestingGuard<'a> {
        ViewCtxNestingGuard::new(self)
    }
    pub fn push_show_type_annotations<'a>(
        &'a mut self,
        show_type_annotations: bool,
    ) -> ViewCtxTAGuard<'a> {
        ViewCtxTAGuard::new(self, show_type_annotations)
    }
    pub fn indent_str(&self) -> &str {
        if self.show_expanded_item_indicator {
            self.visible_indent.as_str()
        } else {
            self.invisible_indent.as_str()
        }
    }
    pub fn should_use_inline(&self) -> bool {
        self.current_nesting_depth >= self.inline_at_nesting_depth
    }
    pub fn layout_mode(&self) -> LayoutMode {
        match self
            .current_nesting_depth
            .cmp(&self.inline_at_nesting_depth)
        {
            std::cmp::Ordering::Less => LayoutMode::Expanded,
            std::cmp::Ordering::Equal => LayoutMode::BoundaryLevelInline,
            std::cmp::Ordering::Greater => LayoutMode::InteriorLevelInline,
        }
    }
    pub fn color_for_type_annotation(&self) -> egui::Color32 {
        ANSIColor::BRIGHT_BLACK
    }
    pub fn color_for_quotes(&self) -> egui::Color32 {
        ANSIColor::DARK_YELLOW
    }
    pub fn color_for_escape_chars(&self) -> egui::Color32 {
        ANSIColor::BRIGHT_RED
    }
    pub fn color_for_indentation_for<T: 'static>(&self) -> egui::Color32 {
        use std::any::TypeId;
        let type_id = TypeId::of::<T>();
        // TODO: More efficient lookup
        if type_id == TypeId::of::<sept::st::Utf8StringTerm>() {
            ANSIColor::DARK_YELLOW
        } else if type_id == TypeId::of::<sept::dy::ArrayTerm>() {
            ANSIColor::DARK_RED
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
        } else if type_id == TypeId::of::<sept::dy::StructTerm>()
            || type_id == TypeId::of::<sept::dy::StructTermTerm>()
        {
            ANSIColor::BRIGHT_BLUE
        } else if type_id == TypeId::of::<sept::dy::TupleTerm>() {
            ANSIColor::BRIGHT_GREEN
        } else {
            // TODO: Use some default from the style
            ANSIColor::DARK_WHITE
        }
    }
}
