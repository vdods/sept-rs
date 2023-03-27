use crate::{ANSIColor, LayoutDiscriminant, LayoutMode, ViewCtx};
use egui::{text::LayoutJob, Ui};

pub trait View {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx) {
        // Do nothing by default.
    }
    /// If continuation_layout_job_o is not None, then it must be used for whatever the first line in
    /// the item rendering is.  If there's only one line in the rendering, then it would also be returned.
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob;
    /// update_inline shouldn't add anything to `ui`, it should add things to `layout_job`.  The `ui`
    /// parameter is only there so update_inline has access to events.
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx);
    fn update(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        match view_ctx.layout_discriminant() {
            LayoutDiscriminant::Expanded => {
                self.update_expanded(ui, view_ctx, continuation_layout_job_o)
            }
            LayoutDiscriminant::BoundaryLevelInline => {
                let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
                self.update_inline(ui, &mut layout_job, view_ctx);
                layout_job
            }
            LayoutDiscriminant::InteriorLevelInline => {
                panic!("programmer error: this should not be called from InteriorLevelInline");
            }
        }
    }
}

fn layout_job_append(
    layout_job: &mut LayoutJob,
    text: &str,
    foreground_color: egui::Color32,
    view_ctx: &ViewCtx,
) {
    let (foreground_color, background_color) =
        view_ctx.set_highlight_if_necessary(foreground_color);
    let text_format = egui::TextFormat {
        font_id: view_ctx.font_id.clone(),
        color: foreground_color,
        background: background_color,
        ..Default::default()
    };
    layout_job.append(text, 0.0, text_format);
}

fn indentation_for<T: 'static>(view_ctx: &ViewCtx) -> LayoutJob {
    let foreground_color = view_ctx.color_for_indentation_for::<T>();
    let (foreground_color, background_color) =
        view_ctx.set_highlight_if_necessary(foreground_color);
    let text_format = egui::TextFormat {
        font_id: view_ctx.font_id.clone(),
        color: foreground_color,
        background: background_color,
        ..Default::default()
    };
    LayoutJob::single_section(view_ctx.indent_str().to_string(), text_format)
}

fn render_type_annotation_for<T: sept::st::TermTrait>(
    term: &T,
    layout_job: &mut LayoutJob,
    view_ctx: &mut ViewCtx,
    extra_text_o: Option<&str>,
) where
    <T as sept::st::TermTrait>::AbstractTypeType: sept::st::Stringifiable,
{
    if view_ctx.show_type_annotations {
        use sept::st::Stringifiable;
        let extra_text = extra_text_o.unwrap_or("");
        layout_job_append(
            layout_job,
            format!(": {}{}", term.abstract_type().stringify(), extra_text).as_str(),
            view_ctx.color_for_type_annotation(),
            view_ctx,
        );
    }
}

// This is probably a TEMP HACK
macro_rules! impl_view_using_to_string {
    ($ty:ty) => {
        impl View for $ty {
            fn update_expanded(
                &self,
                _ui: &mut egui::Ui,
                view_ctx: &mut ViewCtx,
                continuation_layout_job_o: Option<LayoutJob>,
            ) -> egui::text::LayoutJob {
                let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
                use sept::st::Stringifiable;
                layout_job_append(
                    &mut layout_job,
                    self.stringify().as_str(),
                    view_ctx.color_for::<$ty>(),
                    view_ctx,
                );
                render_type_annotation_for(self, &mut layout_job, view_ctx, None);
                layout_job
            }
            fn update_inline(
                &self,
                _ui: &mut Ui,
                layout_job: &mut egui::text::LayoutJob,
                view_ctx: &mut ViewCtx,
            ) {
                use sept::st::Stringifiable;
                layout_job_append(
                    layout_job,
                    self.stringify().as_str(),
                    view_ctx.color_for::<$ty>(),
                    view_ctx,
                );
                render_type_annotation_for(self, layout_job, view_ctx, None);
            }
        }
    };
}

