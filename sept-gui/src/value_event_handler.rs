use crate::{EventHandler, EventHandlerCtx};
use anyhow::Result;

impl EventHandler for sept::dy::Value {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!("Value::handle_event; event: {:?}", event);
        // TODO: Probably use a registration pattern here
        if let Some(x) = self.downcast_ref::<sept::st::Utf8StringTerm>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else if let Some(x) = self.downcast_ref::<sept::dy::ArrayTerm>() {
            x.handle_event(event, event_handler_ctx, cursor_address_token_i)
        } else {
            anyhow::bail!("Unsupported Value variant for handle_event");
        }
    }
}
