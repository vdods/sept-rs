use crate::{
    indentation_for, layout_job_append, render_type_annotation_for, LayoutMode, ValueUI, ViewCtx,
};
use egui::{text::LayoutJob, Ui};

impl ValueUI for sept::dy::StructTermTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx<'_>) {
        use egui::{Key, Modifiers};
        let self_len = self.field_tuple().len() as u32;

        // Here is where we resolve the StructTermTerm's r#type into a StructTerm.
        let dereferenced = self.declared_type().dereferenced().unwrap();
        let dereferenced_g = dereferenced.read();
        let direct_type = dereferenced_g
            .downcast_ref::<sept::dy::StructTerm>()
            .expect("StructTermTerm's r#type field did not dereference into StructTerm");
        assert_eq!(direct_type.len(), self.field_tuple().len());

        ui.input_mut(|input_state| {
            if view_ctx.render_address_is_cursor_address() {
                if input_state.consume_key(Modifiers::NONE, Key::Enter) {
                    // Enter this StructTermTerm at the first field name, but only if there is one.
                    if let Some(first_field_decl) = direct_type.first() {
                        view_ctx.cursor_address_push(first_field_decl.0.clone().into());
                    } else {
                        // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                    }
                    // TODO: Use ui.scroll_to_me
                } else if input_state.consume_key(Modifiers::NONE, Key::T) {
                    // Enter this StructTermTerm at the "type".
                    // TEMP HACK -- use the string "type" for now, but later probably use a non-parametric term.
                    // NOTE: This doesn't work if there's a field called "type" in the StructTerm!
                    view_ctx.cursor_address_push("type".to_string().into());
                }
            } else if view_ctx.render_address_is_parent_of_cursor_address() {
                if input_state.consume_key(Modifiers::ALT, Key::Enter)
                    || input_state.consume_key(Modifiers::NONE, Key::Escape)
                {
                    // Escape back to this StructTermTerm.
                    view_ctx.cursor_address_pop();
                } else if input_state.consume_key(Modifiers::NONE, Key::Home) {
                    view_ctx.cursor_address_pop();
                    if let Some(first_field_decl) = direct_type.first() {
                        view_ctx.cursor_address_push(first_field_decl.0.clone().into());
                    } else {
                        // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                    }
                    // TODO: use ui.scroll_to_me
                } else if input_state.consume_key(Modifiers::NONE, Key::End) {
                    view_ctx.cursor_address_pop();
                    if let Some(last_field_decl) = direct_type.last() {
                        view_ctx.cursor_address_push(last_field_decl.0.clone().into());
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
                                element_index_delta += view_ctx.page_up_down_delta as i32
                            }
                        }
                        LayoutMode::Inline => {
                            // In this case, elements are horizontally, so arrow left/right should increase/decrease the element index.
                            if input_state.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                                // adding `self_len - 1` is equivalent to subtracting 1 in modular arithmetic.
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
                    if element_index_delta != 0 {
                        let field_name_value = view_ctx.cursor_address_pop();
                        let field_name = field_name_value
                            .downcast_ref::<String>()
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        match direct_type.index_of_named_field(field_name) {
                            Ok(field_index) => {
                                let new_field_index = (field_index as u32)
                                    .saturating_add_signed(element_index_delta)
                                    .min(self_len - 1);
                                let new_field_name = direct_type[new_field_index as usize].0.clone();
                                view_ctx.cursor_address_push(new_field_name.into());
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "Invalid address token (field name) {:?} under StructTermTerm with address {}",
                                    field_name,
                                    view_ctx.render_address
                                );
                                // Just push the thing back on so we don't change state.
                                view_ctx.cursor_address_push(field_name_value);
                            }
                        };
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

        let mut layout_job = {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            // TEMP HACK -- use the string "type" for now.  later, probably use a char or a non-parametric term that's even more terse.
            // NOTE: This doesn't work if there's a field called "type" in the StructTerm!
            let mut view_ctx_g = view_ctx_g.push_render_address_token("type".to_string().into());
            self.declared_type()
                .run_ui_expanded(ui, &mut view_ctx_g, continuation_layout_job_o)
        };

        // TODO: Maybe there should be some syntax for "construction"

        if self.field_tuple().is_empty() {
            layout_job_append(
                &mut layout_job,
                " {}",
                view_ctx.color_for::<Self>(),
                view_ctx,
            );
            // TODO: Figure out if this should be conditional somehow, since it's often redundant.
            render_type_annotation_for(self, &mut layout_job, view_ctx, None);
            return layout_job;
        }

        layout_job_append(
            &mut layout_job,
            " {",
            view_ctx.color_for::<Self>(),
            view_ctx,
        );
        ui.label(layout_job);

        // Here is where we resolve the StructTermTerm's r#type into a StructTerm.
        let dereferenced = self.declared_type().dereferenced().unwrap();
        let dereferenced_g = dereferenced.read();
        let direct_type = dereferenced_g
            .downcast_ref::<sept::dy::StructTerm>()
            .expect("StructTermTerm's r#type field did not dereference into StructTerm");

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for ((field_name, _field_type), field_value) in
                std::iter::zip(direct_type.iter(), self.field_tuple().iter())
            {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        // TODO: Is it possible to push a reference to the address token here?
                        let mut view_ctx_g = view_ctx_g
                            .push_render_address_token(sept::dy::Value::from(field_name.clone()));

                        let mut layout_job = LayoutJob::default();
                        // There's probably never a reason to render the field name expanded.
                        if view_ctx_g.show_struct_field_name_hints {
                            layout_job_append(
                                &mut layout_job,
                                format!("{:?}: ", field_name).as_str(),
                                view_ctx_g.color_for_type_annotation(),
                                &mut view_ctx_g,
                            );
                        }
                        let mut layout_job =
                            field_value.run_ui(ui, &mut view_ctx_g, Some(layout_job));
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
        // TODO: Figure out if this should be conditional somehow, since it's often redundant.
        render_type_annotation_for(self, &mut layout_job, view_ctx, None);
        // Return this to the outer context
        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        self.handle_events(ui, view_ctx);

        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            // TEMP HACK -- use the string "type" for now.  later, probably use a char or a non-parametric term that's even more terse.
            // NOTE: This doesn't work if there's a field called "type" in the StructTerm!
            let mut view_ctx_g = view_ctx_g.push_render_address_token("type".to_string().into());
            self.declared_type()
                .run_ui_inline(ui, layout_job, &mut view_ctx_g);
        }

        // TODO: It's a space for now, but maybe there should be some syntax for "construction"

        if self.field_tuple().is_empty() {
            layout_job_append(layout_job, " {}", view_ctx.color_for::<Self>(), view_ctx);
            // TODO: Figure out if this should be conditional somehow.
            render_type_annotation_for(self, layout_job, view_ctx, None);
            return;
        }

        layout_job_append(layout_job, " { ", view_ctx.color_for::<Self>(), view_ctx);

        // Here is where we resolve the StructTermTerm's r#type into a StructTerm.
        let dereferenced = self.declared_type().dereferenced().unwrap();
        let dereferenced_g = dereferenced.read();
        let direct_type = dereferenced_g
            .downcast_ref::<sept::dy::StructTerm>()
            .expect("StructTermTerm's r#type field did not dereference into StructTerm");

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for ((field_name, _field_type), field_value) in
                std::iter::zip(direct_type.iter(), self.field_tuple().iter())
            {
                {
                    // TODO: Is it possible to push a reference to the address token here?
                    let mut view_ctx_g = view_ctx_g
                        .push_render_address_token(sept::dy::Value::from(field_name.clone()));

                    // let mut layout_job = LayoutJob::default();
                    // There's probably never a reason to render the field name expanded.
                    if view_ctx_g.show_struct_field_name_hints {
                        layout_job_append(
                            layout_job,
                            format!("{:?}: ", field_name).as_str(),
                            view_ctx_g.color_for_type_annotation(),
                            &mut view_ctx_g,
                        );
                    }
                    // let mut layout_job =
                    //     field_value.run_ui(ui, &mut view_ctx_g, Some(layout_job));
                    field_value.run_ui_inline(ui, layout_job, &mut view_ctx_g);
                    layout_job_append(
                        layout_job,
                        ",",
                        view_ctx_g.color_for::<Self>(),
                        &mut view_ctx_g,
                    );
                }
                // Have to handle the space separately so it doesn't get highlighted with the item, if the outer
                // data is not highlighted.
                layout_job_append(
                    layout_job,
                    " ",
                    view_ctx_g.color_for::<Self>(),
                    &mut view_ctx_g,
                );
            }
        }

        layout_job_append(layout_job, "}", view_ctx.color_for::<Self>(), view_ctx);
        // TODO: Figure out if this should be conditional somehow.
        render_type_annotation_for(self, layout_job, view_ctx, None);
    }
}
