use crate::{
    indentation_for, layout_job_append, render_type_annotation_for, LayoutMode, ValueUI, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

/// This one is for OrderedMapTerm key-value pairs.
impl ValueUI for (&sept::dy::Value, &sept::dy::Value) {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx<'_>) {
        use egui::{Key, Modifiers};
        // Because self is a pair, its len is 2.
        let self_len = 2u32;

        let mut input_g = ui.input_mut();
        if view_ctx.render_address_is_cursor_address() {
            if input_g.consume_key(Modifiers::NONE, Key::Enter)
                || input_g.consume_key(Modifiers::NONE, Key::K)
            {
                // Enter this key-value pair at element 0.
                // TODO: Consider making a "k" term to use here instead.
                view_ctx.cursor_address_push(0u32.into());
            } else if input_g.consume_key(Modifiers::NONE, Key::V) {
                // Enter this key-value pair at element 1.
                // TODO: Consider making a "v" term to use here instead.
                view_ctx.cursor_address_push(1u32.into());
            }
        } else if view_ctx.render_address_is_parent_of_cursor_address() {
            if input_g.consume_key(Modifiers::ALT, Key::Enter)
                || input_g.consume_key(Modifiers::NONE, Key::Escape)
            {
                // Escape back to this key-value pair.
                view_ctx.cursor_address_pop();
            } else if input_g.consume_key(Modifiers::NONE, Key::Home)
                || input_g.consume_key(Modifiers::NONE, Key::PageUp)
            {
                view_ctx.cursor_address_pop();
                view_ctx.cursor_address_push(0u32.into());
                // TODO: use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::End)
                || input_g.consume_key(Modifiers::NONE, Key::PageDown)
            {
                view_ctx.cursor_address_pop();
                view_ctx.cursor_address_push(1u32.into());
                // TODO: use ui.scroll_to_me
            } else {
                // Handle arrow keys for element navigation.
                // Depending on if this View is Expanded vs Inline, the arrow keys mean different things.
                let mut element_index_delta = 0i32;
                match view_ctx.layout_mode() {
                    LayoutMode::Expanded => {
                        // In this case, elements are vertically, so arrow up/down should increase/decrease the element index.
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowUp) {
                            element_index_delta -= 1;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowDown) {
                            element_index_delta += 1;
                        }
                    }
                    LayoutMode::Inline => {
                        // In this case, elements are horizontally, so arrow left/right should increase/decrease the element index.
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                            // adding `self_len - 1` is equivalent to subtracting 1 in modular arithmetic.
                            element_index_delta -= 1;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowRight) {
                            element_index_delta += 1;
                        }
                        // TODO: Vertical movement; a logical version would simply increment/decrement the parent address index (or key)
                        // and keep the child address index, so that the cursor moves to the analogous element of the "uncle" value.
                    }
                };
                let element_index_value = view_ctx.cursor_address_pop();
                if element_index_value.is::<u32>() {
                    let mut element_index = element_index_value.downcast_into::<u32>();
                    if element_index <= 1 {
                        // TODO: Handle one-past-the-end index for insertions
                        element_index = element_index
                            .saturating_add_signed(element_index_delta)
                            .min(self_len - 1);
                        view_ctx.cursor_address_push(element_index.into());
                        // TODO: use ui.scroll_to_me
                    } else {
                        tracing::warn!(
                            "Invalid address token {} under key-value pair with address {}",
                            element_index,
                            view_ctx.render_address
                        );
                    }
                } else {
                    tracing::warn!(
                        "Invalid address token {} under key-value pair with address {}",
                        element_index_value,
                        view_ctx.render_address
                    );
                }
            }
        }
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        // TODO: Implement addressing of key vs value
        let mut layout_job = {
            let mut view_ctx_g = view_ctx.push_render_address_token(0u32.into());
            self.0
                .run_ui_expanded(ui, &mut view_ctx_g, continuation_layout_job_o)
        };
        layout_job_append(
            &mut layout_job,
            " => ",
            view_ctx.color_for::<sept::dy::OrderedMapTerm>(),
            view_ctx,
        );
        // We pass in layout_job as continuation_layout_job_o so that it renders starting on the same
        // line as " => ".
        let layout_job = {
            let mut view_ctx_g = view_ctx.push_render_address_token(1u32.into());
            self.1
                .run_ui_expanded(ui, &mut view_ctx_g, Some(layout_job))
        };
        // Return this to the outer context
        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        self.handle_events(ui, view_ctx);

        {
            let mut view_ctx_g = view_ctx.push_render_address_token(0u32.into());
            self.0.run_ui_inline(ui, layout_job, &mut view_ctx_g);
        }
        layout_job_append(
            layout_job,
            " => ",
            view_ctx.color_for::<sept::dy::OrderedMapTerm>(),
            view_ctx,
        );
        {
            let mut view_ctx_g = view_ctx.push_render_address_token(1u32.into());
            self.1.run_ui_inline(ui, layout_job, &mut view_ctx_g);
        }
    }
}

