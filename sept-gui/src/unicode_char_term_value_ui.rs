use crate::{layout_job_append, render_type_annotation_for, ValueUIT, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl ValueUIT for sept::st::UnicodeCharTerm {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("should be deprecated");
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
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        // If cursor address is a proper subaddress of render address (meaning render address is a proper prefix of
        // cursor address), then we should "follow" the next token in the cursor, since that's what the cursor is
        // meant to be viewing.
        if let Some(guide_token) = view_ctx.cursor_address_guide_token() {
            match guide_token
                .downcast_ref::<sept::st::UnicodeCharTerm>()
                .map(|x| *x)
            {
                Some('p') => {
                    return sept::qv::UnicodeCharTermPlainView::new(self)
                        .unwrap()
                        .run_ui_inline(ui, layout_job, view_ctx);
                }
                Some('c') => {
                    return sept::qv::UnicodeCharTermEscCView::new(self)
                        .unwrap()
                        .run_ui_inline(ui, layout_job, view_ctx);
                }
                Some(mode) => {
                    tracing::warn!("Invalid view mode {:?} for UnicodeCharTerm", mode);
                }
                None => {
                    use sept::st::StringifiableT;
                    tracing::warn!(
                        "Invalid view mode {} for UnicodeCharTermView",
                        guide_token.stringify()
                    );
                }
            }
        }

        // There was no guide token, so determine the view mode based on the character.
        if sept::qv::UnicodeCharTermView::is_plain_char(*self) {
            return sept::qv::UnicodeCharTermPlainView::new(self)
                .unwrap()
                .run_ui_inline(ui, layout_job, view_ctx);
        } else if sept::qv::UnicodeCharTermView::is_single_char_escape(*self) {
            return sept::qv::UnicodeCharTermEscCView::new(self)
                .unwrap()
                .run_ui_inline(ui, layout_job, view_ctx);
        } else {
            unimplemented!("UnicodeCharTermView::run_ui_inline; only plain and single-char-escape ASCII chars are supported currently; char was {}", self.escape_debug());
        }
    }
}

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
