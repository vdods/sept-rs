use crate::{
    render_postfix_annotation,
    ValueUIT, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UTF8StringTermLineElemCharView<'a> {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("this should not be called");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        // Tack on None at the end of the last line.
        let char_oi = if self.line_index + 1 == self.line_count {
            either::Left(
                self.line
                    .chars()
                    .map(|c| Some(c))
                    .chain(std::iter::once(None)),
            )
        } else {
            either::Right(self.line.chars().map(|c| Some(c)))
        };
        for (c_index, char_o) in char_oi.enumerate() {
            let mut view_ctx_g = view_ctx.push_render_address_token((c_index as u32).into());
            sept::qv::UTF8StringTermLineElemCharElemView::new_with_cached_line_and_char(
                self.string,
                self.line_index,
                c_index,
                self.line_count,
                self.line,
                self.line_char_count,
                char_o,
            )
            .unwrap()
            .run_ui_inline(ui, &mut layout_job, &mut view_ctx_g);
        }

        render_postfix_annotation(
            &mut layout_job,
            view_ctx,
            format!(" (chars: {})", self.line_char_count).as_str(),
        );

        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        // Tack on None at the end of the last line.
        let char_oi = if self.line_index + 1 == self.line_count {
            either::Left(
                self.line
                    .chars()
                    .map(|c| Some(c))
                    .chain(std::iter::once(None)),
            )
        } else {
            either::Right(self.line.chars().map(|c| Some(c)))
        };
        for (c_index, char_o) in char_oi.enumerate() {
            let mut view_ctx_g = view_ctx.push_render_address_token((c_index as u32).into());
            let mut view_ctx_g = view_ctx_g.push_nesting_depth();
            sept::qv::UTF8StringTermLineElemCharElemView::new_with_cached_line_and_char(
                self.string,
                self.line_index,
                c_index,
                self.line_count,
                self.line,
                self.line_char_count,
                char_o,
            )
            .unwrap()
            .run_ui_inline(ui, layout_job, &mut view_ctx_g);
        }
    }
}
