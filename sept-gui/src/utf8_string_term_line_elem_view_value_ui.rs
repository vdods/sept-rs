use crate::{ValueUIT, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UTF8StringTermLineElemView<'a> {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("should be deprecated");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut view_ctx_g = view_ctx.push_render_address_token("char".to_string().into());

        let layout_job = sept::qv::UTF8StringTermLineElemCharView::new_with_cached_line(
            self.string,
            self.line_index,
            self.line_count,
            self.line,
            self.line_char_count,
        )
        .unwrap()
        .run_ui(ui, &mut view_ctx_g, continuation_layout_job_o);

        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        let mut view_ctx_g = view_ctx.push_render_address_token("char".to_string().into());

        sept::qv::UTF8StringTermLineElemCharView::new_with_cached_line(
            self.string,
            self.line_index,
            self.line_count,
            self.line,
            self.line_char_count,
        )
        .unwrap()
        .run_ui_inline(ui, layout_job, &mut view_ctx_g);
    }
}
