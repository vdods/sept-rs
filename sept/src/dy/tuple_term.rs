use crate::{
    dy::{self, Value},
    st::{self, Inhabits, Stringifiable, TermTrait, Tuple},
    Result,
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
        if self.len() != rhs.field_decl_v.len() {
            return false;
        }
        for (i, datum) in self.iter().enumerate() {
            if !datum.inhabits(&rhs.field_decl_v[i].1) {
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
