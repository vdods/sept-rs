use crate::{LayoutDiscriminant, ViewCtx};
use egui::{text::LayoutJob, Ui};
use std::collections::VecDeque;

pub trait ValueUIT {
    fn handle_events(&self, ui: &mut Ui, view_ctx: &mut ViewCtx<'_>) {
        // Do nothing by default.
        let _ = ui;
        let _ = view_ctx;
    }
    /// If continuation_layout_job_o is not None, then it must be used for whatever the first line in
    /// the item rendering is.  If there's only one line in the rendering, then it would also be returned.
    fn run_ui_expanded(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob;
    /// run_ui_inline shouldn't add anything to `ui`, it should add things to `layout_job`.  The `ui`
    /// parameter is only there so run_ui_inline has access to events.
    fn run_ui_inline(&self, ui: &mut Ui, layout_job: &mut LayoutJob, view_ctx: &mut ViewCtx<'_>);
    fn run_ui(
        &self,
        ui: &mut Ui,
        view_ctx: &mut ViewCtx<'_>,
        continuation_layout_job_o: Option<LayoutJob>,
    ) -> LayoutJob {
        match view_ctx.layout_discriminant() {
            LayoutDiscriminant::Expanded => {
                self.run_ui_expanded(ui, view_ctx, continuation_layout_job_o)
            }
            LayoutDiscriminant::BoundaryLevelInline => {
                let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
                self.run_ui_inline(ui, &mut layout_job, view_ctx);
                layout_job
            }
            LayoutDiscriminant::InteriorLevelInline => {
                panic!("programmer error: this should not be called from InteriorLevelInline");
            }
        }
    }
}

pub(crate) fn layout_job_append(
    layout_job: &mut LayoutJob,
    text: &str,
    foreground_color: egui::Color32,
    view_ctx: &ViewCtx<'_>,
) {
    let (foreground_color, background_color, underline_stroke) =
        view_ctx.set_highlight_if_necessary(foreground_color);
    let text_format = egui::TextFormat {
        font_id: view_ctx.font_id.clone(),
        color: foreground_color,
        background: background_color,
        underline: underline_stroke,
        ..Default::default()
    };
    layout_job.append(text, 0.0, text_format);
}

pub(crate) fn indentation_for<T: 'static>(view_ctx: &ViewCtx<'_>) -> LayoutJob {
    let foreground_color = view_ctx.color_for_indentation_for::<T>();
    let (foreground_color, background_color, underline_stroke) =
        view_ctx.set_highlight_if_necessary(foreground_color);
    let text_format = egui::TextFormat {
        font_id: view_ctx.font_id.clone(),
        color: foreground_color,
        background: background_color,
        underline: underline_stroke,
        ..Default::default()
    };
    LayoutJob::single_section(view_ctx.indent_str().to_string(), text_format)
}

pub(crate) fn render_type_annotation_for<T: sept::st::TermT>(
    term: &T,
    layout_job: &mut LayoutJob,
    view_ctx: &mut ViewCtx<'_>,
    extra_text_o: Option<&str>,
) where
    <T as sept::st::TermT>::AbstractTypeType: sept::st::StringifiableT,
{
    if view_ctx.should_show_type_annotations() {
        use sept::st::StringifiableT;
        let extra_text = extra_text_o.unwrap_or("");
        layout_job_append(
            layout_job,
            format!(": {}{}", term.abstract_type().stringify(), extra_text).as_str(),
            view_ctx.color_for_type_annotation(),
            view_ctx,
        );
    }
}

// Bit of a hack because `str` can't impl `sept::st::TermT`.
pub(crate) fn render_type_annotation_for_str(
    layout_job: &mut LayoutJob,
    view_ctx: &mut ViewCtx<'_>,
    extra_text_o: Option<&str>,
) {
    if view_ctx.should_show_type_annotations() {
        use sept::st::StringifiableT;
        let extra_text = extra_text_o.unwrap_or("");
        layout_job_append(
            layout_job,
            format!(": {}{}", sept::st::Utf8String.stringify(), extra_text).as_str(),
            view_ctx.color_for_type_annotation(),
            view_ctx,
        );
    }
}

pub(crate) fn render_postfix_annotation(
    layout_job: &mut LayoutJob,
    view_ctx: &mut ViewCtx<'_>,
    postfix_text: &str,
) {
    // TODO: Use a different ViewCtx config var
    if view_ctx.should_show_type_annotations() {
        layout_job_append(
            layout_job,
            postfix_text,
            view_ctx.color_for_type_annotation(),
            view_ctx,
        );
    }
}

// This is probably a TEMP HACK
macro_rules! impl_value_ui_using_to_string {
    ($ty:ty) => {
        impl ValueUIT for $ty {
            fn run_ui_expanded(
                &self,
                _ui: &mut egui::Ui,
                view_ctx: &mut ViewCtx<'_>,
                continuation_layout_job_o: Option<LayoutJob>,
            ) -> egui::text::LayoutJob {
                let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
                use sept::st::StringifiableT;
                layout_job_append(
                    &mut layout_job,
                    self.stringify().as_str(),
                    view_ctx.color_for::<$ty>(),
                    view_ctx,
                );
                render_type_annotation_for(self, &mut layout_job, view_ctx, None);
                layout_job
            }
            fn run_ui_inline(
                &self,
                _ui: &mut Ui,
                layout_job: &mut egui::text::LayoutJob,
                view_ctx: &mut ViewCtx<'_>,
            ) {
                use sept::st::StringifiableT;
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

// This is probably a TEMP HACK
macro_rules! impl_value_ui_using_debug_format {
    ($ty:ty) => {
        impl ValueUIT for $ty {
            fn run_ui_expanded(
                &self,
                _ui: &mut egui::Ui,
                view_ctx: &mut ViewCtx<'_>,
                continuation_layout_job_o: Option<LayoutJob>,
            ) -> egui::text::LayoutJob {
                let mut layout_job = continuation_layout_job_o.unwrap_or(LayoutJob::default());
                layout_job_append(
                    &mut layout_job,
                    format!("{:?}", self).as_str(),
                    view_ctx.color_for::<$ty>(),
                    view_ctx,
                );
                render_type_annotation_for(self, &mut layout_job, view_ctx, None);
                layout_job
            }
            fn run_ui_inline(
                &self,
                _ui: &mut Ui,
                layout_job: &mut egui::text::LayoutJob,
                view_ctx: &mut ViewCtx<'_>,
            ) {
                layout_job_append(
                    layout_job,
                    format!("{:?}", self).as_str(),
                    view_ctx.color_for::<$ty>(),
                    view_ctx,
                );
                render_type_annotation_for(self, layout_job, view_ctx, None);
            }
        }
    };
}

impl_value_ui_using_to_string!(sept::st::Term);
impl_value_ui_using_to_string!(sept::st::Type);
impl_value_ui_using_to_string!(sept::st::Void);
impl_value_ui_using_to_string!(sept::st::True);
impl_value_ui_using_to_string!(sept::st::False);
impl_value_ui_using_to_string!(sept::st::BoolTerm);
impl_value_ui_using_debug_format!(sept::st::UnicodeCharTerm);
impl_value_ui_using_to_string!(sept::st::Sint8Term);
impl_value_ui_using_to_string!(sept::st::Sint16Term);
impl_value_ui_using_to_string!(sept::st::Sint32Term);
impl_value_ui_using_to_string!(sept::st::Sint64Term);
impl_value_ui_using_to_string!(sept::st::Uint8Term);
impl_value_ui_using_to_string!(sept::st::Uint16Term);
impl_value_ui_using_to_string!(sept::st::Uint32Term);
impl_value_ui_using_to_string!(sept::st::Uint64Term);
impl_value_ui_using_to_string!(sept::st::Float32Term);
impl_value_ui_using_to_string!(sept::st::Float64Term);
impl_value_ui_using_to_string!(sept::st::VoidType);
impl_value_ui_using_to_string!(sept::st::PlaceholderType);
impl_value_ui_using_to_string!(sept::st::EmptyType);
impl_value_ui_using_to_string!(sept::st::TrueType);
impl_value_ui_using_to_string!(sept::st::FalseType);
impl_value_ui_using_to_string!(sept::st::Bool);
impl_value_ui_using_to_string!(sept::st::UnicodeChar);
impl_value_ui_using_to_string!(sept::st::Sint8);
impl_value_ui_using_to_string!(sept::st::Sint16);
impl_value_ui_using_to_string!(sept::st::Sint32);
impl_value_ui_using_to_string!(sept::st::Sint64);
impl_value_ui_using_to_string!(sept::st::Uint8);
impl_value_ui_using_to_string!(sept::st::Uint16);
impl_value_ui_using_to_string!(sept::st::Uint32);
impl_value_ui_using_to_string!(sept::st::Uint64);
impl_value_ui_using_to_string!(sept::st::Float32);
impl_value_ui_using_to_string!(sept::st::Float64);
impl_value_ui_using_to_string!(sept::st::Utf8String);
impl_value_ui_using_to_string!(sept::st::Array);
impl_value_ui_using_to_string!(sept::st::OrderedMap);
impl_value_ui_using_to_string!(sept::st::Struct);
impl_value_ui_using_to_string!(sept::st::Tuple);
impl_value_ui_using_to_string!(sept::st::GlobalSymRef);
impl_value_ui_using_to_string!(sept::st::LocalSymRef);
impl_value_ui_using_to_string!(sept::st::BoolType);
impl_value_ui_using_to_string!(sept::st::UnicodeCharType);
impl_value_ui_using_to_string!(sept::st::Sint8Type);
impl_value_ui_using_to_string!(sept::st::Sint16Type);
impl_value_ui_using_to_string!(sept::st::Sint32Type);
impl_value_ui_using_to_string!(sept::st::Sint64Type);
impl_value_ui_using_to_string!(sept::st::Uint8Type);
impl_value_ui_using_to_string!(sept::st::Uint16Type);
impl_value_ui_using_to_string!(sept::st::Uint32Type);
impl_value_ui_using_to_string!(sept::st::Uint64Type);
impl_value_ui_using_to_string!(sept::st::Float32Type);
impl_value_ui_using_to_string!(sept::st::Float64Type);
impl_value_ui_using_to_string!(sept::st::Utf8StringType);
impl_value_ui_using_to_string!(sept::st::ArrayType);
impl_value_ui_using_to_string!(sept::st::OrderedMapType);
impl_value_ui_using_to_string!(sept::st::StructType);
impl_value_ui_using_to_string!(sept::st::TupleType);
impl_value_ui_using_to_string!(sept::st::GlobalSymRefType);
impl_value_ui_using_to_string!(sept::st::LocalSymRefType);

pub const END_OF_TRANSMISSION_CHAR: char = '¶';
pub const END_OF_TRANSMISSION_STR: &str = "¶";

pub(crate) fn render_str_as_literal_without_quotes(
    text: &str,
    layout_job: &mut LayoutJob,
    view_ctx: &mut ViewCtx<'_>,
    regular_char_color: egui::Color32,
    escape_char_color: egui::Color32,
    char_index_begin: usize,
) {
    for (c_index, c) in text.chars().enumerate() {
        let mut view_ctx_g =
            view_ctx.push_render_address_token(((c_index + char_index_begin) as u32).into());

        // TEMP HACK -- handle char by char for now.  Maybe this is plenty efficient, hopefully LayoutJob does
        // the correct buffering.
        if c == '\\'
            || c == '\"'
            || (c as u32) < (' ' as u32)
            || (c as u32) > ('~' as u32)
            || c == END_OF_TRANSMISSION_CHAR
        {
            layout_job_append(
                layout_job,
                c.escape_default().to_string().as_str(),
                escape_char_color,
                &mut view_ctx_g,
            );
        } else {
            layout_job_append(
                layout_job,
                c.to_string().as_str(),
                regular_char_color,
                &mut view_ctx_g,
            );
        }
    }
}

/// Strip the first char from the String, and if it's not empty, return the remainder.
/// Otherwise return None.
pub fn first_char_stripped_string(mut string: String) -> Option<String> {
    if string.chars().nth(1).is_none() {
        None
    } else {
        string.remove(0);
        Some(string)
    }
}

pub fn extract_text_prefix_from_front_text(
    prefix: &str,
    remaining_event_v: &mut VecDeque<egui::Event>,
) {
    let mut just_pop_front_event = false;
    match remaining_event_v.front_mut() {
        Some(egui::Event::Text(text)) if text.starts_with(prefix) => {
            // Special case for when the prefix is the whole Text content.
            just_pop_front_event = true;
        }
        Some(egui::Event::Text(text)) => {
            // Remove only the prefix from the Text content (i.e. replace it with "")
            text.replace_range(0..prefix.len(), "");
        }
        _ => {
            panic!("programmer error: Expected a Text event starting with the given prefix");
        }
    }
    if just_pop_front_event {
        remaining_event_v.pop_front();
    }
}

/// This operates in-place on the event vector, and returns the text of the egui::Event::Text
/// that was extracted, or None if no Text event was extracted.
#[allow(unused)] // TEMP HACK
fn extract_text_from_events(event_v: &mut Vec<egui::Event>) -> Option<String> {
    let mut text_o: Option<String> = None;
    event_v.retain_mut(|event| {
        if let egui::Event::Text(string) = event {
            // Take the string so we don't alloc.
            let mut s = String::new();
            std::mem::swap(string, &mut s);
            text_o = Some(s);
            // Don't retain this, since we extracted it.
            false
        } else {
            // Retain everything else.
            true
        }
    });
    text_o
}

/// This operates in-place on the event vector, and returns the text of the egui::Event::Paste
/// that was extracted, or None if no Paste event was extracted.
#[allow(unused)] // TEMP HACK
fn extract_paste_from_events(event_v: &mut Vec<egui::Event>) -> Option<String> {
    let mut text_o: Option<String> = None;
    event_v.retain_mut(|event| {
        if let egui::Event::Paste(string) = event {
            // Take the string so we don't alloc.
            let mut s = String::new();
            std::mem::swap(string, &mut s);
            text_o = Some(s);
            // Don't retain this, since we extracted it.
            false
        } else {
            // Retain everything else.
            true
        }
    });
    text_o
}
