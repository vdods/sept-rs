use crate::{layout_job_append, render_type_annotation_for, ValueUIT, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UnicodeCharTermPlainView<'a> {
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
        let mut view_ctx_g = view_ctx.push_render_address_token('p'.into());

        layout_job_append(
            layout_job,
            "'",
            view_ctx_g.color_for_unicode_char_quotes(),
            &mut view_ctx_g,
        );
        let regular_char_color = view_ctx_g.color_for::<sept::st::UnicodeCharTerm>();

        {
            let mut view_ctx_g = view_ctx_g.push_render_address_token(0u32.into());
            layout_job_append(
                layout_job,
                self.c.to_string().as_str(),
                regular_char_color,
                &mut view_ctx_g,
            );
        }

        // Render the closing quote with the cursor address suffix ('p', 1) so that the cursor can appear on the quote.
        {
            let mut view_ctx_g = view_ctx_g.push_render_address_token(1u32.into());
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
