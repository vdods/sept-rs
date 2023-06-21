use crate::{ValueUI, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl ValueUI for sept::dy::Value {
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_ref::<sept::st::BoolTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Term>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::ArrayTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::OrderedMapTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::TupleTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Void>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::VoidType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Bool>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::BoolType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::True>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::TrueType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::False>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::FalseType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::EmptyType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Type>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8String>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Array>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::ArrayType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMap>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMapType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTermTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Struct>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::StructType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Tuple>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::TupleType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::GlobalSymRefTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRef>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRefType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::LocalSymRefTerm>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRef>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRefType>() {
            term.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_ref::<sept::st::BoolTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Term>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::ArrayTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::OrderedMapTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::TupleTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Void>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::VoidType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Bool>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::BoolType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::True>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::TrueType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::False>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::FalseType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::EmptyType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Type>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8String>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Array>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::ArrayType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMap>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMapType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTermTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Struct>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::StructType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Tuple>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::TupleType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::GlobalSymRefTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRef>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRefType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::LocalSymRefTerm>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRef>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRefType>() {
            term.run_ui_inline(ui, layout_job, view_ctx);
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
}
