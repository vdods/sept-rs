use crate::qv;

#[derive(Clone, Debug, derive_more::From)]
pub enum Utf8StringTermQuery<'a> {
    Utf8StringTermCharView(qv::Utf8StringTermCharView<'a>),
    Utf8StringTermLineView(qv::Utf8StringTermLineView<'a>),
}
