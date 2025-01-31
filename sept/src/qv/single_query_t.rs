/// This is meant to provide a specific return type for a single-step query, so that it can be called
/// in other places that require more general query behavior.
pub trait SingleQueryT<AddressToken> {
    type ReturnType<'a>: 'a
    where
        Self: 'a;
    type Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &AddressToken,
    ) -> Result<Self::ReturnType<'a>, Self::Error>;
}
