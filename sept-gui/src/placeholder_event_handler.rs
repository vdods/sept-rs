use crate::{
    first_char_stripped_string, AddressedEdit, Command, EventHandler, EventHandlerCtx,
    RootValueEdit,
};
use anyhow::Result;

impl EventHandler for sept::st::Placeholder {
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
                PlaceholderEventKind::ReplacementTerm,
            )
        }
    }
}

pub enum PlaceholderEventKind {
    /// This indicates that the newly created term is going to be inserted, not replacing anything.
    InsertionTerm,
    /// This indicates that a Placeholder term is going to be replaced by the newly created term.
    ReplacementTerm,
}

/// Slightly more generalized version of Placeholder::event_handler which can handle inserting
/// items at the non-element at the end of an array or other container.
pub fn placeholder_event_handler_impl(
    event: egui::Event,
    event_handler_ctx: &mut EventHandlerCtx<'_>,
    placeholder_event_kind: PlaceholderEventKind,
) -> Result<Option<egui::Event>> {
    match event {
        egui::Event::Text(string) if string.starts_with("\"") || string.starts_with("[") => {
            let address = event_handler_ctx.cursor_address.clone();
            let new_data: sept::dy::Value = if string.starts_with("\"") {
                "".to_string().into()
            } else if string.starts_with("[") {
                sept::dy::ArrayTerm::from(vec![]).into()
            } else {
                unreachable!("programmer error: you missed a case!")
            };
            let command: Command = match placeholder_event_kind {
                PlaceholderEventKind::InsertionTerm => RootValueEdit::from(AddressedEdit {
                    address,
                    edit: sept::qv::InsertionTerm { new_data }.into(),
                })
                .into(),
                PlaceholderEventKind::ReplacementTerm => RootValueEdit::from(AddressedEdit {
                    address,
                    edit: sept::qv::ReplacementTerm {
                        old_data: sept::st::Placeholder.into(),
                        new_data,
                    }
                    .into(),
                })
                .into(),
            };
            event_handler_ctx.enqueue_command(command);
            // Take the used char off the front of the string and push the rest back onto the remaining events,
            // if there's anything left of the string after the first char.
            if let Some(string) = first_char_stripped_string(string) {
                event_handler_ctx
                    .remaining_event_v
                    .push_front(egui::Event::Text(string));
            }
            // This is a bit of a hack, but it's very effective; we want to enter the
            // just-inserted string, so we push an Enter event onto the remaining events.
            event_handler_ctx
                .remaining_event_v
                .push_front(egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                });
            // We consumed the event.
            Ok(None)
        }
        event => {
            // We didn't consume the event, so return it.
            Ok(Some(event))
        }
    }
}
