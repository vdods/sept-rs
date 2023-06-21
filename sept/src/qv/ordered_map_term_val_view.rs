use crate::{dy, qv, Result};

// NOTE: This is not necessary either
#[derive(Clone, Debug)]
pub struct OrderedMapTermValView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an OrderedMapTerm view object.
    pub ordered_map_term: &'a dy::OrderedMapTerm,
    pub key: &'a dy::Value,
}

impl<'a> OrderedMapTermValView<'a> {
    pub fn new(ordered_map_term: &'a dy::OrderedMapTerm, key: &'a dy::Value) -> Result<Self> {
        anyhow::ensure!(
            ordered_map_term.contains_key(key),
            "OrderedMapTerm doesn't contain the specified key"
        );
        Ok(Self {
            ordered_map_term,
            key,
        })
    }
}

impl<'b> qv::QueryTrait for OrderedMapTermValView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            // If we're at the end of the address, then this is the value we're looking for.
            return Ok(self);
        } else {
            // Otherwise, pass it on to the val.  This unwrap can't fail because key containment
            // was checked upon construction.
            let val = self.ordered_map_term.get(self.key).unwrap();
            use qv::QueryableDynTrait;
            val.make_and_run_query(&mut address_token_i)
        }
    }
}

impl<'b> qv::EvalTrait for OrderedMapTermValView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        // This unwrap can't fail because key containment was checked upon construction.
        let val = self.ordered_map_term.get(self.key).unwrap();
        Ok(dy::MaybeDereferencedValue::make_ref(val.as_ref()))
    }
}
