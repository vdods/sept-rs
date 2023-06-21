use crate::qv;

/// This is meant to provide a specific return type for a single-step, mutable query, so that it can
/// be called in other places that require more general query behavior.
// TODO: Consider requiring SingleQuery<AddressToken>
pub trait SingleQueryMut<AddressToken> {
    // TODO: Is it possible to require SingleQueryMut<AddressToken> here?  probably not, since it would
    // be some ridiculous branching type unless somehow interrupted by trait objects.
    // TODO: Make it possible to use &AddressToken in ReturnType.  This probably requires
    // introducing a new lifetime for &AddressToken and maybe one for the return type,
    // and setting proper lifetime bounds.
    type ReturnType<'a>: qv::ApplyEditTrait + qv::QueryMutAndApplyEditTrait + 'a
    where
        Self: 'a;
    type Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &AddressToken,
    ) -> Result<Self::ReturnType<'a>, Self::Error>;
}