impl_view_using_to_string!(sept::st::Void);
impl_view_using_to_string!(sept::st::True);
impl_view_using_to_string!(sept::st::False);
impl_view_using_to_string!(sept::st::BoolTerm);
impl_view_using_to_string!(sept::st::Sint8Term);
impl_view_using_to_string!(sept::st::Sint16Term);
impl_view_using_to_string!(sept::st::Sint32Term);
impl_view_using_to_string!(sept::st::Sint64Term);
impl_view_using_to_string!(sept::st::Uint8Term);
impl_view_using_to_string!(sept::st::Uint16Term);
impl_view_using_to_string!(sept::st::Uint32Term);
impl_view_using_to_string!(sept::st::Uint64Term);
impl_view_using_to_string!(sept::st::Float32Term);
impl_view_using_to_string!(sept::st::Float64Term);
impl_view_using_to_string!(sept::st::VoidType);
impl_view_using_to_string!(sept::st::EmptyType);
impl_view_using_to_string!(sept::st::TrueType);
impl_view_using_to_string!(sept::st::FalseType);
impl_view_using_to_string!(sept::st::Bool);
impl_view_using_to_string!(sept::st::Sint8);
impl_view_using_to_string!(sept::st::Sint16);
impl_view_using_to_string!(sept::st::Sint32);
impl_view_using_to_string!(sept::st::Sint64);
impl_view_using_to_string!(sept::st::Uint8);
impl_view_using_to_string!(sept::st::Uint16);
impl_view_using_to_string!(sept::st::Uint32);
impl_view_using_to_string!(sept::st::Uint64);
impl_view_using_to_string!(sept::st::Float32);
impl_view_using_to_string!(sept::st::Float64);
impl_view_using_to_string!(sept::st::Utf8String);
impl_view_using_to_string!(sept::st::Array);
impl_view_using_to_string!(sept::st::OrderedMap);
impl_view_using_to_string!(sept::st::Struct);
impl_view_using_to_string!(sept::st::Tuple);
impl_view_using_to_string!(sept::st::GlobalSymRef);
impl_view_using_to_string!(sept::st::LocalSymRef);
impl_view_using_to_string!(sept::st::BoolType);
impl_view_using_to_string!(sept::st::Sint8Type);
impl_view_using_to_string!(sept::st::Sint16Type);
impl_view_using_to_string!(sept::st::Sint32Type);
impl_view_using_to_string!(sept::st::Sint64Type);
impl_view_using_to_string!(sept::st::Uint8Type);
impl_view_using_to_string!(sept::st::Uint16Type);
impl_view_using_to_string!(sept::st::Uint32Type);
impl_view_using_to_string!(sept::st::Uint64Type);
impl_view_using_to_string!(sept::st::Float32Type);
impl_view_using_to_string!(sept::st::Float64Type);
impl_view_using_to_string!(sept::st::Utf8StringType);
impl_view_using_to_string!(sept::st::ArrayType);
impl_view_using_to_string!(sept::st::OrderedMapType);
impl_view_using_to_string!(sept::st::StructType);
impl_view_using_to_string!(sept::st::TupleType);
impl_view_using_to_string!(sept::st::GlobalSymRefType);
impl_view_using_to_string!(sept::st::LocalSymRefType);

fn render_str_as_literal_without_quotes(
    text: &str,
    layout_job: &mut LayoutJob,
    view_ctx: &ViewCtx,
    regular_char_color: egui::Color32,
    escape_char_color: egui::Color32,
) {
    let mut buffer = String::new();
    for c in text.chars() {
        // This is a bit inelegant, but fine for now.
        if c == '\\' || c == '\"' || (c as u32) < (' ' as u32) || (c as u32) > ('~' as u32) {
            // Output the existing buffer, if any contents.
            if !buffer.is_empty() {
                layout_job_append(layout_job, buffer.as_str(), regular_char_color, view_ctx);
                buffer.clear();
            }
            // Output the escape char.
            layout_job_append(
                layout_job,
                c.escape_default().to_string().as_str(),
                escape_char_color,
                view_ctx,
            );
        } else {
            buffer.push(c);
        }
    }
    // Output the existing buffer, if any contents.
    if !buffer.is_empty() {
        layout_job_append(layout_job, buffer.as_str(), regular_char_color, view_ctx);
        buffer.clear();
    }
}

