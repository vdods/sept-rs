use crate::{
    dy::{self, ArrayTerm, GlobalSymRefTerm, LocalSymRefTerm, StructTerm, StructTermTerm, TupleTerm, ValueGuts},
    st::{
        self, Array, ArrayType,
        Bool, BoolType, EmptyType, False, FalseType, Float32, Float32Type, Float64, Float64Type,
        GlobalSymRef, GlobalSymRefType, Inhabits, LocalSymRef, LocalSymRefType,
        Result, Sint8, Sint8Type, Sint16, Sint16Type, Sint32, Sint32Type, Sint64, Sint64Type, Stringify,
        Struct, StructType, Term, TermTrait, True, TrueType, Tuple, TupleType, Type,
        Uint8, Uint8Type, Uint16, Uint16Type, Uint32, Uint32Type, Uint64, Uint64Type,
        Utf8String, Utf8StringType, Void, VoidType,
    },
};
use std::{any::TypeId, collections::{HashMap, HashSet}, sync::{Arc, RwLock}};

pub type StringifyFn = fn(x: &ValueGuts) -> String;
pub type LabelFn = fn() -> &'static str;
pub type AbstractTypeFn = fn(x: &ValueGuts) -> Box<ValueGuts>;
pub type CloneFn = fn(x: &ValueGuts) -> Box<ValueGuts>;
pub type UnaryPredicate = fn(x: &ValueGuts) -> bool;
pub type BinaryPredicate = fn(lhs: &ValueGuts, rhs: &ValueGuts) -> bool;
pub type DereferencedOnceFn = fn(x: &ValueGuts) -> anyhow::Result<Arc<RwLock<dy::Value>>>;
pub type DeconstructFn = fn(x: &ValueGuts) -> dy::Deconstruction;

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
    clone_fn_m: HashMap<TypeId, CloneFn>,
    is_parametric_fn_m: HashMap<TypeId, UnaryPredicate>,
    is_type_fn_m: HashMap<TypeId, UnaryPredicate>,
    // TODO: subtype of
    dereferenced_once_fn_m: HashMap<TypeId, DereferencedOnceFn>,
    deconstruct_fn_m: HashMap<TypeId, DeconstructFn>,
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
        runtime.register_term::<String>().unwrap();
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
        runtime.register_type::<Utf8String>().unwrap();
        runtime.register_type::<Utf8StringType>().unwrap();
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

        // Have to go through and explicitly register the Eq types, since that can't be done
        // (to my knowledge) using generics specialization.  There's probably another reasonable
        // way to do this, but I don't know.
        {
            runtime.reregister_as_eq::<bool>().unwrap();
            runtime.reregister_as_eq::<False>().unwrap();
            runtime.reregister_as_eq::<True>().unwrap();
            runtime.reregister_as_eq::<i8>().unwrap();
            runtime.reregister_as_eq::<i16>().unwrap();
            runtime.reregister_as_eq::<i32>().unwrap();
            runtime.reregister_as_eq::<i64>().unwrap();
            runtime.reregister_as_eq::<u8>().unwrap();
            runtime.reregister_as_eq::<u16>().unwrap();
            runtime.reregister_as_eq::<u32>().unwrap();
            runtime.reregister_as_eq::<u64>().unwrap();
            // Note that floating point types are NOT here, since they don't implelement Eq (e.g. NaN != NaN).
            runtime.reregister_as_eq::<Void>().unwrap();
            // ArrayTerm isn't Eq, because it might contain a float.
    //         runtime.reregister_as_eq::<ArrayTerm>().unwrap();
            // StructTermTerm isn't Eq, because it might contain a float.
    //         runtime.reregister_as_eq::<StructTermTerm>().unwrap();

            runtime.reregister_as_eq::<Term>().unwrap();
            runtime.reregister_as_eq::<Type>().unwrap();
            runtime.reregister_as_eq::<Bool>().unwrap();
            runtime.reregister_as_eq::<BoolType>().unwrap();
            runtime.reregister_as_eq::<EmptyType>().unwrap();
            runtime.reregister_as_eq::<FalseType>().unwrap();
            runtime.reregister_as_eq::<TrueType>().unwrap();
            runtime.reregister_as_eq::<Sint8>().unwrap();
            runtime.reregister_as_eq::<Sint16>().unwrap();
            runtime.reregister_as_eq::<Sint32>().unwrap();
            runtime.reregister_as_eq::<Sint64>().unwrap();
            runtime.reregister_as_eq::<Sint8Type>().unwrap();
            runtime.reregister_as_eq::<Sint16Type>().unwrap();
            runtime.reregister_as_eq::<Sint32Type>().unwrap();
            runtime.reregister_as_eq::<Sint64Type>().unwrap();
            runtime.reregister_as_eq::<Uint8>().unwrap();
            runtime.reregister_as_eq::<Uint16>().unwrap();
            runtime.reregister_as_eq::<Uint32>().unwrap();
            runtime.reregister_as_eq::<Uint64>().unwrap();
            runtime.reregister_as_eq::<Uint8Type>().unwrap();
            runtime.reregister_as_eq::<Uint16Type>().unwrap();
            runtime.reregister_as_eq::<Uint32Type>().unwrap();
            runtime.reregister_as_eq::<Uint64Type>().unwrap();
            runtime.reregister_as_eq::<Float32>().unwrap();
            runtime.reregister_as_eq::<Float64>().unwrap();
            runtime.reregister_as_eq::<Float32Type>().unwrap();
            runtime.reregister_as_eq::<Float64Type>().unwrap();
            runtime.reregister_as_eq::<VoidType>().unwrap();
            runtime.reregister_as_eq::<Array>().unwrap();
            runtime.reregister_as_eq::<ArrayType>().unwrap();
            // TupleTerm isn't Eq, because it might contain a float
//             runtime.reregister_as_eq::<TupleTerm>().unwrap();
            runtime.reregister_as_eq::<Tuple>().unwrap();
            runtime.reregister_as_eq::<TupleType>().unwrap();
            // NOTE: This is a special type, and requires special handling (TODO)
    //         runtime.reregister_as_eq::<GlobalSymRefTerm>().unwrap();
    //         runtime.reregister_as_eq::<LocalSymRefTerm>().unwrap();
            runtime.reregister_as_eq::<GlobalSymRef>().unwrap();
            runtime.reregister_as_eq::<GlobalSymRefType>().unwrap();
            runtime.reregister_as_eq::<LocalSymRef>().unwrap();
            runtime.reregister_as_eq::<LocalSymRefType>().unwrap();
            // StructTerm isn't Eq because it's possible that a type might not implement Eq.
//             runtime.reregister_as_eq::<StructTerm>().unwrap();
            runtime.reregister_as_eq::<Struct>().unwrap();
            runtime.reregister_as_eq::<StructType>().unwrap();
        }

        // Other, non-uniform registrations.
        // TODO: Need to somehow make it so that everything inhabits Term
        // TODO: Need to be able to register EmptyType's inhabitation function (it returns false for any term arg)

        runtime.register_label::<GlobalSymRefTerm>().unwrap();
        runtime.register_stringify::<GlobalSymRefTerm>().unwrap();
        runtime.register_label::<LocalSymRefTerm>().unwrap();
        runtime.register_stringify::<LocalSymRefTerm>().unwrap();
        runtime.register_partial_eq::<bool, True>().unwrap();
        runtime.register_partial_eq::<bool, False>().unwrap();
        // TODO: referential transparency has to be handled with special code
        runtime.register_partial_eq::<GlobalSymRefTerm, GlobalSymRefTerm>().unwrap();
        runtime.register_partial_eq::<LocalSymRefTerm, LocalSymRefTerm>().unwrap();
        runtime.register_inhabits::<bool, FalseType>().unwrap();
        runtime.register_inhabits::<bool, TrueType>().unwrap();
        runtime.register_inhabits::<False, Bool>().unwrap();
        runtime.register_inhabits::<True, Bool>().unwrap();
        runtime.register_inhabits::<TupleTerm, StructTerm>().unwrap();

        runtime.register_abstract_type::<GlobalSymRefTerm>().unwrap();
        runtime.register_clone::<GlobalSymRefTerm>().unwrap();
        runtime.register_is_parametric::<GlobalSymRefTerm>().unwrap();
        runtime.register_is_type::<GlobalSymRefTerm>().unwrap();

        runtime.register_abstract_type::<LocalSymRefTerm>().unwrap();
        runtime.register_clone::<LocalSymRefTerm>().unwrap();
        runtime.register_is_parametric::<LocalSymRefTerm>().unwrap();
        runtime.register_is_type::<LocalSymRefTerm>().unwrap();

        runtime.register_dereferenced_once::<GlobalSymRefTerm>().unwrap();
        runtime.register_dereferenced_once::<LocalSymRefTerm>().unwrap();

        runtime
    }

    // TODO: Ideally, this wouldn't require all the traits, but could do compile-time if clauses
    // to call the methods that require those traits.
    pub fn register_term<T>(&mut self) -> Result<()>
    where
        T:  st::TermTrait +
            dy::Deconstruct +
            Stringify +
            std::cmp::PartialEq +
            Inhabits<<T as TermTrait>::AbstractTypeType> +
            'static,
        <T as TermTrait>::AbstractTypeType: st::TypeTrait
    {
        let type_id = TypeId::of::<T>();
        anyhow::ensure!(self.term_s.insert(type_id), "collision with already-registered term {}", self.label_of(type_id));
        self.register_label::<T>()?;
        self.register_stringify::<T>()?;
        self.register_partial_eq::<T, T>()?;
        self.register_inhabits::<T, <T as TermTrait>::AbstractTypeType>()?;
        self.register_abstract_type::<T>()?;
        self.register_clone::<T>()?;
        self.register_is_parametric::<T>()?;
        self.register_is_type::<T>()?;
        self.register_deconstruct::<T>()?;
        Ok(())
    }
    pub fn register_type<T>(&mut self) -> Result<()>
    where
        T:  st::TypeTrait +
            dy::Deconstruct +
            Stringify +
            std::cmp::PartialEq +
            Inhabits<<T as TermTrait>::AbstractTypeType> +
            'static,
        <T as TermTrait>::AbstractTypeType: st::TypeTrait
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
        // log::debug!("register_label_fn; type_id: {:?}; label_fn(): {:?}", type_id, label_fn());
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
    pub(crate) fn register_clone_fn(
        &mut self,
        type_id: TypeId,
        clone_fn: AbstractTypeFn,
    ) -> Result<()> {
        match self.clone_fn_m.insert(type_id, clone_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered clone fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_clone<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let clone_fn = |x: &ValueGuts| -> Box<ValueGuts> {
            // TODO: if the return type is Box<ValueGuts>, then just return that,
            // but otherwise use Box::new on the return value
            let clone = x.downcast_ref::<T>().unwrap().clone();
//             TODO start here
//             if { let at: &ValueGuts = &clone; at.is::<Box<ValueGuts>>() } {
//                 clone
//             } else {
//                 Box::new(clone)
//             }
            // TEMP HACK: If clone is already a Box<ValueGuts>, then this will make a double
            // box, which is not what is wanted.  But for now, whateva.
            // NOTE: I think because of the fixed Value::from situation (using dy::IntoValue to bound
            // `impl From<T> for Value`), this is not a problem anymore, meaning that Box<Box<ValueGuts>>
            // should not be possible, and all this can be cleaned up.
            if { let at: &ValueGuts = &clone; at.is::<Box<ValueGuts>>() } {
                panic!("this situation isn't implemented yet -- panicking here to avoid creating a Box<Box<ValueGuts>>");
            }
            Box::new(clone)
        };
        Ok(self.register_clone_fn(type_id, clone_fn)?)
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_is_parametric<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_parametric_fn = |x: &ValueGuts| -> bool {
            x.downcast_ref::<T>().unwrap().is_parametric()
        };
        match self.is_parametric_fn_m.insert(type_id, is_parametric_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered is_parametric fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_is_type<T: TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_type_fn = |x: &ValueGuts| -> bool {
            x.downcast_ref::<T>().unwrap().is_type()
        };
        match self.is_type_fn_m.insert(type_id, is_type_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered is_type fn for {}", self.label_of(type_id))),
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
    /// Note that this actually requires that there already be an eq_fn for T to itself, and it re-registers it
    /// under the stronger condition that T implement Eq.
    pub fn reregister_as_eq<T: Eq + 'static>(&mut self) -> Result<()> {
        let type_id_pair = (TypeId::of::<T>(), TypeId::of::<T>());
        anyhow::ensure!(self.eq_fn_m.contains_key(&type_id_pair), "reregister_as_eq can only be used if register_partial_eq has been used for ({}, {})", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
        // TODO: If the type is a non-parametric term (i.e. singletons), then we can just compare their TypeId values.
        let eq_fn = |lhs: &ValueGuts, rhs: &ValueGuts| -> bool {
            // Since the type is the same, and that type implements Eq, we can compare the references' pointer values directly.
            std::ptr::eq(lhs, rhs) || *lhs.downcast_ref::<T>().unwrap() == *rhs.downcast_ref::<T>().unwrap()
        };

        let is_transposed = type_id_pair.0 > type_id_pair.1;
        let type_id_pair_ = if is_transposed { (type_id_pair.1, type_id_pair.0) } else { type_id_pair };
        // This unwrap won't panic because of the self.eq_fn_m.contains_key check above.
        self.eq_fn_m.insert(type_id_pair_, RegisteredEqualsFn { eq_fn, is_transposed }).unwrap();
        Ok(())
    }
    pub fn register_partial_eq<Lhs: PartialEq<Rhs> + 'static, Rhs: 'static>(&mut self) -> Result<()> {
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
    pub(crate) fn register_dereferenced_once_fn(&mut self, type_id: TypeId, dereferenced_once_fn: DereferencedOnceFn) -> Result<()> {
        match self.dereferenced_once_fn_m.insert(type_id, dereferenced_once_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered dereferenced_once fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub fn register_dereferenced_once<T: dy::TransparentRefTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let dereferenced_once_fn = |x: &ValueGuts| -> anyhow::Result<Arc<RwLock<dy::Value>>> {
            x.downcast_ref::<T>().unwrap().dereferenced_once()
        };
        Ok(self.register_dereferenced_once_fn(type_id, dereferenced_once_fn)?)
    }
    pub(crate) fn register_deconstruct_fn(&mut self, type_id: TypeId, deconstruct_fn: DeconstructFn) -> Result<()> {
        match self.deconstruct_fn_m.insert(type_id, deconstruct_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered deconstruct fn for {}", self.label_of(type_id))),
            None => Ok(())
        }
    }
    pub fn register_deconstruct<T: dy::Deconstruct + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let deconstruct_fn = |x: &ValueGuts| -> dy::Deconstruction {
            x.downcast_ref::<T>().unwrap().deconstruct()
        };
        Ok(self.register_deconstruct_fn(type_id, deconstruct_fn)?)
    }

    /// This gives the [non-parametric] label of the concrete type.  For example, even though
    /// GlobalSymRefTerm is referentially transparent, its label is still GlobalSymRefTerm.
    pub fn label_of(&self, type_id: TypeId) -> String {
        match self.label_fn_m.get(&type_id) {
            Some(label_fn) => label_fn().into(),
            None => format!("{:?}", type_id),
        }
    }
    // Note that this does not use referential transparency.  Stringify should be renamed to ConcreteText or something.
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
        // Handle referential transparency.
        let lhs_dereferenced = self.dereferenced(lhs).expect("dereferenced failed");
        let rhs_dereferenced = self.dereferenced(rhs).expect("dereferenced failed");
        match (lhs_dereferenced, rhs_dereferenced) {
            (MaybeDereferencedValue::NonRef(lhs_value_guts), MaybeDereferencedValue::NonRef(rhs_value_guts)) => {
                self.eq_impl(lhs_value_guts, rhs_value_guts)
            },
            (MaybeDereferencedValue::NonRef(lhs_value_guts), MaybeDereferencedValue::Ref(rhs_value_la)) => {
                let rhs_value_g = rhs_value_la.read().unwrap();
                self.eq_impl(lhs_value_guts, rhs_value_g.as_ref())
            }
            (MaybeDereferencedValue::Ref(lhs_value_la), MaybeDereferencedValue::NonRef(rhs_value_guts)) => {
                let lhs_value_g = lhs_value_la.read().unwrap();
                self.eq_impl(lhs_value_g.as_ref(), rhs_value_guts)
            },
            (MaybeDereferencedValue::Ref(lhs_value_la), MaybeDereferencedValue::Ref(rhs_value_la)) => {
                let lhs_value_g = lhs_value_la.read().unwrap();
                let rhs_value_g = rhs_value_la.read().unwrap();
                self.eq_impl(lhs_value_g.as_ref(), rhs_value_g.as_ref())
            },
        }
    }
    // This method does only the eq operation, not handling referential transparency.
    fn eq_impl(&self, lhs: &ValueGuts, rhs: &ValueGuts) -> bool {
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
        // Handle referential transparency.
        let x_maybe_dereferenced = self.dereferenced(x).expect("dereferenced failed");
        let t_maybe_dereferenced = self.dereferenced(t).expect("dereferenced failed");
        match (x_maybe_dereferenced, t_maybe_dereferenced) {
            (MaybeDereferencedValue::NonRef(x_value_guts), MaybeDereferencedValue::NonRef(t_value_guts)) => {
                self.inhabits_impl(x_value_guts, t_value_guts)
            },
            (MaybeDereferencedValue::NonRef(x_value_guts), MaybeDereferencedValue::Ref(t_value_la)) => {
                let t_value_g = t_value_la.read().unwrap();
                self.inhabits_impl(x_value_guts, t_value_g.as_ref())
            }
            (MaybeDereferencedValue::Ref(x_value_la), MaybeDereferencedValue::NonRef(t_value_guts)) => {
                let x_value_g = x_value_la.read().unwrap();
                self.inhabits_impl(x_value_g.as_ref(), t_value_guts)
            },
            (MaybeDereferencedValue::Ref(x_value_la), MaybeDereferencedValue::Ref(t_value_la)) => {
                let x_value_g = x_value_la.read().unwrap();
                let t_value_g = t_value_la.read().unwrap();
                self.inhabits_impl(x_value_g.as_ref(), t_value_g.as_ref())
            },
        }
    }
    fn inhabits_impl(&self, x: &ValueGuts, t: &ValueGuts) -> bool {
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
        // Handle referential transparency.
        let x_maybe_dereferenced = self.dereferenced(x).expect("dereferenced failed");
        match x_maybe_dereferenced {
            MaybeDereferencedValue::NonRef(x_value_guts) => {
                self.abstract_type_of_impl(x_value_guts)
            },
            MaybeDereferencedValue::Ref(x_value_la) => {
                let x_value_g = x_value_la.read().unwrap();
                self.abstract_type_of_impl(x_value_g.as_ref())
            },
        }
    }
    fn abstract_type_of_impl(&self, x: &ValueGuts) -> Box<ValueGuts> {
        let type_id = x.type_id();
        match self.abstract_type_fn_m.get(&type_id) {
            Some(abstract_type_fn) => abstract_type_fn(x),
            None => {
                // panic!("no abstract_type fn found for ({}, ())", self.label_of(lhs_type_id), self.label_of(rhs_type_id)),
                log::warn!("no abstract_type fn found for {}; returning default value of Box::<ValueGuts>::new(Type{{ }})", self.label_of(type_id));
                Box::new(Type{})
            }
        }
    }
    // Note that clone doesn't use referential transparency.  TODO: Figure out if this is correct.
    pub fn clone(&self, x: &ValueGuts) -> Box<ValueGuts> {
        let type_id = x.type_id();
        match self.clone_fn_m.get(&type_id) {
            Some(clone_fn) => clone_fn(x),
            None => {
                panic!("no clone fn found for {}", self.label_of(type_id));
                // There's probably no reasonable default.
//                 log::warn!("no clone fn found for {}; returning default value of Box::<ValueGuts>::new(Type{{ }})", self.label_of(type_id));
//                 Box::new(Type{})
            }
        }
    }
    pub fn is_parametric(&self, x: &ValueGuts) -> bool {
        // Handle referential transparency.
        let x_maybe_dereferenced = self.dereferenced(x).expect("dereferenced failed");
        match x_maybe_dereferenced {
            MaybeDereferencedValue::NonRef(x_value_guts) => {
                self.is_parametric_impl(x_value_guts)
            },
            MaybeDereferencedValue::Ref(x_value_la) => {
                let x_value_g = x_value_la.read().unwrap();
                self.is_parametric_impl(x_value_g.as_ref())
            },
        }
    }
    fn is_parametric_impl(&self, x: &ValueGuts) -> bool {
        match self.is_parametric_fn_m.get(&x.type_id()) {
            Some(is_parametric_fn) => is_parametric_fn(x),
            None => {
                panic!("no is_parametric fn found for {}", self.label_of(x.type_id()));
                // NOTE: A default here probably doesn't make any sense.
//                 log::warn!("no is_parametric fn found for ({}, {}); returning default value of false", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
//                 false
            }
        }
    }
    pub fn is_type(&self, x: &ValueGuts) -> bool {
        // Handle referential transparency.
        let x_maybe_dereferenced = self.dereferenced(x).expect("dereferenced failed");
        match x_maybe_dereferenced {
            MaybeDereferencedValue::NonRef(x_value_guts) => {
                self.is_type_impl(x_value_guts)
            },
            MaybeDereferencedValue::Ref(x_value_la) => {
                let x_value_g = x_value_la.read().unwrap();
                self.is_type_impl(x_value_g.as_ref())
            },
        }
    }
    fn is_type_impl(&self, x: &ValueGuts) -> bool {
        match self.is_type_fn_m.get(&x.type_id()) {
            Some(is_type_fn) => is_type_fn(x),
            None => {
                panic!("no is_type fn found for {}", self.label_of(x.type_id()));
                // NOTE: A default here probably doesn't make any sense.
//                 log::warn!("no is_type fn found for ({}, {}); returning default value of false", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
//                 false
            }
        }
    }
    pub fn is_transparent_ref_term(&self, x: &ValueGuts) -> bool {
        self.dereferenced_once_fn_m.contains_key(&x.type_id())
    }
    pub fn dereferenced_once(&self, x: &ValueGuts) -> anyhow::Result<Arc<RwLock<dy::Value>>> {
        match self.dereferenced_once_fn_m.get(&x.type_id()) {
            Some(dereferenced_once_fn) => Ok(dereferenced_once_fn(x)?),
            None => {
                panic!("no dereferenced_once fn found for {}", self.label_of(x.type_id()));
                // NOTE: A reasonable default would be Err(anyhow::anyhow!("no dereferenced_once fn found for {}", self.label_of(x.type_id()))
            }
        }
    }
    /// This fully dereferences a value.  If it's not a transparent reference, it just returns it,
    /// otherwise it iterates dereferenced_once until it's not a transparent reference.
    // TODO: Implement some limit to reference nesting.  Or not, and just let the stack overflow and the process crash.
    pub fn dereferenced<'a>(&self, x: &'a ValueGuts) -> anyhow::Result<MaybeDereferencedValue<'a>> {
        match self.dereferenced_once_fn_m.get(&x.type_id()) {
            Some(dereferenced_once_fn) => Ok(MaybeDereferencedValue::Ref(self.dereferenced_inner(dereferenced_once_fn(x)?)?)),
            None => Ok(MaybeDereferencedValue::NonRef(x))
        }
    }
    // TODO: Implement some limit to reference nesting.  Or not, and just let the stack overflow and the process crash.
    pub(crate) fn dereferenced_inner(&self, value_la: Arc<RwLock<dy::Value>>) -> anyhow::Result<Arc<RwLock<dy::Value>>> {
        let value_g = value_la.read().unwrap();
        match self.dereferenced_once_fn_m.get(&value_g.as_ref().type_id()) {
            Some(dereferenced_once_fn) => Ok(self.dereferenced_inner(dereferenced_once_fn(value_g.as_ref())?)?),
            None => Ok(value_la.clone())
        }
    }
    pub fn deconstruct(&self, x: &ValueGuts) -> dy::Deconstruction {
        match self.deconstruct_fn_m.get(&x.type_id()) {
            Some(deconstruct_fn) => deconstruct_fn(x),
            None => {
                panic!("no deconstruct fn found for {}", self.label_of(x.type_id()));
                // NOTE: A reasonable default would be Err(anyhow::anyhow!("no deconstruct fn found for {}", self.label_of(x.type_id()))
            }
        }
    }
}

// This sucks, and so does Runtime::dereferenced and dereferenced_inner, and all the call sites in this file.
pub enum MaybeDereferencedValue<'a> {
    NonRef(&'a ValueGuts),
    Ref(Arc<RwLock<dy::Value>>),
}

lazy_static::lazy_static! {
    /// This is the static singleton Runtime, where terms and types should be registered for dynamic
    /// operation of the sept data model.
    pub static ref RUNTIME_LA: Arc<RwLock<Runtime>> = Arc::new(RwLock::new(Runtime::new()));
}
