use crate::{dy, st};

// pub trait EditTrait: st::TermTrait {
pub trait EditTrait: Clone {
    /// This is the type of the inverse of this DiffTrait.
    type Inverse: EditTrait;
    /// Convert into the inverse of this DiffTrait.
    fn into_inverse(self) -> Self::Inverse;
    /// Return the inverse of this DiffTrait.  The default implementation simply calls
    /// `self.clone().into_inverse()`, but an impl of DiffTrait may want to specialize this.
    fn inverse(&self) -> Self::Inverse {
        self.clone().into_inverse()
    }
}

// Some canonical ones

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
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

impl EditTrait for NoOp {
    type Inverse = NoOp;
    fn into_inverse(self) -> Self::Inverse {
        self
    }
}

// TODO: make into st::NonParametricTermTrait
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_term_trait(
    AbstractTypeType = "st::Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Insertion;

impl st::Inhabits<st::Type> for Insertion {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

// TODO: make into st::NonParametricTermTrait
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_term_trait(
    AbstractTypeType = "st::Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Deletion;

impl st::Inhabits<st::Type> for Deletion {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

// TODO: make into st::NonParametricTermTrait
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
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
