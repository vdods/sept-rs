use crate::{
    indentation_for, layout_job_append, render_type_annotation_for, ValueUIT, ViewCtx,
    END_OF_TRANSMISSION_STR,
};
use egui::{text::LayoutJob, Ui};

impl ValueUIT for sept::dy::ArrayTerm {
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

        // TODO: Consider making this in-line special case configurable as part of the ViewOptions.
        if self.is_empty() {
            self.run_ui_inline(ui, &mut layout_job, view_ctx);
            return layout_job;
        }

        layout_job_append(&mut layout_job, "[", view_ctx.color_for::<Self>(), view_ctx);
        ui.label(layout_job);

        {
            let mut view_ctx_g = view_ctx.push_nesting_depth();

            let elem_oi = self
                .iter()
                .map(|elem| Some(elem))
                .chain(std::iter::once(None));
            for (elem_index, elem_o) in elem_oi.enumerate() {
                ui.horizontal(|ui| {
                    ui.label(indentation_for::<Self>(&mut view_ctx_g));

                    ui.vertical(|ui| {
                        let mut view_ctx_g = view_ctx_g
                            .push_render_address_token(sept::dy::Value::from(elem_index as u32));
                        let mut layout_job = if let Some(elem) = elem_o {
                            elem.run_ui(ui, &mut view_ctx_g, None)
                        } else {
                            let mut layout_job = LayoutJob::default();
                            layout_job_append(
                                &mut layout_job,
                                END_OF_TRANSMISSION_STR,
                                // TODO: Maybe use a special color for this char
                                view_ctx_g.color_for::<Self>(),
                                &mut view_ctx_g,
                            );
                            layout_job
                        };
                        if elem_index + 1 <= self.len() {
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
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>) {
        layout_job_append(layout_job, "[ ", view_ctx.color_for::<Self>(), view_ctx);
        let elem_oi = self
            .iter()
            .map(|elem| Some(elem))
            .chain(std::iter::once(None));
        for (elem_index, elem_o) in elem_oi.enumerate() {
            {
                let mut view_ctx_g =
                    view_ctx.push_render_address_token(sept::dy::Value::from(elem_index as u32));
                if let Some(elem) = elem_o {
                    elem.run_ui_inline(ui, layout_job, &mut view_ctx_g);
                } else {
                    layout_job_append(
                        layout_job,
                        END_OF_TRANSMISSION_STR,
                        // TODO: Maybe use a special color for this char
                        view_ctx_g.color_for::<Self>(),
                        &mut view_ctx_g,
                    );
                }
                if elem_index + 1 <= self.len() {
                    layout_job_append(layout_job, ",", view_ctx_g.color_for::<Self>(), &view_ctx_g);
                }
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
