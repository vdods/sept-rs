use crate::{
    dy,
    st::{self, Stringifiable},
    Result,
};

/// This is a bit of an awkward name, but if Struct is the constructor for particular structs
/// (i.e. StructTerm), then the terms inhabiting StructTerm are instances of particular structs,
/// and should be called StructTermTerm by this terminology.
// TODO: Figure out how to do this more efficiently, e.g. not having a full copy of r#type (which
// is really just the symbol_id of the StructTerm), and instead have a direct reference to the
// StructTerm itself.
#[derive(Clone, Debug, dy::IntoValue, PartialEq)]
pub struct StructTermTerm {
    /// An instance of StructTermTerm necessarily has a defined type, which is an instance of
    /// StructTerm.  Typically that would be declared in a symbol table (probably the global symbol
    /// table) so that a full copy of the StructTerm instance isn't kept with each instance of
    /// StructTermTerm, and r#type would be a kind of ref, such as GlobalSymRefTerm, LocalSymRefTerm,
    /// or eventually MemRefTerm.
    // TODO: Use type-specifying GlobalSymRefTerm when possible
    // TODO: Maybe this will eventually use a direct reference to the StructTerm instance itself
    // via some ref counted construction, i.e. a [reference-counted] MemRef; this would require
    // a "linker" step upon resolution.
    r#type: dy::Value,
    // This is the ordered sequence of element values.
    pub(crate) field_t: dy::TupleTerm,
}

/// StructTermTerm's canonical implementation of Deconstruct could not be simpler.
impl dy::Deconstruct for StructTermTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(self.r#type, self.field_t).into()
    }
}

// TODO: Impls for st::Inhabits<dy::GlobalSymRefTerm> and st::Inhabits<dy::LocalSymRefTerm>

impl st::Inhabits<dy::StructTerm> for StructTermTerm {
    fn inhabits(&self, rhs: &dy::StructTerm) -> bool {
        rhs.is_inhabited_by(&self.field_t)
    }
}

// This implementation is necessary because StructTermTerm::AbstractTypeType is dy::Value.
impl st::Inhabits<dy::Value> for StructTermTerm {
    fn inhabits(&self, rhs: &dy::Value) -> bool {
        dy::RUNTIME_LA.read().unwrap().inhabits(self, rhs.as_ref())
    }
}

impl StructTermTerm {
    pub fn new_unchecked(r#type: dy::Value, field_t: dy::TupleTerm) -> Self {
        Self { r#type, field_t }
    }
    pub fn new_checked(r#type: dy::Value, field_t: dy::TupleTerm) -> Result<Self> {
        let type_maybe_dereferenced = r#type.dereferenced()?;
        match type_maybe_dereferenced {
            dy::MaybeDereferencedValue::NonRef(type_value_guts) => {
                match type_value_guts.downcast_ref::<dy::StructTerm>() {
                    Some(struct_term) => {
                        struct_term.verify_inhabitation_by(&field_t)?;
                    }
                    None => {
                        anyhow::bail!("can't construct a StructTermTerm with type that isn't a StructTerm; type resolved to {}", dy::RUNTIME_LA.read().unwrap().label_of_type_id(type_value_guts.type_id()));
                    }
                }
            }
            dy::MaybeDereferencedValue::Ref(type_value_la) => {
                let type_value_g = type_value_la.read().unwrap();
                match type_value_g.downcast_ref::<dy::StructTerm>() {
                    Some(struct_term) => {
                        struct_term.verify_inhabitation_by(&field_t)?;
                    }
                    None => {
                        anyhow::bail!("can't construct a StructTermTerm with type that isn't a StructTerm; type resolved to {}", dy::RUNTIME_LA.read().unwrap().label_of_type_id(type_value_g.type_id()));
                    }
                }
            }
        };
        Ok(Self {
            r#type: r#type,
            field_t,
        })
    }
    // TODO: Come up with a better name for this.
    pub fn direct_type(&self) -> &dy::Value {
        &self.r#type
    }
    pub fn direct_type_mut(&mut self) -> &mut dy::Value {
        &mut self.r#type
    }
    pub fn field_tuple(&self) -> &dy::TupleTerm {
        &self.field_t
    }
    pub fn field_tuple_mut(&mut self) -> &mut dy::TupleTerm {
        &mut self.field_t
    }
}

impl std::fmt::Display for StructTermTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.stringify())
    }
}

