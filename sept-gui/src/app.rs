use crate::ANSIColor;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

/// Provides control over how things are rendered.
pub struct ViewCtx {
    /// Current nesting depth.
    pub current_nesting_depth: u32,
    /// Nesting depth at which elements are in-lined.
    pub inline_at_nesting_depth: u32,
}

impl ViewCtx {
    pub fn new() -> Self {
        Self {
            current_nesting_depth: 0,
            inline_at_nesting_depth: 0,
        }
    }
    pub fn push_nesting_depth<'a>(&'a mut self) -> NestingGuard<'a> {
        self.current_nesting_depth = self
            .current_nesting_depth
            .checked_add(1)
            .expect("programmer error: ViewCtx current_nesting_depth overflow");
        NestingGuard { view_ctx: self }
    }
    pub fn should_use_inline(&self) -> bool {
        self.current_nesting_depth >= self.inline_at_nesting_depth
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
            ANSIColor::BRIGHT_YELLOW
        } else if type_id == TypeId::of::<sept::st::Uint8Term>()
            || type_id == TypeId::of::<sept::st::Uint16Term>()
            || type_id == TypeId::of::<sept::st::Uint32Term>()
            || type_id == TypeId::of::<sept::st::Uint64Term>()
        {
            ANSIColor::BRIGHT_CYAN
        } else if type_id == TypeId::of::<sept::st::Float32Term>()
            || type_id == TypeId::of::<sept::st::Float64Term>()
        {
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

pub struct NestingGuard<'a> {
    view_ctx: &'a mut ViewCtx,
}

impl<'a> std::borrow::Borrow<ViewCtx> for NestingGuard<'a> {
    fn borrow(&self) -> &ViewCtx {
        &*self.view_ctx
    }
}

impl<'a> std::borrow::BorrowMut<ViewCtx> for NestingGuard<'a> {
    fn borrow_mut(&mut self) -> &mut ViewCtx {
        &mut *self.view_ctx
    }
}

impl<'a> std::ops::Deref for NestingGuard<'a> {
    type Target = ViewCtx;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a> std::ops::DerefMut for NestingGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

// impl NestingGuard {
//     pub fn view_ctx_mut(&self) -> &mut ViewCtx {
//         self.view_ctx
//     }
// }

impl<'a> std::ops::Drop for NestingGuard<'a> {
    fn drop(&mut self) {
        self.view_ctx.current_nesting_depth = self
            .view_ctx
            .current_nesting_depth
            .checked_sub(1)
            .expect("programmer error: ViewCtx current_nesting_depth underflow");
    }
}

pub trait View {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx);
}

