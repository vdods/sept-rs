use crate::{
    dy::{ArrayTerm, GlobalSymRefTerm, LocalSymRefTerm, TupleTerm, StructTerm, StructTermTerm, SymbolTable, ValueGuts},
    st::{
        self, Array, ArrayType,
        Bool, BoolType, EmptyType, False, FalseType, Float32, Float32Type, Float64, Float64Type,
        GlobalSymRef, GlobalSymRefType, Inhabits, LocalSymRef, LocalSymRefType,
        Result, Sint8, Sint8Type, Sint16, Sint16Type, Sint32, Sint32Type, Sint64, Sint64Type, Stringify,
        Struct, StructType, Term, TermTrait, True, TrueType, Tuple, TupleType, Type,
        Uint8, Uint8Type, Uint16, Uint16Type, Uint32, Uint32Type, Uint64, Uint64Type,
        Void, VoidType,
    },
};
use std::{any::TypeId, collections::{HashMap, HashSet}, sync::{Arc, RwLock}};

pub type StringifyFn = fn(x: &ValueGuts) -> String;
pub type LabelFn = fn() -> &'static str;
pub type AbstractTypeFn = fn(x: &ValueGuts) -> Box<ValueGuts>;
pub type UnaryPredicate = fn(x: &ValueGuts) -> bool;
pub type BinaryPredicate = fn(lhs: &ValueGuts, rhs: &ValueGuts) -> bool;

struct RegisteredEqualsFn {
    eq_fn: BinaryPredicate,
    is_transposed: bool,
}

/// The sept Runtime is what supports the sept data model; it tracks what types are registered
/// and the various inhabitation and subtyping (and other) relationships.
#[derive(Default)]
pub struct Runtime {
    // TODO: [po]set of types (poset based on which relationship?)
    // TODO: [po]set of terms(?)

    term_s: HashSet<TypeId>,
    type_s: HashSet<TypeId>,
    // TODO: This is silly, just map to &'static str
    label_fn_m: HashMap<TypeId, LabelFn>,
    stringify_fn_m: HashMap<TypeId, StringifyFn>,
    eq_fn_m: HashMap<(TypeId, TypeId), RegisteredEqualsFn>,
    inhabits_fn_m: HashMap<(TypeId, TypeId), BinaryPredicate>,
    abstract_type_fn_m: HashMap<TypeId, AbstractTypeFn>,
    is_parametric_term_fn_m: HashMap<TypeId, UnaryPredicate>,
    is_type_term_fn_m: HashMap<TypeId, UnaryPredicate>,
    // TODO: subtype of

    pub global_symbol_table: SymbolTable,
}

