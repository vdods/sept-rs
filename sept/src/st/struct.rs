use crate::{dy, Result, st::{self, Inhabits, Stringify, StructType}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "StructType", is_parametric = "false", is_type = "true")]
pub struct Struct;

impl dy::Constructor for Struct {
    type ConstructedType = dy::StructTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
//         // TEMP HACK: This will be a real type eventually.
//         let field_decl_type = dy::TupleTerm::from((st::Utf8String, st::Type));
        // Verify that the parameters are correctly typed, and form the field_decl_v.
        let field_decl_v/*: Vec<(String, dy::Value)>*/ = parameter_t.into_inner().into_iter().enumerate().map(
            |(i, mut field_decl): (usize, dy::Value)| -> Result<(String, dy::Value)> {
                // TODO: Figure out how to implement `inhabits` for Rust tuples.
//                 anyhow::ensure!(field_decl.inhabits(&field_decl_type), "{}th field_decl of Struct (which was {:?})) was not of type {:?}", i, field_decl, field_decl_type);
//                 let mut field_decl_content_v: Vec<_> = field_decl.downcast_into::<dy::TupleTerm>().into();
//                 assert_eq!(field_decl_content_v.len(), 2);
//                 let field_type = field_decl_content_v.pop().unwrap();
//                 let field_name = field_decl_content_v.pop().unwrap();
//                 Ok((field_name.downcast_into::<String>(), field_type))
                // TEMP HACK implementation of inhabitation check of field_decl in field_decl_type.
                match field_decl.downcast_mut::<dy::TupleTerm>() {
                    Some(field_decl_t) => {
                        anyhow::ensure!(field_decl_t.len() == 2, "Error in {}th field decl of struct: expected field_decl_t to have 2 elements", i);
                        anyhow::ensure!(field_decl_t[0].inhabits(&st::Utf8String), "Error in {}th field decl of struct: expected field_decl_t[0] (which was {:?}) to inhabit Utf8String", i, field_decl_t[0]);
                        anyhow::ensure!(field_decl_t[1].inhabits(&st::Type), "Error in {}th field decl of struct: expected field_decl_t[1] (which was {:?}) to inhabit Type", i, field_decl_t[1]);
                        let field_type = field_decl_t.pop().unwrap();
                        let field_name = field_decl_t.pop().unwrap();
                        Ok((field_name.downcast_into::<String>(), field_type))
                    }
                    None => { anyhow::bail!("Error in {}th field decl of struct: expected field_decl (which was {:?}) to be TupleTerm", i, field_decl); }
                }
            }
        ).collect::<Result<Vec<(String, dy::Value)>>>()?;
        Ok(dy::StructTerm::new(field_decl_v)?)
    }
}

impl Inhabits<StructType> for Struct {
    fn inhabits(&self, _: &StructType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Struct {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl Stringify for Struct {
    fn stringify(&self) -> String {
        "Struct".into()
    }
}
