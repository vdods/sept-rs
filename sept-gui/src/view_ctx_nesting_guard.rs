use crate::ViewCtx;

pub struct ViewCtxNestingGuard<'a> {
    view_ctx: &'a mut ViewCtx,
}

impl<'a> ViewCtxNestingGuard<'a> {
    pub(crate) fn new(view_ctx: &'a mut ViewCtx) -> Self {
        view_ctx.current_nesting_depth = view_ctx
            .current_nesting_depth
            .checked_add(1)
            .expect("programmer error: ViewCtx current_nesting_depth overflow");
        Self { view_ctx }
    }
}

impl<'a> std::ops::Deref for ViewCtxNestingGuard<'a> {
    type Target = ViewCtx;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a> std::ops::DerefMut for ViewCtxNestingGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

impl<'a> std::ops::Drop for ViewCtxNestingGuard<'a> {
    fn drop(&mut self) {
        self.view_ctx.current_nesting_depth = self
            .view_ctx
            .current_nesting_depth
            .checked_sub(1)
            .expect("programmer error: ViewCtx current_nesting_depth underflow");
    }
}
