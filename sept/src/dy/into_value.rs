/// This is a marker trait that indicates a type may be used in `impl From<T> for Value`.
/// It's only meant to avoid the problem with lack of generics specialization.  If it were
/// possible to `impl<T> From<T> for Value where T != Value`, then this trait wouldn't be
/// necessary.  But failing that, you just have to use this marker trait to assure the
/// compiler that, yes, things will be ok.
pub trait IntoValue {}
