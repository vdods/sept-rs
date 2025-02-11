use crate::qv;

#[derive(Clone, Debug, derive_more::From)]
pub enum UnicodeCharTermQuery<'a> {
    UnicodeCharTermPlainView(qv::UnicodeCharTermPlainView<'a>),
    UnicodeCharTermEscCView(qv::UnicodeCharTermEscCView<'a>),
}
