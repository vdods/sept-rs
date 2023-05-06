use crate::ViewCtx;

pub struct ViewCtxTAGuard<'a, 'b> {
    view_ctx: &'a mut ViewCtx<'b>,
}

impl<'a, 'b: 'a> ViewCtxTAGuard<'a, 'b> {
    pub(crate) fn new(view_ctx: &'a mut ViewCtx<'b>, show_type_annotations: bool) -> Self {
        view_ctx.override_show_type_annotations_o = Some(show_type_annotations);
        Self { view_ctx }
    }
}

impl<'a, 'b: 'a> std::ops::Deref for ViewCtxTAGuard<'a, 'b> {
    type Target = ViewCtx<'b>;
    fn deref(&self) -> &Self::Target {
        &*self.view_ctx
    }
}

impl<'a, 'b: 'a> std::ops::DerefMut for ViewCtxTAGuard<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.view_ctx
    }
}

impl<'a, 'b: 'a> std::ops::Drop for ViewCtxTAGuard<'a, 'b> {
    fn drop(&mut self) {
        self.view_ctx.override_show_type_annotations_o = None;
    }
}