impl View for sept::dy::Value {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_mut::<sept::st::BoolTerm>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64Term>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8StringTerm>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::ArrayTerm>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::TupleTerm>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Void>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::VoidType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Bool>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::BoolType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::True>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::TrueType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::False>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::FalseType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::EmptyType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64Type>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8String>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8StringType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Array>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::ArrayType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::StructTermTerm>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::StructTerm>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Struct>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::StructType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Tuple>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::TupleType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::GlobalSymRef>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::GlobalSymRefType>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::LocalSymRef>() {
            term.update(ui, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::LocalSymRefType>() {
            term.update(ui, view_ctx);
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
}

// This is probably a TEMP HACK
macro_rules! impl_view_using_to_string {
    ($ty:ty) => {
        impl View for $ty {
            fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
                use sept::st::Stringifiable;
                ui.colored_label(view_ctx.color_for::<$ty>(), self.stringify().as_str());
            }
        }
    };
}

impl_view_using_to_string!(sept::st::Void);
impl_view_using_to_string!(sept::st::True);
impl_view_using_to_string!(sept::st::False);
impl_view_using_to_string!(sept::st::BoolTerm);
impl_view_using_to_string!(sept::st::Sint8Term);
impl_view_using_to_string!(sept::st::Sint16Term);
impl_view_using_to_string!(sept::st::Sint32Term);
impl_view_using_to_string!(sept::st::Sint64Term);
impl_view_using_to_string!(sept::st::Uint8Term);
impl_view_using_to_string!(sept::st::Uint16Term);
impl_view_using_to_string!(sept::st::Uint32Term);
impl_view_using_to_string!(sept::st::Uint64Term);
impl_view_using_to_string!(sept::st::Float32Term);
impl_view_using_to_string!(sept::st::Float64Term);
impl_view_using_to_string!(sept::st::VoidType);
impl_view_using_to_string!(sept::st::EmptyType);
impl_view_using_to_string!(sept::st::TrueType);
impl_view_using_to_string!(sept::st::FalseType);
impl_view_using_to_string!(sept::st::Bool);
impl_view_using_to_string!(sept::st::Sint8);
impl_view_using_to_string!(sept::st::Sint16);
impl_view_using_to_string!(sept::st::Sint32);
impl_view_using_to_string!(sept::st::Sint64);
impl_view_using_to_string!(sept::st::Uint8);
impl_view_using_to_string!(sept::st::Uint16);
impl_view_using_to_string!(sept::st::Uint32);
impl_view_using_to_string!(sept::st::Uint64);
impl_view_using_to_string!(sept::st::Float32);
impl_view_using_to_string!(sept::st::Float64);
impl_view_using_to_string!(sept::st::Utf8String);
impl_view_using_to_string!(sept::st::Array);
impl_view_using_to_string!(sept::st::Struct);
impl_view_using_to_string!(sept::st::Tuple);
impl_view_using_to_string!(sept::st::GlobalSymRef);
impl_view_using_to_string!(sept::st::LocalSymRef);
impl_view_using_to_string!(sept::st::BoolType);
impl_view_using_to_string!(sept::st::Sint8Type);
impl_view_using_to_string!(sept::st::Sint16Type);
impl_view_using_to_string!(sept::st::Sint32Type);
impl_view_using_to_string!(sept::st::Sint64Type);
impl_view_using_to_string!(sept::st::Uint8Type);
impl_view_using_to_string!(sept::st::Uint16Type);
impl_view_using_to_string!(sept::st::Uint32Type);
impl_view_using_to_string!(sept::st::Uint64Type);
impl_view_using_to_string!(sept::st::Float32Type);
impl_view_using_to_string!(sept::st::Float64Type);
impl_view_using_to_string!(sept::st::Utf8StringType);
impl_view_using_to_string!(sept::st::ArrayType);
impl_view_using_to_string!(sept::st::StructType);
impl_view_using_to_string!(sept::st::TupleType);
impl_view_using_to_string!(sept::st::GlobalSymRefType);
impl_view_using_to_string!(sept::st::LocalSymRefType);

impl View for sept::st::Utf8StringTerm {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
        // TEMP HACK -- in-line for now.
        // TODO: Use color for quotes and escape chars
        ui.horizontal(|ui| {
            ui.colored_label(view_ctx.color_for::<Self>(), format!("{:?}", self).as_str());
        });
    }
}

impl View for sept::dy::ArrayTerm {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
        // TODO: Use color for brackets, commas
        let should_use_inline = view_ctx.should_use_inline();
        if should_use_inline {
            ui.horizontal_wrapped(|ui| {
                let mut view_ctx_g = view_ctx.push_nesting_depth();

                ui.colored_label(view_ctx_g.color_for::<sept::dy::ArrayTerm>(), "[");
                let n = self.len();
                for (i, element) in self.iter_mut().enumerate() {
                    element.update(ui, &mut view_ctx_g);
                    if i + 1 != n {
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::ArrayTerm>(), ",");
                    }
                }
                ui.colored_label(view_ctx_g.color_for::<sept::dy::ArrayTerm>(), "]");
            });
        } else {
            ui.vertical(|ui| {
                let mut view_ctx_g = view_ctx.push_nesting_depth();

                ui.colored_label(view_ctx_g.color_for::<sept::dy::ArrayTerm>(), "[");
                for element in self.iter_mut() {
                    ui.horizontal(|ui| {
                        // TEMP HACK -- hardcoded value
                        ui.add_space(24.0);
                        element.update(ui, &mut view_ctx_g);
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::ArrayTerm>(), ",");
                    });
                }
                ui.colored_label(view_ctx_g.color_for::<sept::dy::ArrayTerm>(), "]");
            });
        }
    }
}

impl View for sept::dy::StructTermTerm {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
        // TEMP HACK -- in-line for now.
        // TODO: Use color for brackets, commas
        // TODO: Fix this silly layout
        ui.horizontal(|ui| {
            self.direct_type_mut().update(ui, view_ctx);
            self.field_tuple_mut().update(ui, view_ctx);
        });
    }
}

impl View for sept::dy::StructTerm {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
        // TEMP HACK -- in-line for now.
        // TODO: Use color for brackets, commas
        let should_use_inline = view_ctx.should_use_inline();
        if should_use_inline {
            ui.horizontal_wrapped(|ui| {
                let mut view_ctx_g = view_ctx.push_nesting_depth();

                // TODO: Actually print `Struct` term directly, so it's rendered properly
                ui.horizontal(|ui| {
                    sept::st::Struct.update(ui, &mut view_ctx_g);
                    ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), "{");
                });
                let n = self.field_decl_v.len();
                for (i, (field_id, field_type)) in self.field_decl_v.iter_mut().enumerate() {
                    field_id.update(ui, &mut view_ctx_g);
                    ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), ":");
                    field_type.update(ui, &mut view_ctx_g);
                    if i + 1 != n {
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), ",");
                    }
                }
                ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), "}");
            });
        } else {
            ui.vertical(|ui| {
                let mut view_ctx_g = view_ctx.push_nesting_depth();

                ui.horizontal(|ui| {
                    sept::st::Struct.update(ui, &mut view_ctx_g);
                    ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), "{");
                });
                for (field_id, field_type) in self.field_decl_v.iter_mut() {
                    ui.horizontal(|ui| {
                        // TEMP HACK -- hardcoded value
                        ui.add_space(24.0);
                        field_id.update(ui, &mut view_ctx_g);
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), ":");
                        field_type.update(ui, &mut view_ctx_g);
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), ",");
                    });
                }
                ui.colored_label(view_ctx_g.color_for::<sept::dy::StructTerm>(), "}");
            });
        }
    }
}

