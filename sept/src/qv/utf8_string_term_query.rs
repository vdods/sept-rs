use crate::qv;

#[derive(Clone, Debug, derive_more::From)]
pub enum UTF8StringTermQuery<'a> {
    UTF8StringTermCharView(qv::UTF8StringTermCharView<'a>),
    UTF8StringTermLineView(qv::UTF8StringTermLineView<'a>),
}
