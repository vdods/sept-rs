use crate::{dy, st};

// pub trait EditT: st::TermT {
pub trait EditT: Clone {
    /// This is the type of the inverse of this DiffT.
    type Inverse: EditT;
    /// Convert into the inverse of this DiffT.
    fn into_inverse(self) -> Self::Inverse;
    /// Return the inverse of this DiffT.  The default implementation simply calls
    /// `self.clone().into_inverse()`, but an impl of DiffT may want to specialize this.
    fn inverse(&self) -> Self::Inverse {
        self.clone().into_inverse()
    }
}

// Some canonical ones

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::Term",
    is_parametric = "false",
    is_type = "false"
)]
pub struct NoOp;

impl EditT for NoOp {
    type Inverse = NoOp;
    fn into_inverse(self) -> Self::Inverse {
        self
    }
}

// TODO: make into st::NonParametricTermT
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Insertion;

impl st::InhabitsT<st::Type> for Insertion {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

// TODO: make into st::NonParametricTermT
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Deletion;

impl st::InhabitsT<st::Type> for Deletion {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

// TODO: make into st::NonParametricTermT
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Replacement;

impl st::InhabitsT<st::Type> for Replacement {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}
