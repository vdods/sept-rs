use crate::{is_mouse_event, AddressedEdit, Edit, EventHandlerCtx, EventHandlerT, RootValueEdit};
use anyhow::Result;

impl EventHandlerT for sept::st::Placeholder {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(_cursor_address_token) = cursor_address_token_i.next() {
            panic!("programmer error: Placeholder does not support query (yet).");
        } else {
            placeholder_event_handler_impl(
                event,
                event_handler_ctx,
                |address: sept::dy::TupleTerm, new_data: sept::dy::Value| -> Edit {
                    RootValueEdit::from(AddressedEdit {
                        address,
                        edit: sept::qv::ReplacementTerm {
                            old_data: sept::st::Placeholder.into(),
                            new_data,
                        }
                        .into(),
                    })
                    .into()
                },
            )
        }
    }
}

/// Slightly more generalized version of Placeholder::event_handler which can handle inserting
/// items at the non-element at the end of an array or other container.  edit_factory takes
/// the address and new_data and should return the command to be executed if the Placeholder
/// is being replaced by the new_data.
pub fn placeholder_event_handler_impl(
    event: egui::Event,
    event_handler_ctx: &mut EventHandlerCtx<'_>,
    edit_factory: impl Fn(sept::dy::TupleTerm, sept::dy::Value) -> Edit,
) -> Result<Option<egui::Event>> {
    // This is the value addressed by the cursor.
    if !is_mouse_event(&event) {
        tracing::trace!(
            "placeholder_event_handler_impl; event: {:?}, cursor: {:?}",
            event,
            event_handler_ctx.cursor_address
        );
    }
    match event {
        egui::Event::Text(string)
            if string.starts_with("\"")
                || string.starts_with("[")
                || string.starts_with("'")
                || string.starts_with("(") =>
        {
            let address = event_handler_ctx.cursor_address.clone();
            let first_char = string.chars().next().unwrap();
            // We leave the first char on the event string, so that it gets processed by the newly-inserted value.
            let new_data: sept::dy::Value = match first_char {
                '"' => "".to_string().into(),
                '[' => sept::dy::ArrayTerm::from(vec![]).into(),
                '\'' => ' '.into(),
                '(' => sept::dy::TupleTerm::from(vec![]).into(),
                _ => unreachable!("programmer error: you missed a case!"),
            };
            let edit = edit_factory(address, new_data);
            event_handler_ctx.enqueue_edit(edit);
            // Push the string back onto the front of the event queue, so that the newly-inserted
            // value can process it.  Generally, the way that it will process it is to make the cursor
            // enter the value "at the beginning".
            event_handler_ctx
                .remaining_event_v
                .push_front(egui::Event::Text(string));
            // We consumed the event.
            Ok(None)
        }
        event => {
            // We didn't consume the event, so return it.
            Ok(Some(event))
        }
    }
}
