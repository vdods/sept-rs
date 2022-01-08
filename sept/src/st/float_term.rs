use crate::{dy, st::{Float32, Float64, Inhabits, Stringify, TermTrait}};

impl dy::IntoValue for f32 {}
impl dy::IntoValue for f64 {}

impl Inhabits<Float32> for f32 {
    fn inhabits(&self, _: &Float32) -> bool {
        true
    }
}

impl Inhabits<Float64> for f64 {
    fn inhabits(&self, _: &Float64) -> bool {
        true
    }
}

impl Stringify for f32 {
    fn stringify(&self) -> String {
        format!("Float32({})", self)
    }
}

impl Stringify for f64 {
    fn stringify(&self) -> String {
        format!("Float64({})", self)
    }
}

impl TermTrait for f32 {
    type AbstractTypeType = Float32;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for f64 {
    type AbstractTypeType = Float64;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}
