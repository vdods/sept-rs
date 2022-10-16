use crate::{dy, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Sint8Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Sint16Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Sint32Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Sint64Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Uint8Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Uint16Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Uint32Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Uint64Type;

impl st::Inhabits<Type> for Sint8Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Sint16Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Sint32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Sint64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Uint8Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Uint16Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Uint32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Uint64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for Sint8Type {
    fn identifier() -> &'static str {
        "Sint8Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Sint8Type
    }
}

impl NonParametricTermTrait for Sint16Type {
    fn identifier() -> &'static str {
        "Sint16Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Sint16Type
    }
}

impl NonParametricTermTrait for Sint32Type {
    fn identifier() -> &'static str {
        "Sint32Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Sint32Type
    }
}

impl NonParametricTermTrait for Sint64Type {
    fn identifier() -> &'static str {
        "Sint64Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Sint64Type
    }
}

impl NonParametricTermTrait for Uint8Type {
    fn identifier() -> &'static str {
        "Uint8Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Uint8Type
    }
}

impl NonParametricTermTrait for Uint16Type {
    fn identifier() -> &'static str {
        "Uint16Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Uint16Type
    }
}

impl NonParametricTermTrait for Uint32Type {
    fn identifier() -> &'static str {
        "Uint32Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Uint32Type
    }
}

impl NonParametricTermTrait for Uint64Type {
    fn identifier() -> &'static str {
        "Uint64Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Uint64Type
    }
}

impl Stringify for Sint8Type {
    fn stringify(&self) -> String {
        "Sint8Type".into()
    }
}

impl Stringify for Sint16Type {
    fn stringify(&self) -> String {
        "Sint16Type".into()
    }
}

impl Stringify for Sint32Type {
    fn stringify(&self) -> String {
        "Sint32Type".into()
    }
}

impl Stringify for Sint64Type {
    fn stringify(&self) -> String {
        "Sint64Type".into()
    }
}

impl Stringify for Uint8Type {
    fn stringify(&self) -> String {
        "Uint8Type".into()
    }
}

impl Stringify for Uint16Type {
    fn stringify(&self) -> String {
        "Uint16Type".into()
    }
}

impl Stringify for Uint32Type {
    fn stringify(&self) -> String {
        "Uint32Type".into()
    }
}

impl Stringify for Uint64Type {
    fn stringify(&self) -> String {
        "Uint64Type".into()
    }
}