impl View for sept::st::Utf8StringTerm {
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        if self.is_empty() {
            layout_job_append(
                &mut layout_job,
                "\"\"",
                view_ctx.color_for_utf8string_quotes(),
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

        layout_job_append(
            &mut layout_job,
            "\"",
            view_ctx.color_for_utf8string_quotes(),
            view_ctx,
        );
        ui.label(layout_job);

        {
            let regular_char_color = view_ctx.color_for::<Self>();
            let escape_char_color = view_ctx.color_for_utf8string_escape_chars();

            let mut view_ctx_g = view_ctx.push_nesting_depth();

            // TODO: Figure out if lines should be addressable (maybe that requires a "line view" object.)
            // TODO: Probably need to have render_str_as_literal_without_quotes handle the cursor
            for line in self.split_inclusive('\n') {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    // The content itself expects to be in a vertical.
                    ui.vertical(|ui| {
                        let mut layout_job = LayoutJob::default();
                        render_str_as_literal_without_quotes(
                            line,
                            &mut layout_job,
                            &mut view_ctx_g,
                            regular_char_color,
                            escape_char_color,
                        );
                        ui.label(layout_job);
                    });
                });
            }
        }

        let mut layout_job = LayoutJob::default();
        layout_job_append(
            &mut layout_job,
            "\"",
            view_ctx.color_for_utf8string_quotes(),
            view_ctx,
        );
        render_type_annotation_for(
            self,
            &mut layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
        // Return this to the outer context.
        layout_job
    }
    fn update_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        layout_job_append(
            layout_job,
            "\"",
            view_ctx.color_for_utf8string_quotes(),
            view_ctx,
        );
        render_str_as_literal_without_quotes(
            self,
            layout_job,
            view_ctx,
            view_ctx.color_for::<Self>(),
            view_ctx.color_for_utf8string_escape_chars(),
        );
        layout_job_append(
            layout_job,
            "\"",
            view_ctx.color_for_utf8string_quotes(),
            view_ctx,
        );
        render_type_annotation_for(
            self,
            layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
    }
}

impl View for sept::dy::GlobalSymRefTerm {
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        self.update_inline(ui, &mut layout_job, view_ctx);
        layout_job
    }
    fn update_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        let global_symbol_table_g = sept::dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
        let (resolved, path, at_color, quote_color, regular_char_color, escape_char_color) =
            match global_symbol_table_g.resolved_symbol_path(self.symbol_id.as_str()) {
                Ok(resolved_symbol_path) => (
                    true,
                    resolved_symbol_path,
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_global_sym_ref_quotes(),
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_global_sym_ref_escape_chars(),
                ),
                Err(_) => (
                    false,
                    global_symbol_table_g
                        .unresolved_symbol_path(self.symbol_id.as_str())
                        .expect("temp hack"),
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_global_sym_ref_quotes(),
                    ANSIColor::BRIGHT_RED,
                    ANSIColor::DARK_RED,
                ),
            };

        layout_job_append(layout_job, "@", at_color, view_ctx);
        layout_job_append(layout_job, "\"", quote_color, view_ctx);
        render_str_as_literal_without_quotes(
            path.as_str(),
            layout_job,
            view_ctx,
            regular_char_color,
            escape_char_color,
        );
        layout_job_append(layout_job, "\"", quote_color, view_ctx);
        if resolved {
            render_type_annotation_for(self, layout_job, view_ctx, None);
        }
    }
}

