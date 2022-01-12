use crate::{dy::{self, DynNPTerm}, st::{self, Utf8StringType, NonParametricTermTrait, Inhabits, Stringify}};

/// This represents the Utf8String type itself, not a boolean value such as true or false.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Utf8StringType", is_parametric = "false", is_type = "true")]
pub struct Utf8String;

impl dy::Constructor for Utf8String {
    type ConstructedType = String;
    fn construct(&self, parameters: dy::TupleTerm) -> anyhow::Result<Self::ConstructedType> {
        anyhow::ensure!(parameters.len() == 1, "{} expected 1 parameter, got {}", self.stringify(), parameters.len());
        let mut parameter_v: Vec<dy::Value> = parameters.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<String>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{} expected parameter of type String, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
}

impl dy::Deconstruct for Utf8String {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Value::from(self).into()
    }
}

impl Inhabits<Utf8StringType> for Utf8String {
    fn inhabits(&self, _: &Utf8StringType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Utf8String {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Utf8String
    }
}

impl Stringify for Utf8String {
    fn stringify(&self) -> String {
        "Utf8String".into()
    }
}