impl ValueUI for sept::dy::OrderedMapTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx<'_>) {
        use egui::{Key, Modifiers};
        // let self_len = self.len() as u32;

        let mut input_g = ui.input_mut();
        if view_ctx.render_address_is_cursor_address() {
            if input_g.consume_key(Modifiers::NONE, Key::Enter) {
                // Enter this OrderedMapTerm at the first key, but only if there is one.
                if let Some(first_key_value) = self.first_key_value() {
                    view_ctx.cursor_address_push(first_key_value.0.clone());
                } else {
                    // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                }
                // TODO: Use ui.scroll_to_me
            }
        } else if view_ctx.render_address_is_parent_of_cursor_address() {
            if input_g.consume_key(Modifiers::ALT, Key::Enter)
                || input_g.consume_key(Modifiers::NONE, Key::Escape)
            {
                // Escape back to this OrderedMapTerm.
                view_ctx.cursor_address_pop();
            } else if input_g.consume_key(Modifiers::NONE, Key::Home) {
                view_ctx.cursor_address_pop();
                if let Some(first_key_value) = self.first_key_value() {
                    view_ctx.cursor_address_push(first_key_value.0.clone());
                } else {
                    // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                }
                // TODO: use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::End) {
                view_ctx.cursor_address_pop();
                if let Some(last_key_value) = self.last_key_value() {
                    view_ctx.cursor_address_push(last_key_value.0.clone());
                } else {
                    // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                }
                // TODO: use ui.scroll_to_me
            } else {
                // Handle arrow keys for element navigation.
                // Depending on if this View is Expanded vs Inline, the arrow keys mean different things.
                // TODO: Factor this out into a function
                let mut element_index_delta = 0i32;
                match view_ctx.layout_mode() {
                    LayoutMode::Expanded => {
                        // In this case, elements are vertically, so arrow up/down should increase/decrease the element index.
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowUp) {
                            element_index_delta -= 1;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowDown) {
                            element_index_delta += 1;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::PageUp) {
                            element_index_delta -= view_ctx.page_up_down_delta as i32;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::PageDown) {
                            element_index_delta += view_ctx.page_up_down_delta as i32
                        }
                    }
                    LayoutMode::Inline => {
                        // In this case, elements are horizontally, so arrow left/right should increase/decrease the element index.
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                            // adding `self_len - 1` is equivalent to subtracting 1 in modular arithmetic.
                            element_index_delta -= 1;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowRight) {
                            element_index_delta += 1;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::PageUp) {
                            element_index_delta -= view_ctx.page_up_down_delta as i32;
                        }
                        if input_g.consume_key(Modifiers::NONE, Key::PageDown) {
                            element_index_delta += view_ctx.page_up_down_delta as i32;
                        }
                        // TODO: Vertical movement; a logical version would simply increment/decrement the parent address index (or key)
                        // and keep the child address index, so that the cursor moves to the analogous element of the "uncle" value.
                    }
                };
                if element_index_delta != 0 {
                    let key = view_ctx.cursor_address_pop();
                    if !self.contains_key(&key) {
                        tracing::warn!(
                            "Invalid address token {} under OrderedMapTerm with address {}",
                            key,
                            view_ctx.render_address
                        );
                    } else {
                        let new_key = if element_index_delta < 0 {
                            let range = self.range::<sept::dy::Value, _>(..=&key);
                            let range_len = range.clone().count() as u32;
                            assert!(range_len > 0);
                            let element_abs_delta =
                                (element_index_delta.abs() as u32).min(range_len - 1);
                            let new_key = range
                                .rev()
                                .nth(element_abs_delta as usize)
                                .unwrap()
                                .0
                                .clone();
                            new_key
                        } else {
                            assert!(element_index_delta > 0);
                            let mut range = self.range::<sept::dy::Value, _>(&key..);
                            let range_len = range.clone().count() as u32;
                            assert!(range_len > 0);
                            let element_abs_delta = (element_index_delta as u32).min(range_len - 1);
                            let new_key = range.nth(element_abs_delta as usize).unwrap().0.clone();
                            new_key
                        };
                        view_ctx.cursor_address_push(new_key);
                    }
                }
            }
        }
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
                "{}",
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

        layout_job_append(&mut layout_job, "{", view_ctx.color_for::<Self>(), view_ctx);
        ui.label(layout_job);

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for key_value_pair in self.iter() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        // TODO: Is it possible to push a reference to the address token here instead?
                        let mut view_ctx_g =
                            view_ctx_g.push_render_address_token(key_value_pair.0.clone());
                        let mut layout_job = key_value_pair.run_ui(ui, &mut view_ctx_g, None);
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
        layout_job_append(&mut layout_job, "}", view_ctx.color_for::<Self>(), view_ctx);
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
            layout_job_append(layout_job, "{}", view_ctx.color_for::<Self>(), view_ctx);
            render_type_annotation_for(
                self,
                layout_job,
                view_ctx,
                Some(format!(" (len: {})", self.len()).as_str()),
            );
            return;
        }

        layout_job_append(layout_job, "{ ", view_ctx.color_for::<Self>(), view_ctx);
        for key_value_pair in self.iter() {
            {
                // TODO: Is it possible to push a reference to the address token here?
                let mut view_ctx_g = view_ctx.push_render_address_token(key_value_pair.0.clone());
                key_value_pair.run_ui_inline(ui, layout_job, &mut view_ctx_g);
                layout_job_append(layout_job, ",", view_ctx_g.color_for::<Self>(), &view_ctx_g);
            }
            // Have to handle the space separately so it doesn't get highlighted with the item, if the outer
            // data is not highlighted.
            layout_job_append(layout_job, " ", view_ctx.color_for::<Self>(), &view_ctx);
        }
        layout_job_append(layout_job, "}", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(
            self,
            layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
    }
}
