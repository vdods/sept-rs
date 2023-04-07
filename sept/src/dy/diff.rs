use crate::{dy, st};

pub trait Diff: st::TermTrait {
    fn into_inverse(self) -> dy::Value;
    fn inverse(&self) -> dy::Value {
        self.clone().into_inverse()
    }
}

// TODO: Move into appropriate places

// TODO: This belongs in st
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    // st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_term_trait(
    AbstractTypeType = "st::Term",
    is_parametric = "false",
    is_type = "false"
)]
pub struct NoOp;

impl Diff for NoOp {
    fn into_inverse(self) -> dy::Value {
        self.into()
    }
}

// TODO: This belongs in st
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    // st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_term_trait(
    AbstractTypeType = "st::Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Replacement;

impl st::Inhabits<st::Type> for Replacement {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

#[derive(Clone, Debug, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
    AbstractTypeType = "Replacement",
    is_parametric = "true",
    is_type = "false"
)]
pub struct ReplacementTerm {
    pub old_data: dy::Value,
    pub new_data: dy::Value,
}

impl Diff for ReplacementTerm {
    fn into_inverse(self) -> dy::Value {
        ReplacementTerm {
            old_data: self.new_data,
            new_data: self.old_data,
        }
        .into()
    }
}

#[derive(Clone, Debug, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
    AbstractTypeType = "st::ElementInsertion",
    is_parametric = "true",
    is_type = "false"
)]
pub struct ElementInsertionTerm {
    pub index: dy::Value,
    pub data: dy::Value,
}

impl Diff for ElementInsertionTerm {
    fn into_inverse(self) -> dy::Value {
        ElementDeletionTerm {
            index: self.index,
            data: self.data,
        }
        .into()
    }
}

#[derive(Clone, Debug, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
    AbstractTypeType = "st::ElementDeletion",
    is_parametric = "true",
    is_type = "false"
)]
pub struct ElementDeletionTerm {
    pub index: dy::Value,
    pub data: dy::Value,
}

impl Diff for ElementDeletionTerm {
    fn into_inverse(self) -> dy::Value {
        ElementInsertionTerm {
            index: self.index,
            data: self.data,
        }
        .into()
    }
}

#[derive(Clone, Debug, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
    AbstractTypeType = "st::ElementReplacement",
    is_parametric = "true",
    is_type = "false"
)]
pub struct ElementReplacementTerm {
    pub index: dy::Value,
    pub old_data: dy::Value,
    pub new_data: dy::Value,
}

impl Diff for ElementReplacementTerm {
    fn into_inverse(self) -> dy::Value {
        ElementReplacementTerm {
            index: self.index,
            old_data: self.new_data,
            new_data: self.old_data,
        }
        .into()
    }
}
