use crate::{dy, st};

pub trait DiffTrait: st::TermTrait {
    /// This is the type of the inverse of this DiffTrait.
    type Inverse: DiffTrait;
    /// Convert into the inverse of this DiffTrait.
    fn into_inverse(self) -> Self::Inverse;
    /// Return the inverse of this DiffTrait.  The default implementation simply calls
    /// `self.clone().into_inverse()`, but an impl of DiffTrait may want to specialize this.
    fn inverse(&self) -> Self::Inverse {
        self.clone().into_inverse()
    }
}

// Some canonical ones

// TODO: Should this be NoOpTerm, and there be a NoOp type?
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

impl DiffTrait for NoOp {
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

// TODO: make into st::NonParametricTermTrait
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
pub struct ElementInsertion;

impl st::Inhabits<st::Type> for ElementInsertion {
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
pub struct ElementDeletion;

impl st::Inhabits<st::Type> for ElementDeletion {
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
pub struct ElementReplacement;

impl st::Inhabits<st::Type> for ElementReplacement {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

// /// C is the container term type, I is the index type, E is the type of data to insert.
// // TODO: Make this into an actual TermTrait; this would require creating an appropriate type hierarchy
// // for this, maybe including `ElementInsertion`, `ElementInsertionTerm<C, I, E>`.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ElementInsertionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     pub element_index: I,
//     pub insertion_data: E,
//     phantom_data: std::marker::PhantomData<C>,
// }

// impl<C, I, E> ElementInsertionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     pub fn new(element_index: I, insertion_data: E) -> Self {
//         Self {
//             element_index,
//             insertion_data,
//             phantom_data: Default::default(),
//         }
//     }
// }

// impl<C, I, E> st::Inhabits<ElementInsertion> for ElementInsertionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     fn inhabits(&self, _rhs: &ElementInsertion) -> bool {
//         true
//     }
// }

// impl<C, I, E> dy::IntoValue for ElementInsertionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
// }

// impl<C, I, E> st::Stringifiable for ElementInsertionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     fn stringify(&self) -> String {
//         // Bit of a temp hack
//         format!("{:?}", self)
//     }
// }

// impl<C, I, E> st::TermTrait for ElementInsertionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     type AbstractTypeType = ElementInsertion;
//     fn is_parametric(&self) -> bool {
//         true
//     }
//     fn is_type(&self) -> bool {
//         false
//     }
//     fn abstract_type(&self) -> Self::AbstractTypeType {
//         ElementInsertion
//     }
// }

// /// C is the container term type, I is the index type, E is the type of data to delete.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ElementDeletionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     pub element_index: I,
//     pub deletion_data: E,
//     phantom_data: std::marker::PhantomData<C>,
// }

// impl<C, I, E> ElementDeletionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     pub fn new(element_index: I, deletion_data: E) -> Self {
//         Self {
//             element_index,
//             deletion_data,
//             phantom_data: Default::default(),
//         }
//     }
// }

// impl<C, I, E> st::Inhabits<ElementDeletion> for ElementDeletionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     fn inhabits(&self, _rhs: &ElementDeletion) -> bool {
//         true
//     }
// }

// impl<C, I, E> dy::IntoValue for ElementDeletionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
// }

// impl<C, I, E> st::Stringifiable for ElementDeletionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     fn stringify(&self) -> String {
//         // Bit of a temp hack
//         format!("{:?}", self)
//     }
// }

// impl<C, I, E> st::TermTrait for ElementDeletionTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     type AbstractTypeType = ElementDeletion;
//     fn is_parametric(&self) -> bool {
//         true
//     }
//     fn is_type(&self) -> bool {
//         false
//     }
//     fn abstract_type(&self) -> Self::AbstractTypeType {
//         ElementDeletion
//     }
// }

// /// C is the container term type, I is the index type, E is the type of data to replace
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ElementReplacementTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     pub element_index: I,
//     pub old_data: E,
//     pub new_data: E,
//     phantom_data: std::marker::PhantomData<C>,
// }

// impl<C, I, E> ElementReplacementTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     pub fn new(element_index: I, old_data: E, new_data: E) -> Self {
//         Self {
//             element_index,
//             old_data,
//             new_data,
//             phantom_data: Default::default(),
//         }
//     }
// }

// impl<C, I, E> st::Inhabits<ElementReplacement> for ElementReplacementTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     fn inhabits(&self, _rhs: &ElementReplacement) -> bool {
//         true
//     }
// }

// impl<C, I, E> dy::IntoValue for ElementReplacementTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
// }

// impl<C, I, E> st::Stringifiable for ElementReplacementTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     fn stringify(&self) -> String {
//         // Bit of a temp hack
//         format!("{:?}", self)
//     }
// }

// impl<C, I, E> st::TermTrait for ElementReplacementTerm<C, I, E>
// where
//     C: st::TermTrait,
//     I: st::TermTrait,
//     E: st::TermTrait,
// {
//     type AbstractTypeType = ElementReplacement;
//     fn is_parametric(&self) -> bool {
//         true
//     }
//     fn is_type(&self) -> bool {
//         false
//     }
//     fn abstract_type(&self) -> Self::AbstractTypeType {
//         ElementReplacement
//     }
// }
