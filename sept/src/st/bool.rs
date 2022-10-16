use crate::{dy, Result, st::{self, BoolType, NonParametricTermTrait, Inhabits, Stringify}};

/// This represents the Bool type itself, not a boolean value such as true or false.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "BoolType", is_parametric = "false", is_type = "true")]
pub struct Bool;

impl dy::Constructor for Bool {
    type ConstructedType = bool;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<bool>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Bool, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
}

impl Inhabits<BoolType> for Bool {
    fn inhabits(&self, _: &BoolType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Bool {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for Bool {
    fn identifier() -> &'static str {
        "Bool"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Bool
    }
}

impl Stringify for Bool {
    fn stringify(&self) -> String {
        "Bool".into()
    }
}
