use crate::{ANSIColor, ViewCtxNestingGuard, ViewCtxTAGuard};

/// Provides control over how things are rendered.
pub struct ViewCtx {
    /// Current nesting depth.
    pub current_nesting_depth: u32,
    /// Nesting depth at which elements are in-lined.
    pub inline_at_nesting_depth: u32,
    /// Indicates if type annotations should be shown.
    pub show_type_annotations: bool,
}

impl ViewCtx {
    pub fn new() -> Self {
        Self {
            current_nesting_depth: 0,
            inline_at_nesting_depth: 0,
            show_type_annotations: true,
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
    pub fn should_use_inline(&self) -> bool {
        self.current_nesting_depth >= self.inline_at_nesting_depth
    }
    pub fn color_for_type_annotation(&self) -> egui::Color32 {
        ANSIColor::BRIGHT_BLACK
    }
    pub fn color_for<T: 'static>(&self) -> egui::Color32 {
        use std::any::TypeId;
        let type_id = TypeId::of::<T>();
        // TODO: More efficient lookup
        if type_id == TypeId::of::<sept::st::Utf8StringTerm>() {
            ANSIColor::BRIGHT_GREEN
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
        {
            ANSIColor::BRIGHT_WHITE
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        // } else if type_id == TypeId::of::<sept::st::>() {
        //     ANSIColor::
        } else if type_id == TypeId::of::<sept::st::BoolTerm>() {
            ANSIColor::BRIGHT_CYAN
        } else if type_id == TypeId::of::<sept::dy::ArrayTerm>() {
            ANSIColor::BRIGHT_RED
        } else if type_id == TypeId::of::<sept::dy::StructTerm>() {
            ANSIColor::BRIGHT_BLUE
        } else if type_id == TypeId::of::<sept::dy::TupleTerm>() {
            ANSIColor::DARK_GREEN
        } else if type_id == TypeId::of::<sept::st::Struct>() {
            ANSIColor::BRIGHT_WHITE
        } else {
            // TODO: Use some default from the style
            ANSIColor::DARK_WHITE
        }
    }
}
