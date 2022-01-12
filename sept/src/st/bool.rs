use crate::{dy::{self, DynNPTerm}, st::{self, BoolType, NonParametricTermTrait, Inhabits, Stringify}};

/// This represents the Bool type itself, not a boolean value such as true or false.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "BoolType", is_parametric = "false", is_type = "true")]
pub struct Bool;

impl dy::Constructor for Bool {
    type ConstructedType = bool;
    fn construct(&self, parameters: dy::TupleTerm) -> anyhow::Result<Self::ConstructedType> {
        anyhow::ensure!(parameters.len() == 1, "{} expected 1 parameter, got {}", self.stringify(), parameters.len());
        let mut parameter_v: Vec<dy::Value> = parameters.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<bool>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{} expected parameter of type String, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
}

impl dy::Deconstruct for Bool {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Value::from(self).into()
    }
}

impl Inhabits<BoolType> for Bool {
    fn inhabits(&self, _: &BoolType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Bool {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Bool
    }
}

impl Stringify for Bool {
    fn stringify(&self) -> String {
        "Bool".into()
    }
}
