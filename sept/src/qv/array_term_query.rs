use crate::{dy, qv, Result};

#[derive(Clone, Debug, derive_more::From)]
pub enum ArrayTermQuery<'a> {
    ArrayTermElemView(qv::ArrayTermElemView<'a>),
}

// TODO: Derive
impl<'b> qv::EvalT for ArrayTermQuery<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        match self {
            Self::ArrayTermElemView(v) => v.eval(),
        }
    }
}
