use crate::{dy, Result, st::{self, Inhabits, Stringifiable}};
use std::fmt::Debug;

/// This represents the Sint8 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Sint8Type", is_parametric = "false", is_type = "true")]
pub struct Sint8;

/// This represents the Sint16 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Sint16Type", is_parametric = "false", is_type = "true")]
pub struct Sint16;

/// This represents the Sint32 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Sint32Type", is_parametric = "false", is_type = "true")]
pub struct Sint32;

/// This represents the Sint64 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Sint64Type", is_parametric = "false", is_type = "true")]
pub struct Sint64;

/// This represents the Uint8 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Uint8Type", is_parametric = "false", is_type = "true")]
pub struct Uint8;

/// This represents the Uint16 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Uint16Type", is_parametric = "false", is_type = "true")]
pub struct Uint16;

/// This represents the Uint32 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Uint32Type", is_parametric = "false", is_type = "true")]
pub struct Uint32;

/// This represents the Uint64 type itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "st::Uint64Type", is_parametric = "false", is_type = "true")]
pub struct Uint64;

impl dy::Constructor for Sint8 {
    type ConstructedType = i8;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<i8>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Sint8, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Sint16 {
    type ConstructedType = i16;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<i16>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Sint16, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Sint32 {
    type ConstructedType = i32;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<i32>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Sint32, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Sint64 {
    type ConstructedType = i64;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<i64>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Sint64, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Uint8 {
    type ConstructedType = u8;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<u8>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Uint8, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Uint16 {
    type ConstructedType = u16;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<u16>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Uint16, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Uint32 {
    type ConstructedType = u32;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<u32>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Uint32, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::Constructor for Uint64 {
    type ConstructedType = u64;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<u64>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!("{}.construct expected parameter of type Uint64, but got one of type {:?}", self.stringify(), parameter.type_id()))
        }
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl Inhabits<st::Sint8Type> for Sint8 {
    fn inhabits(&self, _: &st::Sint8Type) -> bool {
        true
    }
}

impl Inhabits<st::Sint16Type> for Sint16 {
    fn inhabits(&self, _: &st::Sint16Type) -> bool {
        true
    }
}

impl Inhabits<st::Sint32Type> for Sint32 {
    fn inhabits(&self, _: &st::Sint32Type) -> bool {
        true
    }
}

impl Inhabits<st::Sint64Type> for Sint64 {
    fn inhabits(&self, _: &st::Sint64Type) -> bool {
        true
    }
}

impl Inhabits<st::Uint8Type> for Uint8 {
    fn inhabits(&self, _: &st::Uint8Type) -> bool {
        true
    }
}

impl Inhabits<st::Uint16Type> for Uint16 {
    fn inhabits(&self, _: &st::Uint16Type) -> bool {
        true
    }
}

impl Inhabits<st::Uint32Type> for Uint32 {
    fn inhabits(&self, _: &st::Uint32Type) -> bool {
        true
    }
}

impl Inhabits<st::Uint64Type> for Uint64 {
    fn inhabits(&self, _: &st::Uint64Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Sint8 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Sint16 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Sint32 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Sint64 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Uint8 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Uint16 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Uint32 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Uint64 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
