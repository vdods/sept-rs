use crate::{dy, Result};

#[derive(Clone, Debug)]
pub struct ArrayTermView<'a>(&'a dy::ArrayTerm);

impl<'a> ArrayTermView<'a> {
    pub fn new(array_term: &'a dy::ArrayTerm) -> Box<Self> {
        Box::new(Self(array_term))
    }
}

impl<'b> dy::QueryTrait for ArrayTermView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_i.next().unwrap();
        if let Some(element_index) = first_address.downcast_ref::<u32>() {
            // Otherwise index into the array.
            anyhow::ensure!(
                *element_index as usize <= self.0.len(),
                "ArrayTerm query address index out of bounds: {}",
                element_index
            );
            let element = &self.0[*element_index as usize];
            use dy::QueryableDynTrait;
            element.make_and_run_query(&mut address_i)
        } else {
            // TODO: Handle array length, etc.
            anyhow::bail!(
                "ArrayTerm query doesn't support address: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(first_address)
            );
        }
    }
}

impl<'b> dy::QueryViewTrait for ArrayTermView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}
