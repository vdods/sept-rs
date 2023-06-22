use crate::EventHandlerCtx;
use anyhow::Result;

pub trait EventHandler {
    /// If the event wasn't handled, it should be returned; otherwise None.  The event handler may decide
    /// to alter `remaining_event_v`, e.g. if it only a part of an `egui::Event::Text` and wants to push the
    /// rest of it back onto the front of `remaining_event_v`, or if it wants to filter out some
    /// redundant `egui::Event::Key` events based on already having processed them via `egui::Event::Text`.
    // TODO: Should view_ctx actually be &ViewCtx and not &mut ViewCtx?  Since cursor edits are supposed
    // to happen through Command now, not directly altering the cursor.  Though this is complicated by
    // the fact that commands are enqueued via ViewCtx.
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>>;

    // TODO: Create handle_nonterminal_event method and then a bunch of different methods for the
    // various event types.
}
