use crate::{ValueUIT, ViewCtx};
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
