use crate::{ValueUIT, ViewCtx};
use egui::{text::LayoutJob, Ui};

impl ValueUIT for sept::st::UTF8StringTerm {
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
                    return sept::qv::UTF8StringTermLineView::new(self).run_ui_expanded(
                        ui,
                        view_ctx,
                        continuation_layout_job_o,
                    );
                }
                Some("char") => {
                    // TODO: Does this need to call run_ui instead of run_ui_expanded?
                    return sept::qv::UTF8StringTermCharView::new(self).run_ui_expanded(
                        ui,
                        view_ctx,
                        continuation_layout_job_o,
                    );
                }
                Some(mode) => {
                    tracing::warn!("Invalid view mode {:?} for UTF8StringTerm", mode);
                }
                None => {
                    use sept::st::StringifiableT;
                    tracing::warn!(
                        "Invalid view mode {} for UTF8StringTerm",
                        guide_token.stringify()
                    );
                }
            }
        }

        // There was no guide token, so just do what the LayoutMode expects.  Because we're in run_ui_expanded,
        // use "line" view.
        // TODO: Does this need to call run_ui instead of run_ui_expanded?
        // Just use "line" mode for everything.
        sept::qv::UTF8StringTermLineView::new(self).run_ui_expanded(
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
                    return sept::qv::UTF8StringTermLineView::new(self)
                        .run_ui_inline(ui, layout_job, view_ctx);
                }
                Some("char") => {
                    // TODO: Does this need to call run_ui instead of run_ui_expanded?
                    return sept::qv::UTF8StringTermCharView::new(self)
                        .run_ui_inline(ui, layout_job, view_ctx);
                }
                Some(mode) => {
                    tracing::warn!("Invalid view mode {:?} for UTF8StringTerm", mode);
                }
                None => {
                    use sept::st::StringifiableT;
                    tracing::warn!(
                        "Invalid view mode {} for UTF8StringTerm",
                        guide_token.stringify()
                    );
                }
            }
        }

        // // There was no guide token, so just do what the LayoutMode expects.  Because we're in run_ui_inline,
        // // use "char" view.
        // // TODO: Does this need to call run_ui instead of run_ui_inline?
        // sept::qv::UTF8StringTermCharView::new(self).run_ui_inline(ui, layout_job, view_ctx)

        // Just use "line" mode for everything.
        // TODO: Does this need to call run_ui instead of run_ui_expanded?
        return sept::qv::UTF8StringTermLineView::new(self).run_ui_inline(ui, layout_job, view_ctx);
    }
}
