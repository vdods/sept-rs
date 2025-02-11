use crate::{
    layout_job_append, value_ui_t::END_OF_TRANSMISSION_CHAR, ValueUIT, ViewCtx, END_OF_TRANSMISSION_STR,
};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UTF8StringTermLineElemCharElemView<'a> {
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
        self.run_ui_inline(ui, &mut layout_job, view_ctx);
        layout_job
    }
    fn run_ui_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        let regular_char_color = view_ctx.color_for::<sept::st::UTF8StringTerm>();
        let escape_char_color = view_ctx.color_for_utf8string_escape_chars();

        if let Some(c) = self.char_o {
            // TEMP HACK -- handle char by char for now.  Maybe this is plenty efficient, hopefully LayoutJob does
            // the correct buffering.
            if c == '\\'
                || c == '\"'
                || (c as u32) < (' ' as u32)
                || (c as u32) > ('~' as u32)
                || c == END_OF_TRANSMISSION_CHAR
            {
                layout_job_append(
                    layout_job,
                    c.escape_default().to_string().as_str(),
                    escape_char_color,
                    view_ctx,
                );
            } else {
                layout_job_append(
                    layout_job,
                    c.to_string().as_str(),
                    regular_char_color,
                    view_ctx,
                );
            }
        } else {
            // char_o == None means that we're at the end of the last line.

            // This is a bit of a hack in order to have a place for the cursor to be rendered
            // when the cursor is at the exact end of the line.
            layout_job_append(
                layout_job,
                END_OF_TRANSMISSION_STR,
                escape_char_color,
                view_ctx,
            );
        }
    }
}
