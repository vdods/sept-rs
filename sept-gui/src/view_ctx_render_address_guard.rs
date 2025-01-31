use crate::ViewCtx;

pub struct ViewCtxRenderAddressGuard<'a, 'b> {
    view_ctx: &'a mut ViewCtx<'b>,
}

impl<'a, 'b: 'a> ViewCtxRenderAddressGuard<'a, 'b> {
    pub(crate) fn new(view_ctx: &'a mut ViewCtx<'b>, address_token: sept::dy::Value) -> Self {
        view_ctx.render_address.push(address_token);
        // use sept::st::StringifiableT;
        // tracing::debug!(
        //     "push; render_address: {}",
        //     view_ctx.render_address.stringify()
        // );
        Self { view_ctx }
    }
}

impl<'a, 'b: 'a> std::ops::Deref for ViewCtxRenderAddressGuard<'a, 'b> {
    type Target = ViewCtx<'b>;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a, 'b: 'a> std::ops::DerefMut for ViewCtxRenderAddressGuard<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

impl<'a, 'b: 'a> std::ops::Drop for ViewCtxRenderAddressGuard<'a, 'b> {
    fn drop(&mut self) {
        self.view_ctx
            .render_address
            .pop()
            .expect("programmer error: pop didn't match push");
        // use sept::st::StringifiableT;
        // tracing::debug!(
        //     "pop;  render_address: {}",
        //     self.view_ctx.render_address.stringify()
        // )
    }
}
