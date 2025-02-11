use crate::{
    indentation_for, layout_job_append, render_type_annotation_for_str, ValueUIT, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

impl<'a> ValueUIT for sept::qv::UTF8StringTermLineView<'a> {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("should be deprecated");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut view_ctx_g = view_ctx.push_render_address_token("line".to_string().into());

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        let line_i = sept::st::split_inclusive_allow_trailing_empty(self, '\n');
        let line_count = line_i.clone().count();
        assert!(line_count > 0);
        // TODO: Consider making this in-line special case configurable as part of the ViewOptions.
        if line_count == 1 {
            // if false {
            layout_job_append(
                &mut layout_job,
                "\"",
                view_ctx_g.color_for_utf8string_quotes(),
                &mut view_ctx_g,
            );
            {
                // We don't want the number of chars to show up within the quotes,
                // so disable type annotations (char count annotation is considered a type annotation).
                let mut view_ctx_g = view_ctx_g.push_show_type_annotations(false);
                for (line_index, line) in line_i.enumerate() {
                    let mut view_ctx_g =
                        view_ctx_g.push_render_address_token((line_index as u32).into());
                    layout_job = sept::qv::UTF8StringTermLineElemView::new_with_cached_line(
                        self,
                        line_index,
                        self.line_count,
                        line,
                        line.chars().count(),
                    )
                    .unwrap()
                    .run_ui(ui, &mut view_ctx_g, Some(layout_job));
                }
            }
            layout_job_append(
                &mut layout_job,
                "\"",
                view_ctx_g.color_for_utf8string_quotes(),
                &mut view_ctx_g,
            );
            // TODO: Maybe also annotate number of bytes
            let char_count = self.chars().count();
            render_type_annotation_for_str(
                &mut layout_job,
                &mut view_ctx_g,
                Some(format!(" (lines: {}, chars: {})", line_count, char_count).as_str()),
            );
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
            for (line_index, line) in line_i.enumerate() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<sept::st::UTF8StringTerm>(&mut view_ctx_g));

                    // The content itself expects to be in a vertical.
                    ui.vertical(|ui| {
                        let mut view_ctx_g =
                            view_ctx_g.push_render_address_token((line_index as u32).into());
                        let layout_job =
                            sept::qv::UTF8StringTermLineElemView::new_with_cached_line(
                                self,
                                line_index,
                                self.line_count,
                                line,
                                line.chars().count(),
                            )
                            .unwrap()
                            .run_ui(ui, &mut view_ctx_g, None);

                        ui.label(layout_job);
                    });
                });
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
        let char_count = self.chars().count();
        render_type_annotation_for_str(
            &mut layout_job,
            &mut view_ctx_g,
            Some(format!(" (lines: {}, chars: {})", line_count, char_count).as_str()),
        );
        // Return this to the outer context.
        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        let mut view_ctx_g = view_ctx.push_render_address_token("line".to_string().into());

        layout_job_append(
            layout_job,
            "\"",
            view_ctx_g.color_for_utf8string_quotes(),
            &mut view_ctx_g,
        );
        {
            for (line_index, line) in
                sept::st::split_inclusive_allow_trailing_empty(self, '\n').enumerate()
            {
                let mut view_ctx_g =
                    view_ctx_g.push_render_address_token((line_index as u32).into());
                sept::qv::UTF8StringTermLineElemView::new_with_cached_line(
                    self,
                    line_index,
                    self.line_count,
                    line,
                    line.chars().count(),
                )
                .unwrap()
                .run_ui_inline(ui, layout_job, &mut view_ctx_g);
            }
        }
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
