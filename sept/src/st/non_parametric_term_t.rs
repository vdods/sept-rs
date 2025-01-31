use crate::{dy, qv, st, Error, Result};

/// A NonParametricTermT (NonParametricTermT) is one that has no "state", i.e. each
/// NonParametricTermT is a singleton.  It's recommended to derive this trait using
/// derive(st::NonParametricTermT).
pub trait NonParametricTermT: st::TermT + dy::IntoValueT + Eq + PartialEq + Clone + Copy {
    /// The name of this term.
    // TODO: Might need to worry about namespacing later.  For now, this is considered a kind of keyword.
    const IDENTIFIER: &'static str;
    /// The serialization code for this NonParametricTerm.
    const NON_PARAMETRIC_TERM_CODE: st::NonParametricTermCode;
    /// Instantiate this term.  By construction, no parameters are needed.
    // TODO: Is this really necessary?  T::instantiate() returns T.
    fn instantiate() -> Self;
}

impl<N: NonParametricTermT> qv::ApplyEditT for N {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // This is valid only so long as the edits are NoOp and ReplacementTerm (which is itself a no-op).
        qv::generic_apply_edit(self, edit)
    }
}

impl<N: NonParametricTermT> dy::DeconstructT for N {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: Consider making this take self.as_non_parametric_term_code instead.
        dy::NonParametricDeconstruction::from(N::NON_PARAMETRIC_TERM_CODE).into()
    }
}

impl<N: NonParametricTermT> st::DeserializableT for N {
    fn deserialize(_reader: &mut dyn std::io::Read) -> Result<Self> {
        // A NonParametricTerm has no parameters by definition.  Just instantiate.
        Ok(Self::instantiate())
    }
}

impl<N: NonParametricTermT> qv::QueryableDynT for N {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

impl<N: NonParametricTermT> st::SerializableT for N {
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

impl<N: NonParametricTermT> qv::SingleQueryMutT<dy::Value> for N {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("{} does not support queries at this time", Self::IDENTIFIER);
    }
}

impl<N: NonParametricTermT> st::StringifiableT for N {
    fn stringify(&self) -> String {
        N::IDENTIFIER.to_string()
    }
}
