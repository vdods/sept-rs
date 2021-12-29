use crate::{
    Array, ArrayTerm, ArrayType,
    Bool, BoolType, EmptyType, False, FalseType, Float32, Float32Type, Float64, Float64Type,
    Inhabits, Result, Sint8, Sint8Type, Sint16, Sint16Type, Sint32, Sint32Type, Sint64, Sint64Type,
    Stringify, Term, TermTrait, True, TrueType, Type,
    Uint8, Uint8Type, Uint16, Uint16Type, Uint32, Uint32Type, Uint64, Uint64Type,
    Void, VoidType,
};
use std::{any::{Any, TypeId}, collections::{HashMap, HashSet}};

pub type StringifyFn = fn(x: &dyn Any) -> String;
pub type LabelFn = fn() -> &'static str;
pub type AbstractTypeFn = fn(x: &dyn Any) -> Box<dyn Any>;
pub type UnaryPredicate = fn(x: &dyn Any) -> bool;
pub type BinaryPredicate = fn(lhs: &dyn Any, rhs: &dyn Any) -> bool;

struct RegisteredEqualsFn {
    eq_fn: BinaryPredicate,
    is_transposed: bool,
}

/// The sept Runtime is what supports the sept data model; ittracks what types are registered
/// and the various inhabitation and subtyping (and other) relationships.
#[derive(Default)]
pub struct Runtime {
    // TODO: [po]set of types (poset based on which relationship?)
    // TODO: [po]set of terms(?)

    term_s: HashSet<TypeId>,
    // TODO: This is silly, just map to &'static str
    label_fn_m: HashMap<TypeId, LabelFn>,
    stringify_fn_m: HashMap<TypeId, StringifyFn>,
    eq_fn_m: HashMap<(TypeId, TypeId), RegisteredEqualsFn>,
    inhabits_fn_m: HashMap<(TypeId, TypeId), BinaryPredicate>,
    abstract_type_fn_m: HashMap<TypeId, AbstractTypeFn>,
    is_parametric_term_fn_m: HashMap<TypeId, UnaryPredicate>,
    is_type_term_fn_m: HashMap<TypeId, UnaryPredicate>,
    // TODO: subtype of
}

