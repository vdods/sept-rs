use crate::{dy, qv, Result};

pub struct GenericView<'a, T>(&'a T);

impl<'a, T> GenericView<'a, T> {
    pub fn new(x: &'a T) -> Self {
        Self(x)
    }
}

impl<'b, T: std::any::Any + Send + Sync> qv::QueryTrait for GenericView<'b, T> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            Ok(self)
        } else {
            anyhow::bail!("GenericView doesn't support query on any nonempty address");
        }
    }
}

impl<'b, T: std::any::Any + Send + Sync> qv::EvalTrait for GenericView<'b, T> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0))
    }
}