impl View for sept::dy::LocalSymRefTerm {
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        self.update_inline(ui, &mut layout_job, view_ctx);
        layout_job
    }
    fn update_inline(&self, _ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        let local_symbol_table_g = self.local_symbol_table().read().unwrap();
        let (resolved, path, dollar_color, quote_color, regular_char_color, escape_char_color) =
            match local_symbol_table_g.resolved_symbol_path(self.symbol_id.as_str()) {
                Ok(resolved_symbol_path) => (
                    true,
                    resolved_symbol_path,
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_local_sym_ref_quotes(),
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_local_sym_ref_escape_chars(),
                ),
                Err(_) => (
                    false,
                    local_symbol_table_g
                        .unresolved_symbol_path(self.symbol_id.as_str())
                        .expect("temp hack"),
                    view_ctx.color_for::<Self>(),
                    view_ctx.color_for_local_sym_ref_quotes(),
                    ANSIColor::BRIGHT_RED,
                    ANSIColor::DARK_RED,
                ),
            };

        layout_job_append(layout_job, "$", dollar_color, view_ctx);
        layout_job_append(layout_job, "\"", quote_color, view_ctx);
        render_str_as_literal_without_quotes(
            path.as_str(),
            layout_job,
            view_ctx,
            regular_char_color,
            escape_char_color,
        );
        layout_job_append(layout_job, "\"", quote_color, view_ctx);
        if resolved {
            render_type_annotation_for(self, layout_job, view_ctx, None);
        }
    }
}

impl View for sept::dy::ArrayTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
        use egui::{Key, Modifiers};
        let self_len = self.len() as u32;

        let mut input_g = ui.input_mut();
        if view_ctx.render_address_is_cursor_address() {
            if input_g.consume_key(Modifiers::NONE, Key::Enter) {
                // Enter this ArrayTerm at element 0.
                view_ctx.cursor_address_push(0u32.into());
            }
        } else if view_ctx.render_address_is_parent_of_cursor_address() {
            if input_g.consume_key(Modifiers::ALT, Key::Enter)
                || input_g.consume_key(Modifiers::NONE, Key::Escape)
            {
                // Escape back to this ArrayTerm.
                view_ctx.cursor_address_pop();
            } else if input_g.consume_key(Modifiers::NONE, Key::Home) {
                view_ctx.cursor_address_pop();
                view_ctx.cursor_address_push(0u32.into());
                // TODO: use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::End) {
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
                            element_index_delta += view_ctx.page_up_down_delta as i32;
                        }
                    }
                    LayoutMode::Inline => {
                        // In this case, elements are horizontally, so arrow left/right should increase/decrease the element index.
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowLeft) {
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
                        "Invalid address token {} under ArrayTerm with address {}",
                        element_index_value,
                        view_ctx.render_address
                    );
                }
            }
        }
    }
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        if self.is_empty() {
            layout_job_append(
                &mut layout_job,
                "[]",
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

        layout_job_append(&mut layout_job, "[", view_ctx.color_for::<Self>(), view_ctx);
        ui.label(layout_job);

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for (i, element) in self.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        let mut view_ctx_g =
                            view_ctx_g.push_address_token(sept::dy::Value::from(i as u32));
                        let mut layout_job = element.update(ui, &mut view_ctx_g, None);
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
        layout_job_append(&mut layout_job, "]", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(
            self,
            &mut layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
        // Return this to the outer context.
        layout_job
    }
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        self.handle_events(ui, view_ctx);

        if self.is_empty() {
            layout_job_append(layout_job, "[]", view_ctx.color_for::<Self>(), view_ctx);
            render_type_annotation_for(
                self,
                layout_job,
                view_ctx,
                Some(format!(" (len: {})", self.len()).as_str()),
            );
            return;
        }

        layout_job_append(layout_job, "[ ", view_ctx.color_for::<Self>(), view_ctx);
        for (i, element) in self.iter().enumerate() {
            {
                let mut view_ctx_g = view_ctx.push_address_token(sept::dy::Value::from(i as u32));
                element.update_inline(ui, layout_job, &mut view_ctx_g);
                layout_job_append(layout_job, ",", view_ctx_g.color_for::<Self>(), &view_ctx_g);
            }
            // Have to handle the space separately so it doesn't get highlighted with the item, if the outer
            // data is not highlighted.
            layout_job_append(layout_job, " ", view_ctx.color_for::<Self>(), view_ctx);
        }
        layout_job_append(layout_job, "]", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(
            self,
            layout_job,
            view_ctx,
            Some(format!(" (len: {})", self.len()).as_str()),
        );
    }
}

/// This one is for OrderedMapTerm key-value pairs.
impl View for (&sept::dy::Value, &sept::dy::Value) {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
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
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        // TODO: Implement addressing of key vs value
        let mut layout_job = {
            let mut view_ctx_g = view_ctx.push_address_token(0u32.into());
            self.0
                .update_expanded(ui, &mut view_ctx_g, continuation_layout_job_o)
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
            let mut view_ctx_g = view_ctx.push_address_token(1u32.into());
            self.1
                .update_expanded(ui, &mut view_ctx_g, Some(layout_job))
        };
        // Return this to the outer context
        layout_job
    }
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        self.handle_events(ui, view_ctx);

        {
            let mut view_ctx_g = view_ctx.push_address_token(0u32.into());
            self.0.update_inline(ui, layout_job, &mut view_ctx_g);
        }
        layout_job_append(
            layout_job,
            " => ",
            view_ctx.color_for::<sept::dy::OrderedMapTerm>(),
            view_ctx,
        );
        {
            let mut view_ctx_g = view_ctx.push_address_token(1u32.into());
            self.1.update_inline(ui, layout_job, &mut view_ctx_g);
        }
    }
}