impl st::Deserializable for StructTermTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let type_ = dy::Value::deserialize(reader)?;
        let field_t = dy::TupleTerm::deserialize(reader)?;
        // NOTE: This check might cause dereferences (e.g. in GlobalSymRefTerm) which may fail to
        // resolve if they're not defined yet (e.g. in a mutually nested struct).
        Ok(Self::new_checked(type_, field_t)?)
    }
}

impl st::Serializable for StructTermTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // This is a bit redundant, in that the case of serializing dy::Value::from(struct_term_term),
        // it serializes the type twice, but it drastically simplifies the logic.
        let mut bytes_written = self.r#type.serialize(writer)?;
        bytes_written += self.field_t.serialize(writer)?;
        Ok(bytes_written)
    }
}

// /// Note that StructTermTerm doesn't implement st::Deserializable, because its constructor is
// /// a parametric type (StructTerm).  Instead, it's deserialized using StructTerm's impl of
// /// dy::Constructor::deserialize_parameters_and_construct.
// impl st::Serializable for StructTermTerm {
// //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
// //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
// //     }
// //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
// //         Ok(self.type_.serialize(writer)?)
// //     }
//     fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         // NOTE: The assumption "the construction is known when calling serialize" is a bit tricky;
//         // it's known that ultimately the constructor is [resolves to] an instance of StructTerm,
//         // but the exact value of that instance of StructTerm (i.e. the type info of the fields
//         // in this StructTermTerm) is necessary in order to fully understand the serialization here.
//
//         // It's assumed that the constructor is known, which carries the field type information,
//         // so only field_t needs to be serialized.
//         Ok(self.field_t.serialize(writer)?)
//     }
// }

impl st::Stringifiable for StructTermTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        // NOTE: This doesn't guarantee any of:
        // -    self.r#type.symbol_id is a C-style (i.e. Rust-style) identifier
        // -    self.r#type.symbol_id doesn't collide with the other type names like "Array"
        s.push_str(&format!("{}(", self.r#type.stringify()));
        for (i, element) in self.field_t.iter().enumerate() {
            s.push_str(&element.stringify());
            if i + 1 < self.field_t.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl st::TermTrait for StructTermTerm {
    type AbstractTypeType = dy::Value;

    /// Because of the dynamic nature of StructTermTerm (along with other implementation choices
    /// regarding dy::Value and dy::Runtime), even an empty StructTerm has to be considered parametric,
    /// because its type is represented by a generic dy::Value.
    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        self.r#type.clone()
    }
}

impl st::TestValues for StructTermTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec![
            // Empty struct
            Self::new_checked(
                dy::StructTerm::new(vec![]).unwrap().into(),
                dy::TupleTerm::from(vec![]),
            )
            .unwrap(),
            // Struct with single attribute
            Self::new_checked(
                dy::StructTerm::new(vec![("x".to_string(), st::Float64.into())])
                    .unwrap()
                    .into(),
                dy::TupleTerm::from(vec![4.01f64.into()]),
            )
            .unwrap(),
            // Struct with two attributes
            Self::new_checked(
                dy::StructTerm::new(vec![
                    ("x".to_string(), st::Float64.into()),
                    ("Y".to_string(), st::Array.into()),
                ])
                .unwrap()
                .into(),
                dy::TupleTerm::from(vec![
                    4.01f64.into(),
                    dy::ArrayTerm::from(vec![true.into(), 123u32.into()]).into(),
                ]),
            )
            .unwrap(),
            // Nested struct -- TODO: Clearly we need some macros to make assembling these things
            // more ergonomic.
            {
                let inner_struct_term = dy::StructTerm::new(vec![
                    ("x".to_string(), st::Float64.into()),
                    ("Y".to_string(), st::Array.into()),
                ])
                .unwrap();
                Self::new_checked(
                    dy::StructTerm::new(vec![
                        ("b".to_string(), st::Bool.into()),
                        ("t".to_string(), st::TrueType.into()),
                        ("g".to_string(), inner_struct_term.clone().into()),
                    ])
                    .unwrap()
                    .into(),
                    dy::TupleTerm::from(vec![
                        false.into(),
                        st::True.into(),
                        Self::new_checked(
                            inner_struct_term.into(),
                            dy::TupleTerm::from(vec![
                                4.01f64.into(),
                                dy::ArrayTerm::from(vec![true.into(), 123u32.into()]).into(),
                            ]),
                        )
                        .unwrap()
                        .into(),
                    ]),
                )
                .unwrap()
                .into()
            },
        ]
    }
}
