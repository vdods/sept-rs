use crate::{dy, qv, Error};

#[derive(Debug)]
pub struct OrderedMapTermKeyMutView<'a> {
    pub ordered_map_term: &'a mut dy::OrderedMapTerm,
}

impl<'a> OrderedMapTermKeyMutView<'a> {
    pub fn new(ordered_map_term: &'a mut dy::OrderedMapTerm) -> Self {
        Self { ordered_map_term }
    }
}

impl<'a> qv::ApplyEditT for OrderedMapTermKeyMutView<'a> {
    fn apply_edit(&mut self, _edit: dy::Value) -> anyhow::Result<()> {
        unimplemented!("blah");
    }
}

impl<'b> qv::SingleQueryMutT<dy::Value> for OrderedMapTermKeyMutView<'b> {
    type ReturnType<'a> = qv::OrderedMapTermKeyElemMutView<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        Ok(qv::OrderedMapTermKeyElemMutView::new(
            self.ordered_map_term,
            address_token.clone(),
        )?)
    }
}