impl View for sept::dy::OrderedMapTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
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
                            let range = self.range(..=&key);
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
                            let mut range = self.range(&key..);
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
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
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
                            view_ctx_g.push_address_token(key_value_pair.0.clone());
                        let mut layout_job = key_value_pair.update(ui, &mut view_ctx_g, None);
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
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
                let mut view_ctx_g = view_ctx.push_address_token(key_value_pair.0.clone());
                key_value_pair.update_inline(ui, layout_job, &mut view_ctx_g);
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

impl View for sept::dy::TupleTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
        use egui::{Key, Modifiers};
        let self_len = self.len() as u32;

        let mut input_g = ui.input_mut();
        if view_ctx.render_address_is_cursor_address() {
            if input_g.consume_key(Modifiers::NONE, Key::Enter) {
                // Enter this TupleTerm at element 0.
                view_ctx.cursor_address_push(0u32.into());
            }
        } else if view_ctx.render_address_is_parent_of_cursor_address() {
            if input_g.consume_key(Modifiers::ALT, Key::Enter)
                || input_g.consume_key(Modifiers::NONE, Key::Escape)
            {
                // Escape back to this TupleTerm.
                view_ctx.cursor_address_pop();
            } else if input_g.consume_key(Modifiers::NONE, Key::Home) {
                view_ctx.cursor_address_pop();
                view_ctx.cursor_address_push(0u32.into());
                // TODO: use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::End) {
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
                            element_index_delta += view_ctx.page_up_down_delta as i32;
                        }
                    }
                    LayoutMode::Inline => {
                        // In this case, elements are horizontally, so arrow left/right should increase/decrease the element index.
                        if input_g.consume_key(Modifiers::NONE, Key::ArrowLeft) {
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
    }
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
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
                            view_ctx_g.push_address_token(sept::dy::Value::from(i as u32));
                        let mut layout_job = element.update(ui, &mut view_ctx_g, None);
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
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
                let mut view_ctx_g = view_ctx.push_address_token(sept::dy::Value::from(i as u32));
                element.update_inline(ui, layout_job, &mut view_ctx_g);
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

impl View for (String, sept::dy::Value) {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
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
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        let (field_name, field_type) = self;

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        // TODO: Need to figure out how to address field name vs field type
        let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
        {
            let mut view_ctx_g = view_ctx_g.push_address_token(0u32.into());
            // There's probably never a reason to render a field_name expanded.
            field_name.update_inline(ui, &mut layout_job, &mut view_ctx_g);
        }
        layout_job_append(
            &mut layout_job,
            ": ",
            view_ctx_g.color_for::<sept::dy::StructTerm>(),
            &mut view_ctx_g,
        );
        // We pass in layout_job as continuation_layout_job_o so that it renders starting on the same
        // line as ": ".
        let layout_job = {
            let mut view_ctx_g = view_ctx_g.push_address_token(1u32.into());
            field_type.update_expanded(ui, &mut view_ctx_g, Some(layout_job))
        };
        // Return this to the outer context
        layout_job
    }
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        self.handle_events(ui, view_ctx);

        let (field_name, field_type) = self;

        let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
        {
            let mut view_ctx_g = view_ctx_g.push_address_token(0u32.into());
            field_name.update_inline(ui, layout_job, &mut view_ctx_g);
        }
        layout_job_append(
            layout_job,
            ": ",
            view_ctx_g.color_for::<sept::dy::StructTerm>(),
            &mut view_ctx_g,
        );
        {
            let mut view_ctx_g = view_ctx_g.push_address_token(1u32.into());
            field_type.update_inline(ui, layout_job, &mut view_ctx_g);
        }
    }
}

