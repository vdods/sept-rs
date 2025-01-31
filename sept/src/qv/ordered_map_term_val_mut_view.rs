use crate::{dy, qv, Error, Result};

// NOTE: This doesn't need to exist, it can simply pass to the value.
#[derive(Debug)]
pub struct OrderedMapTermValMutView<'a> {
    pub ordered_map_term: &'a mut dy::OrderedMapTerm,
}

impl<'a> OrderedMapTermValMutView<'a> {
    pub fn new(ordered_map_term: &'a mut dy::OrderedMapTerm) -> Self {
        Self { ordered_map_term }
    }
}

impl<'a> qv::ApplyEditT for OrderedMapTermValMutView<'a> {
    fn apply_edit(&mut self, _edit: dy::Value) -> Result<()> {
        anyhow::bail!("OrderedMapTermValMutView doesn't support any edits yet");
    }
}

impl<'b> qv::SingleQueryMutT<dy::Value> for OrderedMapTermValMutView<'b> {
    type ReturnType<'a> = qv::OrderedMapTermValElemMutView<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        Ok(qv::OrderedMapTermValElemMutView::new(
            self.ordered_map_term,
            address_token.clone(),
        )?)
    }
}
