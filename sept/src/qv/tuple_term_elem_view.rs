use crate::{dy, qv, Error, Result};

#[derive(Clone, Debug)]
pub struct TupleTermElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    pub tuple_term: &'a dy::TupleTerm,
    pub elem_index: usize,
}

impl<'a> TupleTermElemView<'a> {
    pub fn new(tuple_term: &'a dy::TupleTerm, elem_index: usize) -> Result<Self> {
        anyhow::ensure!(
            elem_index < tuple_term.len(),
            "TupleTermElemView elem_index out of bounds"
        );
        Ok(Self {
            tuple_term,
            elem_index,
        })
    }
}

impl<'b> qv::QueryT for TupleTermElemView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalT + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        unimplemented!("blah");
        // let first_address = address_token_i.next().unwrap();
        // TODO: use single_query
        // TODO: If char ever gets further queries (e.g. numeric unicode value), then pass them on here.
        // anyhow::bail!(
        //     "TupleTermElemView query doesn't support address: {}",
        //     dy::RUNTIME_LA.read().unwrap().stringify(first_address)
        // );
    }
}

impl<'b> qv::EvalT for TupleTermElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(
            self.tuple_term.get(self.elem_index).unwrap(),
        ))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum TupleTermElemViewQuery {}

impl<'b> qv::SingleQueryT<dy::Value> for TupleTermElemView<'b> {
    type ReturnType<'a> = TupleTermElemViewQuery where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        // TODO: Forward to element.
        unimplemented!("blah");
        // anyhow::bail!("TupleTermElemView::run_single_query does not support any queries");
    }
}

impl qv::EvalT for TupleTermElemViewQuery {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        unimplemented!("this shouldn't exist");
    }
}
