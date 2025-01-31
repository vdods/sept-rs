use crate::{
    indentation_for, layout_job_append, render_type_annotation_for, LayoutMode, ValueUIT, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

impl ValueUIT for sept::dy::TupleTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx<'_>) {
        use egui::{Key, Modifiers};
        let self_len = self.len() as u32;

        ui.input_mut(|input_state| {
            if view_ctx.render_address_is_cursor_address() {
                if input_state.consume_key(Modifiers::NONE, Key::Enter) {
                    // Enter this TupleTerm at element 0.
                    view_ctx.cursor_address_push(0u32.into());
                }
            } else if view_ctx.render_address_is_parent_of_cursor_address() {
                if input_state.consume_key(Modifiers::ALT, Key::Enter)
                    || input_state.consume_key(Modifiers::NONE, Key::Escape)
                {
                    // Escape back to this TupleTerm.
                    view_ctx.cursor_address_pop();
                } else if input_state.consume_key(Modifiers::NONE, Key::Home) {
                    view_ctx.cursor_address_pop();
                    view_ctx.cursor_address_push(0u32.into());
                    // TODO: use ui.scroll_to_me
                } else if input_state.consume_key(Modifiers::NONE, Key::End) {
                    view_ctx.cursor_address_pop();
                    view_ctx.cursor_address_push((self_len - 1).into());
                    // TODO: use ui.scroll_to_me
                } else if self_len > 0 {
                    // Handle arrow keys for element navigation.
                    // Depending on if this View is Expanded vs Inline, the arrow keys mean different things.
                    let mut element_index_delta = 0i32;
                    match view_ctx.layout_mode() {
                        LayoutMode::Expanded => {
                            // In this case, elements are vertically, so arrow up/down should increase/decrease the element index.
                            if input_state.consume_key(Modifiers::NONE, Key::ArrowUp) {
                                element_index_delta -= 1;
                            }
                            if input_state.consume_key(Modifiers::NONE, Key::ArrowDown) {
                                element_index_delta += 1;
                            }
                            if input_state.consume_key(Modifiers::NONE, Key::PageUp) {
                                element_index_delta -= view_ctx.page_up_down_delta as i32;
                            }
                            if input_state.consume_key(Modifiers::NONE, Key::PageDown) {
                                element_index_delta += view_ctx.page_up_down_delta as i32;
                            }
                        }
                        LayoutMode::Inline => {
                            // In this case, elements are horizontally, so arrow left/right should increase/decrease the element index.
                            if input_state.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                                element_index_delta -= 1;
                            }
                            if input_state.consume_key(Modifiers::NONE, Key::ArrowRight) {
                                element_index_delta += 1;
                            }
                            if input_state.consume_key(Modifiers::NONE, Key::PageUp) {
                                element_index_delta -= view_ctx.page_up_down_delta as i32;
                            }
                            if input_state.consume_key(Modifiers::NONE, Key::PageDown) {
                                element_index_delta += view_ctx.page_up_down_delta as i32;
                            }
                            // TODO: Vertical movement; a logical version would simply increment/decrement the parent address index (or key)
                            // and keep the child address index, so that the cursor moves to the analogous element of the "uncle" value.
                        }
                    };
                    let element_index_value = view_ctx.cursor_address_pop();
                    if element_index_value.is::<u32>() {
                        let mut element_index = element_index_value.downcast_into::<u32>();
                        // TODO: Handle one-past-the-end index for insertions
                        element_index = element_index
                            .saturating_add_signed(element_index_delta)
                            .min(self_len - 1);
                        view_ctx.cursor_address_push(element_index.into());
                        // TODO: use ui.scroll_to_me
                    } else {
                        tracing::warn!(
                            "Invalid address token {} under TupleTerm with address {}",
                            element_index_value,
                            view_ctx.render_address
                        );
                    }
                }
            }
        });
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        if self.is_empty() {
            layout_job_append(
                &mut layout_job,
                "()",
                view_ctx.color_for::<Self>(),
                view_ctx,
            );
            render_type_annotation_for(
                self,
                &mut layout_job,
                view_ctx,
                Some(format!(" (len: {})", self.len()).as_str()),
            );
            return layout_job;
        }

        layout_job_append(&mut layout_job, "(", view_ctx.color_for::<Self>(), view_ctx);
        ui.label(layout_job);

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for (i, element) in self.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        let mut view_ctx_g =
                            view_ctx_g.push_render_address_token(sept::dy::Value::from(i as u32));
                        let mut layout_job = element.run_ui(ui, &mut view_ctx_g, None);
                        layout_job_append(
                            &mut layout_job,
                            ",",
                            view_ctx_g.color_for::<Self>(),
                            &mut view_ctx_g,
                        );
                        ui.label(layout_job);
                    });
                });
            }
        }

        let mut layout_job = LayoutJob::default();
        layout_job_append(&mut layout_job, ")", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(
            self,
            &mut layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
        // Return this to the outer context.
        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        self.handle_events(ui, view_ctx);

        if self.is_empty() {
            layout_job_append(layout_job, "()", view_ctx.color_for::<Self>(), view_ctx);
            render_type_annotation_for(
                self,
                layout_job,
                view_ctx,
                Some(format!(" (len: {})", self.len()).as_str()),
            );
            return;
        }

        layout_job_append(layout_job, "( ", view_ctx.color_for::<Self>(), view_ctx);
        for (i, element) in self.iter().enumerate() {
            {
                let mut view_ctx_g =
                    view_ctx.push_render_address_token(sept::dy::Value::from(i as u32));
                element.run_ui_inline(ui, layout_job, &mut view_ctx_g);
                layout_job_append(layout_job, ",", view_ctx_g.color_for::<Self>(), &view_ctx_g);
            }
            // Have to handle the space separately so it doesn't get highlighted with the item, if the outer
            // data is not highlighted.
            layout_job_append(layout_job, " ", view_ctx.color_for::<Self>(), view_ctx);
        }
        layout_job_append(layout_job, ")", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(
            self,
            layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
    }
}
