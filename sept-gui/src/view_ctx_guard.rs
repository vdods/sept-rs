use crate::ViewCtx;

pub struct ViewCtxGuard<'a> {
    view_ctx: &'a mut ViewCtx,
}

impl<'a> ViewCtxGuard<'a> {
    pub fn new(view_ctx: &'a mut ViewCtx) -> Self {
        Self { view_ctx }
    }
}

// impl<'a> std::borrow::Borrow<ViewCtx> for ViewCtxGuard<'a> {
//     fn borrow(&self) -> &ViewCtx {
//         &*self.view_ctx
//     }
// }

// impl<'a> std::borrow::BorrowMut<ViewCtx> for ViewCtxGuard<'a> {
//     fn borrow_mut(&mut self) -> &mut ViewCtx {
//         &mut *self.view_ctx
//     }
// }

impl<'a> std::ops::Deref for ViewCtxGuard<'a> {
    type Target = ViewCtx;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a> std::ops::DerefMut for ViewCtxGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

// impl NestingGuard {
//     pub fn view_ctx_mut(&self) -> &mut ViewCtx {
//         self.view_ctx
//     }
// }

impl<'a> std::ops::Drop for ViewCtxGuard<'a> {
    fn drop(&mut self) {
        self.view_ctx.current_nesting_depth = self
            .view_ctx
            .current_nesting_depth
            .checked_sub(1)
            .expect("programmer error: ViewCtx current_nesting_depth underflow");
    }
}