impl Runtime {
    pub fn new() -> Self {
        let mut runtime: Runtime = Default::default();

        // TODO: Figure out how to move these into something like "init" fns
        // in the respective modules

        // TODO: Order these in some sensible way

        runtime.register_label::<Term>().unwrap();
        runtime.register_label::<Type>().unwrap();
        runtime.register_label::<bool>().unwrap();
        runtime.register_label::<Bool>().unwrap();
        runtime.register_label::<BoolType>().unwrap();
        runtime.register_label::<EmptyType>().unwrap();
        runtime.register_label::<False>().unwrap();
        runtime.register_label::<FalseType>().unwrap();
        runtime.register_label::<i8>().unwrap();
        runtime.register_label::<i16>().unwrap();
        runtime.register_label::<i32>().unwrap();
        runtime.register_label::<i64>().unwrap();
        runtime.register_label::<Sint8>().unwrap();
        runtime.register_label::<Sint16>().unwrap();
        runtime.register_label::<Sint32>().unwrap();
        runtime.register_label::<Sint64>().unwrap();
        runtime.register_label::<Sint8Type>().unwrap();
        runtime.register_label::<Sint16Type>().unwrap();
        runtime.register_label::<Sint32Type>().unwrap();
        runtime.register_label::<Sint64Type>().unwrap();
        runtime.register_label::<True>().unwrap();
        runtime.register_label::<TrueType>().unwrap();
        runtime.register_label::<u8>().unwrap();
        runtime.register_label::<u16>().unwrap();
        runtime.register_label::<u32>().unwrap();
        runtime.register_label::<u64>().unwrap();
        runtime.register_label::<f32>().unwrap();
        runtime.register_label::<f64>().unwrap();
        runtime.register_label::<Uint8>().unwrap();
        runtime.register_label::<Uint16>().unwrap();
        runtime.register_label::<Uint32>().unwrap();
        runtime.register_label::<Uint64>().unwrap();
        runtime.register_label::<Uint8Type>().unwrap();
        runtime.register_label::<Uint16Type>().unwrap();
        runtime.register_label::<Uint32Type>().unwrap();
        runtime.register_label::<Uint64Type>().unwrap();
        runtime.register_label::<Float32>().unwrap();
        runtime.register_label::<Float64>().unwrap();
        runtime.register_label::<Float32Type>().unwrap();
        runtime.register_label::<Float64Type>().unwrap();
        runtime.register_label::<Void>().unwrap();
        runtime.register_label::<VoidType>().unwrap();
        runtime.register_label::<ArrayTerm>().unwrap();
        runtime.register_label::<Array>().unwrap();
        runtime.register_label::<ArrayType>().unwrap();

        runtime.register_stringify::<Term>().unwrap();
        runtime.register_stringify::<Type>().unwrap();
        runtime.register_stringify::<bool>().unwrap();
        runtime.register_stringify::<Bool>().unwrap();
        runtime.register_stringify::<BoolType>().unwrap();
        runtime.register_stringify::<EmptyType>().unwrap();
        runtime.register_stringify::<False>().unwrap();
        runtime.register_stringify::<FalseType>().unwrap();
        runtime.register_stringify::<i8>().unwrap();
        runtime.register_stringify::<i16>().unwrap();
        runtime.register_stringify::<i32>().unwrap();
        runtime.register_stringify::<i64>().unwrap();
        runtime.register_stringify::<Sint8>().unwrap();
        runtime.register_stringify::<Sint16>().unwrap();
        runtime.register_stringify::<Sint32>().unwrap();
        runtime.register_stringify::<Sint64>().unwrap();
        runtime.register_stringify::<Sint8Type>().unwrap();
        runtime.register_stringify::<Sint16Type>().unwrap();
        runtime.register_stringify::<Sint32Type>().unwrap();
        runtime.register_stringify::<Sint64Type>().unwrap();
        runtime.register_stringify::<True>().unwrap();
        runtime.register_stringify::<TrueType>().unwrap();
        runtime.register_stringify::<u8>().unwrap();
        runtime.register_stringify::<u16>().unwrap();
        runtime.register_stringify::<u32>().unwrap();
        runtime.register_stringify::<u64>().unwrap();
        runtime.register_stringify::<f32>().unwrap();
        runtime.register_stringify::<f64>().unwrap();
        runtime.register_stringify::<Uint8>().unwrap();
        runtime.register_stringify::<Uint16>().unwrap();
        runtime.register_stringify::<Uint32>().unwrap();
        runtime.register_stringify::<Uint64>().unwrap();
        runtime.register_stringify::<Uint8Type>().unwrap();
        runtime.register_stringify::<Uint16Type>().unwrap();
        runtime.register_stringify::<Uint32Type>().unwrap();
        runtime.register_stringify::<Uint64Type>().unwrap();
        runtime.register_stringify::<Float32>().unwrap();
        runtime.register_stringify::<Float64>().unwrap();
        runtime.register_stringify::<Float32Type>().unwrap();
        runtime.register_stringify::<Float64Type>().unwrap();
        runtime.register_stringify::<Void>().unwrap();
        runtime.register_stringify::<VoidType>().unwrap();
        runtime.register_stringify::<ArrayTerm>().unwrap();
        runtime.register_stringify::<Array>().unwrap();
        runtime.register_stringify::<ArrayType>().unwrap();

        runtime.register_eq_fn::<Term, Term>().unwrap();
        runtime.register_eq_fn::<Type, Type>().unwrap();
        runtime.register_eq_fn::<bool, bool>().unwrap();
        runtime.register_eq_fn::<bool, True>().unwrap();
        runtime.register_eq_fn::<bool, False>().unwrap();
        runtime.register_eq_fn::<Bool, Bool>().unwrap();
        runtime.register_eq_fn::<BoolType, BoolType>().unwrap();
        runtime.register_eq_fn::<False, False>().unwrap();
        runtime.register_eq_fn::<False, True>().unwrap();
        runtime.register_eq_fn::<FalseType, FalseType>().unwrap();
        runtime.register_eq_fn::<i8, i8>().unwrap();
        runtime.register_eq_fn::<i16, i16>().unwrap();
        runtime.register_eq_fn::<i32, i32>().unwrap();
        runtime.register_eq_fn::<i64, i64>().unwrap();
        runtime.register_eq_fn::<Sint8, Sint8>().unwrap();
        runtime.register_eq_fn::<Sint16, Sint16>().unwrap();
        runtime.register_eq_fn::<Sint32, Sint32>().unwrap();
        runtime.register_eq_fn::<Sint64, Sint64>().unwrap();
        runtime.register_eq_fn::<Sint8Type, Sint8Type>().unwrap();
        runtime.register_eq_fn::<Sint16Type, Sint16Type>().unwrap();
        runtime.register_eq_fn::<Sint32Type, Sint32Type>().unwrap();
        runtime.register_eq_fn::<Sint64Type, Sint64Type>().unwrap();
        runtime.register_eq_fn::<True, True>().unwrap();
        runtime.register_eq_fn::<TrueType, TrueType>().unwrap();
        runtime.register_eq_fn::<EmptyType, EmptyType>().unwrap();
        runtime.register_eq_fn::<u8, u8>().unwrap();
        runtime.register_eq_fn::<u16, u16>().unwrap();
        runtime.register_eq_fn::<u32, u32>().unwrap();
        runtime.register_eq_fn::<u64, u64>().unwrap();
        runtime.register_eq_fn::<f32, f32>().unwrap();
        runtime.register_eq_fn::<f64, f64>().unwrap();
        runtime.register_eq_fn::<Uint8, Uint8>().unwrap();
        runtime.register_eq_fn::<Uint16, Uint16>().unwrap();
        runtime.register_eq_fn::<Uint32, Uint32>().unwrap();
        runtime.register_eq_fn::<Uint64, Uint64>().unwrap();
        runtime.register_eq_fn::<Uint8Type, Uint8Type>().unwrap();
        runtime.register_eq_fn::<Uint16Type, Uint16Type>().unwrap();
        runtime.register_eq_fn::<Uint32Type, Uint32Type>().unwrap();
        runtime.register_eq_fn::<Uint64Type, Uint64Type>().unwrap();
        runtime.register_eq_fn::<Float32, Float32>().unwrap();
        runtime.register_eq_fn::<Float64, Float64>().unwrap();
        runtime.register_eq_fn::<Float32Type, Float32Type>().unwrap();
        runtime.register_eq_fn::<Float64Type, Float64Type>().unwrap();
        runtime.register_eq_fn::<Void, Void>().unwrap();
        runtime.register_eq_fn::<VoidType, VoidType>().unwrap();
        runtime.register_eq_fn::<ArrayTerm, ArrayTerm>().unwrap();
        runtime.register_eq_fn::<Array, Array>().unwrap();
        runtime.register_eq_fn::<ArrayType, ArrayType>().unwrap();

        // TODO: Need to somehow make it so that everything inhabits Term
        runtime.register_inhabits_fn::<Type, Type>().unwrap();
        runtime.register_inhabits_fn::<bool, Bool>().unwrap();
        runtime.register_inhabits_fn::<bool, FalseType>().unwrap();
        runtime.register_inhabits_fn::<bool, TrueType>().unwrap();
        runtime.register_inhabits_fn::<False, Bool>().unwrap();
        runtime.register_inhabits_fn::<True, Bool>().unwrap();
        runtime.register_inhabits_fn::<Bool, BoolType>().unwrap();
        runtime.register_inhabits_fn::<Void, VoidType>().unwrap();
        // TODO: Need to be able to register EmptyType's inhabitation function (it returns false for any term arg)
        runtime.register_inhabits_fn::<i8, Sint8>().unwrap();
        runtime.register_inhabits_fn::<i16, Sint16>().unwrap();
        runtime.register_inhabits_fn::<i32, Sint32>().unwrap();
        runtime.register_inhabits_fn::<i64, Sint64>().unwrap();
        runtime.register_inhabits_fn::<u8, Uint8>().unwrap();
        runtime.register_inhabits_fn::<u16, Uint16>().unwrap();
        runtime.register_inhabits_fn::<u32, Uint32>().unwrap();
        runtime.register_inhabits_fn::<u64, Uint64>().unwrap();
        runtime.register_inhabits_fn::<f32, Float32>().unwrap();
        runtime.register_inhabits_fn::<f64, Float64>().unwrap();
        runtime.register_inhabits_fn::<Sint8, Sint8Type>().unwrap();
        runtime.register_inhabits_fn::<Sint16, Sint16Type>().unwrap();
        runtime.register_inhabits_fn::<Sint32, Sint32Type>().unwrap();
        runtime.register_inhabits_fn::<Sint64, Sint64Type>().unwrap();
        runtime.register_inhabits_fn::<Uint8, Uint8Type>().unwrap();
        runtime.register_inhabits_fn::<Uint16, Uint16Type>().unwrap();
        runtime.register_inhabits_fn::<Uint32, Uint32Type>().unwrap();
        runtime.register_inhabits_fn::<Uint64, Uint64Type>().unwrap();
        runtime.register_inhabits_fn::<Float32, Float32Type>().unwrap();
        runtime.register_inhabits_fn::<Float64, Float64Type>().unwrap();
        runtime.register_inhabits_fn::<ArrayTerm, Array>().unwrap();
        runtime.register_inhabits_fn::<Array, ArrayType>().unwrap();

        runtime.register_abstract_type::<Term>().unwrap();
        runtime.register_abstract_type::<Type>().unwrap();
        runtime.register_abstract_type::<bool>().unwrap();
        runtime.register_abstract_type::<Bool>().unwrap();
        runtime.register_abstract_type::<BoolType>().unwrap();
        runtime.register_abstract_type::<EmptyType>().unwrap();
        runtime.register_abstract_type::<False>().unwrap();
        runtime.register_abstract_type::<FalseType>().unwrap();
        runtime.register_abstract_type::<i8>().unwrap();
        runtime.register_abstract_type::<i16>().unwrap();
        runtime.register_abstract_type::<i32>().unwrap();
        runtime.register_abstract_type::<i64>().unwrap();
        runtime.register_abstract_type::<Sint8>().unwrap();
        runtime.register_abstract_type::<Sint16>().unwrap();
        runtime.register_abstract_type::<Sint32>().unwrap();
        runtime.register_abstract_type::<Sint64>().unwrap();
        runtime.register_abstract_type::<Sint8Type>().unwrap();
        runtime.register_abstract_type::<Sint16Type>().unwrap();
        runtime.register_abstract_type::<Sint32Type>().unwrap();
        runtime.register_abstract_type::<Sint64Type>().unwrap();
        runtime.register_abstract_type::<True>().unwrap();
        runtime.register_abstract_type::<TrueType>().unwrap();
        runtime.register_abstract_type::<u8>().unwrap();
        runtime.register_abstract_type::<u16>().unwrap();
        runtime.register_abstract_type::<u32>().unwrap();
        runtime.register_abstract_type::<u64>().unwrap();
        runtime.register_abstract_type::<f32>().unwrap();
        runtime.register_abstract_type::<f64>().unwrap();
        runtime.register_abstract_type::<Uint8>().unwrap();
        runtime.register_abstract_type::<Uint16>().unwrap();
        runtime.register_abstract_type::<Uint32>().unwrap();
        runtime.register_abstract_type::<Uint64>().unwrap();
        runtime.register_abstract_type::<Uint8Type>().unwrap();
        runtime.register_abstract_type::<Uint16Type>().unwrap();
        runtime.register_abstract_type::<Uint32Type>().unwrap();
        runtime.register_abstract_type::<Uint64Type>().unwrap();
        runtime.register_abstract_type::<Float32>().unwrap();
        runtime.register_abstract_type::<Float64>().unwrap();
        runtime.register_abstract_type::<Float32Type>().unwrap();
        runtime.register_abstract_type::<Float64Type>().unwrap();
        runtime.register_abstract_type::<Void>().unwrap();
        runtime.register_abstract_type::<VoidType>().unwrap();
        runtime.register_abstract_type::<ArrayTerm>().unwrap();
        runtime.register_abstract_type::<Array>().unwrap();
        runtime.register_abstract_type::<ArrayType>().unwrap();

        runtime.register_is_parametric_term::<Term>().unwrap();
        runtime.register_is_parametric_term::<Type>().unwrap();
        runtime.register_is_parametric_term::<bool>().unwrap();
        runtime.register_is_parametric_term::<Bool>().unwrap();
        runtime.register_is_parametric_term::<BoolType>().unwrap();
        runtime.register_is_parametric_term::<EmptyType>().unwrap();
        runtime.register_is_parametric_term::<False>().unwrap();
        runtime.register_is_parametric_term::<FalseType>().unwrap();
        runtime.register_is_parametric_term::<i8>().unwrap();
        runtime.register_is_parametric_term::<i16>().unwrap();
        runtime.register_is_parametric_term::<i32>().unwrap();
        runtime.register_is_parametric_term::<i64>().unwrap();
        runtime.register_is_parametric_term::<Sint8>().unwrap();
        runtime.register_is_parametric_term::<Sint16>().unwrap();
        runtime.register_is_parametric_term::<Sint32>().unwrap();
        runtime.register_is_parametric_term::<Sint64>().unwrap();
        runtime.register_is_parametric_term::<Sint8Type>().unwrap();
        runtime.register_is_parametric_term::<Sint16Type>().unwrap();
        runtime.register_is_parametric_term::<Sint32Type>().unwrap();
        runtime.register_is_parametric_term::<Sint64Type>().unwrap();
        runtime.register_is_parametric_term::<True>().unwrap();
        runtime.register_is_parametric_term::<TrueType>().unwrap();
        runtime.register_is_parametric_term::<u8>().unwrap();
        runtime.register_is_parametric_term::<u16>().unwrap();
        runtime.register_is_parametric_term::<u32>().unwrap();
        runtime.register_is_parametric_term::<u64>().unwrap();
        runtime.register_is_parametric_term::<f32>().unwrap();
        runtime.register_is_parametric_term::<f64>().unwrap();
        runtime.register_is_parametric_term::<Uint8>().unwrap();
        runtime.register_is_parametric_term::<Uint16>().unwrap();
        runtime.register_is_parametric_term::<Uint32>().unwrap();
        runtime.register_is_parametric_term::<Uint64>().unwrap();
        runtime.register_is_parametric_term::<Uint8Type>().unwrap();
        runtime.register_is_parametric_term::<Uint16Type>().unwrap();
        runtime.register_is_parametric_term::<Uint32Type>().unwrap();
        runtime.register_is_parametric_term::<Uint64Type>().unwrap();
        runtime.register_is_parametric_term::<Float32>().unwrap();
        runtime.register_is_parametric_term::<Float64>().unwrap();
        runtime.register_is_parametric_term::<Float32Type>().unwrap();
        runtime.register_is_parametric_term::<Float64Type>().unwrap();
        runtime.register_is_parametric_term::<Void>().unwrap();
        runtime.register_is_parametric_term::<VoidType>().unwrap();
        runtime.register_is_parametric_term::<ArrayTerm>().unwrap();
        runtime.register_is_parametric_term::<Array>().unwrap();
        runtime.register_is_parametric_term::<ArrayType>().unwrap();

        runtime.register_is_type_term::<Term>().unwrap();
        runtime.register_is_type_term::<Type>().unwrap();
        runtime.register_is_type_term::<bool>().unwrap();
        runtime.register_is_type_term::<Bool>().unwrap();
        runtime.register_is_type_term::<BoolType>().unwrap();
        runtime.register_is_type_term::<EmptyType>().unwrap();
        runtime.register_is_type_term::<False>().unwrap();
        runtime.register_is_type_term::<FalseType>().unwrap();
        runtime.register_is_type_term::<i8>().unwrap();
        runtime.register_is_type_term::<i16>().unwrap();
        runtime.register_is_type_term::<i32>().unwrap();
        runtime.register_is_type_term::<i64>().unwrap();
        runtime.register_is_type_term::<Sint8>().unwrap();
        runtime.register_is_type_term::<Sint16>().unwrap();
        runtime.register_is_type_term::<Sint32>().unwrap();
        runtime.register_is_type_term::<Sint64>().unwrap();
        runtime.register_is_type_term::<Sint8Type>().unwrap();
        runtime.register_is_type_term::<Sint16Type>().unwrap();
        runtime.register_is_type_term::<Sint32Type>().unwrap();
        runtime.register_is_type_term::<Sint64Type>().unwrap();
        runtime.register_is_type_term::<True>().unwrap();
        runtime.register_is_type_term::<TrueType>().unwrap();
        runtime.register_is_type_term::<u8>().unwrap();
        runtime.register_is_type_term::<u16>().unwrap();
        runtime.register_is_type_term::<u32>().unwrap();
        runtime.register_is_type_term::<u64>().unwrap();
        runtime.register_is_type_term::<f32>().unwrap();
        runtime.register_is_type_term::<f64>().unwrap();
        runtime.register_is_type_term::<Uint8>().unwrap();
        runtime.register_is_type_term::<Uint16>().unwrap();
        runtime.register_is_type_term::<Uint32>().unwrap();
        runtime.register_is_type_term::<Uint64>().unwrap();
        runtime.register_is_type_term::<Uint8Type>().unwrap();
        runtime.register_is_type_term::<Uint16Type>().unwrap();
        runtime.register_is_type_term::<Uint32Type>().unwrap();
        runtime.register_is_type_term::<Uint64Type>().unwrap();
        runtime.register_is_type_term::<Float32>().unwrap();
        runtime.register_is_type_term::<Float64>().unwrap();
        runtime.register_is_type_term::<Float32Type>().unwrap();
        runtime.register_is_type_term::<Float64Type>().unwrap();
        runtime.register_is_type_term::<Void>().unwrap();
        runtime.register_is_type_term::<VoidType>().unwrap();
        runtime.register_is_type_term::<ArrayTerm>().unwrap();
        runtime.register_is_type_term::<Array>().unwrap();
        runtime.register_is_type_term::<ArrayType>().unwrap();

        runtime
    }
    // TODO: This could be used to register everything that TermTrait specifies
    pub fn register_term(&mut self, type_id: TypeId) -> Result<()> {
        match self.term_s.insert(type_id) {
            false => Err(anyhow::anyhow!("collision with already-registered term {}", self.label_of(type_id))),
            true => Ok(())
        }
    }
    pub fn register_label_fn(
        &mut self,
        type_id: TypeId,
        label_fn: LabelFn,
    ) -> Result<()> {
        match self.label_fn_m.insert(type_id, label_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered label fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub fn register_label<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        match self.label_fn_m.insert(type_id, T::label) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered label fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub fn register_stringify_fn(
        &mut self,
        type_id: TypeId,
        stringify_fn: StringifyFn,
    ) -> Result<()> {
        match self.stringify_fn_m.insert(type_id, stringify_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered stringify fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub fn register_stringify<S: Stringify + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<S>();
        let stringify_fn = |x: &dyn Any| -> String { S::stringify(x.downcast_ref::<S>().unwrap()) };
        match self.stringify_fn_m.insert(type_id, stringify_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered stringify fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub fn register_abstract_type_fn(
        &mut self,
        type_id: TypeId,
        abstract_type_fn: AbstractTypeFn,
    ) -> Result<()> {
        match self.abstract_type_fn_m.insert(type_id, abstract_type_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered abstract_type fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub fn register_abstract_type<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let abstract_type_fn = |x: &dyn Any| -> Box<dyn Any> {
            // TODO: if the return type is Box<dyn Any>, then just return that,
            // but otherwise use Box::new on the return value
            let abstract_type = x.downcast_ref::<T>().unwrap().abstract_type();
//             TODO start here
//             if { let at: &dyn Any = &abstract_type; at.is::<Box<dyn Any>>() } {
//                 abstract_type
//             } else {
//                 Box::new(abstract_type)
//             }
            // TEMP HACK: If abstract_type is already a Box<dyn Any>, then this will make a double
            // box, which is not what is wanted.  But for now, whateva.
            if { let at: &dyn Any = &abstract_type; at.is::<Box<dyn Any>>() } {
                panic!("this situation isn't implemented yet -- panicking here to avoid creating a Box<Box<dyn Any>>");
            }
            Box::new(abstract_type)
        };
        match self.abstract_type_fn_m.insert(type_id, abstract_type_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered abstract_type fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub fn register_is_parametric_term<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_parametric_term_fn = |x: &dyn Any| -> bool {
            x.downcast_ref::<T>().unwrap().is_parametric_term()
        };
        match self.is_parametric_term_fn_m.insert(type_id, is_parametric_term_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered is_parametric_term fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub fn register_is_type_term<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_type_term_fn = |x: &dyn Any| -> bool {
            x.downcast_ref::<T>().unwrap().is_type_term()
        };
        match self.is_type_term_fn_m.insert(type_id, is_type_term_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered is_type_term fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    fn register_eq_fn_impl(
        &mut self,
        type_id_pair: (TypeId, TypeId),
        eq_fn: BinaryPredicate,
    ) -> Result<()> {
        let is_transposed = type_id_pair.0 > type_id_pair.1;
        let type_id_pair_ = if is_transposed { (type_id_pair.1, type_id_pair.0) } else { type_id_pair };
        match self.eq_fn_m.insert(type_id_pair_, RegisteredEqualsFn { eq_fn, is_transposed }) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered eq fn for ({}, {})", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1))),
            None => Ok(())
        }
    }
    pub fn register_eq_fn<Lhs: PartialEq<Rhs> + 'static, Rhs: 'static>(&mut self) -> Result<()> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        let eq_fn = |lhs: &dyn Any, rhs: &dyn Any| -> bool {
            *lhs.downcast_ref::<Lhs>().unwrap() == *rhs.downcast_ref::<Rhs>().unwrap()
        };
        Ok(self.register_eq_fn_impl(type_id_pair, eq_fn)?)
    }
    pub fn register_inhabits_fn<Lhs: Inhabits<Rhs> + 'static, Rhs: 'static>(&mut self) -> Result<()> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        let inhabits_fn = |lhs: &dyn Any, rhs: &dyn Any| -> bool {
            lhs.downcast_ref::<Lhs>().unwrap().inhabits(rhs.downcast_ref::<Rhs>().unwrap())
        };
        match self.inhabits_fn_m.insert(type_id_pair, inhabits_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered inhabits fn for ({}, {})", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1))),
            None => Ok(())
        }
    }

    pub fn label_of(&self, type_id: TypeId) -> String {
        match self.label_fn_m.get(&type_id) {
            Some(label_fn) => label_fn().into(),
            None => format!("{:?}", type_id),
        }
    }
    pub fn stringify(&self, x: &dyn Any) -> String {
        match self.stringify_fn_m.get(&x.type_id()) {
            Some(stringify_fn) => stringify_fn(x),
            // None => Err(anyhow::anyhow!("no stringify fn found for {:?}", x.type_id())),
            None => {
                // panic!("no stringify fn found for {:?}", x.type_id()),
                log::warn!("no stringify fn found for {}; returning generic default", self.label_of(x.type_id()));
                format!("InstanceOf({})", self.label_of(x.type_id()))
            }
        }
    }
    pub fn eq(&self, lhs: &dyn Any, rhs: &dyn Any) -> bool {
        // TODO: Check if the types are singletons (i.e. NonParametricTerm) and then can just compare their type id.
        // Actually this isn't exactly true because of DynNPTerm.  So it would need to check against that.
        let lhs_type_id = lhs.type_id();
        let rhs_type_id = rhs.type_id();
        let is_transposed = lhs_type_id > rhs_type_id;
        let type_id_pair = if is_transposed { (rhs_type_id, lhs_type_id) } else { (lhs_type_id, rhs_type_id) };
        match self.eq_fn_m.get(&type_id_pair) {
            Some(registered_eq_fn) => if registered_eq_fn.is_transposed == is_transposed {
                (registered_eq_fn.eq_fn)(lhs, rhs)
            } else {
                (registered_eq_fn.eq_fn)(rhs, lhs)
            },
            None => {
                // panic!("no eq fn found for {:?}", (lhs_type_id, rhs_type_id)),
                log::warn!("no eq fn found for ({}, {}); returning default value of false", self.label_of(lhs_type_id), self.label_of(rhs_type_id));
                false
            },
        }
    }
    pub fn ne(&self, lhs: &dyn Any, rhs: &dyn Any) -> bool {
        !self.eq(lhs, rhs)
    }
    pub fn inhabits(&self, x: &dyn Any, t: &dyn Any) -> bool {
        let type_id_pair = (x.type_id(), t.type_id());
        match self.inhabits_fn_m.get(&type_id_pair) {
            Some(inhabits_fn) => inhabits_fn(x, t),
            None => {
                // panic!("no inhabits fn found for {:?}", (lhs_type_id, rhs_type_id)),
                log::warn!("no inhabits fn found for ({}, {}); returning default value of false", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
                false
            }
        }
    }
    pub fn abstract_type_of(&self, x: &dyn Any) -> Box<dyn Any> {
        let type_id = x.type_id();
        match self.abstract_type_fn_m.get(&type_id) {
            Some(abstract_type_fn) => abstract_type_fn(x),
            None => {
                // panic!("no abstract_type fn found for {:?}", (lhs_type_id, rhs_type_id)),
                log::warn!("no abstract_type fn found for {}; returning default value of Box::<dyn Any>::new(Type{{ }})", self.label_of(type_id));
                Box::new(Type{})
            }
        }
    }
    pub fn is_parametric_term(&self, x: &dyn Any) -> bool {
        match self.is_parametric_term_fn_m.get(&x.type_id()) {
            Some(is_parametric_term_fn) => is_parametric_term_fn(x),
            None => {
                panic!("no is_parametric_term fn found for {:?}", x.type_id());
                // NOTE: A default here probably doesn't make any sense.
//                 log::warn!("no is_parametric_term fn found for ({}, {}); returning default value of false", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
//                 false
            }
        }
    }
    pub fn is_type_term(&self, x: &dyn Any) -> bool {
        match self.is_type_term_fn_m.get(&x.type_id()) {
            Some(is_type_term_fn) => is_type_term_fn(x),
            None => {
                panic!("no is_type_term fn found for {:?}", x.type_id());
                // NOTE: A default here probably doesn't make any sense.
//                 log::warn!("no is_type_term fn found for ({}, {}); returning default value of false", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
//                 false
            }
        }
    }
}

lazy_static::lazy_static! {
    /// This is the static singleton Runtime.  TODO: This probably won't suffice once a program can
    /// add stuff to Runtime at runtime.  But maybe some kind of layering structure could work.
    pub static ref RUNTIME: Runtime = Runtime::new();
}
