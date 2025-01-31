use crate::{layout_job_append, ValueUIT, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl ValueUIT for sept::st::Placeholder {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("should be deprecated");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        // Placeholder is simple enough, just run it inline.
        self.run_ui_inline(ui, &mut layout_job, view_ctx);
        // Return this to the outer context.
        layout_job
    }
    fn run_ui_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        // Maybe use a special char to keep things compact?
        layout_job_append(
            layout_job,
            "Placeholder",
            view_ctx.color_for::<Self>(),
            view_ctx,
        );
        // render_type_annotation_for(self, layout_job, view_ctx, None);
    }
}
