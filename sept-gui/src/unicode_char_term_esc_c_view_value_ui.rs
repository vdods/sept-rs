use crate::{layout_job_append, render_type_annotation_for, ValueUIT, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UnicodeCharTermEscCView<'a> {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("this should not be called");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        // Always run this UI inline.
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        self.run_ui_inline(ui, &mut layout_job, view_ctx);
        return layout_job;
    }
    fn run_ui_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        let mut view_ctx_g = view_ctx.push_render_address_token('c'.into());

        layout_job_append(
            layout_job,
            "'",
            view_ctx_g.color_for_unicode_char_quotes(),
            &mut view_ctx_g,
        );
        let escape_char_color = view_ctx_g.color_for_unicode_char_escape_chars();

        for i in 0..Self::indexed_char_count() {
            let mut view_ctx_g = view_ctx_g.push_render_address_token((i as u32).into());
            layout_job_append(
                layout_job,
                self.indexed_char_v[i].to_string().as_str(),
                escape_char_color,
                &mut view_ctx_g,
            );
        }

        // Render the closing quote with the cursor address suffix ('c', 2) so that the cursor can appear on the quote.
        {
            let mut view_ctx_g =
                view_ctx_g.push_render_address_token((Self::indexed_char_count() as u32).into());
            layout_job_append(
                layout_job,
                "'",
                view_ctx_g.color_for_unicode_char_quotes(),
                &mut view_ctx_g,
            );
        }

        // Annotate with the Unicode code point.
        render_type_annotation_for(
            self.c,
            layout_job,
            &mut view_ctx_g,
            Some(format!(" ({})", self.c.escape_unicode()).as_str()),
        );
    }
}
