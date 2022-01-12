use crate::{dy, st::{self, Stringify}};

/// This is a bit of an awkward name, but if Struct is the constructor for particular structs
/// (i.e. StructTerm), then the terms inhabiting StructTerm are instances of particular structs,
/// and should be called StructTermTerm by this terminology.
// TODO: Figure out how to do this more efficiently, e.g. not having a full copy of type_ (which
// is really just the symbol_id of the StructTerm), and instead have a direct reference to the
// StructTerm itself.
#[derive(Clone, Debug, dy::IntoValue, PartialEq)]
pub struct StructTermTerm {
    /// A StructTerm necessarily has a defined type.  Typically that would be declared in a symbol
    /// table (probably the global symbol table), and type_ would be a GlobalSymRefTerm.
    // TODO: Use type-specifying GlobalSymRefTerm when possible
    // TODO: Maybe this will eventually use a direct reference to the StructTerm instance itself
    // via some ref counted construction, i.e. a [reference-counted] MemRef
    type_: dy::Value,
    // This is the ordered sequence of element values.
    pub(crate) element_tuple_term: dy::TupleTerm,
}

/// StructTermTerm's canonical implementation of Deconstruct could not be simpler.
impl dy::Deconstruct for StructTermTerm {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: self.type_,
            parameters: self.element_tuple_term,
        }.into()
    }
}

impl st::Inhabits<dy::StructTerm> for StructTermTerm {
    fn inhabits(&self, rhs: &dy::StructTerm) -> bool {
        rhs.is_inhabited_by(&self.element_tuple_term)
    }
}

// This implementation is necessary because StructTermTerm::AbstractTypeType is dy::Value.
impl st::Inhabits<dy::Value> for StructTermTerm {
    fn inhabits(&self, rhs: &dy::Value) -> bool {
        dy::RUNTIME_LA.read().unwrap().inhabits(self, rhs.as_ref())
    }
}

impl StructTermTerm {
    pub fn new_unchecked(type_: dy::Value, element_tuple_term: dy::TupleTerm) -> anyhow::Result<Self> {
        Ok(Self { type_, element_tuple_term })
    }
    pub fn new_checked(type_: dy::Value, element_tuple_term: dy::TupleTerm) -> anyhow::Result<Self> {
        let type_maybe_dereferenced = type_.dereferenced()?;
        match type_maybe_dereferenced {
            dy::MaybeDereferencedValue::NonRef(type_value_guts) => {
                match type_value_guts.downcast_ref::<dy::StructTerm>() {
                    Some(struct_term) => { struct_term.verify_inhabitation_by(&element_tuple_term)?; },
                    None => { anyhow::bail!("can't construct a StructTermTerm with type_ that isn't a StructTerm; type_ resolved to {}", dy::RUNTIME_LA.read().unwrap().label_of(type_value_guts.type_id())); }
                }
            },
            dy::MaybeDereferencedValue::Ref(type_value_la) => {
                let type_value_g = type_value_la.read().unwrap();
                match type_value_g.downcast_ref::<dy::StructTerm>() {
                    Some(struct_term) => { struct_term.verify_inhabitation_by(&element_tuple_term)?; },
                    None => { anyhow::bail!("can't construct a StructTermTerm with type_ that isn't a StructTerm; type_ resolved to {}", dy::RUNTIME_LA.read().unwrap().label_of(type_value_g.type_id())); }
                }
            }
        };
        Ok(Self { type_, element_tuple_term })
    }
}

impl std::fmt::Display for StructTermTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.stringify())
    }
}

impl st::Stringify for StructTermTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        // NOTE: This doesn't guarantee any of:
        // -    self.type_.symbol_id is a C-style (i.e. Rust-style) identifier
        // -    self.type_.symbol_id doesn't collide with the other type names like "Array"
        s.push_str(&format!("{}(", self.type_.stringify()));
        for (i, element) in self.element_tuple_term.iter().enumerate() {
            s.push_str(&element.stringify());
            if i+1 < self.element_tuple_term.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl st::TermTrait for StructTermTerm {
    type AbstractTypeType = dy::Value;

    /// A StructTermTerm instance is parametric if there is at least one element.
    fn is_parametric(&self) -> bool {
        self.element_tuple_term.len() > 0
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        self.type_.clone()
    }
}
