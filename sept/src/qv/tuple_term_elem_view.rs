use crate::{
    dy,
    qv::{self, QueryableDynT},
    Error, Result,
};

// NOTE: This is almost identical to ArrayTermElemView.  TODO: Try to de-duplicate somehow.
#[derive(Clone, Debug)]
pub struct TupleTermElemView<'a> {
    pub tuple_term: &'a dy::TupleTerm,
    pub elem_index: usize,

    // Cached values
    pub elem_o: Option<&'a dy::Value>,
}

impl<'a> TupleTermElemView<'a> {
    pub fn new(tuple_term: &'a dy::TupleTerm, elem_index: usize) -> Result<Self> {
        anyhow::ensure!(
            elem_index <= tuple_term.len(),
            "TupleTermElemView index out of bounds"
        );
        let elem_o = tuple_term.get(elem_index);
        Ok(Self {
            tuple_term,
            elem_index,
            elem_o,
        })
    }
    pub fn new_with_cached_values(
        tuple_term: &'a dy::TupleTerm,
        elem_index: usize,
        elem_o: Option<&'a dy::Value>,
    ) -> Result<Self> {
        anyhow::ensure!(
            elem_index <= tuple_term.len(),
            "TupleTermElemView index out of bounds"
        );
        assert_eq!(
            elem_o,
            tuple_term.get(elem_index),
            "programmer error: actual elem_o does not match specified cached elem_o"
        );
        Ok(Self {
            tuple_term,
            elem_index,
            elem_o,
        })
    }
    pub fn go_home(&mut self) {
        self.elem_index = 0;
        self.elem_o = self.tuple_term.get(self.elem_index);
    }
    pub fn go_end(&mut self) {
        self.elem_index = self.max_elem_index();
        assert!(self.tuple_term.get(self.elem_index) == None);
        self.elem_o = None;
    }
    pub fn increment_elem_index_by(&mut self, elem_index_delta: isize) {
        self.elem_index = self
            .elem_index
            .saturating_add_signed(elem_index_delta)
            .min(self.max_elem_index());
        self.elem_o = self.tuple_term.get(self.elem_index);
    }
    fn max_elem_index(&self) -> usize {
        self.tuple_term.len()
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
        } else {
            match self.elem_o {
                Some(elem) => {
                    // Otherwise forward to the element Value.
                    elem.make_and_run_query(&mut address_token_i)
                }
                None => {
                    // If there is no element, then we can't forward to it.
                    anyhow::bail!(
                        "TupleTermElemView is at end, so has no element to forward query to"
                    );
                }
            }
            // // Otherwise forward to the element Value.
            // self.tuple_term
            //     .get(self.elem_index)
            //     .unwrap()
            //     .make_and_run_query(&mut address_token_i)
        }
        // let first_address = address_token_i.next().unwrap();
        // // TODO: If char ever gets further queries (e.g. numeric unicode value), then pass them on here.
        // use st::StringifiableT;
        // anyhow::bail!(
        //     "TupleTermElemView query doesn't support address: {}",
        //     first_address.stringify()
        // );
    }
}

impl<'b> qv::EvalT for TupleTermElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        // let elem = self.tuple_term.get(self.elem_index).unwrap();
        let elem = self.elem_o.as_deref().ok_or_else(|| {
            anyhow::anyhow!("TupleTermElemView is at end, so has no element to eval")
        })?;
        assert!(elem.type_id() != std::any::TypeId::of::<dy::Value>(), "something constructed a &ValueGuts which refers to a Value, which is not what is wanted");
        assert!(elem.type_id() != std::any::TypeId::of::<Box<dy::ValueGuts>>(), "something constructed a &ValueGuts which refers to a Box<ValueGuts>, which is not what is wanted");
        let elem_ref = elem.as_ref();
        assert!(elem_ref.type_id() != std::any::TypeId::of::<dy::Value>(), "something constructed a &ValueGuts which refers to a Value, which is not what is wanted");
        assert!(elem_ref.type_id() != std::any::TypeId::of::<Box<dy::ValueGuts>>(), "something constructed a &ValueGuts which refers to a Box<ValueGuts>, which is not what is wanted");
        Ok(dy::MaybeDereferencedValue::make_ref(elem_ref))
    }
}

// TODO: This shouldn't exist, it should go to Value somehow.
#[derive(Clone, Debug, derive_more::From)]
pub enum TupleTermElemViewQuery {}

impl<'b> qv::SingleQueryT<dy::Value> for TupleTermElemView<'b> {
    type ReturnType<'a> = TupleTermElemViewQuery where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("TupleTermElemView::run_single_query does not support any queries");
    }
}

impl qv::EvalT for TupleTermElemViewQuery {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        unimplemented!("this shouldn't exist");
    }
}
