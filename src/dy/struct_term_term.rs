use crate::{dy, st::{self, Stringify}};

/// This is a bit of an awkward name, but if Struct is the constructor for particular structs
/// (i.e. StructTerm), then the terms inhabiting StructTerm are instances of particular structs,
/// and should be called StructTermTerm by this terminology.
// TODO: Figure out how to do this more efficiently, e.g. not having a full copy of type_ (which
// is really just the symbol_id of the StructTerm), and instead have a direct reference to the
// StructTerm itself.
#[derive(Debug, PartialEq)]
pub struct StructTermTerm {
    /// A StructTerm necessarily has a name and is declared in the global SymbolTable.
    // TODO: Probably should use dy::Value here, since a dy::Value could be a GlobalSymRefTerm,
    // or a StructTerm value, or a LocalSymRefTerm (when that exists), etc.
    // TODO: Use type-specifying GlobalSymRefTerm when possible
    // TODO: Maybe this will eventually a direct reference to the StructTerm instance itself
    // via some ref counted construction.
    type_: dy::GlobalSymRefTerm,
    // This is the ordered sequence of element values.
    element_tuple_term: dy::TupleTerm,
}

impl dy::IntoValue for StructTermTerm {}

impl StructTermTerm {
    // NOTE/TODO: This currently checks the type inhabitation of element_tuple_term with type_, but there will
    // probably eventually be cases where it'll be necessary to construct a StructTermTerm before
    // its StructTerm type_ exists, and therefore the type can't be checked.  On the other hand,
    // maybe not.
    pub fn new(type_: dy::GlobalSymRefTerm, element_tuple_term: dy::TupleTerm) -> anyhow::Result<Self> {
        // Verify type inhabitation.
        // TODO: Use GlobalSymRefTermReadLock
        dy::RUNTIME.read().unwrap()
            .global_symbol_table
            .resolve_symbol(&type_.symbol_id)?
            .downcast_ref::<dy::StructTerm>()
            .expect("expected StructTerm")
            .verify_inhabitation_by(&element_tuple_term)?;
        Ok(Self { type_, element_tuple_term })
    }
}

impl std::fmt::Display for StructTermTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.stringify())
    }
}

impl st::Inhabits<dy::StructTerm> for StructTermTerm {
    fn inhabits(&self, struct_term: &dy::StructTerm) -> bool {
        struct_term.verify_inhabitation_by(&self.element_tuple_term).is_ok()
    }
}

impl st::Stringify for StructTermTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        // NOTE: This doesn't guarantee any of:
        // -    self.type_.symbol_id is a C-style (i.e. Rust-style) identifier
        // -    self.type_.symbol_id doesn't collide with the other type names like "Array"
        s.push_str(&format!("{}(", self.type_.symbol_id));
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
    type AbstractTypeFnReturnType = dy::GlobalSymRefTerm;

    fn label() -> &'static str {
        "StructTermTerm"
    }
    /// A StructTermTerm term is parametric if there is at least one element.
    fn is_parametric_term(&self) -> bool {
        self.element_tuple_term.len() > 0
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        self.type_.clone()
    }
}
