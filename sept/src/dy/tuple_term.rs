use crate::{
    dy::{self, Value},
    qv,
    st::{self, Inhabits, Stringifiable, TermTrait, Tuple},
    Error, Result,
};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(
    derive_more::AsRef,
    Clone,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::Into,
    dy::IntoValue,
    PartialEq,
    st::TypeTrait,
)]
pub struct TupleTerm(Vec<Value>);

impl dy::Constructor for TupleTerm {
    type ConstructedType = TupleTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(
            parameter_t.len() == self.len(),
            "{}.construct expected {} parameter(s), got {}",
            self.stringify(),
            self.len(),
            parameter_t.len()
        );
        // TODO: Use zip iterator when available.
        for i in 0..self.len() {
            anyhow::ensure!(
                parameter_t[i].inhabits(&self[i]),
                "{}.construct expected {}th parameter (which was {}) to inhabit {}, but it did not",
                self.stringify(),
                i,
                parameter_t[i],
                self[i]
            );
        }
        // Passed type check, now can use the parameter_t tuple directly.
        Ok(parameter_t)
    }
    fn deserialize_parameters_and_construct(
        &self,
        _reader: &mut dyn std::io::Read,
    ) -> Result<Self> {
        unimplemented!("todo");
        //         // Each element is the constructor for each element in the parameters.
        //         for (i, element) in self.iter().enumerate() {
        //             element.deserialize_parameters_and_construct(reader
        //         }
    }
}

impl dy::Deconstruct for TupleTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        // This looks like it might incur infinite recursion, but it won't.
        dy::ParametricDeconstruction::new_recursive(st::Tuple.into(), self).into()
    }
}

impl std::fmt::Display for TupleTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Tuple(")?;
        for (i, element) in self.iter().enumerate() {
            write!(f, "{}", element)?;
            if i + 1 < self.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

// Hacky way to get Rust-syntax-tuple-valued constructors for a tuples of length 0-6.
// TODO: Maybe implement a macro to handle these.

impl From<()> for TupleTerm {
    fn from(_: ()) -> Self {
        vec![].into()
    }
}

impl<T0> From<(T0,)> for TupleTerm
where
    T0: TermTrait + Into<Value>,
{
    fn from(t: (T0,)) -> Self {
        vec![t.0.into()].into()
    }
}

impl<T0, T1> From<(T0, T1)> for TupleTerm
where
    T0: TermTrait + Into<Value>,
    T1: TermTrait + Into<Value>,
{
    fn from(t: (T0, T1)) -> Self {
        vec![t.0.into(), t.1.into()].into()
    }
}

impl<T0, T1, T2> From<(T0, T1, T2)> for TupleTerm
where
    T0: TermTrait + Into<Value>,
    T1: TermTrait + Into<Value>,
    T2: TermTrait + Into<Value>,
{
    fn from(t: (T0, T1, T2)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into()].into()
    }
}

impl<T0, T1, T2, T3> From<(T0, T1, T2, T3)> for TupleTerm
where
    T0: TermTrait + Into<Value>,
    T1: TermTrait + Into<Value>,
    T2: TermTrait + Into<Value>,
    T3: TermTrait + Into<Value>,
{
    fn from(t: (T0, T1, T2, T3)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into(), t.3.into()].into()
    }
}

impl<T0, T1, T2, T3, T4> From<(T0, T1, T2, T3, T4)> for TupleTerm
where
    T0: TermTrait + Into<Value>,
    T1: TermTrait + Into<Value>,
    T2: TermTrait + Into<Value>,
    T3: TermTrait + Into<Value>,
    T4: TermTrait + Into<Value>,
{
    fn from(t: (T0, T1, T2, T3, T4)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into(), t.3.into(), t.4.into()].into()
    }
}

