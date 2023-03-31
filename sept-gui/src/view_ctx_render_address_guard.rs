use crate::ViewCtx;

pub struct ViewCtxRenderAddressGuard<'a> {
    view_ctx: &'a mut ViewCtx,
}

impl<'a> ViewCtxRenderAddressGuard<'a> {
    pub(crate) fn new(view_ctx: &'a mut ViewCtx, address_token: sept::dy::Value) -> Self {
        view_ctx.render_address.push(address_token);
        use sept::st::Stringifiable;
        tracing::debug!(
            "push; render_address: {}",
            view_ctx.render_address.stringify()
        );
        Self { view_ctx }
    }
}

impl<'a> std::ops::Deref for ViewCtxRenderAddressGuard<'a> {
    type Target = ViewCtx;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a> std::ops::DerefMut for ViewCtxRenderAddressGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

impl<'a> std::ops::Drop for ViewCtxRenderAddressGuard<'a> {
    fn drop(&mut self) {
        self.view_ctx
            .render_address
            .pop()
            .expect("programmer error: pop didn't match push");
        use sept::st::Stringifiable;
        tracing::debug!(
            "pop;  render_address: {}",
            self.view_ctx.render_address.stringify()
        )
    }
}
