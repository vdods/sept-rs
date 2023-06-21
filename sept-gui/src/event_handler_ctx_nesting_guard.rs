use crate::EventHandlerCtx;

pub struct EventHandlerCtxNestingGuard<'a, 'b> {
    event_handler_ctx: &'a mut EventHandlerCtx<'b>,
}

impl<'a, 'b: 'a> EventHandlerCtxNestingGuard<'a, 'b> {
    pub(crate) fn new(event_handler_ctx: &'a mut EventHandlerCtx<'b>) -> Self {
        event_handler_ctx.current_nesting_depth = event_handler_ctx
            .current_nesting_depth
            .checked_add(1)
            .expect("programmer error: EventHandlerCtx current_nesting_depth overflow");
        Self { event_handler_ctx }
    }
}

impl<'a, 'b: 'a> std::ops::Deref for EventHandlerCtxNestingGuard<'a, 'b> {
    type Target = EventHandlerCtx<'b>;
    fn deref(&self) -> &Self::Target {
        &*self.event_handler_ctx
    }
}

impl<'a, 'b: 'a> std::ops::DerefMut for EventHandlerCtxNestingGuard<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.event_handler_ctx
    }
}

impl<'a, 'b: 'a> std::ops::Drop for EventHandlerCtxNestingGuard<'a, 'b> {
    fn drop(&mut self) {
        self.event_handler_ctx.current_nesting_depth = self
            .event_handler_ctx
            .current_nesting_depth
            .checked_sub(1)
            .expect("programmer error: EventHandlerCtx current_nesting_depth underflow");
    }
}
