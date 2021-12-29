use crate::st::{Float32, Float64, Inhabits, Stringify, TermTrait};

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
    type AbstractTypeFnReturnType = Float32;

    fn label() -> &'static str {
        "f32"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for f64 {
    type AbstractTypeFnReturnType = Float64;

    fn label() -> &'static str {
        "f64"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}
