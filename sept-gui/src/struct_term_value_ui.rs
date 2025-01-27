use crate::{
    indentation_for, layout_job_append, render_type_annotation_for, ValueUI, ViewCtx,
    END_OF_TRANSMISSION_STR,
};
use egui::{text::LayoutJob, Ui};

impl ValueUI for sept::dy::StructTerm {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("this should be deprecated");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            sept::st::Struct.run_ui_inline(ui, &mut layout_job, &mut view_ctx_g);
        }
        layout_job_append(
            &mut layout_job,
            " {",
            view_ctx.color_for::<Self>(),
            view_ctx,
        );
        ui.label(layout_job);

        {
            // NOTE: This should probably go into StructTermFieldView::run_ui_expanded
            let mut view_ctx_g = view_ctx.push_nesting_depth();
            for field_index in 0..self.len() + 1 {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        // TODO: Is it possible to push a reference to the address token here?
                        let mut view_ctx_g =
                            view_ctx_g.push_render_address_token((field_index as u32).into());
                        // let mut layout_job = if field_index < self.len() {
                        let mut layout_job = if true {
                            sept::qv::StructTermFieldElemView::new(self, field_index)
                                .unwrap()
                                .run_ui(ui, &mut view_ctx_g, None)
                        } else {
                            let mut layout_job = LayoutJob::default();
                            // TODO: Highlighting when the cursor is within this element at all.
                            layout_job_append(
                                &mut layout_job,
                                END_OF_TRANSMISSION_STR,
                                // TODO: Maybe use a special color for this char
                                view_ctx_g.color_for::<Self>(),
                                &mut view_ctx_g,
                            );
                            layout_job
                        };
                        if field_index + 1 <= self.len() {
                            layout_job_append(
                                &mut layout_job,
                                ",",
                                view_ctx_g.color_for::<Self>(),
                                &mut view_ctx_g,
                            );
                        }
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
        {
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            sept::st::Struct.run_ui_inline(ui, layout_job, &mut view_ctx_g);
        }
        layout_job_append(layout_job, " { ", view_ctx.color_for::<Self>(), view_ctx);
        for field_index in 0..self.len() + 1 {
            {
                // TODO: Is it possible to push a reference to the address token here?
                let mut view_ctx_g =
                    view_ctx.push_render_address_token((field_index as u32).into());
                // if field_index < self.len() {
                if true {
                    sept::qv::StructTermFieldElemView::new(self, field_index)
                        .unwrap()
                        .run_ui_inline(ui, layout_job, &mut view_ctx_g);
                } else {
                    layout_job_append(
                        layout_job,
                        END_OF_TRANSMISSION_STR,
                        // TODO: Maybe use a special color for this char
                        view_ctx_g.color_for::<Self>(),
                        &mut view_ctx_g,
                    );
                }
                if field_index + 1 <= self.len() {
                    layout_job_append(layout_job, ",", view_ctx_g.color_for::<Self>(), &view_ctx_g);
                }
            }
            // Have to handle the space separately so it doesn't get highlighted with the item, if the outer
            // data is not highlighted.
            layout_job_append(layout_job, " ", view_ctx.color_for::<Self>(), view_ctx);
        }
        layout_job_append(layout_job, "}", view_ctx.color_for::<Self>(), view_ctx);
        render_type_annotation_for(self, layout_job, view_ctx, None);
    }
}

impl<'a> ValueUI for sept::qv::StructTermFieldElemView<'a> {
    fn handle_events(&self, _ui: &mut Ui, _view_ctx: &mut ViewCtx<'_>) {
        panic!("this should be deprecated");
    }
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        // self.handle_events(ui, view_ctx);

        let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());

        let layout_job = if self.field_index < self.struct_term.len() {
            let field_name = self.struct_term.get_field_name(self.field_index).unwrap();
            let field_type = self.struct_term.get_field_type(self.field_index).unwrap();

            // TODO: Need to figure out how to address field name vs field type
            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            {
                let mut view_ctx_g = view_ctx_g.push_render_address_token(0u32.into());
                // There's probably never a reason to render a field_name expanded.
                field_name.run_ui_inline(ui, &mut layout_job, &mut view_ctx_g);
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
                let mut view_ctx_g = view_ctx_g.push_render_address_token(1u32.into());
                field_type.run_ui_expanded(ui, &mut view_ctx_g, Some(layout_job))
            };

            layout_job
        } else {
            // Render the placeholder field.
            let mut layout_job = LayoutJob::default();
            {
                let mut view_ctx_g = view_ctx.push_render_address_token(0u32.into());
                // TODO: Highlighting when the cursor is within this element at all.
                layout_job_append(
                    &mut layout_job,
                    END_OF_TRANSMISSION_STR,
                    // TODO: Maybe use a special color for this char
                    view_ctx_g.color_for::<sept::dy::StructTerm>(),
                    &mut view_ctx_g,
                );
            }
            layout_job_append(
                &mut layout_job,
                ": ",
                view_ctx.color_for::<sept::dy::StructTerm>(),
                view_ctx,
            );
            {
                let mut view_ctx_g = view_ctx.push_render_address_token(1u32.into());
                // TODO: Highlighting when the cursor is within this element at all.
                layout_job_append(
                    &mut layout_job,
                    END_OF_TRANSMISSION_STR,
                    // TODO: Maybe use a special color for this char
                    view_ctx_g.color_for::<sept::dy::StructTerm>(),
                    &mut view_ctx_g,
                );
            }
            layout_job
        };
        // Return this to the outer context
        layout_job
    }
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        // self.handle_events(ui, view_ctx);

        if self.field_index < self.struct_term.len() {
            let field_name = self.struct_term.get_field_name(self.field_index).unwrap();
            let field_type = self.struct_term.get_field_type(self.field_index).unwrap();

            let mut view_ctx_g = view_ctx.push_show_type_annotations(false);
            {
                let mut view_ctx_g = view_ctx_g.push_render_address_token(0u32.into());
                field_name.run_ui_inline(ui, layout_job, &mut view_ctx_g);
            }
            layout_job_append(
                layout_job,
                ": ",
                view_ctx_g.color_for::<sept::dy::StructTerm>(),
                &mut view_ctx_g,
            );
            {
                let mut view_ctx_g = view_ctx_g.push_render_address_token(1u32.into());
                field_type.run_ui_inline(ui, layout_job, &mut view_ctx_g);
            }
        } else {
            // Render the placeholder field.
            {
                let mut view_ctx_g = view_ctx.push_render_address_token(0u32.into());
                // TODO: Highlighting when the cursor is within this element at all.
                layout_job_append(
                    layout_job,
                    END_OF_TRANSMISSION_STR,
                    // TODO: Maybe use a special color for this char
                    view_ctx_g.color_for::<sept::dy::StructTerm>(),
                    &mut view_ctx_g,
                );
            }
            layout_job_append(
                layout_job,
                ": ",
                view_ctx.color_for::<sept::dy::StructTerm>(),
                view_ctx,
            );
            {
                let mut view_ctx_g = view_ctx.push_render_address_token(1u32.into());
                // TODO: Highlighting when the cursor is within this element at all.
                layout_job_append(
                    layout_job,
                    END_OF_TRANSMISSION_STR,
                    // TODO: Maybe use a special color for this char
                    view_ctx_g.color_for::<sept::dy::StructTerm>(),
                    &mut view_ctx_g,
                );
            }
        }
    }
}
