use crate::{dy, st, Result};

/// A NonParametricTermTrait (NonParametricTermTrait) is one that has no "state", i.e. each
/// NonParametricTermTrait is a singleton.  It's recommended to derive this trait using
/// derive(st::NonParametricTermTrait).
pub trait NonParametricTermTrait:
    st::TermTrait + dy::IntoValue + Eq + PartialEq + Clone + Copy
{
    /// The name of this term.
    // TODO: Might need to worry about namespacing later.  For now, this is considered a kind of keyword.
    const IDENTIFIER: &'static str;
    /// The serialization code for this NonParametricTerm.
    const NON_PARAMETRIC_TERM_CODE: st::NonParametricTermCode;
    /// Instantiate this term.  By construction, no parameters are needed.
    // TODO: Is this really necessary?  T::instantiate() returns T.
    fn instantiate() -> Self;
}

impl<N: NonParametricTermTrait> dy::Deconstruct for N {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: Consider making this take self.as_non_parametric_term_code instead.
        dy::NonParametricDeconstruction::from(N::NON_PARAMETRIC_TERM_CODE).into()
    }
}

impl<N: NonParametricTermTrait> st::Deserializable for N {
    fn deserialize(_reader: &mut dyn std::io::Read) -> Result<Self> {
        // A NonParametricTerm has no parameters by definition.  Just instantiate.
        Ok(Self::instantiate())
    }
}

impl<N: NonParametricTermTrait> dy::Queryable for N {
    fn query<'a>(&'a self, address_v: &[dy::Value]) -> Result<&'a dy::ValueGuts> {
        if address_v.is_empty() {
            Ok(self)
        } else {
            unimplemented!("TODO: implement views, if any");
        }
    }
    fn query_mut<'a>(&'a mut self, _address_v: &[dy::Value]) -> Result<&'a mut dy::ValueGuts> {
        unimplemented!("blah");
        // TODO: This should basically be the same as query, though maybe non-l-values (e.g. querying
        // `Len`) wouldn't support this.
    }
}

impl<N: NonParametricTermTrait> dy::QueryableDynTrait for N {
    fn make_query<'a>(&'a self) -> Box<dyn dy::QueryTrait + 'a> {
        dy::GenericView::new(self)
    }
}

impl<N: NonParametricTermTrait> dy::QueryableMutDynTrait for N {
    fn make_query_mut<'a>(&'a mut self) -> Box<dyn dy::QueryMutTrait + 'a> {
        dy::GenericMutView::new(self)
    }
}

impl<N: NonParametricTermTrait> st::Serializable for N {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::NonParametric.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(N::NON_PARAMETRIC_TERM_CODE.write(writer)?)
    //     }
    fn serialize(&self, _writer: &mut dyn std::io::Write) -> Result<usize> {
        // A NonParametricTerm has no parameters by definition.  If its type is known, then its
        // value is known, so nothing has to be serialized.
        Ok(0)
    }
}

impl<N: NonParametricTermTrait> st::Stringifiable for N {
    fn stringify(&self) -> String {
        N::IDENTIFIER.to_string()
    }
}
