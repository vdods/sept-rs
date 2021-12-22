use crate::{
    Bool, BoolType, EmptyType, False, FalseType, Inhabits, Result, Sint8, Sint8Type, Sint16, Sint16Type,
    Sint32, Sint32Type, Sint64, Sint64Type, Stringify, True, TrueType, Uint8, Uint8Type, Uint16, Uint16Type,
    Uint32, Uint32Type, Uint64, Uint64Type, Void, VoidType,
};
use std::{any::{Any, TypeId}, collections::{HashMap, HashSet}};

pub type StringifyFn = fn(x: &dyn Any) -> String;
pub type UnaryPredicate = fn(x: &dyn Any) -> bool;
pub type BinaryPredicate = fn(lhs: &dyn Any, rhs: &dyn Any) -> bool;

struct RegisteredEqualsFn {
    eq_fn: BinaryPredicate,
    is_transposed: bool,
}

/// The sept Runtime is what tracks what types are registered, and the various subtyping
/// (and other) relationships.
#[derive(Default)]
pub struct Runtime {
    // TODO: [po]set of types (poset based on which relationship?)
    // TODO: [po]set of terms(?)

    term_s: HashSet<TypeId>,
    stringify_fn_m: HashMap<TypeId, StringifyFn>,
    eq_fn_m: HashMap<(TypeId, TypeId), RegisteredEqualsFn>,
    inhabits_fn_m: HashMap<(TypeId, TypeId), BinaryPredicate>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut runtime: Runtime = Default::default();

        // TODO: Figure out how to move these into something like "init" fns
        // in the respective modules

        // TODO: Order these in some sensible way

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
        runtime.register_stringify::<Uint8>().unwrap();
        runtime.register_stringify::<Uint16>().unwrap();
        runtime.register_stringify::<Uint32>().unwrap();
        runtime.register_stringify::<Uint64>().unwrap();
        runtime.register_stringify::<Uint8Type>().unwrap();
        runtime.register_stringify::<Uint16Type>().unwrap();
        runtime.register_stringify::<Uint32Type>().unwrap();
        runtime.register_stringify::<Uint64Type>().unwrap();
        runtime.register_stringify::<Void>().unwrap();
        runtime.register_stringify::<VoidType>().unwrap();

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
        runtime.register_eq_fn::<Uint8, Uint8>().unwrap();
        runtime.register_eq_fn::<Uint16, Uint16>().unwrap();
        runtime.register_eq_fn::<Uint32, Uint32>().unwrap();
        runtime.register_eq_fn::<Uint64, Uint64>().unwrap();
        runtime.register_eq_fn::<Uint8Type, Uint8Type>().unwrap();
        runtime.register_eq_fn::<Uint16Type, Uint16Type>().unwrap();
        runtime.register_eq_fn::<Uint32Type, Uint32Type>().unwrap();
        runtime.register_eq_fn::<Uint64Type, Uint64Type>().unwrap();
        runtime.register_eq_fn::<Void, Void>().unwrap();
        runtime.register_eq_fn::<VoidType, VoidType>().unwrap();

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
        runtime.register_inhabits_fn::<Sint8, Sint8Type>().unwrap();
        runtime.register_inhabits_fn::<Sint16, Sint16Type>().unwrap();
        runtime.register_inhabits_fn::<Sint32, Sint32Type>().unwrap();
        runtime.register_inhabits_fn::<Sint64, Sint64Type>().unwrap();
        runtime.register_inhabits_fn::<Uint8, Uint8Type>().unwrap();
        runtime.register_inhabits_fn::<Uint16, Uint16Type>().unwrap();
        runtime.register_inhabits_fn::<Uint32, Uint32Type>().unwrap();
        runtime.register_inhabits_fn::<Uint64, Uint64Type>().unwrap();

        runtime
    }
    pub fn register_term(&mut self, type_id: TypeId) -> Result<()> {
        match self.term_s.insert(type_id) {
            false => Err(anyhow::anyhow!("collision with already-registered term {:?}", type_id)),
            true => Ok(())
        }
    }
    pub fn register_stringify_fn(
        &mut self,
        type_id: TypeId,
        stringify_fn: StringifyFn,
    ) -> Result<()> {
        match self.stringify_fn_m.insert(type_id, stringify_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered stringify fn for {:?}", type_id)),
            None => Ok(())
        }
    }
    pub fn register_stringify<S: Stringify + 'static>(&mut self) -> Result<()> {
        let type_id = TypeId::of::<S>();
        let stringify_fn = |x: &dyn Any| -> String { S::stringify(x.downcast_ref::<S>().unwrap()) };
        match self.stringify_fn_m.insert(type_id, stringify_fn) {
            Some(_) => Err(anyhow::anyhow!("collision with already-registered stringify fn for {:?}", type_id)),
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
            Some(_) => Err(anyhow::anyhow!("collision with already-registered eq fn for {:?}", type_id_pair)),
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
            Some(_) => Err(anyhow::anyhow!("collision with already-registered inhabits fn for {:?}", type_id_pair)),
            None => Ok(())
        }
    }
    pub fn stringify(&self, x: &dyn Any) -> String {
        match self.stringify_fn_m.get(&x.type_id()) {
            Some(stringify_fn) => stringify_fn(x),
            // None => Err(anyhow::anyhow!("no stringify fn found for {:?}", x.type_id())),
            None => {
                // panic!("no stringify fn found for {:?}", x.type_id()),
                log::warn!("no stringify fn found for {:?}; returning generic default", x.type_id());
                format!("ValueHavingTypeId({:?})", x.type_id())
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
                log::warn!("no eq fn found for {:?}; returning default value of false", (lhs_type_id, rhs_type_id));
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
                log::warn!("no inhabits fn found for {:?}; returning default value of false", type_id_pair);
                false
            }
        }
    }
}