impl<T0, T1, T2, T3, T4, T5> From<(T0, T1, T2, T3, T4, T5)> for TupleTerm
where
    T0: TermTrait + Into<Value>,
    T1: TermTrait + Into<Value>,
    T2: TermTrait + Into<Value>,
    T3: TermTrait + Into<Value>,
    T4: TermTrait + Into<Value>,
    T5: TermTrait + Into<Value>,
{
    fn from(t: (T0, T1, T2, T3, T4, T5)) -> Self {
        vec![
            t.0.into(),
            t.1.into(),
            t.2.into(),
            t.3.into(),
            t.4.into(),
            t.5.into(),
        ]
        .into()
    }
}

impl Inhabits<Tuple> for TupleTerm {
    fn inhabits(&self, _: &Tuple) -> bool {
        true
    }
}

impl Inhabits<TupleTerm> for TupleTerm {
    fn inhabits(&self, rhs: &TupleTerm) -> bool {
        if rhs.0.len() != self.len() {
            return false;
        }
        // TODO: Use std::iter::zip here when it's stable
        for i in 0..self.len() {
            if !self[i].inhabits(&rhs.0[i]) {
                return false;
            }
        }
        true
    }
}

impl st::Inhabits<st::Type> for TupleTerm {
    /// A TupleTerm is a type only if each of its elements are types.
    fn inhabits(&self, t: &st::Type) -> bool {
        for tuple_term_element in self.iter() {
            if !tuple_term_element.inhabits(t) {
                return false;
            }
        }
        true
    }
}

// Because a StructTerm is effectively an (ordered) tuple of types, TupleTerm can naturally inhabit StructTerm.
impl Inhabits<dy::StructTerm> for TupleTerm {
    fn inhabits(&self, rhs: &dy::StructTerm) -> bool {
        if self.len() != rhs.len() {
            return false;
        }
        for (i, datum) in self.iter().enumerate() {
            if !datum.inhabits(&rhs[i].1) {
                return false;
            }
        }
        true
    }
}

impl st::Deserializable for TupleTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let len = st::read_len(reader)?;
        let mut element_v = Vec::with_capacity(len);
        for _ in 0..len {
            element_v.push(dy::Value::deserialize(reader)?);
        }
        Ok(Self::from(element_v))
    }
}

impl qv::QueryableDynTrait for TupleTerm {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryTrait + 'a> {
        Box::new(qv::TupleTermView::new(self))
    }
}

impl qv::EvalTrait for TupleTerm {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self))
    }
}

impl qv::ApplyEditTrait for TupleTerm {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: "clear" edit
        if edit.is::<qv::ReplacementTerm>() {
            let edit = edit.downcast_into::<qv::ReplacementTerm>();
            anyhow::ensure!(
                edit.old_data.is::<dy::TupleTerm>(),
                "TupleTerm ReplacementTerm edit expected old_data to be TupleTerm"
            );
            anyhow::ensure!(
                edit.new_data.is::<dy::TupleTerm>(),
                "TupleTerm ReplacementTerm edit expected new_data to be TupleTerm"
            );
            let old_tuple_term = edit.old_data.downcast_into::<dy::TupleTerm>();
            let new_tuple_term = edit.new_data.downcast_into::<dy::TupleTerm>();
            anyhow::ensure!(*self == old_tuple_term, "TupleTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self, old_tuple_term);
            *self = new_tuple_term;
        } else {
            anyhow::bail!("TupleTerm only supports ReplacementTerm edits")
        }
        Ok(())
    }
}

impl st::Serializable for TupleTerm {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Tuple.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        use dy::Deconstruct;
        log::debug!("TupleTerm::serialize(); self: {}", self.textified());
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where a tuple smaller than 8 bytes is encoded in exactly 8 bytes.
        let mut bytes_written = st::write_len(self.len(), writer)?;
        for element in self.iter() {
            //             use dy::Deconstruct;
            log::debug!(
                "TupleTerm::serialize(); element.type_id(): {:?}; element: {}",
                element.type_id(),
                element.textified()
            );
            bytes_written += element.serialize(writer)?;
        }
        Ok(bytes_written)
    }
}