impl View for sept::dy::StructTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
        use egui::{Key, Modifiers};
        let self_len = self.field_decl_v.len() as u32;

        let mut input_g = ui.input_mut();
        if view_ctx.render_address_is_cursor_address() {
            if input_g.consume_key(Modifiers::NONE, Key::Enter) {
                // Enter this StructTerm at the first field, but only if there is one.
                if !self.field_decl_v.is_empty() {
                    view_ctx.cursor_address_push(self.field_decl_v[0].0.clone().into());
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
                if !self.field_decl_v.is_empty() {
                    view_ctx.cursor_address_push(self.field_decl_v[0].0.clone().into());
                } else {
                    // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                }
                // TODO: use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::End) {
                view_ctx.cursor_address_pop();
                if !self.field_decl_v.is_empty() {
                    view_ctx.cursor_address_push(
                        self.field_decl_v[self.field_decl_v.len() - 1]
                            .0
                            .clone()
                            .into(),
                    );
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
                    let field_name_value = view_ctx.cursor_address_pop();
                    if field_name_value.is::<String>() {
                        let field_name = field_name_value.downcast_into::<String>();
                        match self.index_of_named_field(&field_name) {
                            Ok(element_index) => {
                                let mut element_index = element_index as u32;
                                // TODO: Handle one-past-the-end index for insertions
                                element_index = element_index
                                    .saturating_add_signed(element_index_delta)
                                    .min(self_len - 1);
                                view_ctx.cursor_address_push(
                                    self.field_decl_v[element_index as usize].0.clone().into(),
                                );
                                // TODO: use ui.scroll_to_me
                            }
                            Err(e) => {
                                tracing::warn!("In StructTerm::handle_events: {}", e);
                            }
                        }
                    } else {
                        tracing::warn!("In StructTerm::handle_events: Invalid cursor address token {}; expected Utf8String", field_name_value);
                    }
                }
            }
        }
    }
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        if self.field_decl_v.is_empty() {
            {
                let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
                sept::st::Struct.update_inline(ui, &mut layout_job, &mut view_ctx_g);
            }
            layout_job_append(
                &mut layout_job,
                " {}",
                view_ctx.color_for::<Self>(),
                view_ctx,
            );
            render_type_annotation_for(
                self,
                &mut layout_job,
                view_ctx,
                Some(format!(" (len: {})", self.field_decl_v.len()).as_str()),
            );
            return layout_job;
        }

        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            sept::st::Struct.update_inline(ui, &mut layout_job, &mut view_ctx_g);
        }
        layout_job_append(
            &mut layout_job,
            " {",
            view_ctx.color_for::<Self>(),
            view_ctx,
        );
        ui.label(layout_job);

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for element in self.field_decl_v.iter() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        // TODO: Is it possible to push a reference to the address token here?
                        let mut view_ctx_g =
                            view_ctx_g.push_address_token(sept::dy::Value::from(element.0.clone()));
                        let mut layout_job = element.update(ui, &mut view_ctx_g, None);
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
            Some(format!(" (len: {})", self.field_decl_v.len()).as_str()),
        );
        // Return this to the outer context.
        layout_job
    }
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        self.handle_events(ui, view_ctx);

        if self.field_decl_v.is_empty() {
            {
                let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
                sept::st::Struct.update_inline(ui, layout_job, &mut view_ctx_g);
            }
            layout_job_append(layout_job, " {}", view_ctx.color_for::<Self>(), view_ctx);
            render_type_annotation_for(self, layout_job, view_ctx, None);
            // Return this to the outer context.
            return;
        }

        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            sept::st::Struct.update_inline(ui, layout_job, &mut view_ctx_g);
        }
        layout_job_append(layout_job, " { ", view_ctx.color_for::<Self>(), view_ctx);
        for element in self.field_decl_v.iter() {
            {
                // TODO: Is it possible to push a reference to the address token here?
                let mut view_ctx_g =
                    view_ctx.push_address_token(sept::dy::Value::from(element.0.clone()));
                element.update_inline(ui, layout_job, &mut view_ctx_g);
                layout_job_append(layout_job, ",", view_ctx_g.color_for::<Self>(), &view_ctx_g);
            }
            // Have to handle the space separately so it doesn't get highlighted with the item, if the outer
            // data is not highlighted.
            layout_job_append(layout_job, " ", view_ctx.color_for::<Self>(), view_ctx);
        }
        layout_job_append(layout_job, "}", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(self, layout_job, view_ctx, None);
    }
}

