use crate::{
    dy::{
        self, ArrayTerm, GlobalSymRefTerm, LocalSymRefTerm, StructTerm, StructTermTerm, TupleTerm,
        ValueGuts,
    },
    st::{
        self, Array, ArrayType, Bool, BoolType, EmptyType, False, FalseType, Float32, Float32Type,
        Float64, Float64Type, GlobalSymRef, GlobalSymRefType, Inhabits, LocalSymRef,
        LocalSymRefType, Sint16, Sint16Type, Sint32, Sint32Type, Sint64, Sint64Type, Sint8,
        Sint8Type, Struct, StructType, Term, True, TrueType, Tuple, TupleType, Type, Uint16,
        Uint16Type, Uint32, Uint32Type, Uint64, Uint64Type, Uint8, Uint8Type, Utf8String,
        Utf8StringType, Void, VoidType,
    },
    Result,
};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

pub type DebugFn =
    fn(x: &ValueGuts, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>;
pub type StringifyFn = fn(x: &ValueGuts) -> String;
pub type SerializeFn = fn(x: &ValueGuts, writer: &mut dyn std::io::Write) -> Result<usize>;
pub type LabelFn = fn() -> &'static str;
pub type AbstractTypeFn = fn(x: &ValueGuts) -> Box<ValueGuts>;
pub type CloneFn = fn(x: &ValueGuts) -> Box<ValueGuts>;
pub type UnaryPredicate = fn(x: &ValueGuts) -> bool;
pub type BinaryPredicate = fn(lhs: &ValueGuts, rhs: &ValueGuts) -> bool;
pub type DereferencedOnceFn = fn(x: &ValueGuts) -> Result<Arc<RwLock<dy::Value>>>;
pub type ConstructFn = fn(constructor: &ValueGuts, parameter_t: dy::TupleTerm) -> Result<dy::Value>;
pub type DeserializeParametersAndConstructFn =
    fn(constructor: &ValueGuts, reader: &mut dyn std::io::Read) -> Result<dy::Value>;
pub type DeconstructFn = fn(x: &ValueGuts) -> dy::Deconstruction;
pub type NonParametricTermInstantiateFn = fn() -> dy::Value;

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

    // TODO: See about collecting many of these into a common map, since many of them will have
    // identical indexes.

    // TODO: A way to iterate over NonParametricTerms, Terms, Types, etc.
    non_parametric_term_code_m: HashMap<TypeId, st::NonParametricTermCode>,
    term_s: HashSet<TypeId>,
    type_s: HashSet<TypeId>,
    // TODO: This is silly, just map to &'static str
    label_fn_m: HashMap<TypeId, LabelFn>,
    debug_fn_m: HashMap<TypeId, DebugFn>,
    stringify_fn_m: HashMap<TypeId, StringifyFn>,
    //     serialize_top_level_code_fn_m: HashMap<TypeId, SerializeFn>,
    //     serialize_construct_fn_m: HashMap<TypeId, SerializeFn>,
    serialize_fn_m: HashMap<TypeId, SerializeFn>,
    eq_fn_m: HashMap<(TypeId, TypeId), RegisteredEqualsFn>,
    inhabits_fn_m: HashMap<(TypeId, TypeId), BinaryPredicate>,
    abstract_type_fn_m: HashMap<TypeId, AbstractTypeFn>,
    clone_fn_m: HashMap<TypeId, CloneFn>,
    is_parametric_fn_m: HashMap<TypeId, UnaryPredicate>,
    is_type_fn_m: HashMap<TypeId, UnaryPredicate>,
    // TODO: subtype of
    dereferenced_once_fn_m: HashMap<TypeId, DereferencedOnceFn>,
    construct_fn_m: HashMap<TypeId, ConstructFn>,
    deserialize_parameters_and_construct_fn_m: HashMap<TypeId, DeserializeParametersAndConstructFn>,
    deconstruct_fn_m: HashMap<TypeId, DeconstructFn>,
    non_parametric_term_instantiate_from_identifier_fn_m:
        HashMap<&'static str, NonParametricTermInstantiateFn>,
    non_parametric_term_instantiate_from_code_fn_m:
        HashMap<st::NonParametricTermCode, NonParametricTermInstantiateFn>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut runtime = Runtime::default();

        // dy::Value is not a term itself, but it's useful to register its label, since it comes up.
        runtime.register_label::<dy::Value>().unwrap();
        runtime.register_debug::<dy::Value>().unwrap();

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

        // Register non-parametric term instantiate functions.
        runtime.register_non_parametric_term::<Term>().unwrap();
        //         runtime.register_non_parametric_term::<NonParametricTerm>().unwrap();
        //         runtime.register_non_parametric_term::<ParametricTerm>().unwrap();
        runtime.register_non_parametric_term::<Type>().unwrap();
        //         runtime.register_non_parametric_term::<NonType>().unwrap();
        //         runtime.register_non_parametric_term::<NonParametricType>().unwrap();
        //         runtime.register_non_parametric_term::<ParametricType>().unwrap();
        runtime.register_non_parametric_term::<Void>().unwrap();
        runtime.register_non_parametric_term::<True>().unwrap();
        runtime.register_non_parametric_term::<False>().unwrap();
        runtime.register_non_parametric_term::<VoidType>().unwrap();
        runtime.register_non_parametric_term::<TrueType>().unwrap();
        runtime.register_non_parametric_term::<FalseType>().unwrap();
        runtime.register_non_parametric_term::<EmptyType>().unwrap();
        //         runtime.register_non_parametric_term::<FormalTypeOf>().unwrap();
        runtime.register_non_parametric_term::<Bool>().unwrap();
        runtime.register_non_parametric_term::<Sint8>().unwrap();
        runtime.register_non_parametric_term::<Sint16>().unwrap();
        runtime.register_non_parametric_term::<Sint32>().unwrap();
        runtime.register_non_parametric_term::<Sint64>().unwrap();
        runtime.register_non_parametric_term::<Uint8>().unwrap();
        runtime.register_non_parametric_term::<Uint16>().unwrap();
        runtime.register_non_parametric_term::<Uint32>().unwrap();
        runtime.register_non_parametric_term::<Uint64>().unwrap();
        runtime.register_non_parametric_term::<Float32>().unwrap();
        runtime.register_non_parametric_term::<Float64>().unwrap();
        runtime.register_non_parametric_term::<BoolType>().unwrap();
        runtime.register_non_parametric_term::<Sint8Type>().unwrap();
        runtime
            .register_non_parametric_term::<Sint16Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Sint32Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Sint64Type>()
            .unwrap();
        runtime.register_non_parametric_term::<Uint8Type>().unwrap();
        runtime
            .register_non_parametric_term::<Uint16Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Uint32Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Uint64Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Float32Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Float64Type>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Utf8String>()
            .unwrap();
        runtime
            .register_non_parametric_term::<Utf8StringType>()
            .unwrap();
        runtime.register_non_parametric_term::<ArrayType>().unwrap();
        runtime.register_non_parametric_term::<Array>().unwrap();
        runtime.register_non_parametric_term::<TupleType>().unwrap();
        runtime.register_non_parametric_term::<Tuple>().unwrap();
        runtime
            .register_non_parametric_term::<StructType>()
            .unwrap();
        runtime.register_non_parametric_term::<Struct>().unwrap();
        runtime
            .register_non_parametric_term::<GlobalSymRefType>()
            .unwrap();
        runtime
            .register_non_parametric_term::<GlobalSymRef>()
            .unwrap();
        runtime
            .register_non_parametric_term::<LocalSymRefType>()
            .unwrap();
        runtime
            .register_non_parametric_term::<LocalSymRef>()
            .unwrap();

        // Have to go through and explicitly register the Constructor types, until ParametricType
        // is a thing.
        runtime.register_constructor::<Bool>().unwrap();
        runtime.register_constructor::<Sint8>().unwrap();
        runtime.register_constructor::<Sint16>().unwrap();
        runtime.register_constructor::<Sint32>().unwrap();
        runtime.register_constructor::<Sint64>().unwrap();
        runtime.register_constructor::<Uint8>().unwrap();
        runtime.register_constructor::<Uint16>().unwrap();
        runtime.register_constructor::<Uint32>().unwrap();
        runtime.register_constructor::<Uint64>().unwrap();
        runtime.register_constructor::<Float32>().unwrap();
        runtime.register_constructor::<Float64>().unwrap();
        runtime.register_constructor::<Utf8String>().unwrap();
        runtime.register_constructor::<Array>().unwrap();
        runtime.register_constructor::<GlobalSymRef>().unwrap();
        // runtime.register_constructor::<GlobalSymRefTerm>().unwrap();
        // runtime.register_constructor::<LocalSymRef>().unwrap();
        runtime.register_constructor::<Tuple>().unwrap();
        runtime.register_constructor::<TupleTerm>().unwrap();
        runtime.register_constructor::<Struct>().unwrap();
        runtime.register_constructor::<StructTerm>().unwrap();

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

        runtime.term_s.insert(TypeId::of::<GlobalSymRefTerm>());
        runtime.term_s.insert(TypeId::of::<LocalSymRefTerm>());

        runtime.register_label::<GlobalSymRefTerm>().unwrap();
        runtime.register_stringify::<GlobalSymRefTerm>().unwrap();
        runtime.register_label::<LocalSymRefTerm>().unwrap();
        runtime.register_stringify::<LocalSymRefTerm>().unwrap();
        runtime.register_partial_eq::<bool, True>().unwrap();
        runtime.register_partial_eq::<bool, False>().unwrap();
        // TODO: referential transparency has to be handled with special code
        //         runtime.register_partial_eq::<GlobalSymRefTerm, GlobalSymRefTerm>().unwrap();
        runtime
            .register_partial_eq::<LocalSymRefTerm, LocalSymRefTerm>()
            .unwrap();
        runtime.register_inhabits::<bool, FalseType>().unwrap();
        runtime.register_inhabits::<bool, TrueType>().unwrap();
        runtime.register_inhabits::<False, Bool>().unwrap();
        runtime.register_inhabits::<True, Bool>().unwrap();
        runtime
            .register_inhabits::<TupleTerm, StructTerm>()
            .unwrap();
        //         runtime.register_inhabits::<StructTermTerm, GlobalSymRefTerm>().unwrap();
        //         runtime.register_inhabits::<StructTermTerm, LocalSymRefTerm>().unwrap();
        runtime
            .register_inhabits::<StructTermTerm, StructTerm>()
            .unwrap();

        runtime
            .register_abstract_type::<GlobalSymRefTerm>()
            .unwrap();
        runtime.register_clone::<GlobalSymRefTerm>().unwrap();
        runtime.register_debug::<GlobalSymRefTerm>().unwrap();
        runtime
            .register_is_parametric::<GlobalSymRefTerm>()
            .unwrap();
        runtime.register_is_type::<GlobalSymRefTerm>().unwrap();

        runtime.register_abstract_type::<LocalSymRefTerm>().unwrap();
        runtime.register_clone::<LocalSymRefTerm>().unwrap();
        runtime.register_debug::<LocalSymRefTerm>().unwrap();
        runtime.register_is_parametric::<LocalSymRefTerm>().unwrap();
        runtime.register_is_type::<LocalSymRefTerm>().unwrap();

        runtime
            .register_dereferenced_once::<GlobalSymRefTerm>()
            .unwrap();
        runtime
            .register_dereferenced_once::<LocalSymRefTerm>()
            .unwrap();

        runtime
    }

    // TODO: Ideally, this wouldn't require all the traits, but could do compile-time if clauses
    // to call the methods that require those traits.
    pub fn register_term<T>(&mut self) -> Result<()>
    where
        T: st::TermTrait
            + dy::Deconstruct
            + std::fmt::Debug
            + st::Serializable
            + st::Stringifiable
            + std::cmp::PartialEq
            + Inhabits<<T as st::TermTrait>::AbstractTypeType>
            + 'static,
        <T as st::TermTrait>::AbstractTypeType: st::TypeTrait,
    {
        let type_id = TypeId::of::<T>();
        log::trace!(
            "Runtime::register_term; {} {:?}",
            std::any::type_name::<T>(),
            type_id
        );
        anyhow::ensure!(
            self.term_s.insert(type_id),
            "collision with already-registered term {}; term type that produced the collision was {}",
            self.label_of_type_id(type_id),
            std::any::type_name::<T>()
        );
        self.register_label::<T>()?;
        self.register_debug::<T>()?;
        self.register_serialize::<T>()?;
        self.register_stringify::<T>()?;
        self.register_partial_eq::<T, T>()?;
        self.register_inhabits::<T, <T as st::TermTrait>::AbstractTypeType>()?;
        self.register_abstract_type::<T>()?;
        self.register_clone::<T>()?;
        self.register_is_parametric::<T>()?;
        self.register_is_type::<T>()?;
        self.register_deconstruct::<T>()?;
        Ok(())
    }
    pub fn register_type<T>(&mut self) -> Result<()>
    where
        T: st::TypeTrait
            + dy::Deconstruct
            + std::fmt::Debug
            + st::Serializable
            + st::Stringifiable
            + std::cmp::PartialEq
            + Inhabits<<T as st::TermTrait>::AbstractTypeType>
            + Inhabits<st::Type>
            + 'static,
        <T as st::TermTrait>::AbstractTypeType: st::TypeTrait,
    {
        self.register_term::<T>()?;
        if self.inhabits_fn::<T, st::Type>().is_none() {
            self.register_inhabits::<T, st::Type>()?;
        }
        let type_id = TypeId::of::<T>();
        anyhow::ensure!(
            self.type_s.insert(type_id),
            "collision with already-registered type {}; term type that produced the collision was {}",
            self.label_of_type_id(type_id),
            std::any::type_name::<T>()
        );
        Ok(())
    }

    // TODO: Rename to register_term_name?
    pub(crate) fn register_label<T: st::TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let label_fn = || -> &'static str { std::any::type_name::<T>() };
        match self.label_fn_m.insert(type_id, label_fn) {
            Some(_) => Err(anyhow::anyhow!(
                "collision with already-registered label fn for {}; term type that produced the collision was {}",
                self.label_of_type_id(type_id),
                std::any::type_name::<T>()
            )),
            None => Ok(()),
        }
    }
    pub(crate) fn register_debug<T: std::fmt::Debug + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let debug_fn = |x: &ValueGuts,
                        f: &mut std::fmt::Formatter<'_>|
         -> std::result::Result<(), std::fmt::Error> {
            Ok(x.downcast_ref::<T>().unwrap().fmt(f)?)
        };
        match self.debug_fn_m.insert(type_id, debug_fn) {
            Some(_) => Err(anyhow::anyhow!(
                "collision with already-registered debug fn for {}; term type that produced the collision was {}",
                self.label_of_type_id(type_id),
                std::any::type_name::<T>()
            )),
            None => Ok(()),
        }
    }
    pub(crate) fn register_stringify<T: st::Stringifiable + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let stringify_fn =
            |x: &ValueGuts| -> String { T::stringify(x.downcast_ref::<T>().unwrap()) };
        match self.stringify_fn_m.insert(type_id, stringify_fn) {
            Some(_) => Err(anyhow::anyhow!(
                "collision with already-registered stringify fn for {}; term type that produced the collision was {}",
                self.label_of_type_id(type_id),
                std::any::type_name::<T>()
            )),
            None => Ok(()),
        }
    }
    //     fn register_serialize_top_level_code_fn(
    //         &mut self,
    //         type_id: TypeId,
    //         serialize_top_level_code_fn: SerializeFn,
    //     ) -> Result<()> {
    //         match self.serialize_top_level_code_fn_m.insert(type_id, serialize_top_level_code_fn) {
    //             Some(_) => Err(anyhow::anyhow!("collision with already-registered serialize_top_level_code fn for {}", self.label_of(type_id))),
    //             None => Ok(())
    //         }
    //     }
    //     fn register_serialize_construct_fn(
    //         &mut self,
    //         type_id: TypeId,
    //         serialize_construct_fn: SerializeFn,
    //     ) -> Result<()> {
    //         match self.serialize_construct_fn_m.insert(type_id, serialize_construct_fn) {
    //             Some(_) => Err(anyhow::anyhow!("collision with already-registered serialize_constructor fn for {}", self.label_of(type_id))),
    //             None => Ok(())
    //         }
    //     }
    pub(crate) fn register_serialize<T: st::Serializable + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        //         let serialize_top_level_code_fn = |x: &ValueGuts, writer: &mut dyn std::io::Write| -> Result<usize> {
        //             Ok(x.downcast_ref::<S>().unwrap().serialize_top_level_code(writer)?)
        //         };
        //         let serialize_construct_fn = |x: &ValueGuts, writer: &mut dyn std::io::Write| -> Result<usize> {
        //             Ok(x.downcast_ref::<S>().unwrap().serialize_constructor(writer)?)
        //         };
        let serialize_fn = |x: &ValueGuts, writer: &mut dyn std::io::Write| -> Result<usize> {
            Ok(x.downcast_ref::<T>().unwrap().serialize(writer)?)
        };
        //         self.register_serialize_top_level_code_fn(type_id, serialize_top_level_code_fn)?;
        //         self.register_serialize_construct_fn(type_id, serialize_construct_fn)?;
        match self.serialize_fn_m.insert(type_id, serialize_fn) {
            Some(_) => {
                anyhow::bail!("collision with already-registered serialize fn for {}; term type that produced the collision was {}",
                self.label_of_type_id(type_id),
                std::any::type_name::<T>());
            }
            None => {}
        }
        // self.register_serialize_fn(type_id, serialize_fn)?;
        Ok(())
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_abstract_type<T: st::TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let abstract_type_fn = |x: &ValueGuts| -> Box<ValueGuts> {
            let abstract_type = x.downcast_ref::<T>().unwrap().abstract_type();
            if (&abstract_type as &ValueGuts).is::<dy::Value>() {
                // If the thing is already a value, return its inner Box<ValueGuts>, because we don't want
                // to create a Value(Value(...)).  The pedantic sequence of `let` statements is just to
                // be really clear about what types we're working with.

                // TODO: Figure out if there's a more minimal way to do this.

                // This Box<ValueGuts> takes ownership of the stack variable abstract_type and allows us to
                // use it as an implementor of the std::any::Any trait.
                let abstract_type_b: Box<ValueGuts> = Box::new(abstract_type);
                // Downcast the Box's inner type.
                let abstract_type_as_value_b = abstract_type_b.downcast::<dy::Value>().unwrap();
                // Take the guts out of the Box.
                let abstract_type_as_value = *abstract_type_as_value_b;
                // Take the guts out of the dy::Value.
                abstract_type_as_value.into_inner()
            } else if (&abstract_type as &ValueGuts).is::<Box<ValueGuts>>() {
                // Just need to return that box.  The pedantic sequence of `let` statements is just to
                // be really clear about what types we're working with.

                // TODO: Figure out if there's a more minimal way to do this.

                // This Box<ValueGuts> takes ownership of the stack variable abstract_type and allows us to
                // use it as an implementor of the std::any::Any trait.
                let abstract_type_b: Box<ValueGuts> = Box::new(abstract_type);
                // Downcast the Box's inner type.
                let abstract_value_as_value_guts_bb =
                    abstract_type_b.downcast::<Box<ValueGuts>>().unwrap();
                // Take the guts out of the Box.
                let abstract_value_as_value_guts_b = *abstract_value_as_value_guts_bb;
                // Return that Box.
                abstract_value_as_value_guts_b
            } else {
                Box::new(abstract_type)
            }
        };
        match self.abstract_type_fn_m.insert(type_id, abstract_type_fn) {
            Some(_) => {
                anyhow::bail!("collision with already-registered abstract_type fn for {}; term type that produced the collision was {}",
                self.label_of_type_id(type_id),
                std::any::type_name::<T>());
            }
            None => Ok(()),
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_clone<T: st::TermTrait + 'static>(&mut self) -> Result<()> {
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
            if {
                let at: &ValueGuts = &clone;
                at.is::<Box<ValueGuts>>() || at.is::<dy::Value>()
            } {
                unimplemented!("TODO: implement the same situation as in abstract_type to prevent creating a Box<Box<ValueGuts>> or Value(Value(...))");
            }
            Box::new(clone)
        };
        match self.clone_fn_m.insert(type_id, clone_fn) {
            Some(_) => {
                anyhow::bail!(
                "collision with already-registered clone fn for {}; term type that produced the collision was {}",
                self.label_of_type_id(type_id),
                std::any::type_name::<T>()
            );
            }
            None => Ok(()),
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_is_parametric<T: st::TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_parametric_fn =
            |x: &ValueGuts| -> bool { x.downcast_ref::<T>().unwrap().is_parametric() };
        match self.is_parametric_fn_m.insert(type_id, is_parametric_fn) {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered is_parametric fn for {}; term type that produced the collision was {}",
                    self.label_of_type_id(type_id),
                    std::any::type_name::<T>()
                );
            }
            None => Ok(()),
        }
    }
    // TODO: Rename this something different (this was copied and pasted from register_stringify
    // and the semantics don't match).
    pub(crate) fn register_is_type<T: st::TermTrait + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let is_type_fn = |x: &ValueGuts| -> bool { x.downcast_ref::<T>().unwrap().is_type() };
        match self.is_type_fn_m.insert(type_id, is_type_fn) {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered is_type fn for {}; term type that produced the collision was {}",
                    self.label_of_type_id(type_id),
                    std::any::type_name::<T>()
                );
            }
            None => Ok(()),
        }
    }
    fn register_eq_fn_impl(
        &mut self,
        type_id_pair: (TypeId, TypeId),
        eq_fn: BinaryPredicate,
    ) -> Result<()> {
        let is_transposed = type_id_pair.0 > type_id_pair.1;
        let type_id_pair_ = if is_transposed {
            (type_id_pair.1, type_id_pair.0)
        } else {
            type_id_pair
        };
        match self.eq_fn_m.insert(
            type_id_pair_,
            RegisteredEqualsFn {
                eq_fn,
                is_transposed,
            },
        ) {
            // TODO: Could add the extended message that indicates the term types that produced the collision.
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered eq fn for ({}, {})",
                    self.label_of_type_id(type_id_pair.0),
                    self.label_of_type_id(type_id_pair.1)
                );
            }
            None => Ok(()),
        }
    }
    /// Note that this actually requires that there already be an eq_fn for T to itself, and it re-registers it
    /// under the stronger condition that T implement Eq.
    pub fn reregister_as_eq<T: Eq + 'static>(&mut self) -> Result<()> {
        let type_id_pair = (TypeId::of::<T>(), TypeId::of::<T>());
        anyhow::ensure!(
            self.eq_fn_m.contains_key(&type_id_pair),
            "reregister_as_eq can only be used if register_partial_eq has been used for ({}, {})",
            self.label_of_type_id(type_id_pair.0),
            self.label_of_type_id(type_id_pair.1)
        );
        // TODO: If the type is a non-parametric term (i.e. singletons), then we can just compare their TypeId values.
        let eq_fn = |lhs: &ValueGuts, rhs: &ValueGuts| -> bool {
            // Since the type is the same, and that type implements Eq, we can compare the references' pointer values directly.
            std::ptr::eq(lhs, rhs)
                || *lhs.downcast_ref::<T>().unwrap() == *rhs.downcast_ref::<T>().unwrap()
        };

        let is_transposed = type_id_pair.0 > type_id_pair.1;
        let type_id_pair_ = if is_transposed {
            (type_id_pair.1, type_id_pair.0)
        } else {
            type_id_pair
        };
        // This unwrap won't panic because of the self.eq_fn_m.contains_key check above.
        self.eq_fn_m
            .insert(
                type_id_pair_,
                RegisteredEqualsFn {
                    eq_fn,
                    is_transposed,
                },
            )
            .unwrap();
        Ok(())
    }
    pub fn register_partial_eq<Lhs: PartialEq<Rhs> + 'static, Rhs: 'static>(
        &mut self,
    ) -> Result<()> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        let eq_fn = |lhs: &ValueGuts, rhs: &ValueGuts| -> bool {
            *lhs.downcast_ref::<Lhs>().unwrap() == *rhs.downcast_ref::<Rhs>().unwrap()
        };
        Ok(self.register_eq_fn_impl(type_id_pair, eq_fn)?)
    }
    pub fn register_inhabits<Lhs: Inhabits<Rhs> + 'static, Rhs: st::TypeTrait + 'static>(
        &mut self,
    ) -> Result<()> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        let inhabits_fn = |lhs: &ValueGuts, rhs: &ValueGuts| -> bool {
            lhs.downcast_ref::<Lhs>()
                .unwrap()
                .inhabits(rhs.downcast_ref::<Rhs>().unwrap())
        };
        match self.inhabits_fn_m.insert(type_id_pair, inhabits_fn) {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered inhabits fn for ({}, {}); term types that produced the collision were ({}, {})",
                    self.label_of_type_id(type_id_pair.0),
                    self.label_of_type_id(type_id_pair.1),
                    std::any::type_name::<Lhs>(),
                    std::any::type_name::<Rhs>()
                );
            }
            None => Ok(()),
        }
    }
    pub fn register_dereferenced_once<T: dy::TransparentRefTrait + 'static>(
        &mut self,
    ) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let dereferenced_once_fn = |x: &ValueGuts| -> Result<Arc<RwLock<dy::Value>>> {
            x.downcast_ref::<T>().unwrap().dereferenced_once()
        };
        match self
            .dereferenced_once_fn_m
            .insert(type_id, dereferenced_once_fn)
        {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered dereferenced_once fn for {}; term type that produced the collision was {}",
                    self.label_of_type_id(type_id),
                    std::any::type_name::<T>()
                );
            }
            None => Ok(()),
        }
    }
    pub fn register_deconstruct<T: dy::Deconstruct + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let deconstruct_fn = |x: &ValueGuts| -> dy::Deconstruction {
            x.downcast_ref::<T>().unwrap().deconstructed()
        };
        match self.deconstruct_fn_m.insert(type_id, deconstruct_fn) {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered deconstructed fn for {}; term type that produced the collision was {}",
                    self.label_of_type_id(type_id),
                    std::any::type_name::<T>()
                );
            }
            None => Ok(()),
        }
    }
    pub fn register_constructor<T: dy::Constructor + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let construct_fn =
            |constructor: &ValueGuts, parameter_t: dy::TupleTerm| -> Result<dy::Value> {
                Ok(constructor
                    .downcast_ref::<T>()
                    .unwrap()
                    .construct(parameter_t)?
                    .into())
            };
        match self.construct_fn_m.insert(type_id, construct_fn) {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered construct fn for {}; term type that produced the collision was {}",
                    self.label_of_type_id(type_id),
                    std::any::type_name::<T>()
                );
            }
            None => {}
        }
        let deserialize_parameters_and_construct_fn =
            |constructor: &ValueGuts, reader: &mut dyn std::io::Read| -> Result<dy::Value> {
                Ok(constructor
                    .downcast_ref::<T>()
                    .unwrap()
                    .deserialize_parameters_and_construct(reader)?
                    .into())
            };
        match self
            .deserialize_parameters_and_construct_fn_m
            .insert(type_id, deserialize_parameters_and_construct_fn)
        {
            Some(_) => {
                anyhow::bail!(
                    "collision with already-registered deserialize_parameters_and_construct fn for {}; term type that produced the collision was {}",
                    self.label_of_type_id(type_id),
                    std::any::type_name::<T>()
                );
            }
            None => {}
        }
        Ok(())
    }
    pub fn register_non_parametric_term<T: st::NonParametricTermTrait + 'static>(
        &mut self,
    ) -> Result<()> {
        self.non_parametric_term_code_m
            .insert(TypeId::of::<T>(), T::NON_PARAMETRIC_TERM_CODE);
        let non_parametric_term_instantiate_fn =
            || -> dy::Value { dy::Value::from(T::instantiate()) };
        match self
            .non_parametric_term_instantiate_from_identifier_fn_m
            .insert(T::IDENTIFIER, non_parametric_term_instantiate_fn)
        {
            Some(_) => {
                anyhow::bail!("collision with already-registered non_parametric_term_instantiate_from_identifier fn for {}", T::IDENTIFIER);
            }
            None => {}
        }
        match self.non_parametric_term_instantiate_from_code_fn_m.insert(
            T::NON_PARAMETRIC_TERM_CODE,
            non_parametric_term_instantiate_fn,
        ) {
            Some(_) => {
                anyhow::bail!("collision with already-registered non_parametric_term_instantiate_from_code fn for {}", T::NON_PARAMETRIC_TERM_CODE);
            }
            None => {}
        }
        Ok(())
    }

    pub(crate) fn inhabits_fn<'a, Lhs: Inhabits<Rhs> + 'static, Rhs: st::TypeTrait + 'static>(
        &'a self,
    ) -> Option<&'a BinaryPredicate> {
        let type_id_pair = (TypeId::of::<Lhs>(), TypeId::of::<Rhs>());
        self.inhabits_fn_m.get(&type_id_pair)
    }

    /// This gives the [non-parametric] label of the concrete type.  For example, even though
    /// GlobalSymRefTerm is referentially transparent, its label is still GlobalSymRefTerm.
    pub fn label_of_type_id(&self, type_id: TypeId) -> String {
        match self.label_fn_m.get(&type_id) {
            Some(label_fn) => label_fn().into(),
            None => format!("{:?}", type_id),
        }
    }
    /// This gives the [non-parametric] label of the concrete type.  For example, even though
    /// GlobalSymRefTerm is referentially transparent, its label is still GlobalSymRefTerm.
    pub fn label_of_value_guts(&self, x: &ValueGuts) -> String {
        let type_id = x.type_id();
        match self.label_fn_m.get(&type_id) {
            Some(label_fn) => label_fn().into(),
            None => format!("{:?}", type_id),
        }
    }
    // Note that this does not use referential transparency.
    pub fn debug(
        &self,
        x: &ValueGuts,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self.debug_fn_m.get(&x.type_id()) {
            Some(debug_fn) => Ok(debug_fn(x, f)?),
            None => {
                // panic!("no debug fn found for {}", self.label_of_value_guts(x));
                log::warn!(
                    "no debug fn found for {}; returning generic default",
                    self.label_of_value_guts(x)
                );
                Ok(write!(f, "!InstanceOf!({})", self.label_of_value_guts(x))?)
            }
        }
    }
    // Note that this does not use referential transparency.  Stringifiable should be renamed to ConcreteText or something.
    pub fn stringify(&self, x: &ValueGuts) -> String {
        match self.stringify_fn_m.get(&x.type_id()) {
            Some(stringify_fn) => stringify_fn(x),
            None => {
                panic!("no stringify fn found for {}", self.label_of_value_guts(x));
                //                 log::warn!("no stringify fn found for {}; returning generic default", self.label_of_value_guts(x));
                //                 format!("InstanceOf({})", self.label_of_value_guts(x))
            }
        }
    }
    //     // Note that this does not use referential transparency.
    //     pub fn serialize_top_level_code(&self, x: &ValueGuts, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         match self.serialize_top_level_code_fn_m.get(&x.type_id()) {
    //             Some(serialize_top_level_code_fn) => Ok(serialize_top_level_code_fn(x, writer)?),
    //             None => {
    //                 panic!("no serialize_top_level_code fn found for {}", self.label_of_value_guts(x));
    //             }
    //         }
    //     }
    //     // Note that this does not use referential transparency.
    //     pub fn serialize_constructor(&self, x: &ValueGuts, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         match self.serialize_construct_fn_m.get(&x.type_id()) {
    //             Some(serialize_construct_fn) => Ok(serialize_construct_fn(x, writer)?),
    //             None => {
    //                 panic!("no serialize_constructor fn found for {}", self.label_of_value_guts(x));
    //             }
    //         }
    //     }
    // Note that this does not use referential transparency.
    // TODO: Should rename to nondereferencing_serialize
    pub fn serialize(&self, x: &ValueGuts, writer: &mut dyn std::io::Write) -> Result<usize> {
        match self.serialize_fn_m.get(&x.type_id()) {
            Some(serialize_fn) => Ok(serialize_fn(x, writer)?),
            None => {
                panic!("no serialize fn found for {}", self.label_of_value_guts(x));
            }
        }
    }
    pub fn eq(&self, lhs: &ValueGuts, rhs: &ValueGuts) -> bool {
        // Handle referential transparency.
        let lhs_dereferenced = self.dereferenced(lhs).expect("dereferenced failed");
        let rhs_dereferenced = self.dereferenced(rhs).expect("dereferenced failed");
        match (lhs_dereferenced, rhs_dereferenced) {
            (
                MaybeDereferencedValue::NonRef(lhs_value_guts),
                MaybeDereferencedValue::NonRef(rhs_value_guts),
            ) => self.eq_impl(lhs_value_guts, rhs_value_guts),
            (
                MaybeDereferencedValue::NonRef(lhs_value_guts),
                MaybeDereferencedValue::Ref(rhs_value_la),
            ) => {
                let rhs_value_g = rhs_value_la.read().unwrap();
                self.eq_impl(lhs_value_guts, rhs_value_g.as_ref())
            }
            (
                MaybeDereferencedValue::Ref(lhs_value_la),
                MaybeDereferencedValue::NonRef(rhs_value_guts),
            ) => {
                let lhs_value_g = lhs_value_la.read().unwrap();
                self.eq_impl(lhs_value_g.as_ref(), rhs_value_guts)
            }
            (
                MaybeDereferencedValue::Ref(lhs_value_la),
                MaybeDereferencedValue::Ref(rhs_value_la),
            ) => {
                let lhs_value_g = lhs_value_la.read().unwrap();
                let rhs_value_g = rhs_value_la.read().unwrap();
                self.eq_impl(lhs_value_g.as_ref(), rhs_value_g.as_ref())
            }
        }
    }
    // This method does only the eq operation, not handling referential transparency.
    fn eq_impl(&self, lhs: &ValueGuts, rhs: &ValueGuts) -> bool {
        let lhs_type_id = lhs.type_id();
        let rhs_type_id = rhs.type_id();
        let is_transposed = lhs_type_id > rhs_type_id;
        let type_id_pair = if is_transposed {
            (rhs_type_id, lhs_type_id)
        } else {
            (lhs_type_id, rhs_type_id)
        };
        match self.eq_fn_m.get(&type_id_pair) {
            Some(registered_eq_fn) => {
                if registered_eq_fn.is_transposed == is_transposed {
                    (registered_eq_fn.eq_fn)(lhs, rhs)
                } else {
                    (registered_eq_fn.eq_fn)(rhs, lhs)
                }
            }
            None => {
                // panic!("no eq fn found for {:?}", (lhs_type_id, rhs_type_id)),
                log::warn!(
                    "no eq fn found for ({}, {}); returning default value of false",
                    self.label_of_value_guts(lhs),
                    self.label_of_value_guts(rhs)
                );
                false
            }
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
            (
                MaybeDereferencedValue::NonRef(x_value_guts),
                MaybeDereferencedValue::NonRef(t_value_guts),
            ) => self.inhabits_impl(x_value_guts, t_value_guts),
            (
                MaybeDereferencedValue::NonRef(x_value_guts),
                MaybeDereferencedValue::Ref(t_value_la),
            ) => {
                let t_value_g = t_value_la.read().unwrap();
                self.inhabits_impl(x_value_guts, t_value_g.as_ref())
            }
            (
                MaybeDereferencedValue::Ref(x_value_la),
                MaybeDereferencedValue::NonRef(t_value_guts),
            ) => {
                let x_value_g = x_value_la.read().unwrap();
                self.inhabits_impl(x_value_g.as_ref(), t_value_guts)
            }
            (MaybeDereferencedValue::Ref(x_value_la), MaybeDereferencedValue::Ref(t_value_la)) => {
                let x_value_g = x_value_la.read().unwrap();
                let t_value_g = t_value_la.read().unwrap();
                self.inhabits_impl(x_value_g.as_ref(), t_value_g.as_ref())
            }
        }
    }
    fn inhabits_impl(&self, x: &ValueGuts, t: &ValueGuts) -> bool {
        let type_id_pair = (x.type_id(), t.type_id());
        match self.inhabits_fn_m.get(&type_id_pair) {
            Some(inhabits_fn) => inhabits_fn(x, t),
            None => {
                // panic!("no inhabits fn found for {:?}", (lhs_type_id, rhs_type_id)),
                log::warn!(
                    "no inhabits fn found for ({}, {}); returning default value of false",
                    // self.label_of(type_id_pair.0),
                    // self.label_of(type_id_pair.1)
                    self.label_of_value_guts(x),
                    self.label_of_value_guts(t)
                );
                false
            }
        }
    }
    pub fn abstract_type_of(&self, x: &ValueGuts) -> Box<ValueGuts> {
        // Handle referential transparency.
        let x_maybe_dereferenced = self.dereferenced(x).expect("dereferenced failed");
        match x_maybe_dereferenced {
            MaybeDereferencedValue::NonRef(x_value_guts) => {
                self.nondereferencing_abstract_type_of(x_value_guts)
            }
            MaybeDereferencedValue::Ref(x_value_la) => {
                let x_value_g = x_value_la.read().unwrap();
                self.nondereferencing_abstract_type_of(x_value_g.as_ref())
            }
        }
    }
    pub(crate) fn nondereferencing_abstract_type_of(&self, x: &ValueGuts) -> Box<ValueGuts> {
        let type_id = x.type_id();
        match self.abstract_type_fn_m.get(&type_id) {
            Some(abstract_type_fn) => abstract_type_fn(x),
            None => {
                panic!(
                    "no abstract_type fn found for {}",
                    self.label_of_value_guts(x)
                );
                //                 log::warn!("no abstract_type fn found for {}; returning default value of Box::<ValueGuts>::new(Type{{ }})", self.label_of(type_id));
                //                 Box::new(Type)
            }
        }
    }
    // Note that clone doesn't use referential transparency.  TODO: Figure out if this is correct.
    pub fn clone(&self, x: &ValueGuts) -> Box<ValueGuts> {
        let type_id = x.type_id();
        match self.clone_fn_m.get(&type_id) {
            Some(clone_fn) => clone_fn(x),
            None => {
                panic!("no clone fn found for {}", self.label_of_value_guts(x));
                // There's probably no reasonable default.
                //                 log::warn!("no clone fn found for {}; returning default value of Box::<ValueGuts>::new(Type{{ }})", self.label_of(type_id));
                //                 Box::new(Type)
            }
        }
    }
    // TODO: Consider renaming this to is_parametric_term
    pub fn is_parametric(&self, x: &ValueGuts) -> bool {
        // Handle referential transparency.
        let x_maybe_dereferenced = self.dereferenced(x).expect("dereferenced failed");
        match x_maybe_dereferenced {
            MaybeDereferencedValue::NonRef(x_value_guts) => {
                self.nondereferencing_is_parametric(x_value_guts)
            }
            MaybeDereferencedValue::Ref(x_value_la) => {
                let x_value_g = x_value_la.read().unwrap();
                self.nondereferencing_is_parametric(x_value_g.as_ref())
            }
        }
    }
    // TODO: Consider renaming this to is_parametric_term_impl
    pub(crate) fn nondereferencing_is_parametric(&self, x: &ValueGuts) -> bool {
        match self.is_parametric_fn_m.get(&x.type_id()) {
            Some(is_parametric_fn) => is_parametric_fn(x),
            None => {
                panic!(
                    "no is_parametric fn found for {}",
                    self.label_of_value_guts(x)
                );
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
            MaybeDereferencedValue::NonRef(x_value_guts) => self.is_type_impl(x_value_guts),
            MaybeDereferencedValue::Ref(x_value_la) => {
                let x_value_g = x_value_la.read().unwrap();
                self.is_type_impl(x_value_g.as_ref())
            }
        }
    }
    fn is_type_impl(&self, x: &ValueGuts) -> bool {
        match self.is_type_fn_m.get(&x.type_id()) {
            Some(is_type_fn) => is_type_fn(x),
            None => {
                panic!("no is_type fn found for {}", self.label_of_value_guts(x));
                // NOTE: A default here probably doesn't make any sense.
                //                 log::warn!("no is_type fn found for ({}, {}); returning default value of false", self.label_of(type_id_pair.0), self.label_of(type_id_pair.1));
                //                 false
            }
        }
    }
    pub fn is_non_parametric_term(&self, x: &ValueGuts) -> bool {
        self.non_parametric_term_code_m.contains_key(&x.type_id())
    }
    /// Returns the NonParametricTermCode value for x if it's a NonParametricTerm, otherwise error.
    pub fn non_parametric_term_code(&self, x: &ValueGuts) -> Result<st::NonParametricTermCode> {
        Ok(self
            .non_parametric_term_code_m
            .get(&x.type_id())
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("this type is not registered as a NonParametricTerm"))?)
    }
    pub fn is_transparent_ref_term(&self, x: &ValueGuts) -> bool {
        self.dereferenced_once_fn_m.contains_key(&x.type_id())
    }
    pub fn dereferenced_once(&self, x: &ValueGuts) -> Result<Arc<RwLock<dy::Value>>> {
        match self.dereferenced_once_fn_m.get(&x.type_id()) {
            Some(dereferenced_once_fn) => Ok(dereferenced_once_fn(x)?),
            None => {
                panic!(
                    "no dereferenced_once fn found for {}",
                    self.label_of_value_guts(x)
                );
                // NOTE: A reasonable default would be Err(anyhow::anyhow!("no dereferenced_once fn found for {}", self.label_of_value_guts(x))
            }
        }
    }
    /// This fully dereferences a value.  If it's not a transparent reference, it just returns it,
    /// otherwise it iterates dereferenced_once until it's not a transparent reference.
    // TODO: Implement some limit to reference nesting.  Or not, and just let the stack overflow and the process crash.
    pub fn dereferenced<'a>(&self, x: &'a ValueGuts) -> Result<MaybeDereferencedValue<'a>> {
        match self.dereferenced_once_fn_m.get(&x.type_id()) {
            Some(dereferenced_once_fn) => Ok(MaybeDereferencedValue::Ref(
                self.dereferenced_inner(dereferenced_once_fn(x)?)?,
            )),
            None => Ok(MaybeDereferencedValue::NonRef(x)),
        }
    }
    // TODO: Implement some limit to reference nesting.  Or not, and just let the stack overflow and the process crash.
    pub(crate) fn dereferenced_inner(
        &self,
        value_la: Arc<RwLock<dy::Value>>,
    ) -> Result<Arc<RwLock<dy::Value>>> {
        let value_g = value_la.read().unwrap();
        match self.dereferenced_once_fn_m.get(&value_g.as_ref().type_id()) {
            Some(dereferenced_once_fn) => {
                Ok(self.dereferenced_inner(dereferenced_once_fn(value_g.as_ref())?)?)
            }
            None => Ok(value_la.clone()),
        }
    }
    pub fn construct(
        &self,
        constructor: &ValueGuts,
        parameter_t: dy::TupleTerm,
    ) -> Result<dy::Value> {
        match self.construct_fn_m.get(&constructor.type_id()) {
            Some(construct_fn) => Ok(construct_fn(constructor, parameter_t)?),
            None => {
                panic!(
                    "no construct fn found for {}",
                    self.label_of_value_guts(constructor)
                );
                // NOTE: A reasonable default would be Err(anyhow::anyhow!("no construct fn found for {}", self.label_of_value_guts(constructor))
            }
        }
    }
    pub fn deserialize_parameters_and_construct(
        &self,
        constructor: &ValueGuts,
        reader: &mut dyn std::io::Read,
    ) -> Result<dy::Value> {
        match self
            .deserialize_parameters_and_construct_fn_m
            .get(&constructor.type_id())
        {
            Some(deserialize_parameters_and_construct_fn) => Ok(
                deserialize_parameters_and_construct_fn(constructor, reader)?,
            ),
            None => {
                panic!(
                    "no deserialize_parameters_and_construct fn found for {}",
                    self.label_of_value_guts(constructor)
                );
                // NOTE: A reasonable default would be Err(anyhow::anyhow!("no deserialize_parameters_and_construct fn found for {}", self.label_of(constructor.type_id()))
            }
        }
    }
    pub fn deconstructed(&self, x: &ValueGuts) -> dy::Deconstruction {
        match self.deconstruct_fn_m.get(&x.type_id()) {
            Some(deconstruct_fn) => deconstruct_fn(x),
            None => {
                panic!(
                    "no deconstructed fn found for {}",
                    self.label_of_value_guts(x)
                );
                // NOTE: A reasonable default would be Err(anyhow::anyhow!("no deconstructed fn found for {}", self.label_of_value_guts(x))
            }
        }
    }
    pub fn non_parametric_term_from_identifier(&self, identifier: &str) -> Result<dy::Value> {
        match self
            .non_parametric_term_instantiate_from_identifier_fn_m
            .get(identifier)
        {
            Some(non_parametric_term_instantiate_from_identifier_fn) => {
                Ok(non_parametric_term_instantiate_from_identifier_fn())
            }
            None => Err(anyhow::anyhow!(
                "NonParametricTerm `{}` not found",
                identifier
            )),
        }
    }
    pub fn non_parametric_term_from_code(
        &self,
        code: st::NonParametricTermCode,
    ) -> Result<dy::Value> {
        match self
            .non_parametric_term_instantiate_from_code_fn_m
            .get(&code)
        {
            Some(non_parametric_term_instantiate_fn) => Ok(non_parametric_term_instantiate_fn()),
            None => Err(anyhow::anyhow!("NonParametricTerm `{}` not found", code)),
        }
    }
    /// Returns true iff T is a term that's been registered in this Runtime.
    pub fn is_registered_term<T: st::TermTrait>(&self) -> bool {
        self.term_s.contains(&TypeId::of::<T>())
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
