use crate::{dy, Result, st::{self, GlobalSymRefType, Inhabits, NonParametricTermTrait, Stringify}};
use anyhow::Context;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "GlobalSymRefType", is_parametric = "false", is_type = "true")]
pub struct GlobalSymRef;

impl dy::Constructor for GlobalSymRef {
    type ConstructedType = dy::GlobalSymRefTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(parameter_t.len() == 1, "{}.construct expected 1 parameter, got {}", self.stringify(), parameter_t.len());
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        let symbol_id = match parameter.downcast_mut::<String>() {
            Some(string) => std::mem::take(string),
            None => { anyhow::bail!("{}.construct expected parameter of type Utf8String, but got one of type {:?}", self.stringify(), parameter.type_id()); }
        };

        // Check that the symbol resolves here.
        Ok(dy::GlobalSymRefTerm::new_checked(symbol_id)
            .context("GlobalSymRef didn't resolve; it may be that this check shouldn't be done in impl dy::Constructor")?
        )
//         // NOTE: We don't check that the reference resolves here, but this could mean that
//         // undefined-symbol errors leak through serialization and only get detected upon attempting
//         // to resolve the symbol.
//         dy::GlobalSymRef::new_unchecked(symbol_id);
    }
}

impl Inhabits<GlobalSymRefType> for GlobalSymRef {
    fn inhabits(&self, _: &GlobalSymRefType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for GlobalSymRef {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for GlobalSymRef {
    fn identifier() -> &'static str {
        "GlobalSymRef"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::GlobalSymRef
    }
}

impl Stringify for GlobalSymRef {
    fn stringify(&self) -> String {
        "GlobalSymRef".into()
    }
}
