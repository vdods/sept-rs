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
                |address: sept::dy::TupleTerm, new_data: sept::dy::Value| -> Command {
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
/// items at the non-element at the end of an array or other container.  command_factory takes
/// the address and new_data and should return the command to be executed if the Placeholder
/// is being replaced by the new_data.
pub fn placeholder_event_handler_impl(
    event: egui::Event,
    event_handler_ctx: &mut EventHandlerCtx<'_>,
    command_factory: impl Fn(sept::dy::TupleTerm, sept::dy::Value) -> Command,
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
            let command = command_factory(address, new_data);
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
