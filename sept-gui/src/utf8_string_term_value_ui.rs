use crate::{
    indentation_for, layout_job_append, render_postfix_annotation,
    render_str_as_literal_without_quotes, render_type_annotation_for_str,
    value_ui::END_OF_TRANSMISSION_CHAR, ValueUI, ViewCtx, END_OF_TRANSMISSION_STR,
};
use egui::{text::LayoutJob, Ui};

impl ValueUI for sept::st::Utf8StringTerm {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("this should not be called");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        // If cursor address is a proper subaddress of render address (meaning render address is a proper prefix of
        // cursor address), then we should "follow" the next token in the cursor, since that's what the cursor is
        // meant to be viewing.
        if let Some(guide_token) = view_ctx.cursor_address_guide_token() {
            match guide_token.downcast_ref::<String>().map(|x| x.as_str()) {
                Some("line") => {
                    // TODO: Does this need to call run_ui instead of run_ui_expanded?
                    return sept::qv::Utf8StringTermLineView::new(self).run_ui_expanded(
                        ui,
                        view_ctx,
                        continuation_layout_job_o,
                    );
                }
                Some("char") => {
                    // TODO: Does this need to call run_ui instead of run_ui_expanded?
                    return sept::qv::Utf8StringTermCharView::new(self).run_ui_expanded(
                        ui,
                        view_ctx,
                        continuation_layout_job_o,
                    );
                }
                Some(mode) => {
                    tracing::warn!("Invalid view mode {:?} for Utf8StringTerm", mode);
                }
                None => {
                    use sept::st::Stringifiable;
                    tracing::warn!(
                        "Invalid view mode {} for Utf8StringTerm",
                        guide_token.stringify()
                    );
                }
            }
        }

        // There was no guide token, so just do what the LayoutMode expects.  Because we're in run_ui_expanded,
        // use "line" view.
        // TODO: Does this need to call run_ui instead of run_ui_expanded?
        // Just use "line" mode for everything.
        sept::qv::Utf8StringTermLineView::new(self).run_ui_expanded(
            ui,
            view_ctx,
            continuation_layout_job_o,
        )
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        // If cursor address is a proper subaddress of render address (meaning render address is a proper prefix of
        // cursor address), then we should "follow" the next token in the cursor, since that's what the cursor is
        // meant to be viewing.
        if let Some(guide_token) = view_ctx.cursor_address_guide_token() {
            match guide_token.downcast_ref::<String>().map(|x| x.as_str()) {
                Some("line") => {
                    // TODO: Does this need to call run_ui instead of run_ui_expanded?
                    return sept::qv::Utf8StringTermLineView::new(self)
                        .run_ui_inline(ui, layout_job, view_ctx);
                }
                Some("char") => {
                    // TODO: Does this need to call run_ui instead of run_ui_expanded?
                    return sept::qv::Utf8StringTermCharView::new(self)
                        .run_ui_inline(ui, layout_job, view_ctx);
                }
                Some(mode) => {
                    tracing::warn!("Invalid view mode {:?} for Utf8StringTerm", mode);
                }
                None => {
                    use sept::st::Stringifiable;
                    tracing::warn!(
                        "Invalid view mode {} for Utf8StringTerm",
                        guide_token.stringify()
                    );
                }
            }
        }

        // // There was no guide token, so just do what the LayoutMode expects.  Because we're in run_ui_inline,
        // // use "char" view.
        // // TODO: Does this need to call run_ui instead of run_ui_inline?
        // sept::qv::Utf8StringTermCharView::new(self).run_ui_inline(ui, layout_job, view_ctx)

        // Just use "line" mode for everything.
        // TODO: Does this need to call run_ui instead of run_ui_expanded?
        return sept::qv::Utf8StringTermLineView::new(self).run_ui_inline(ui, layout_job, view_ctx);
    }
}

impl<'a> ValueUI for sept::qv::Utf8StringTermCharView<'a> {
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
            let regular_char_color = view_ctx_g.color_for::<sept::st::Utf8StringTerm>();
            let escape_char_color = view_ctx_g.color_for_utf8string_escape_chars();

            let mut view_ctx_g = view_ctx_g.push_nesting_depth();

            let line_i = sept::st::split_inclusive_allow_trailing_empty(self, '\n');
            let line_count = line_i.clone().count();
            assert!(line_count > 0);

            let mut char_index_begin = 0usize;
            for (line_index, line) in line_i.enumerate() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<sept::st::Utf8StringTerm>(&mut view_ctx_g));

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
        let regular_char_color = view_ctx_g.color_for::<sept::st::Utf8StringTerm>();
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

impl<'a> ValueUI for sept::qv::Utf8StringTermLineView<'a> {
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
                    layout_job = sept::qv::Utf8StringTermLineElemView::new_with_cached_line(
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
                    ui.label(indentation_for::<sept::st::Utf8StringTerm>(&mut view_ctx_g));

                    // The content itself expects to be in a vertical.
                    ui.vertical(|ui| {
                        let mut view_ctx_g =
                            view_ctx_g.push_render_address_token((line_index as u32).into());
                        let layout_job =
                            sept::qv::Utf8StringTermLineElemView::new_with_cached_line(
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
                sept::qv::Utf8StringTermLineElemView::new_with_cached_line(
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

impl<'a> ValueUI for sept::qv::Utf8StringTermLineElemView<'a> {
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

        let layout_job = sept::qv::Utf8StringTermLineElemCharView::new_with_cached_line(
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

        sept::qv::Utf8StringTermLineElemCharView::new_with_cached_line(
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

impl<'a> ValueUI for sept::qv::Utf8StringTermLineElemCharView<'a> {
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
            sept::qv::Utf8StringTermLineElemCharElemView::new_with_cached_line_and_char(
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
            sept::qv::Utf8StringTermLineElemCharElemView::new_with_cached_line_and_char(
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

impl<'a> ValueUI for sept::qv::Utf8StringTermLineElemCharElemView<'a> {
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
        let regular_char_color = view_ctx.color_for::<sept::st::Utf8StringTerm>();
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
