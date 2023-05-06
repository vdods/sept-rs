// NOTE: This will become defunct. The new diff interface is much more flexible, and doesn't
// require the Diffable trait to enumerate the kinds of diffs it supports. Instead, it just
// requires the Diffable trait to implement a single method, apply_diff_in_place, which can
// handle any kind of diff. This is much more flexible, and allows for diffs to be defined
// dynamically, rather than statically (as is the case with the Diff trait below).

use crate::{dy, st};

// NOTE: This will become defunct.
pub trait Diff: st::TermTrait {
    fn into_inverse(self) -> dy::Value;
    fn inverse(&self) -> dy::Value {
        self.clone().into_inverse()
    }
}

// TODO: Move into appropriate places

// #[derive(Clone, Debug, dy::IntoValue, PartialEq, st::TermTrait)]
// #[st_term_trait(
//     AbstractTypeType = "Replacement",
//     is_parametric = "true",
//     is_type = "false"
// )]
// pub struct ReplacementTerm {
//     pub old_data: dy::Value,
//     pub new_data: dy::Value,
// }

// impl Diff for ReplacementTerm {
//     fn into_inverse(self) -> dy::Value {
//         ReplacementTerm {
//             old_data: self.new_data,
//             new_data: self.old_data,
//         }
//         .into()
//     }
// }

// TEMP HACK: keep these for now.

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
