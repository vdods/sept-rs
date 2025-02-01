use crate::{EventHandlerCtx, EventHandlerT};
use anyhow::Result;

impl EventHandlerT for sept::dy::Value {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!("Value::handle_event; event: {:?}", event);
        // TODO: Probably use a registration pattern here
        if let Some(x) = self.downcast_ref::<sept::dy::ArrayTerm>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else if let Some(x) = self.downcast_ref::<sept::st::Placeholder>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else if let Some(x) = self.downcast_ref::<sept::dy::StructTerm>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else if let Some(x) = self.downcast_ref::<sept::dy::TupleTerm>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else if let Some(x) = self.downcast_ref::<sept::st::UTF8StringTerm>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else {
            // This is just a temporary warning, until we support all Value variants.
            use sept::st::StringifiableT;
            tracing::warn!(
                "Unsupported Value variant encountered in handle_event: self: {}",
                self.stringify()
            );
            // We didn't handle the event, so return it.
            Ok(Some(event))
        }
    }
}