impl Runtime {
    pub fn new() -> Self {
        let mut runtime = Runtime::default();

        // TODO: Figure out how to move these into something like "init" fns
        // in the respective modules

        // TODO: Order these in some sensible way

        // Register non-type terms
        runtime.register_term::<bool>().unwrap();
        runtime.register_term::<False>().unwrap();
        runtime.register_term::<True>().unwrap();
        runtime.register_term::<i8>().unwrap();
        runtime.register_term::<i16>().unwrap();
        runtime.register_term::<i32>().unwrap();
        runtime.register_term::<i64>().unwrap();
        runtime.register_term::<u8>().unwrap();
        runtime.register_term::<u16>().unwrap();
        runtime.register_term::<u32>().unwrap();
        runtime.register_term::<u64>().unwrap();
        runtime.register_term::<f32>().unwrap();
        runtime.register_term::<f64>().unwrap();
        runtime.register_term::<Void>().unwrap();
        runtime.register_term::<ArrayTerm>().unwrap();
        runtime.register_term::<StructTermTerm>().unwrap();

        // Register types
        runtime.register_type::<Term>().unwrap();
        runtime.register_type::<Type>().unwrap();
        runtime.register_type::<Bool>().unwrap();
        runtime.register_type::<BoolType>().unwrap();
        runtime.register_type::<EmptyType>().unwrap();
        runtime.register_type::<FalseType>().unwrap();
        runtime.register_type::<TrueType>().unwrap();
        runtime.register_type::<Sint8>().unwrap();
        runtime.register_type::<Sint16>().unwrap();
        runtime.register_type::<Sint32>().unwrap();
        runtime.register_type::<Sint64>().unwrap();
        runtime.register_type::<Sint8Type>().unwrap();
        runtime.register_type::<Sint16Type>().unwrap();
        runtime.register_type::<Sint32Type>().unwrap();
        runtime.register_type::<Sint64Type>().unwrap();
        runtime.register_type::<Uint8>().unwrap();
        runtime.register_type::<Uint16>().unwrap();
        runtime.register_type::<Uint32>().unwrap();
        runtime.register_type::<Uint64>().unwrap();
        runtime.register_type::<Uint8Type>().unwrap();
        runtime.register_type::<Uint16Type>().unwrap();
        runtime.register_type::<Uint32Type>().unwrap();
        runtime.register_type::<Uint64Type>().unwrap();
        runtime.register_type::<Float32>().unwrap();
        runtime.register_type::<Float64>().unwrap();
        runtime.register_type::<Float32Type>().unwrap();
        runtime.register_type::<Float64Type>().unwrap();
        runtime.register_type::<VoidType>().unwrap();
        runtime.register_type::<Array>().unwrap();
        runtime.register_type::<ArrayType>().unwrap();
        runtime.register_type::<TupleTerm>().unwrap();
        runtime.register_type::<Tuple>().unwrap();
        runtime.register_type::<TupleType>().unwrap();
        // NOTE: This is a special type, and requires special handling (TODO)
//         runtime.register_type::<GlobalSymRefTerm>().unwrap();
//         runtime.register_type::<LocalSymRefTerm>().unwrap();
        runtime.register_type::<GlobalSymRef>().unwrap();
        runtime.register_type::<GlobalSymRefType>().unwrap();
        runtime.register_type::<LocalSymRef>().unwrap();
        runtime.register_type::<LocalSymRefType>().unwrap();
        runtime.register_type::<StructTerm>().unwrap();
        runtime.register_type::<Struct>().unwrap();
        runtime.register_type::<StructType>().unwrap();

        // Other, non-uniform registrations.
        // TODO: Need to somehow make it so that everything inhabits Term
        // TODO: Need to be able to register EmptyType's inhabitation function (it returns false for any term arg)

        runtime.register_label::<GlobalSymRefTerm>().unwrap();
        runtime.register_stringify::<GlobalSymRefTerm>().unwrap();
        runtime.register_label::<LocalSymRefTerm>().unwrap();
        runtime.register_stringify::<LocalSymRefTerm>().unwrap();
        runtime.register_eq::<bool, True>().unwrap();
        runtime.register_eq::<bool, False>().unwrap();
        // TODO: referential transparency has to be handled with special code
//         runtime.register_eq::<GlobalSymRefTerm, GlobalSymRefTerm>().unwrap();
        runtime.register_inhabits::<bool, FalseType>().unwrap();
        runtime.register_inhabits::<bool, TrueType>().unwrap();
        runtime.register_inhabits::<False, Bool>().unwrap();
        runtime.register_inhabits::<True, Bool>().unwrap();
        // TODO: special handling for inhabitation of and by GlobalSymRefTerm
//         runtime.register_inhabits::<GlobalSymRef, GlobalSymRefType>().unwrap();
        runtime.register_inhabits::<TupleTerm, StructTerm>().unwrap();

        runtime.register_abstract_type::<GlobalSymRefTerm>().unwrap();
        runtime.register_is_parametric_term::<GlobalSymRefTerm>().unwrap();
        runtime.register_is_type_term::<GlobalSymRefTerm>().unwrap();

        runtime.register_abstract_type::<LocalSymRefTerm>().unwrap();
        runtime.register_is_parametric_term::<LocalSymRefTerm>().unwrap();
        runtime.register_is_type_term::<LocalSymRefTerm>().unwrap();

        runtime
    }

    // TODO: Ideally, this wouldn't require all the traits, but could do compile-time if clauses
    // to call the methods that require those traits.
    pub fn register_term<T>(&mut self) -> Result<()>
    where
        T:  st::TermTrait +
            Stringify +
            std::cmp::PartialEq +
            Inhabits<<T as TermTrait>::AbstractTypeFnReturnType> +
            'static,
        <T as TermTrait>::AbstractTypeFnReturnType: st::TypeTrait
    {
        let type_id = TypeId::of::<T>();
        anyhow::ensure!(self.term_s.insert(type_id), "collision with already-registered term {}", self.label_of(type_id));
        self.register_label::<T>()?;
        self.register_stringify::<T>()?;
        self.register_eq::<T, T>()?;
        self.register_inhabits::<T, <T as TermTrait>::AbstractTypeFnReturnType>()?;
        self.register_abstract_type::<T>()?;
        self.register_is_parametric_term::<T>()?;
        self.register_is_type_term::<T>()?;
        Ok(())
    }
    pub fn register_type<T>(&mut self) -> Result<()>
    where
        T:  st::TypeTrait +
            Stringify +
            std::cmp::PartialEq +
            Inhabits<<T as TermTrait>::AbstractTypeFnReturnType> +
            'static,
        <T as TermTrait>::AbstractTypeFnReturnType: st::TypeTrait
    {
        self.register_term::<T>()?;
        let type_id = TypeId::of::<T>();
        anyhow::ensure!(self.type_s.insert(type_id), "collision with already-registered type {}", self.label_of(type_id));
        Ok(())
    }