impl View for sept::dy::StructTermTerm {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx) {
        use egui::{Key, Modifiers};
        let self_len = self.field_tuple().len() as u32;

        // Here is where we resolve the StructTermTerm's r#type into a StructTerm.
        let dereferenced = self.declared_type().dereferenced().unwrap();
        let dereferenced_g = dereferenced.read();
        let direct_type = dereferenced_g
            .downcast_ref::<sept::dy::StructTerm>()
            .expect("StructTermTerm's r#type field did not dereference into StructTerm");
        assert_eq!(direct_type.field_decl_v.len(), self.field_tuple().len());

        let mut input_g = ui.input_mut();
        if view_ctx.render_address_is_cursor_address() {
            if input_g.consume_key(Modifiers::NONE, Key::Enter) {
                // Enter this StructTermTerm at the first field name, but only if there is one.
                if let Some(first_field_decl) = direct_type.field_decl_v.first() {
                    view_ctx.cursor_address_push(first_field_decl.0.clone().into());
                } else {
                    // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                }
                // TODO: Use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::T) {
                // Enter this StructTermTerm at the "type".
                // TEMP HACK -- use the string "type" for now, but later probably use a non-parametric term.
                // NOTE: This doesn't work if there's a field called "type" in the StructTerm!
                view_ctx.cursor_address_push("type".to_string().into());
            }
        } else if view_ctx.render_address_is_parent_of_cursor_address() {
            if input_g.consume_key(Modifiers::ALT, Key::Enter)
                || input_g.consume_key(Modifiers::NONE, Key::Escape)
            {
                // Escape back to this StructTermTerm.
                view_ctx.cursor_address_pop();
            } else if input_g.consume_key(Modifiers::NONE, Key::Home) {
                view_ctx.cursor_address_pop();
                if let Some(first_field_decl) = direct_type.field_decl_v.first() {
                    view_ctx.cursor_address_push(first_field_decl.0.clone().into());
                } else {
                    // TODO: Figure out how to enter it with a placeholder cursor to prep for editing
                }
                // TODO: use ui.scroll_to_me
            } else if input_g.consume_key(Modifiers::NONE, Key::End) {
                view_ctx.cursor_address_pop();
                if let Some(last_field_decl) = direct_type.field_decl_v.last() {
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
                            let new_field_name =
                                direct_type.field_decl_v[new_field_index as usize].0.clone();
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
    }
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        self.handle_events(ui, view_ctx);

        let mut layout_job = {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            // TEMP HACK -- use the string "type" for now.  later, probably use a char or a non-parametric term that's even more terse.
            // NOTE: This doesn't work if there's a field called "type" in the StructTerm!
            let mut view_ctx_g = view_ctx_g.push_address_token("type".to_string().into());
            self.declared_type()
                .update_expanded(ui, &mut view_ctx_g, continuation_layout_job_o)
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
                std::iter::zip(direct_type.field_decl_v.iter(), self.field_tuple().iter())
            {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        // TODO: Is it possible to push a reference to the address token here?
                        let mut view_ctx_g = view_ctx_g
                            .push_address_token(sept::dy::Value::from(field_name.clone()));

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
                            field_value.update(ui, &mut view_ctx_g, Some(layout_job));
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
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        self.handle_events(ui, view_ctx);

        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            // TEMP HACK -- use the string "type" for now.  later, probably use a char or a non-parametric term that's even more terse.
            // NOTE: This doesn't work if there's a field called "type" in the StructTerm!
            let mut view_ctx_g = view_ctx_g.push_address_token("type".to_string().into());
            self.declared_type()
                .update_inline(ui, layout_job, &mut view_ctx_g);
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
                std::iter::zip(direct_type.field_decl_v.iter(), self.field_tuple().iter())
            {
                {
                    // TODO: Is it possible to push a reference to the address token here?
                    let mut view_ctx_g =
                        view_ctx_g.push_address_token(sept::dy::Value::from(field_name.clone()));

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
                    //     field_value.update(ui, &mut view_ctx_g, Some(layout_job));
                    field_value.update_inline(ui, layout_job, &mut view_ctx_g);
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

impl View for sept::dy::Value {
    fn update_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_ref::<sept::st::BoolTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::ArrayTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::OrderedMapTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::TupleTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Void>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::VoidType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Bool>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::BoolType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::True>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::TrueType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::False>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::FalseType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::EmptyType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8String>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Array>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::ArrayType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMap>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMapType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTermTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Struct>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::StructType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::Tuple>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::TupleType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::GlobalSymRefTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRef>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRefType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::dy::LocalSymRefTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRef>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRefType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
    fn update_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_ref::<sept::st::BoolTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Term>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::ArrayTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::OrderedMapTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::TupleTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Void>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::VoidType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Bool>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::BoolType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::True>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::TrueType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::False>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::FalseType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::EmptyType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint8Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint16Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint32Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Sint64Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint8Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint16Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint32Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Uint64Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float32Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Float64Type>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8String>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Utf8StringType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Array>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::ArrayType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMap>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::OrderedMapType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTermTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::StructTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Struct>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::StructType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::Tuple>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::TupleType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::GlobalSymRefTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRef>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::GlobalSymRefType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::dy::LocalSymRefTerm>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRef>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else if let Some(term) = self.downcast_ref::<sept::st::LocalSymRefType>() {
            term.update_inline(ui, layout_job, view_ctx);
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
}
