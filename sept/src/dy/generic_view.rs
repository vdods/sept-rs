use crate::{dy, Result};

pub struct GenericView<'a, T>(&'a T);

impl<'a, T> GenericView<'a, T> {
    pub fn new(x: &'a T) -> Box<Self> {
        Box::new(Self(x))
    }
}

impl<'b, T: std::any::Any + Send + Sync> dy::QueryTrait for GenericView<'b, T> {
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            Ok(self)
        } else {
            anyhow::bail!("GenericView doesn't support query on any nonempty address");
        }
    }
}

impl<'b, T: std::any::Any + Send + Sync> dy::QueryViewTrait for GenericView<'b, T> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}
