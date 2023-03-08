use crate::{ANSIColor, LayoutMode, ViewCtx};
use egui::{text::LayoutJob, Ui};

pub trait View {
    /// If continuation_layout_job_o is not None, then it must be used for whatever the first line in
    /// the item rendering is.  If there's only one line in the rendering, then it would also be returned.
    fn update_expanded(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob;
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx);
    fn update(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        match view_ctx.layout_mode() {
            LayoutMode::Expanded => self.update_expanded(ui, view_ctx, continuation_layout_job_o),
            LayoutMode::BoundaryLevelInline => {
                let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
                self.update_inline(&mut layout_job, view_ctx);
                layout_job
            }
            LayoutMode::InteriorLevelInline => {
                panic!("programmer error: this should not be called from InteriorLevelInline");
            }
        }
    }
}

fn layout_job_append(
    layout_job: &mut LayoutJob,
    text: &str,
    color: egui::Color32,
    view_ctx: &ViewCtx,
) {
    layout_job.append(
        text,
        0.0,
        egui::TextFormat::simple(view_ctx.font_id.clone(), color),
    );
}

fn indentation_for<T: 'static>(view_ctx: &ViewCtx) -> LayoutJob {
    LayoutJob::simple_singleline(
        view_ctx.indent_str().to_string(),
        view_ctx.font_id.clone(),
        view_ctx.color_for_indentation_for::<T>(),
    )
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
                &mut self,
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
                &mut self,
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
        &mut self,
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
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
        &mut self,
        _ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        self.update_inline(&mut layout_job, view_ctx);
        layout_job
    }
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
        &mut self,
        _ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
        self.update_inline(&mut layout_job, view_ctx);
        layout_job
    }
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
    fn update_expanded(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
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
            for element in self.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
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
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
        for element in self.iter_mut() {
            element.update_inline(layout_job, view_ctx);
            layout_job_append(layout_job, ", ", view_ctx.color_for::<Self>(), view_ctx);
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

impl View for sept::dy::TupleTerm {
    fn update_expanded(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
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
            for element in self.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
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
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
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
        for element in self.iter_mut() {
            element.update_inline(layout_job, view_ctx);
            layout_job_append(layout_job, ", ", view_ctx.color_for::<Self>(), view_ctx);
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
    fn update_expanded(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let (field_name, field_type) = self;

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
        // There's probably never a reason to render a field_name expanded.
        field_name.update_inline(&mut layout_job, &mut view_ctx_g);
        layout_job_append(
            &mut layout_job,
            ": ",
            view_ctx_g.color_for::<sept::dy::StructTerm>(),
            &mut view_ctx_g,
        );
        // We pass in layout_job as continuation_layout_job_o so that it renders starting on the same
        // line as ": ".
        let layout_job = field_type.update_expanded(ui, &mut view_ctx_g, Some(layout_job));
        // Return this to the outer context
        layout_job
    }
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        let (field_name, field_type) = self;

        let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
        field_name.update_inline(layout_job, &mut view_ctx_g);
        layout_job_append(
            layout_job,
            ": ",
            view_ctx_g.color_for::<sept::dy::StructTerm>(),
            &mut view_ctx_g,
        );
        field_type.update_inline(layout_job, &mut view_ctx_g);
    }
}

impl View for sept::dy::StructTerm {
    fn update_expanded(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        if self.field_decl_v.is_empty() {
            {
                let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
                sept::st::Struct.update_inline(&mut layout_job, &mut view_ctx_g);
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
            sept::st::Struct.update_inline(&mut layout_job, &mut view_ctx_g);
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
            for element in self.field_decl_v.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
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
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        if self.field_decl_v.is_empty() {
            {
                let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
                sept::st::Struct.update_inline(layout_job, &mut view_ctx_g);
            }
            layout_job_append(layout_job, " {}", view_ctx.color_for::<Self>(), view_ctx);
            render_type_annotation_for(self, layout_job, view_ctx, None);
            // Return this to the outer context.
            return;
        }

        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            sept::st::Struct.update_inline(layout_job, &mut view_ctx_g);
        }
        layout_job_append(layout_job, " { ", view_ctx.color_for::<Self>(), view_ctx);
        for element in self.field_decl_v.iter_mut() {
            element.update_inline(layout_job, view_ctx);
            layout_job_append(layout_job, ", ", view_ctx.color_for::<Self>(), view_ctx);
        }
        layout_job_append(layout_job, "}", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(self, layout_job, view_ctx, None);
    }
}

impl View for sept::dy::StructTermTerm {
    fn update_expanded(
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            self.declared_type_mut()
                .update_expanded(ui, &mut view_ctx_g, continuation_layout_job_o)
        };
        // TODO: Maybe there should be some syntax for "construction"
        layout_job_append(
            &mut layout_job,
            " {",
            view_ctx.color_for::<Self>(),
            view_ctx,
        );
        ui.label(layout_job);

        // TODO: Figure out how to guarantee correct resolution of self.r#type into StructTerm.
        // TEMP HACK -- this is very wasteful, but just clone it for now to get the job done, and
        // sort out the efficient way later.  This awkwardness is needed partly because the "update"
        // methods expect `&mut self`, and can't use `&self` to simply render.
        let mut direct_type = match self.declared_type().dereferenced().unwrap() {
            sept::dy::MaybeDereferencedValue::NonRef(value_guts) => value_guts
                .downcast_ref::<sept::dy::StructTerm>()
                .expect("StructTermTerm's r#type field did not dereference into StructTerm")
                .clone(),
            sept::dy::MaybeDereferencedValue::Ref(value_la) => value_la
                .read()
                .unwrap()
                .downcast_ref::<sept::dy::StructTerm>()
                .expect("StructTermTerm's r#type field did not dereference into StructTerm")
                .clone(),
        };

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for ((field_name, _field_type), field_value) in std::iter::zip(
                direct_type.field_decl_v.iter_mut(),
                self.field_tuple_mut().iter_mut(),
            ) {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
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
        // TODO: Figure out if this should be conditional somehow.
        render_type_annotation_for(self, &mut layout_job, view_ctx, None);
        // Return this to the outer context
        layout_job
    }
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            self.declared_type_mut()
                .update_inline(layout_job, &mut view_ctx_g);
        }
        // TODO: It's a space for now, but maybe there should be some syntax for "construction"
        layout_job_append(layout_job, " { ", view_ctx.color_for::<Self>(), view_ctx);

        // TODO: Figure out how to guarantee correct resolution of self.r#type into StructTerm.
        // TEMP HACK -- this is very wasteful, but just clone it for now to get the job done, and
        // sort out the efficient way later.  This awkwardness is needed partly because the "update"
        // methods expect `&mut self`, and can't use `&self` to simply render.
        let mut direct_type = match self.declared_type().dereferenced().unwrap() {
            sept::dy::MaybeDereferencedValue::NonRef(value_guts) => value_guts
                .downcast_ref::<sept::dy::StructTerm>()
                .expect("StructTermTerm's r#type field did not dereference into StructTerm")
                .clone(),
            sept::dy::MaybeDereferencedValue::Ref(value_la) => value_la
                .read()
                .unwrap()
                .downcast_ref::<sept::dy::StructTerm>()
                .expect("StructTermTerm's r#type field did not dereference into StructTerm")
                .clone(),
        };

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for ((field_name, _field_type), field_value) in std::iter::zip(
                direct_type.field_decl_v.iter_mut(),
                self.field_tuple_mut().iter_mut(),
            ) {
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
                field_value.update_inline(layout_job, &mut view_ctx_g);
                layout_job_append(
                    layout_job,
                    ", ",
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
        &mut self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_mut::<sept::st::BoolTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64Term>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8StringTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::dy::ArrayTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::dy::TupleTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Void>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::VoidType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Bool>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::BoolType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::True>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::TrueType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::False>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::FalseType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::EmptyType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64Type>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8String>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8StringType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Array>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::ArrayType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::dy::StructTermTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::dy::StructTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Struct>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::StructType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::Tuple>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::TupleType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::dy::GlobalSymRefTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::GlobalSymRef>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::GlobalSymRefType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::dy::LocalSymRefTerm>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::LocalSymRef>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else if let Some(term) = self.downcast_mut::<sept::st::LocalSymRefType>() {
            term.update_expanded(ui, view_ctx, continuation_layout_job_o)
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
    fn update_inline(&mut self, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx) {
        // TODO: figure out best way to efficiently get the View trait out of here,
        // ideally without having to add it to the sept runtime.
        if let Some(term) = self.downcast_mut::<sept::st::BoolTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64Term>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8StringTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::ArrayTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::TupleTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Void>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::VoidType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Bool>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::BoolType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::True>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::TrueType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::False>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::FalseType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::EmptyType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint8Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint16Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint32Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Sint64Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint8Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint16Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint32Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Uint64Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float32Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Float64Type>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8String>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Utf8StringType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Array>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::ArrayType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::StructTermTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::StructTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Struct>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::StructType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::Tuple>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::TupleType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::GlobalSymRefTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::GlobalSymRef>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::GlobalSymRefType>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::dy::LocalSymRefTerm>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::LocalSymRef>() {
            term.update_inline(layout_job, view_ctx);
        } else if let Some(term) = self.downcast_mut::<sept::st::LocalSymRefType>() {
            term.update_inline(layout_job, view_ctx);
        } else {
            use sept::st::Stringifiable;
            tracing::error!("View not implemented for {}", self.stringify());
            unimplemented!("not yet");
        }
    }
}
