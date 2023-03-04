use crate::ViewCtx;

pub struct ViewCtxTAGuard<'a> {
    view_ctx: &'a mut ViewCtx,
    previous_show_type_annotations: bool,
}

impl<'a> ViewCtxTAGuard<'a> {
    pub(crate) fn new(view_ctx: &'a mut ViewCtx, show_type_annotations: bool) -> Self {
        let previous_show_type_annotations = view_ctx.show_type_annotations;
        view_ctx.show_type_annotations = show_type_annotations;
        Self {
            view_ctx,
            previous_show_type_annotations,
        }
    }
}

impl<'a> std::ops::Deref for ViewCtxTAGuard<'a> {
    type Target = ViewCtx;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a> std::ops::DerefMut for ViewCtxTAGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

impl<'a> std::ops::Drop for ViewCtxTAGuard<'a> {
    fn drop(&mut self) {
        self.view_ctx.show_type_annotations = self.previous_show_type_annotations;
    }
}