    pub(crate) fn register_label_fn(
        &mut self,
        type_id: TypeId,
        label_fn: LabelFn,
    ) -> Result<()> {
        match self.label_fn_m.insert(type_id, label_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered label fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub(crate) fn register_label<T: TermTrait + 'static>(&mut self) -> Result<()> {
        Ok(self.register_label_fn(TypeId::of::<T>(), T::label)?)
    }
    pub(crate) fn register_stringify_fn(
        &mut self,
        type_id: TypeId,
        stringify_fn: StringifyFn,
    ) -> Result<()> {
        match self.stringify_fn_m.insert(type_id, stringify_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered stringify fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub(crate) fn register_stringify<S: Stringify + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<S>();
        let stringify_fn = |x: &ValueGuts| -> String { S::stringify(x.downcast_ref::<S>().unwrap()) };
        Ok(self.register_stringify_fn(type_id, stringify_fn)?)
    }
    pub(crate) fn register_abstract_type_fn(
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
    pub(crate) fn register_abstract_type<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let abstract_type_fn = |x: &ValueGuts| -> Box<ValueGuts> {
            // TODO: if the return type is Box<ValueGuts>, then just return that,
            // but otherwise use Box::new on the return value
            let abstract_type = x.downcast_ref::<T>().unwrap().abstract_type();
//             TODO start here
//             if { let at: &ValueGuts = &abstract_type; at.is::<Box<ValueGuts>>() } {
//                 abstract_type
//             } else {
//                 Box::new(abstract_type)
//             }
            // TEMP HACK: If abstract_type is already a Box<ValueGuts>, then this will make a double
            // box, which is not what is wanted.  But for now, whateva.
            // NOTE: I think because of the fixed Value::from situation (using dy::IntoValue to bound
            // `impl From<T> for Value`), this is not a problem anymore, meaning that Box<Box<ValueGuts>>
            // should not be possible, and all this can be cleaned up.
            if { let at: &ValueGuts = &abstract_type; at.is::<Box<ValueGuts>>() } {
                panic!("this situation isn't implemented yet -- panicking here to avoid creating a Box<Box<ValueGuts>>");
            }
            Box::new(abstract_type)
        };
        Ok(self.register_abstract_type_fn(type_id, abstract_type_fn)?)
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_is_parametric_term<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_parametric_term_fn = |x: &ValueGuts| -> bool {
            x.downcast_ref::<T>().unwrap().is_parametric_term()
        };
        match self.is_parametric_term_fn_m.insert(type_id, is_parametric_term_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered is_parametric_term fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_is_type_term<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_type_term_fn = |x: &ValueGuts| -> bool {
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
    pub fn register_eq<Lhs: PartialEq<Rhs> + 'static, Rhs: 'static>(&mut self) -> Result<()> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        let eq_fn = |lhs: &ValueGuts, rhs: &ValueGuts| -> bool {
            *lhs.downcast_ref::<Lhs>().unwrap() == *rhs.downcast_ref::<Rhs>().unwrap()
        };
        Ok(self.register_eq_fn_impl(type_id_pair, eq_fn)?)
    }
    pub fn register_inhabits<Lhs: Inhabits<Rhs> + 'static, Rhs: st::TypeTrait + 'static>(&mut self) -> Result<()> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        let inhabits_fn = |lhs: &ValueGuts, rhs: &ValueGuts| -> bool {
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
    pub fn stringify(&self, x: &ValueGuts) -> String {
        match self.stringify_fn_m.get(&x.type_id()) {
            Some(stringify_fn) => stringify_fn(x),
            None => {
                // panic!("no stringify fn found for {:?}", x.type_id()),
                log::warn!("no stringify fn found for {}; returning generic default", self.label_of(x.type_id()));
                format!("InstanceOf({})", self.label_of(x.type_id()))
            }
        }
    }
    pub fn eq(&self, lhs: &ValueGuts, rhs: &ValueGuts) -> bool {
        // TODO: Check if the types are singletons (i.e. NonParametricTerm) and then can just compare their type id.
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
    pub fn ne(&self, lhs: &ValueGuts, rhs: &ValueGuts) -> bool {
        !self.eq(lhs, rhs)
    }
    pub fn inhabits(&self, x: &ValueGuts, t: &ValueGuts) -> bool {
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
    pub fn abstract_type_of(&self, x: &ValueGuts) -> Box<ValueGuts> {
        let type_id = x.type_id();
        match self.abstract_type_fn_m.get(&type_id) {
            Some(abstract_type_fn) => abstract_type_fn(x),
            None => {
                // panic!("no abstract_type fn found for {:?}", (lhs_type_id, rhs_type_id)),
                log::warn!("no abstract_type fn found for {}; returning default value of Box::<ValueGuts>::new(Type{{ }})", self.label_of(type_id));
                Box::new(Type{})
            }
        }
    }
    pub fn is_parametric_term(&self, x: &ValueGuts) -> bool {
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
    pub fn is_type_term(&self, x: &ValueGuts) -> bool {
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
    pub static ref RUNTIME_LA: Arc<RwLock<Runtime>> = Arc::new(RwLock::new(Runtime::new()));
}
