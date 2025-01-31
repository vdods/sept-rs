use crate::{dy, qv, Result};

#[derive(Clone, Debug, derive_more::From)]
pub enum StructTermQuery<'a> {
    StructTermFieldElemView(qv::StructTermFieldElemView<'a>),
    // StructTermFieldNameView(qv::StructTermFieldNameView<'a>),
    // StructTermFieldTypeView(qv::StructTermFieldTypeView<'a>),
}

// TODO: Derive
impl<'b> qv::EvalT for StructTermQuery<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        match self {
            Self::StructTermFieldElemView(v) => v.eval(),
            // Self::StructTermFieldNameView(v) => v.eval(),
            // Self::StructTermFieldTypeView(v) => v.eval(),
        }
    }
}
