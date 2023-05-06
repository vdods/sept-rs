use crate::ViewCtx;

pub struct ViewCtxNestingGuard<'a, 'b> {
    view_ctx: &'a mut ViewCtx<'b>,
}

impl<'a, 'b: 'a> ViewCtxNestingGuard<'a, 'b> {
    pub(crate) fn new(view_ctx: &'a mut ViewCtx<'b>) -> Self {
        view_ctx.current_nesting_depth = view_ctx
            .current_nesting_depth
            .checked_add(1)
            .expect("programmer error: ViewCtx current_nesting_depth overflow");
        Self { view_ctx }
    }
}

impl<'a, 'b: 'a> std::ops::Deref for ViewCtxNestingGuard<'a, 'b> {
    type Target = ViewCtx<'b>;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a, 'b: 'a> std::ops::DerefMut for ViewCtxNestingGuard<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

impl<'a, 'b: 'a> std::ops::Drop for ViewCtxNestingGuard<'a, 'b> {
    fn drop(&mut self) {
        self.view_ctx.current_nesting_depth = self
            .view_ctx
            .current_nesting_depth
            .checked_sub(1)
            .expect("programmer error: ViewCtx current_nesting_depth underflow");
    }
}
