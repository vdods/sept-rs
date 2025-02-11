use crate::{
    indentation_for, layout_job_append, render_postfix_annotation,
    render_str_as_literal_without_quotes, render_type_annotation_for_str,
    ValueUIT, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UTF8StringTermCharView<'a> {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("this should not be called");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut view_ctx_g = view_ctx.push_render_address_token("char".to_string().into());

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        // TODO: Consider making this in-line special case configurable as part of the ViewOptions.
        if self.is_empty() {
            self.run_ui_inline(ui, &mut layout_job, &mut view_ctx_g);
            return layout_job;
        }

        layout_job_append(
            &mut layout_job,
            "\"",
            view_ctx_g.color_for_utf8string_quotes(),
            &mut view_ctx_g,
        );
        ui.label(layout_job);

        {
            let regular_char_color = view_ctx_g.color_for::<sept::st::UTF8StringTerm>();
            let escape_char_color = view_ctx_g.color_for_utf8string_escape_chars();

            let mut view_ctx_g = view_ctx_g.push_nesting_depth();

            let line_i = sept::st::split_inclusive_allow_trailing_empty(self, '\n');
            let line_count = line_i.clone().count();
            assert!(line_count > 0);

            let mut char_index_begin = 0usize;
            for (line_index, line) in line_i.enumerate() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<sept::st::UTF8StringTerm>(&mut view_ctx_g));

                    // The content itself expects to be in a vertical.
                    ui.vertical(|ui| {
                        let mut layout_job = LayoutJob::default();
                        render_str_as_literal_without_quotes(
                            line,
                            &mut layout_job,
                            &mut view_ctx_g,
                            regular_char_color,
                            escape_char_color,
                            char_index_begin,
                        );
                        let line_char_count = line.chars().count();
                        if line_index + 1 == line_count {
                            // This is a bit of a hack in order to render the cursor when the cursor is at
                            // the exact end of the line.  TODO: Consider not rendering the space before
                            // "(chars: {})" in the following call.
                            render_str_as_literal_without_quotes(
                                " ",
                                &mut layout_job,
                                &mut view_ctx_g,
                                regular_char_color,
                                escape_char_color,
                                char_index_begin + line_char_count,
                            );
                        }
                        // Annotation which shows how many chars are in this line (TODO: Make this configurable)
                        render_postfix_annotation(
                            &mut layout_job,
                            &mut view_ctx_g,
                            format!(" (chars: {})", line_char_count).as_str(),
                        );
                        ui.label(layout_job);
                    });
                });
                char_index_begin += line.chars().count();
            }
        }

        let mut layout_job = LayoutJob::default();
        layout_job_append(
            &mut layout_job,
            "\"",
            view_ctx_g.color_for_utf8string_quotes(),
            &mut view_ctx_g,
        );
        // TODO: Maybe also annotate number of bytes
        let line_count = sept::st::split_inclusive_allow_trailing_empty(self, '\n').count();
        let char_count = self.chars().count();
        render_type_annotation_for_str(
            &mut layout_job,
            &mut view_ctx_g,
            Some(format!(" (lines: {}, chars: {})", line_count, char_count).as_str()),
        );
        // Return this to the outer context.
        layout_job
    }
    fn run_ui_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        let mut view_ctx_g = view_ctx.push_render_address_token("char".to_string().into());

        layout_job_append(
            layout_job,
            "\"",
            view_ctx_g.color_for_utf8string_quotes(),
            &mut view_ctx_g,
        );
        let regular_char_color = view_ctx_g.color_for::<sept::st::UTF8StringTerm>();
        let escape_char_color = view_ctx_g.color_for_utf8string_escape_chars();
        render_str_as_literal_without_quotes(
            self,
            layout_job,
            &mut view_ctx_g,
            regular_char_color,
            escape_char_color,
            0,
        );

        layout_job_append(
            layout_job,
            "\"",
            view_ctx_g.color_for_utf8string_quotes(),
            &mut view_ctx_g,
        );
        // TODO: Maybe also annotate number of bytes
        let line_count = sept::st::split_inclusive_allow_trailing_empty(self, '\n').count();
        let char_count = self.chars().count();
        render_type_annotation_for_str(
            layout_job,
            &mut view_ctx_g,
            Some(format!(" (lines: {}, chars: {})", line_count, char_count).as_str()),
        );
    }
}
