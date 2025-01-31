use crate::{
    layout_job_append, render_str_as_literal_without_quotes, render_type_annotation_for, ANSIColor,
    ValueUIT, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

impl ValueUIT for sept::dy::LocalSymRefTerm {
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        self.run_ui_inline(ui, &mut layout_job, view_ctx);
        layout_job
    }
    fn run_ui_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        let local_symbol_table_g = self.local_symbol_table().read().unwrap();
        let (resolved, path, dollar_color, quote_color, regular_char_color, escape_char_color) =
            match local_symbol_table_g.resolved_symbol_path(self.symbol_id.as_str()) {
                Ok(resolved_symbol_path) => (
                    true,
                    resolved_symbol_path,
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_local_sym_ref_quotes(),
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_local_sym_ref_escape_chars(),
                ),
                Err(_) => (
                    false,
                    local_symbol_table_g
                        .unresolved_symbol_path(self.symbol_id.as_str())
                        .expect("temp hack"),
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_local_sym_ref_quotes(),
                    ANSIColor::BRIGHT_RED,
                    ANSIColor::DARK_RED,
                ),
            };

        layout_job_append(layout_job, "$", dollar_color, view_ctx);
        layout_job_append(layout_job, "\"", quote_color, view_ctx);
        render_str_as_literal_without_quotes(
            path.as_str(),
            layout_job,
            view_ctx,
            regular_char_color,
            escape_char_color,
            0,
        );
        layout_job_append(layout_job, "\"", quote_color, view_ctx);
        if resolved {
            render_type_annotation_for(self, layout_job, view_ctx, None);
        }
    }
}