impl qv::SingleQuery<dy::Value> for TupleTerm {
    type ReturnType<'a> = TupleTermQuery<'a>;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(elem_index) = address_token.downcast_ref::<u32>() {
            Ok(qv::TupleTermElemView::new(self, *elem_index as usize)?.into())
        } else {
            anyhow::bail!(
                "TupleTerm::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

impl qv::SingleQueryMut<dy::Value> for TupleTerm {
    type ReturnType<'a> = TupleTermQueryMut<'a>;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(elem_index) = address_token.downcast_ref::<u32>() {
            Ok(qv::TupleTermElemMutView::new(self, *elem_index as usize)?.into())
        } else {
            anyhow::bail!(
                "TupleTerm::run_single_query_mut; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

impl Stringifiable for TupleTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Tuple(");
        for (i, element) in self.iter().enumerate() {
            s.push_str(&element.stringify());
            if i + 1 < self.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl TermTrait for TupleTerm {
    type AbstractTypeType = TupleTerm;

    /// Because of the dynamic nature of TupleTerm (along with other implementation choices
    /// regarding dy::Value and dy::Runtime), even an empty TupleTerm has to be considered
    /// parametric, because its type is represented by a generic dy::Value.
    fn is_parametric(&self) -> bool {
        true
    }
    /// A Tuple term is a type if all of its elements are types.
    fn is_type(&self) -> bool {
        self.iter().all(|element| element.is_type())
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        let mut type_element_v = Vec::new();
        for self_element in self.iter() {
            type_element_v.push(self_element.abstract_type());
        }
        TupleTerm(type_element_v)
    }
}

impl TupleTerm {
    pub fn into_inner(self) -> Vec<Value> {
        self.0
    }
}

/// Let A and B be sequences.  A <= B is defined by A being a prefix of B, i.e. len(A) <= len(B) and
/// A[i] == B[i] for i in 0..len(A).  Obviously A == B if their elements are the same.  A is not comparable with B
/// if neither is a prefix of the other, i.e. if there is some j such that A[j] != B[j].
// TODO: Formalize this somehow if/when appropriate, this random function isn't actually part of the sept data model.
pub fn prefix_partial_cmp(lhs: &[dy::Value], rhs: &[dy::Value]) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let (shorter, longer, reversed) = match lhs.len().cmp(&rhs.len()) {
        Less => (lhs, rhs, false),
        Equal => (lhs, rhs, false),
        Greater => (rhs, lhs, true),
    };
    for i in 0..shorter.len() {
        if shorter[i] != longer[i] {
            // If they differ in any element, they're not comparable.
            return None;
        }
    }
    // If it made it this far, they are equal in all mutually-indexed elements.
    if shorter.len() == longer.len() {
        // If they also have the same length, then they're simply equal.
        Some(Equal)
    } else {
        // Otherwise, one is less than the other.
        if reversed {
            // Have to account for if lhs and rhs were reversed.
            Some(Greater)
        } else {
            Some(Less)
        }
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum TupleTermQuery<'a> {
    TupleTermElemView(qv::TupleTermElemView<'a>),
}

// TODO: Derive
impl<'b> qv::EvalTrait for TupleTermQuery<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        match self {
            Self::TupleTermElemView(v) => v.eval(),
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum TupleTermQueryMut<'a> {
    TupleTermElemMutView(qv::TupleTermElemMutView<'a>),
}

// TODO: Derive this
impl<'b> qv::QueryMutAndApplyEditTrait for TupleTermQueryMut<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::TupleTermElemMutView(v) => v.query_mut_and_apply_edit(address_token_i, edit),
        }
    }
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditTrait for TupleTermQueryMut<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::TupleTermElemMutView(v) => v.apply_edit(edit),
        }
    }
}