impl View for sept::dy::TupleTerm {
    fn update(&mut self, ui: &mut egui::Ui, view_ctx: &mut ViewCtx) {
        // TEMP HACK -- in-line for now.
        // TODO: Use color for brackets, commas
        let should_use_inline = view_ctx.should_use_inline();
        if should_use_inline {
            ui.horizontal_wrapped(|ui| {
                let mut view_ctx_g = view_ctx.push_nesting_depth();

                ui.colored_label(view_ctx_g.color_for::<sept::dy::TupleTerm>(), "(");
                let n = self.len();
                for (i, element) in self.iter_mut().enumerate() {
                    element.update(ui, &mut view_ctx_g);
                    if i + 1 != n {
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::TupleTerm>(), ",");
                    }
                }
                ui.colored_label(view_ctx_g.color_for::<sept::dy::TupleTerm>(), ")");
            });
        } else {
            ui.vertical(|ui| {
                let mut view_ctx_g = view_ctx.push_nesting_depth();

                ui.colored_label(view_ctx_g.color_for::<sept::dy::TupleTerm>(), "(");
                for element in self.iter_mut() {
                    ui.horizontal(|ui| {
                        // TEMP HACK -- hardcoded value
                        ui.add_space(24.0);
                        element.update(ui, &mut view_ctx_g);
                        ui.colored_label(view_ctx_g.color_for::<sept::dy::TupleTerm>(), ",");
                    });
                }
                ui.colored_label(view_ctx_g.color_for::<sept::dy::TupleTerm>(), ")");
            });
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let Self { label, value } = self;

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        // egui::SidePanel::right("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");
        //     egui::warn_if_debug_build(ui);
        // });

        // Note that the CentralPanel must be added after side panels.
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut view_ctx = ViewCtx::new();
            view_ctx.inline_at_nesting_depth = 1;

            let a1 = sept::dy::ArrayTerm::from(vec![
                true.into(),
                false.into(),
                123i8.into(),
                200u8.into(),
                12345i16.into(),
                45678u16.into(),
                1234567i32.into(),
                4567890u32.into(),
                1000000000000i64.into(),
                9223372036854775808u64.into(),
                10101.202f32.into(),
                1.01020304050607f64.into(),
                sept::st::Void.into(),
                sept::st::True.into(),
                sept::st::False.into(),
                sept::st::EmptyType.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
                sept::st::Void.into(),
            ]);
            let mut a2 = sept::dy::ArrayTerm::from(vec![true.into(), a1.into(), false.into()]);
            a2.update(ui, &mut view_ctx);

            let mut t1 = sept::dy::TupleTerm::from(vec![
                true.into(),
                false.into(),
                123i8.into(),
                sept::dy::TupleTerm::from(vec![
                    200u8.into(),
                    sept::dy::TupleTerm::from(vec![
                        12345i16.into(),
                        45678u16.into(),
                        1234567i32.into(),
                    ])
                    .into(),
                    4567890u32.into(),
                    1000000000000i64.into(),
                ])
                .into(),
                9223372036854775808u64.into(),
                10101.202f32.into(),
                1.01020304050607f64.into(),
            ]);
            t1.update(ui, &mut view_ctx);

            let mut t2 = sept::dy::TupleTerm::from(vec![
                sept::st::Void.into(),
                sept::st::VoidType.into(),
                sept::st::Bool.into(),
                sept::st::BoolType.into(),
                "blah\nthing\they".to_string().into(),
            ]);
            t2.update(ui, &mut view_ctx);

            let mut st1 = sept::dy::StructTerm::new(
                vec![
                    ("age".into(), sept::st::Uint8.into()),
                    ("gravity".into(), sept::st::Float64.into()),
                ]
                .into(),
            )
            .unwrap();
            st1.update(ui, &mut view_ctx);

            use sept::dy::Constructor;
            let mut stt1 = st1
                .construct(sept::dy::TupleTerm::from(vec![
                    28u8.into(),
                    4035.56f64.into(),
                ]))
                .unwrap();
            stt1.update(ui, &mut view_ctx);
        });

        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally chose either panels OR windows.");
        //     });
        // }
    }
}
