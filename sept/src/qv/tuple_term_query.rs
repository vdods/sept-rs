use crate::{dy, qv, Result};

#[derive(Clone, Debug, derive_more::From)]
pub enum TupleTermQuery<'a> {
    TupleTermElemView(qv::TupleTermElemView<'a>),
}

// TODO: Derive
impl<'b> qv::EvalT for TupleTermQuery<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        match self {
            Self::TupleTermElemView(v) => v.eval(),
        }
    }
}
