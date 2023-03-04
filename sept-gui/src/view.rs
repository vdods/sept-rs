use crate::ViewCtx;

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
