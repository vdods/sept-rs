#![allow(unused_imports)]

use sept::{
    dy::{
        self, ArrayTerm, Constructor, Deconstruct, GlobalSymRefTerm, IntoValue, RUNTIME_LA,
        StructTerm, StructTermTerm, SymbolTable, Textifier, TupleTerm, Value,
    },
    parser,
    scanner,
    st::{
        self,
        Array, ArrayType, Bool, BoolType, EmptyType, False, FalseType,
        Float32, Float32Type, Float64, Float64Type, Inhabits,
        Sint8, Sint8Type, Sint16, Sint16Type, Sint32, Sint32Type, Sint64, Sint64Type, Stringify,
        Struct, StructType,
        TermTrait, True, TrueType, Type, TypeTrait,
        Uint8, Uint8Type, Uint16, Uint16Type, Uint32, Uint32Type, Uint64, Uint64Type,
        Utf8String, Void, VoidType,
    },
};
use std::{any::Any, sync::{Arc, RwLock}};

/// This will run once at load time (i.e. presumably before main function is called).
#[ctor::ctor]
fn overall_init() {
    env_logger::try_init().unwrap();
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_term_and_type() {
    log::debug!("True: {:#?}", True);
    log::debug!("TrueType: {:#?}", TrueType);

    // NOTE: The commented out ones asserting non-inhabitation, if uncommented, would produce
    // compile errors to the effect of "Void doesn't implement Inhabits<FalseType>", which
    // is correct and desired, since these types are known at compile time.

    assert!(Void.inhabits(&VoidType));
//     assert!(!Void.inhabits(&FalseType));
//     assert!(!Void.inhabits(&Type));
//     assert!(!Void.inhabits(&Bool));
//     assert!(!Void.inhabits(&BoolType));

    assert!(VoidType.inhabits(&Type));

    assert!(True.inhabits(&TrueType));
//     assert!(!True.inhabits(&FalseType));
    assert!(True.inhabits(&Bool));
//     assert!(!True.inhabits(&BoolType));

//     assert!(!False.inhabits(&TrueType));
    assert!(False.inhabits(&FalseType));
    assert!(False.inhabits(&Bool));
//     assert!(!False.inhabits(&BoolType));

    assert!(TrueType.inhabits(&BoolType));
    assert!(FalseType.inhabits(&BoolType));
    assert!(Bool.inhabits(&BoolType));
//     assert!(!Bool.inhabits(&TrueType));
//     assert!(!Bool.inhabits(&FalseType));


    assert!(!True.is_parametric());
    assert!(!True.is_type());
    assert!(!TrueType.is_parametric());
    assert!(TrueType.is_type());

    assert!(!False.is_parametric());
    assert!(!False.is_type());
    assert!(!FalseType.is_parametric());
    assert!(FalseType.is_type());

    assert!(true.is_parametric());
    assert!(!true.is_type());
    assert!(false.is_parametric());
    assert!(!false.is_type());
    assert!(!Bool.is_parametric());
    assert!(Bool.is_type());
    assert!(!BoolType.is_parametric());
    assert!(BoolType.is_type());
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_stringify() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert_eq!(runtime_g.stringify(&true), "true");
    assert_eq!(runtime_g.stringify(&false), "false");
    assert_eq!(runtime_g.stringify(&True), "True");
    assert_eq!(runtime_g.stringify(&False), "False");
    assert_eq!(runtime_g.stringify(&TrueType), "TrueType");
    assert_eq!(runtime_g.stringify(&FalseType), "FalseType");
    assert_eq!(runtime_g.stringify(&Bool), "Bool");
    assert_eq!(runtime_g.stringify(&BoolType), "BoolType");

    log::debug!("RUNTIME_LA.stringify(&123): {:#?}", runtime_g.stringify(&123));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_eq() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert!(runtime_g.eq(&true, &true));
    assert!(!runtime_g.eq(&true, &false));
    assert!(runtime_g.eq(&true, &True));
    assert!(!runtime_g.eq(&true, &False));

    assert!(!runtime_g.eq(&false, &true));
    assert!(runtime_g.eq(&false, &false));
    assert!(!runtime_g.eq(&false, &True));
    assert!(runtime_g.eq(&false, &False));

    assert!(runtime_g.eq(&True, &true));
    assert!(!runtime_g.eq(&True, &false));
    assert!(runtime_g.eq(&True, &True));
    assert!(!runtime_g.eq(&True, &False));

    assert!(!runtime_g.eq(&False, &true));
    assert!(runtime_g.eq(&False, &false));
    assert!(!runtime_g.eq(&False, &True));
    assert!(runtime_g.eq(&False, &False));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_inhabits() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert!(runtime_g.inhabits(&true, &Bool));
    assert!(runtime_g.inhabits(&false, &Bool));
    assert!(!runtime_g.inhabits(&true, &FalseType));
    assert!(runtime_g.inhabits(&false, &FalseType));
    assert!(runtime_g.inhabits(&true, &TrueType));
    assert!(!runtime_g.inhabits(&false, &TrueType));
    assert!(runtime_g.inhabits(&True, &Bool));
    assert!(runtime_g.inhabits(&False, &Bool));
    assert!(runtime_g.inhabits(&Bool, &BoolType));
    assert!(!runtime_g.inhabits(&BoolType, &Bool));
    assert!(runtime_g.inhabits(&Void, &VoidType));
    assert!(!runtime_g.inhabits(&VoidType, &Void));

    assert!(!runtime_g.inhabits(&Bool, &EmptyType));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_ints() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert!(runtime_g.inhabits(&123i8, &Sint8));
    assert!(runtime_g.inhabits(&123i16, &Sint16));
    assert!(runtime_g.inhabits(&123i32, &Sint32));
    assert!(runtime_g.inhabits(&123i64, &Sint64));

    assert!(runtime_g.inhabits(&123u8, &Uint8));
    assert!(runtime_g.inhabits(&123u16, &Uint16));
    assert!(runtime_g.inhabits(&123u32, &Uint32));
    assert!(runtime_g.inhabits(&123u64, &Uint64));

    assert!(runtime_g.inhabits(&Sint8, &Sint8Type));
    assert!(runtime_g.inhabits(&Sint16, &Sint16Type));
    assert!(runtime_g.inhabits(&Sint32, &Sint32Type));
    assert!(runtime_g.inhabits(&Sint64, &Sint64Type));

    assert!(runtime_g.inhabits(&Uint8, &Uint8Type));
    assert!(runtime_g.inhabits(&Uint16, &Uint16Type));
    assert!(runtime_g.inhabits(&Uint32, &Uint32Type));
    assert!(runtime_g.inhabits(&Uint64, &Uint64Type));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_floats() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert!(runtime_g.inhabits(&5.875f32, &Float32));
    assert!(runtime_g.inhabits(&5.875f64, &Float64));

    assert!(runtime_g.inhabits(&Float32, &Float32Type));
    assert!(runtime_g.inhabits(&Float64, &Float64Type));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_arrays() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    // Note that Vec<Value> is ArrayTerm.
    // Note also that this is constructing a Vec with nonhomogeneous elements, because
    // Value stores Box<dyn Any>.
    let a0 = ArrayTerm::from(vec![3i32.into(), 5.5f32.into()]);
    log::debug!("a0: {}", a0);
    log::debug!("a0 (as Debug): {:?}", a0);
    log::debug!("a0.stringify(): {}", a0.stringify());

    assert!(runtime_g.inhabits(&a0, &Array));
    assert!(runtime_g.inhabits(&Array, &ArrayType));

//     let a1 = vec![100i8, 101i8, 99i8, 10i8];
//     log::debug!("a1: {:?}", a1);
//     log::debug!("a1.stringify(): {}", a1.stringify());
//
//     assert!(a1.inhabits(&Array));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_tuples() {
    let t1 = TupleTerm::from(vec![3i32.into(), 5.5f32.into()]);
    let t2 = TupleTerm::from(vec![Sint32.into(), Float32.into()]);
    log::debug!("t1: {}", t1);
    log::debug!("t2: {}", t2);
    log::debug!("t1.abstract_type(): {}", t1.abstract_type());
    log::debug!("t2.abstract_type(): {}", t2.abstract_type());

    assert!(t1.inhabits(&t2));
    assert!(t1.is_parametric());
    assert!(t2.is_parametric());
    assert!(!t1.is_type());
    assert!(t2.is_type());

    let t3 = TupleTerm::from((147u32, 5.67f32));
    let t4 = TupleTerm::from((147u32, Value::from(5.67f32)));
    let t5 = TupleTerm::from((Value::from(147u32), Value::from(5.67f32)));
    log::debug!("t3: {}", t3.stringify());
    log::debug!("t4: {}", t4.stringify());
    log::debug!("t5: {}", t5.stringify());

    assert_eq!(t3, t4);
    assert_eq!(t3, t5);
    assert_eq!(t4, t5);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_abstract_type() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    {
        let x = &Void;
        log::debug!("runtime_g.abstract_type_of({}): {}", runtime_g.stringify(x), runtime_g.stringify(runtime_g.abstract_type_of(x).as_ref()));
    }

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Type).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Void).as_ref(), &VoidType));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&VoidType).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Bool).as_ref(), &BoolType));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&BoolType).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&True).as_ref(), &TrueType));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&TrueType).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&False).as_ref(), &FalseType));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&FalseType).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint8).as_ref(), &Sint8Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint8Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint16).as_ref(), &Sint16Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint16Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint32).as_ref(), &Sint32Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint32Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint64).as_ref(), &Sint64Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint64Type).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint8).as_ref(), &Uint8Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint8Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint16).as_ref(), &Uint16Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint16Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint32).as_ref(), &Uint32Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint32Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint64).as_ref(), &Uint64Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint64Type).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float32).as_ref(), &Float32Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float32Type).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float64).as_ref(), &Float64Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float64Type).as_ref(), &Type));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_value() {
    let v1 = Value::from(3i32);
    let v2 = Value::from(7i32);

    log::debug!("Value::label(): {:?}", Value::label());
    log::debug!("v1.stringify(): {:?}", v1.stringify());
    log::debug!("v2.stringify(): {:?}", v2.stringify());
    log::debug!("v1.abstract_type(): {:?}", v1.abstract_type());

    log::debug!("v1.inhabits(&Sint32): {:?}", v1.inhabits(&Sint32));
    log::debug!("v1.inhabits(&Bool): {:?}", v1.inhabits(&Bool));
    log::debug!("v1.inhabits(&Value::from(Sint32)): {:?}", v1.inhabits(&Value::from(Sint32)));
    log::debug!("v1.inhabits(&Value::from(Bool)): {:?}", v1.inhabits(&Value::from(Bool)));
    log::debug!("v1.inhabits(&v2): {:?}", v1.inhabits(&v2));
    let v3 = Value::from(Sint32);
    log::debug!("v1.inhabits(&v3): {:?}", v1.inhabits(&v3));

    log::debug!("v1: {}", v1);
    log::debug!("v2: {}", v2);

    log::debug!("v1 (as Debug): {:?}", v1);
    log::debug!("v2 (as Debug): {:?}", v2);

    log::debug!("v1 == v1: {:?}", v1 == v1);
    log::debug!("v1 == v2: {:?}", v1 == v2);
    log::debug!("v2 == v1: {:?}", v2 == v1);
    log::debug!("v2 == v2: {:?}", v2 == v2);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_symbol_table() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    let mut symbol_table = SymbolTable::new_with_parent(None);
    assert!(!symbol_table.symbol_is_defined("blah"));
    symbol_table.define_symbol("blah", Value::from(123i32)).expect("test");
    assert!(symbol_table.symbol_is_defined("blah"));
    assert_eq!(*symbol_table.resolved_symbol("blah").expect("test").read().unwrap(), Value::from(123i32));

    log::debug!("symbol_table: {:#?}", symbol_table);

    // Now check GLOBAL_SYMBOL_TABLE_LA
    {
        assert!(!dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap().symbol_is_defined("bleh"));

        // Have to acquire a separate write lock.
        dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().define_symbol("bleh", Value::from(456f32)).expect("test");

        // Now acquire a read lock.
        let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
        assert!(global_symbol_table_g.symbol_is_defined("bleh"));
        assert_eq!(*global_symbol_table_g.resolved_symbol("bleh").expect("test").read().unwrap(), Value::from(456f32));

        log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
    }

    // Test out parent symbol tables.
    let parent_symbol_table_la = Arc::new(RwLock::new(SymbolTable::new_with_parent(None)));
    let child_symbol_table_la = Arc::new(RwLock::new(SymbolTable::new_with_parent(Some(parent_symbol_table_la.clone()))));

    parent_symbol_table_la.write().unwrap().define_symbol("stuff", Value::from(200u32)).expect("test");
    parent_symbol_table_la.write().unwrap().define_symbol("hippo", Value::from(300u32)).expect("test");

    child_symbol_table_la.write().unwrap().define_symbol("stuff", Value::from(444u32)).expect("test");

    assert_eq!(*parent_symbol_table_la.read().unwrap().resolved_symbol("stuff").expect("test").read().unwrap(), Value::from(200u32));
    assert_eq!(*parent_symbol_table_la.read().unwrap().resolved_symbol("hippo").expect("test").read().unwrap(), Value::from(300u32));

    assert_eq!(*child_symbol_table_la.read().unwrap().resolved_symbol("stuff").expect("test").read().unwrap(), Value::from(444u32));
    assert_eq!(*child_symbol_table_la.read().unwrap().resolved_symbol("hippo").expect("test").read().unwrap(), Value::from(300u32));

    log::debug!("child_symbol_table:\n{:#?}", child_symbol_table_la.read().unwrap());
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_global_sym_ref_term() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    // Write a bunch of stuff into the global_symbol_table
    {
        let mut global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap();
        global_symbol_table_g.define_symbol("bleh", Value::from(456f32)).expect("test");
        global_symbol_table_g.define_symbol("stuff", Value::from(True{})).expect("test");
        global_symbol_table_g.define_symbol("andthings", Value::from(Void{})).expect("test");
    }

    // Now check GlobalSymRefTerm.
    let r = GlobalSymRefTerm::new_unchecked("bleh".into());
    log::debug!("r (as Debug): {:#?}", r);
    log::debug!("r (as Display): {}" , r);
    log::debug!("r: {}" , r.stringify());

    let t = TupleTerm::from(vec![
        GlobalSymRefTerm::new_unchecked("bleh".into()).into(),
        GlobalSymRefTerm::new_unchecked("stuff".into()).into(),
        GlobalSymRefTerm::new_unchecked("andthings".into()).into(),
    ]);
    log::debug!("t (as Debug): {:#?}", t);
    log::debug!("t (as Display): {}", t);
    log::debug!("t: {}", t.stringify());

    // Test dereferenced
    {
        let r_resolved_la = r.resolved().expect("test");
        let r_resolved_g = r_resolved_la.read().unwrap();
        log::debug!("r_resolved_g (as Debug): {:#?}", r_resolved_g);
        log::debug!("r_resolved_g.as_ref() (as Debug): {:#?}", r_resolved_g.as_ref());

    //     use std::ops::Deref; // Is this somehow unnecessary?
        log::debug!("r_resolved_g (as Display): {}", r_resolved_g);
        log::debug!("r_resolved_g: {}", r_resolved_g.stringify());

        // TODO: Figure out how to not have to use * (or maybe that's not actually possible).
        assert_eq!(*r_resolved_g, Value::from(456f32));
    }

    // Test mutation
    {
        let r_resolved_la = r.resolved().expect("test");
        let mut r_resolved_g = r_resolved_la.write().unwrap();
        log::debug!("r_resolved_g (as Debug): {:#?}", r_resolved_g);
        log::debug!("r_resolved_g.as_ref() (as Debug): {:#?}", r_resolved_g.as_ref());

    //     use std::ops::Deref; // Is this somehow unnecessary?
        log::debug!("r_resolved_g (as Display): {}", r_resolved_g);
        log::debug!("r_resolved_g: {}", r_resolved_g.stringify());

        // Now try modifying it
        log::debug!("adding 1.0 to r...");
        *r_resolved_g.as_mut().downcast_mut::<f32>().unwrap() += 1.0f32;

        log::debug!("r_resolved_g: {}", r_resolved_g.stringify());
        assert_eq!(*r_resolved_g, Value::from(457f32));
    }

    // Test nested references
    {
        let test_value = Value::from(40404u32);

        // Write more stuff into the global_symbol_table
        {
            let mut global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap();
            global_symbol_table_g.define_symbol("inner", Value::from(40404u32)).expect("test");
            global_symbol_table_g.define_symbol("outer", Value::from(GlobalSymRefTerm::new_unchecked("inner".into()))).expect("test");
            global_symbol_table_g.define_symbol("outerer", Value::from(GlobalSymRefTerm::new_unchecked("outer".into()))).expect("test");
        }

        {
            let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
            log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
        }

        let inner_ref = GlobalSymRefTerm::new_unchecked("inner".into());
        let outer_ref = GlobalSymRefTerm::new_unchecked("outer".into());
        let outerer_ref = GlobalSymRefTerm::new_unchecked("outerer".into());

        let runtime_g = dy::RUNTIME_LA.read().unwrap();

        {
            let inner_dereferenced_once_la = runtime_g.dereferenced_once(&inner_ref).expect("test");
            let inner_dereferenced_once_g = inner_dereferenced_once_la.read().unwrap();
            log::debug!("inner_dereferenced_once_g (as Debug): {:#?}", inner_dereferenced_once_g);
            log::debug!("inner_dereferenced_once_g: {}", inner_dereferenced_once_g.stringify());
            assert_eq!(*inner_dereferenced_once_g, test_value);
        }

        {
            let outer_dereferenced_once_la = runtime_g.dereferenced_once(&outer_ref).expect("test");
            let outer_dereferenced_once_g = outer_dereferenced_once_la.read().unwrap();
            log::debug!("outer_dereferenced_once_g (as Debug): {:#?}", outer_dereferenced_once_g);
            log::debug!("outer_dereferenced_once_g: {}", outer_dereferenced_once_g.stringify());
            assert_eq!(*outer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        {
            let outerer_dereferenced_once_la = runtime_g.dereferenced_once(&outerer_ref).expect("test");
            let outerer_dereferenced_once_g = outerer_dereferenced_once_la.read().unwrap();
            log::debug!("outerer_dereferenced_once_g (as Debug): {:#?}", outerer_dereferenced_once_g);
            log::debug!("outerer_dereferenced_once_g: {}", outerer_dereferenced_once_g.stringify());
            assert_eq!(*outerer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        assert_eq!(*inner_ref.resolved().expect("test").read().unwrap(), test_value);
        assert_eq!(*outer_ref.resolved().expect("test").read().unwrap(), test_value);
        assert_eq!(*outerer_ref.resolved().expect("test").read().unwrap(), test_value);
    }
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_local_sym_ref_term() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    let local_symbol_table_la = Arc::new(RwLock::new(SymbolTable::new_with_parent(None)));
    local_symbol_table_la.write().unwrap().define_symbol("blah", dy::Value::from(123i32)).expect("test");
    log::debug!("local_symbol_table_la: {:#?}", local_symbol_table_la.read().unwrap());

    let local_sym_ref_term = dy::LocalSymRefTerm::new_checked(local_symbol_table_la.clone(), "blah".into()).expect("test");
    log::debug!("local_sym_ref_term: (as Debug) {:#?}", local_sym_ref_term);
    log::debug!("local_sym_ref_term: (as Display) {}", local_sym_ref_term);
    log::debug!("local_sym_ref_term: {}", local_sym_ref_term.stringify());

    // Test nested references
    {
        let test_value = Value::from(51515u32);

        // Write more stuff into the local_symbol_table
        {
            let mut local_symbol_table_g = local_symbol_table_la.write().unwrap();
            local_symbol_table_g.define_symbol("inner", Value::from(51515u32)).expect("test");
            local_symbol_table_g.define_symbol("outer", Value::from(dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "inner".into()))).expect("test");
            local_symbol_table_g.define_symbol("outerer", Value::from(dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "outer".into()))).expect("test");
        }

        {
            let local_symbol_table_g = local_symbol_table_la.read().unwrap();
            log::debug!("local_symbol_table_g: {:#?}", local_symbol_table_g);
        }

        let inner_ref = dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "inner".into());
        let outer_ref = dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "outer".into());
        let outerer_ref = dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "outerer".into());

        let runtime_g = dy::RUNTIME_LA.read().unwrap();

        {
            let inner_dereferenced_once_la = runtime_g.dereferenced_once(&inner_ref).expect("test");
            let inner_dereferenced_once_g = inner_dereferenced_once_la.read().unwrap();
            log::debug!("inner_dereferenced_once_g (as Debug): {:#?}", inner_dereferenced_once_g);
            log::debug!("inner_dereferenced_once_g: {}", inner_dereferenced_once_g.stringify());
            assert_eq!(*inner_dereferenced_once_g, test_value);
        }

        {
            let outer_dereferenced_once_la = runtime_g.dereferenced_once(&outer_ref).expect("test");
            let outer_dereferenced_once_g = outer_dereferenced_once_la.read().unwrap();
            log::debug!("outer_dereferenced_once_g (as Debug): {:#?}", outer_dereferenced_once_g);
            log::debug!("outer_dereferenced_once_g: {}", outer_dereferenced_once_g.stringify());
            assert_eq!(*outer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        {
            let outerer_dereferenced_once_la = runtime_g.dereferenced_once(&outerer_ref).expect("test");
            let outerer_dereferenced_once_g = outerer_dereferenced_once_la.read().unwrap();
            log::debug!("outerer_dereferenced_once_g (as Debug): {:#?}", outerer_dereferenced_once_g);
            log::debug!("outerer_dereferenced_once_g: {}", outerer_dereferenced_once_g.stringify());
            assert_eq!(*outerer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        assert_eq!(*inner_ref.resolved().expect("test").read().unwrap(), test_value);
        assert_eq!(*outer_ref.resolved().expect("test").read().unwrap(), test_value);
        assert_eq!(*outerer_ref.resolved().expect("test").read().unwrap(), test_value);
    }
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_structs() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    log::debug!("Struct: {}", Struct.stringify());
    log::debug!("StructType: {}", StructType.stringify());

    assert!(Struct.inhabits(&StructType));

    // Create the Hippo struct
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap()
        .define_symbol(
            "Hippo",
            StructTerm::new(
                vec![("age".into(), Uint8.into()), ("gravity".into(), Float64.into())].into()
            ).expect("test").into()
        ).expect("test");

    let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
    log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
    let hippo = GlobalSymRefTerm::new_unchecked("Hippo".into());
    log::debug!("hippo: {}", hippo.stringify());

    let x = global_symbol_table_g
        .resolved_symbol("Hippo").expect("test")
        .read().unwrap()
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![23u8.into(), 999.0f64.into()].into()).expect("test");
    let y = global_symbol_table_g
        .resolved_symbol("Hippo").expect("test")
        .read().unwrap()
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![100u8.into(), (-3.0f64).into()].into()).expect("test");
    log::debug!("x: {}", x.stringify());
    log::debug!("y: {}", y.stringify());
    log::debug!("x == y: {}", x == y);

    assert_eq!(x, x);
    assert_eq!(y, y);
    assert!(x != y);
    assert!(y != x);

    let x2 = dy::StructTermTerm::new_checked(hippo.clone().into(), vec![23u8.into(), 999.0f64.into()].into()).expect("test");
    let y2 = dy::StructTermTerm::new_checked(hippo.clone().into(), vec![100u8.into(), (-3.0f64).into()].into()).expect("test");

    log::debug!("x2: {}", x2.stringify());
    log::debug!("y2: {}", y2.stringify());
    log::debug!("x2 == y2: {}", x2 == y2);

    assert_eq!(x2, x2);
    assert_eq!(y2, y2);
    assert!(x2 != y2);
    assert!(y2 != x2);

    assert_eq!(x, x2);
    assert_eq!(y, y2);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_deconstruct() {
    let n = 123u32;
    log::debug!("n (stringify): {}", n.stringify());
    {
        let deconstruction = n.deconstructed();
        log::debug!("n.deconstructed(): {:#?}", deconstruction);
    }
    {
        let deconstruction = n.deconstruct();
        log::debug!("n.deconstruct(): {:#?}", deconstruction);
    }

    let x = 5.67f64;
    log::debug!("x (stringify): {}", x.stringify());
    {
        let deconstruction = x.deconstructed();
        log::debug!("x.deconstructed(): {:#?}", deconstruction);
    }
    {
        let deconstruction = x.deconstruct();
        log::debug!("x.deconstruct(): {:#?}", deconstruction);
    }

    let b = true;
    log::debug!("b (stringify): {}", b.stringify());
    {
        let deconstruction = b.deconstructed();
        log::debug!("b.deconstructed(): {:#?}", deconstruction);
    }
    {
        let deconstruction = b.deconstruct();
        log::debug!("b.deconstruct(): {:#?}", deconstruction);
    }

    let a = Array;
    log::debug!("a (stringify): {}", a.stringify());
    {
        let deconstruction = a.deconstructed();
        log::debug!("a.deconstructed(): {:#?}", deconstruction);
    }
    {
        let deconstruction = a.deconstruct();
        log::debug!("a.deconstruct(): {:#?}", deconstruction);
    }

    {
        let dy_tt = TupleTerm::from(vec![n.into(), x.into(), b.into(), a.into()]);
        log::debug!("dy_tt (stringify): {}", dy_tt.stringify());
        {
            let deconstruction = dy_tt.deconstructed();
            log::debug!("dy_tt.deconstructed(): {:#?}", deconstruction);
        }
        {
            let deconstruction = dy_tt.deconstruct();
            log::debug!("dy_tt.deconstruct(): {:#?}", deconstruction);
        }
    }


    {
        let st_tt = TupleTerm::from((n, x, b, a));
        log::debug!("st_tt (stringify): {}", st_tt.stringify());
        {
            let deconstruction = st_tt.deconstructed();
            log::debug!("st_tt.deconstructed(): {:#?}", deconstruction);
        }
        {
            let deconstruction = st_tt.deconstruct();
            log::debug!("st_tt.deconstruct(): {:#?}", deconstruction);
        }
    }

    {
        let at = ArrayTerm::from(vec![n.into(), x.into(), b.into(), a.into()]);
        log::debug!("at (stringify): {}", at.stringify());
        {
            let deconstruction = at.deconstructed();
            log::debug!("at.deconstructed(): {:#?}", deconstruction);
        }
        {
            let deconstruction = at.deconstruct();
            log::debug!("at.deconstruct(): {:#?}", deconstruction);
        }
    }


    {
        let s = StructTerm::new(vec![("name".into(), Utf8String.into()), ("age".into(), Uint8.into())]).expect("test");
        log::debug!("s (stringify): {}", s.stringify());
        {
            let deconstruction = s.deconstructed();
            log::debug!("s.deconstructed(): {:#?}", deconstruction);
        }
        {
            let deconstruction = s.clone().deconstruct();
            log::debug!("s.deconstruct(): {:#?}", deconstruction);
        }

        let s_term = StructTermTerm::new_checked(s.clone().into(), TupleTerm::from((String::from("Hippo"), 99u8))).expect("test");
        log::debug!("s_term (stringify): {}", s_term.stringify());
        {
            let deconstruction = s_term.deconstructed();
            log::debug!("s_term.deconstructed(): {:#?}", deconstruction);
        }
        {
            let deconstruction = s_term.deconstruct();
            log::debug!("s_term.deconstruct(): {:#?}", deconstruction);
        }
    }
}

fn test_deconstruct_reconstruct_roundtrip<T, C>(x: T)
where
    T: Deconstruct + Stringify + PartialEq,
    C: Constructor + Stringify,
    <C as Constructor>::ConstructedType: std::fmt::Display + PartialEq<T>
{
    log::debug!("x: {}", x.stringify());
    log::debug!("x (as Debug): {:#?}", x);
    let x_deconstruction = x.deconstructed();
    log::debug!("x_deconstruction (as Debug): {:#?}", x_deconstruction);
    assert_eq!(x_deconstruction.kind(), dy::DeconstructionKind::Parametric);
    let parameterization = x_deconstruction.clone().into_parametric().unwrap();
    log::debug!("parameterization (as Debug): {:#?}", parameterization);
    assert!(parameterization.constructor_d.reconstruct().expect("test").is::<C>());
    let x_reconstructed = x_deconstruction.reconstruct().expect("test");
    log::debug!("x_reconstructed (as Debug): {:#?}", x_reconstructed);
    log::debug!("x_reconstructed: {:#?}", x_reconstructed.stringify());
    assert!(x_reconstructed.is::<T>());
    assert_eq!(*x_reconstructed.downcast_ref::<T>().unwrap(), x);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_constructor() {
    test_deconstruct_reconstruct_roundtrip::<bool, Bool>(true);
    test_deconstruct_reconstruct_roundtrip::<bool, Bool>(false);
    test_deconstruct_reconstruct_roundtrip::<i8, Sint8>(123i8);
    test_deconstruct_reconstruct_roundtrip::<i16, Sint16>(123i16);
    test_deconstruct_reconstruct_roundtrip::<i32, Sint32>(123i32);
    test_deconstruct_reconstruct_roundtrip::<i64, Sint64>(123i64);
    test_deconstruct_reconstruct_roundtrip::<u8, Uint8>(99u8);
    test_deconstruct_reconstruct_roundtrip::<u16, Uint16>(99u16);
    test_deconstruct_reconstruct_roundtrip::<u32, Uint32>(99u32);
    test_deconstruct_reconstruct_roundtrip::<u64, Uint64>(99u64);
    test_deconstruct_reconstruct_roundtrip::<f32, Float32>(100.25f32);
    test_deconstruct_reconstruct_roundtrip::<f64, Float64>(100.25f64);
    test_deconstruct_reconstruct_roundtrip::<String, Utf8String>("BLAH".into());

    test_deconstruct_reconstruct_roundtrip::<TupleTerm, st::Tuple>(TupleTerm::from((123i8, 99u32, 100.25f32, String::from("HIPPO"))));
}

fn test_textify_case<T: std::fmt::Debug + dy::Deconstruct>(value: T, expected_text: &str) {
    let text = value.textified();
    log::debug!("value `{:?}` textified: {}", value, text);
    assert_eq!(text, expected_text);

    let text2 = format!("{}", Textifier::from(&value));
    assert_eq!(text2, expected_text);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_textify() {
    test_textify_case(true, "Bool(true)");
    test_textify_case(false, "Bool(false)");
    test_textify_case(True, "True");
    test_textify_case(False, "False");
    test_textify_case(Void, "Void");
    test_textify_case(VoidType, "VoidType");
    test_textify_case(123i8, "Sint8(123)");
    test_textify_case(123i16, "Sint16(123)");
    test_textify_case(123i32, "Sint32(123)");
    test_textify_case(123i64, "Sint64(123)");
    test_textify_case(123u8, "Uint8(123)");
    test_textify_case(123u16, "Uint16(123)");
    test_textify_case(123u32, "Uint32(123)");
    test_textify_case(123u64, "Uint64(123)");
    test_textify_case(4.75f32, "Float32(4.75)");
    test_textify_case(4.75f64, "Float64(4.75)");
    test_textify_case(String::from("Hippos and Hippas"), "Utf8String(\"Hippos and Hippas\")");
    test_textify_case(TupleTerm::from((123i8, 99u32, 100.25f32, String::from("HIPPO"))), "Tuple(Sint8(123), Uint32(99), Float32(100.25), Utf8String(\"HIPPO\"))");
    test_textify_case(
        StructTerm::new(
            vec![("name".into(), Utf8String.into()), ("score".into(), Uint64.into())],
        ).expect("test"),
        "Struct(Tuple(Utf8String(\"name\"), Utf8String), Tuple(Utf8String(\"score\"), Uint64))",
    );
}

fn test_try_scanning_case(token_kind: scanner::TokenKind, input: &str, expected_token_o: Option<scanner::Token>) {
    let token_o = scanner::try_scanning(token_kind, input).map(|(token, _match_stats)| token);
    assert_eq!(token_o, expected_token_o);
}

fn test_try_scanning_case_ascii_string_literal(input: &str) {
    // NOTE: Not 100% sure if Rust's string literals match this.  That isn't the design requirement though,
    // it should be somewhat platform agnostic, but generally should work with C and Rust.
    let input_str_literal = format!("{:?}", input);
    test_try_scanning_case(
        scanner::TokenKind::AsciiStringLiteral,
        &input_str_literal,
        Some(scanner::AsciiStringLiteral::from(input_str_literal.as_ref()).into()),
    );
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_try_scanning() {
    use scanner::{Token, TokenKind};

    test_try_scanning_case(TokenKind::EndOfInput, "", Some(Token::EndOfInput));
    test_try_scanning_case(TokenKind::EndOfInput, "x", None);

    test_try_scanning_case(TokenKind::OpenParen, "(", Some(Token::OpenParen));
    test_try_scanning_case(TokenKind::OpenParen, "(x", Some(Token::OpenParen));
    test_try_scanning_case(TokenKind::OpenParen, "((", Some(Token::OpenParen));
    test_try_scanning_case(TokenKind::OpenParen, "x", None);
    test_try_scanning_case(TokenKind::OpenParen, ")", None);
    test_try_scanning_case(TokenKind::OpenParen, "", None);
    test_try_scanning_case(TokenKind::OpenParen, "\x7F", None);

    test_try_scanning_case(TokenKind::CloseParen, ")", Some(Token::CloseParen));
    test_try_scanning_case(TokenKind::CloseParen, ")x", Some(Token::CloseParen));
    test_try_scanning_case(TokenKind::CloseParen, "))", Some(Token::CloseParen));
    test_try_scanning_case(TokenKind::CloseParen, "(", None);
    test_try_scanning_case(TokenKind::CloseParen, "x", None);
    test_try_scanning_case(TokenKind::CloseParen, "", None);
    test_try_scanning_case(TokenKind::CloseParen, "\x7F", None);

    test_try_scanning_case(TokenKind::Comma, ",", Some(Token::Comma));
    test_try_scanning_case(TokenKind::Comma, ",x", Some(Token::Comma));
    test_try_scanning_case(TokenKind::Comma, ",,", Some(Token::Comma));
    test_try_scanning_case(TokenKind::Comma, "x", None);
    test_try_scanning_case(TokenKind::Comma, "", None);
    test_try_scanning_case(TokenKind::Comma, "\x7F", None);

    test_try_scanning_case(TokenKind::Whitespace, " ", Some(Token::Whitespace(" ".into())));
    test_try_scanning_case(TokenKind::Whitespace, "\t", Some(Token::Whitespace("\t".into())));
    test_try_scanning_case(TokenKind::Whitespace, "\n", Some(Token::Whitespace("\n".into())));
    test_try_scanning_case(TokenKind::Whitespace, "    \t  \n\n\n", Some(Token::Whitespace("    \t  \n\n\n".into())));
    test_try_scanning_case(TokenKind::Whitespace, "    \t  \n\n\n!", Some(Token::Whitespace("    \t  \n\n\n".into())));
    test_try_scanning_case(TokenKind::Whitespace, "x", None);
    test_try_scanning_case(TokenKind::Whitespace, "", None);
    test_try_scanning_case(TokenKind::Whitespace, "\x7F", None);

    test_try_scanning_case(TokenKind::CIdentifier, "xyz", Some(Token::CIdentifier("xyz".into())));
    test_try_scanning_case(TokenKind::CIdentifier, "ABC__123", Some(Token::CIdentifier("ABC__123".into())));
    test_try_scanning_case(TokenKind::CIdentifier, "_x_100y_z", Some(Token::CIdentifier("_x_100y_z".into())));
    test_try_scanning_case(TokenKind::CIdentifier, "_x_100y_z.", Some(Token::CIdentifier("_x_100y_z".into())));
    test_try_scanning_case(TokenKind::CIdentifier, ".", None);
    test_try_scanning_case(TokenKind::CIdentifier, "1", None);
    test_try_scanning_case(TokenKind::CIdentifier, "(", None);
    test_try_scanning_case(TokenKind::CIdentifier, ")", None);
    test_try_scanning_case(TokenKind::CIdentifier, "", None);
    test_try_scanning_case(TokenKind::CIdentifier, "\x7F", None);


    test_try_scanning_case(TokenKind::DecimalLiteral, "0.", Some(Token::DecimalLiteral("0.".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, ".0", Some(Token::DecimalLiteral(".0".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "1.", Some(Token::DecimalLiteral("1.".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, ".2", Some(Token::DecimalLiteral(".2".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "3.4", Some(Token::DecimalLiteral("3.4".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "-3.4", Some(Token::DecimalLiteral("-3.4".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "-3.", Some(Token::DecimalLiteral("-3.".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "-.4", Some(Token::DecimalLiteral("-.4".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+3.4", Some(Token::DecimalLiteral("+3.4".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+3.", Some(Token::DecimalLiteral("+3.".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+.4", Some(Token::DecimalLiteral("+.4".into())));

    test_try_scanning_case(TokenKind::DecimalLiteral, "0.e0", Some(Token::DecimalLiteral("0.e0".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, ".0e1", Some(Token::DecimalLiteral(".0e1".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "1.e100", Some(Token::DecimalLiteral("1.e100".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, ".2e-0", Some(Token::DecimalLiteral(".2e-0".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "3.4e-10", Some(Token::DecimalLiteral("3.4e-10".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "-3.4e8", Some(Token::DecimalLiteral("-3.4e8".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "-3.e8", Some(Token::DecimalLiteral("-3.e8".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "-.4e8", Some(Token::DecimalLiteral("-.4e8".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+3.4e8", Some(Token::DecimalLiteral("+3.4e8".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+3.e8", Some(Token::DecimalLiteral("+3.e8".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+.4e8", Some(Token::DecimalLiteral("+.4e8".into())));

    test_try_scanning_case(TokenKind::DecimalLiteral, "+.4e8xyz", Some(Token::DecimalLiteral("+.4e8".into())));
    test_try_scanning_case(TokenKind::DecimalLiteral, "+.4e8)", Some(Token::DecimalLiteral("+.4e8".into())));

    test_try_scanning_case(TokenKind::DecimalLiteral, ".", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "0", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "1", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "4e100", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "abcxyz", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "__", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "(", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, ")", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "", None);
    test_try_scanning_case(TokenKind::DecimalLiteral, "\x7F", None);



    test_try_scanning_case(TokenKind::IntegerLiteral, "0", Some(Token::IntegerLiteral("0".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "1", Some(Token::IntegerLiteral("1".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "1234", Some(Token::IntegerLiteral("1234".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "99999999999999999999999", Some(Token::IntegerLiteral("99999999999999999999999".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "-0", Some(Token::IntegerLiteral("-0".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "-1", Some(Token::IntegerLiteral("-1".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "-1234", Some(Token::IntegerLiteral("-1234".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "-99999999999999999999999", Some(Token::IntegerLiteral("-99999999999999999999999".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "+0", Some(Token::IntegerLiteral("+0".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "+1", Some(Token::IntegerLiteral("+1".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "+1234", Some(Token::IntegerLiteral("+1234".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "+99999999999999999999999", Some(Token::IntegerLiteral("+99999999999999999999999".into())));
    // Note that this doesn't try to match the whole string.
    test_try_scanning_case(TokenKind::IntegerLiteral, "1.", Some(Token::IntegerLiteral("1".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "1x", Some(Token::IntegerLiteral("1".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "1)", Some(Token::IntegerLiteral("1".into())));
    test_try_scanning_case(TokenKind::IntegerLiteral, "abcxyz", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, "__", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, ".", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, ".1", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, "(", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, ")", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, "", None);
    test_try_scanning_case(TokenKind::IntegerLiteral, "\x7F", None);


    for input in vec![
        r#""""#,
        r#""\0""#, r#""\a""#, r#""\b""#, r#""\t""#, r#""\n""#, r#""\v""#, r#""\f""#, r#""\r""#, r#""\"""#, r#""\\""#,
        r#""\x00""#, r#""\xAA""#, r#""\x93""#, r#""\x7f""#, r#""\xee""#, r#""\xFf""#,
        r#"" ""#,
        // TODO: More test cases
    ] {
        test_try_scanning_case(
            scanner::TokenKind::AsciiStringLiteral,
            input,
            Some(scanner::AsciiStringLiteral::from(input).into()),
        );
    }
    test_try_scanning_case_ascii_string_literal("");
    test_try_scanning_case_ascii_string_literal("");
    test_try_scanning_case_ascii_string_literal("\n  ");
    test_try_scanning_case_ascii_string_literal("blah");
    test_try_scanning_case_ascii_string_literal("\"thingy'\\");
//     test_try_scanning_case_ascii_string_literal("\t\f\v\n\r\a\b");
    // TODO: More exhaustive test cases where Rust's format("{:?}", s) isn't getting in the way.
}

fn test_scan_case(input: &str, expected_token_v: Vec<scanner::Token>) {
    log::debug!("test_scan_case; input: {}", input);
    let token_v = scanner::scan(input).expect("test");
    log::debug!("scanner::scan({:?}):\n{:?}", input, token_v);
    assert_eq!(token_v, expected_token_v);
}

fn test_scan_case_negative(input: &str) {
    log::debug!("test_scan_case_negative; input: {}", input);
    scanner::scan(input).expect_err("negative test");
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_scan() {
    use scanner::Token;
    test_scan_case(
        "",
        vec![],
    );
    test_scan_case(
        "blah, (thing, hippo)",
        vec![
            Token::CIdentifier("blah".into()),
            Token::Comma,
            Token::OpenParen,
                Token::CIdentifier("thing".into()),
                Token::Comma,
                Token::CIdentifier("hippo".into()),
            Token::CloseParen,
        ],
    );
    for input in vec!["3", "+8", "-9", "10", "0", "+0", "-0", "-1000", "999999999999999999999999999999999999999999999999999999999999999999"] {
        test_scan_case(input, vec![Token::IntegerLiteral(input.into())]);
    }
    for input in vec!["4.", ".5", "6.7", "0.0", "+8.", "-9.", "4.e0", "4.e+0", "4.e-0", ".5e1", "1.6e10", "8.05e-20", "-999999.1010101010e+666666666"] {
        test_scan_case(input, vec![Token::DecimalLiteral(input.into())]);
    }
    for input in vec![
        r#""""#,
        r#""\0""#, r#""\a""#, r#""\b""#, r#""\t""#, r#""\n""#, r#""\v""#, r#""\f""#, r#""\r""#, r#""\"""#, r#""\\""#,
        r#""\x00""#, r#""\xAA""#, r#""\x93""#, r#""\x7f""#, r#""\xee""#, r#""\xFf""#,
        r#"" !""#,
        r#"" !#""#,
        r#"" !#$""#,
        r#"" !#$%&'""#,
        r#"" !#$%&'()*+,-""#,
        r#"" !#$%&'()*+,-./0123456789:;<=>""#,
        r#"" !#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]""#,
        r#"" !#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz{|}~""#,
    ] {
        test_scan_case(input, vec![Token::AsciiStringLiteral(input.into())]);
    }

    //
    // Negative test cases; expected error.
    //

    test_scan_case_negative(")");
    test_scan_case_negative(".");
}

fn test_parse_case(input: &str) {
    log::debug!("input: {:?}", input);
    let token_v = scanner::scan(input).expect("test");
    log::debug!("token_v ({} elements): {:?}", token_v.len(), token_v);
    let (expr_sequence, parse_stats) = parser::parse_expr_sequence(&token_v, parser::ExprSequenceEnd::OnlyOnEndOfInput).expect("test");
    log::debug!("expr_sequence:\n{:#?}", expr_sequence);
    log::debug!("parse_stats: {:?}", parse_stats);
    assert_eq!(parse_stats.scanner_token_count, token_v.len());
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_parse() {
    test_parse_case("Tuple");
    test_parse_case("123");
    test_parse_case("Float64 34.002e10");
    test_parse_case("(Float64 34.002e10)");
    test_parse_case(" (Float64, 34.002e10) ");
    test_parse_case(" (Float64, 34.002e10 ()) ");
    test_parse_case("Tuple(Sint8(123), Bool(true), Void, Utf8String)");
    test_parse_case("ArrayES(Float32, 4)(100.0, 8.9, 0.0, 1.0)");
}

fn test_detextify_case(input: &str, expected_value: dy::Value) {
    log::debug!("test_detextify_case; input: {}", input);
    let deconstruction = dy::detextify(input).expect("test");
    log::debug!("deconstruction: {:?}", deconstruction);
    let reconstructed_value = deconstruction.reconstruct().expect("test");
    log::debug!("reconstructed_value: {:?}", reconstructed_value);
    assert_eq!(reconstructed_value, expected_value);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_detextify() {
    test_detextify_case("true", dy::Value::from(true));
    test_detextify_case("false", dy::Value::from(false));
    test_detextify_case("True", dy::Value::from(st::True));
    test_detextify_case("False", dy::Value::from(st::False));
    test_detextify_case("Tuple", dy::Value::from(st::Tuple));
    test_detextify_case("Tuple()", dy::Value::from(dy::TupleTerm::from(vec![])));
    test_detextify_case("Tuple(true, false)", dy::Value::from(dy::TupleTerm::from((true, false))));

    test_detextify_case("Float64(4.5)", dy::Value::from(4.5f64));
    test_detextify_case("Float64(4.e10)", dy::Value::from(4.0e10f64));
    test_detextify_case("Float64(-.01)", dy::Value::from(-0.01f64));

//     test_detextify_case("Float32(4.5)", dy::Value::from(4.5f32));
//     test_detextify_case("Float32(4.e10)", dy::Value::from(4.0e10f32));
//     test_detextify_case("Float32(-.01)", dy::Value::from(-0.01f32));

    test_detextify_case("Utf8String(\"blah\\n\\thh\")", dy::Value::from(String::from("blah\n\thh")));

    test_detextify_case(
        "Struct(Tuple(Utf8String(\"name\"), Utf8String), Tuple(Utf8String(\"score\"), Uint64))",
        dy::Value::from(
            StructTerm::new(
                vec![("name".into(), Utf8String.into()), ("score".into(), Uint64.into())],
            ).expect("test")
        )
    );


}

//
// TEMP TESTING
//

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct BinOp;

impl dy::Deconstruct for BinOp {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for BinOp {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct UnOp;

impl dy::Deconstruct for UnOp {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for UnOp {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

trait BinOpTermTrait {
    // TODO: A BinOp whose character is defined at runtime (analogous to DynNPTerm) would need
    // a &self parameter.  Could distingish this by having st::BinOpTermTrait and dy::BinOpTermTrait
    // or actually, maybe static vs dynamic isn't exactly right.. nonparametric vs parametric?
    fn is_commutative() -> bool;
}

trait UnOpTermTrait {}

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Add;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Sub;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Mul;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Div;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Pow;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "UnOp", is_parametric = "false", is_type = "false")]
pub struct Neg;

impl dy::Deconstruct for Add {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl dy::Deconstruct for Sub {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl dy::Deconstruct for Mul {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl dy::Deconstruct for Div {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl dy::Deconstruct for Pow {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl dy::Deconstruct for Neg {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl BinOpTermTrait for Add {
    fn is_commutative() -> bool {
        true
    }
}

impl BinOpTermTrait for Sub {
    fn is_commutative() -> bool {
        false
    }
}

impl BinOpTermTrait for Mul {
    fn is_commutative() -> bool {
        true
    }
}

impl BinOpTermTrait for Div {
    fn is_commutative() -> bool {
        false
    }
}

impl BinOpTermTrait for Pow {
    fn is_commutative() -> bool {
        false
    }
}

impl UnOpTermTrait for Neg {}

impl Stringify for BinOp {
    fn stringify(&self) -> String {
        "BinOp".into()
    }
}

impl Stringify for UnOp {
    fn stringify(&self) -> String {
        "UnOp".into()
    }
}

impl Stringify for Add {
    fn stringify(&self) -> String {
        "Add".into()
    }
}

impl Stringify for Sub {
    fn stringify(&self) -> String {
        "Sub".into()
    }
}

impl Stringify for Mul {
    fn stringify(&self) -> String {
        "Mul".into()
    }
}

impl Stringify for Div {
    fn stringify(&self) -> String {
        "Div".into()
    }
}

impl Stringify for Pow {
    fn stringify(&self) -> String {
        "Pow".into()
    }
}

impl Stringify for Neg {
    fn stringify(&self) -> String {
        "Neg".into()
    }
}

impl Inhabits<BinOp> for Add {
    fn inhabits(&self, _rhs: &BinOp) -> bool {
        true
    }
}

impl Inhabits<BinOp> for Sub {
    fn inhabits(&self, _rhs: &BinOp) -> bool {
        true
    }
}

impl Inhabits<BinOp> for Mul {
    fn inhabits(&self, _rhs: &BinOp) -> bool {
        true
    }
}

impl Inhabits<BinOp> for Div {
    fn inhabits(&self, _rhs: &BinOp) -> bool {
        true
    }
}

impl Inhabits<BinOp> for Pow {
    fn inhabits(&self, _rhs: &BinOp) -> bool {
        true
    }
}

impl Inhabits<UnOp> for Neg {
    fn inhabits(&self, _rhs: &UnOp) -> bool {
        true
    }
}

// TEMP HACK
// NOTE: In order to have this in lazy_static, Value would need to use `dyn Any + Sync`, but that's
// a pretty big bump in type requirement.
// lazy_static::lazy_static!{
//     static BIN_OP_EXPR: TupleTerm = TupleTerm::from(vec![Sint32.into(), BinOp.into(), Sint32.into()]);
// }
// std::thread_local!{
//     pub static BIN_OP_EXPR: TupleTerm = TupleTerm::from(vec![Sint32.into(), BinOp.into(), Sint32.into()]);
// }


#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Expr;

impl dy::Deconstruct for Expr {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for Expr {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for Expr {}

impl Stringify for Expr {
    fn stringify(&self) -> String {
        "Expr".into()
    }
}

impl Inhabits<Expr> for f64 {
    fn inhabits(&self, _rhs: &Expr) -> bool {
        true
    }
}

impl Inhabits<Expr> for TupleTerm {
    fn inhabits(&self, _rhs: &Expr) -> bool {
        // TODO: Expr should really be Union(BinOpExpr, LiteralExpr, UnOpExpr)
        // TODO: Either register this with the runtime or make a const
        let bin_op_expr = TupleTerm::from(vec![Expr{}.into(), BinOp{}.into(), Expr{}.into()]);
        // TODO: Left and right unary ops
        self.inhabits(&bin_op_expr)
    }
}

fn eval_expr(expr: &Value) -> f64 {
    use std::ops::Deref;

    // TODO: Either register this with the runtime or make a const
    let bin_op_expr = TupleTerm::from(vec![Expr{}.into(), BinOp{}.into(), Expr{}.into()]);

    // TODO: This should be a poset search under Expr (which is really a Union of types)
    if expr.inhabits(&Float64) {
        *expr.downcast_ref::<f64>().unwrap()
    } else if expr.inhabits(&bin_op_expr) {
        let inner_tuple_term = expr.downcast_ref::<TupleTerm>().unwrap();
        let lhs = eval_expr(&inner_tuple_term[0]);
        let bin_op = &inner_tuple_term[1];
        let rhs = eval_expr(&inner_tuple_term[2]);
        // This sequence of conditionals would be part of the poset search above.
        if bin_op.is::<Add>() {
            lhs + rhs
        } else if bin_op.is::<Sub>() {
            lhs - rhs
        } else if bin_op.is::<Mul>() {
            lhs * rhs
        } else if bin_op.is::<Div>() {
            lhs / rhs
        } else if bin_op.is::<Pow>() {
            lhs.powf(rhs)
        } else {
            panic!("unrecognized BinOp: {}", bin_op.stringify());
        }
    } else {
        panic!("unrecognized expr: {}", expr.stringify());
    }
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_ast() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    {
        let mut runtime_g = RUNTIME_LA.write().unwrap();

        runtime_g.register_term::<Add>().expect("test");
        runtime_g.register_term::<Sub>().expect("test");
        runtime_g.register_term::<Mul>().expect("test");
        runtime_g.register_term::<Div>().expect("test");
        runtime_g.register_term::<Pow>().expect("test");
        runtime_g.register_term::<Neg>().expect("test");

        runtime_g.register_type::<BinOp>().expect("test");
        runtime_g.register_type::<UnOp>().expect("test");
        runtime_g.register_type::<Expr>().expect("test");

        // Non-uniform registrations.
        runtime_g.register_inhabits::<f64,Expr>().unwrap();
        runtime_g.register_inhabits::<TupleTerm,Expr>().unwrap();
    }

    let expr1 = TupleTerm::from(vec![123.0f64.into(), Add{}.into(), 456.0f64.into()]);
    let bin_op_expr = TupleTerm::from(vec![Expr{}.into(), BinOp{}.into(), Expr{}.into()]);
    log::debug!("expr1: {}", expr1);
    log::debug!("bin_op_expr: {}", bin_op_expr);
    log::debug!("Expr{{}}: {}", Expr{}.stringify());
    assert!(expr1.inhabits(&bin_op_expr));
    assert!(expr1.inhabits(&Expr{}));

    let expr2 = TupleTerm::from(vec![
        TupleTerm::from(vec![77.75f64.into(), Mul{}.into(), 900.125f64.into()]).into(),
        Add{}.into(),
        1.0f64.into(),
    ]);
    log::debug!("expr2: {}", expr2);
    assert!(expr2.inhabits(&Expr{}));
    assert!(expr2.inhabits(&bin_op_expr));

    log::debug!("eval_expr({}): {}", expr2.stringify(), eval_expr(&Value::from(expr2)));

    let expr3 = TupleTerm::from((TupleTerm::from((77.75f64, Mul{}, 900.125f64)), Add{}, 1.0f64));
    log::debug!("expr3: {}", expr3);

    // TEMP TESTING
    {
        // Create the BinOpExpr struct
        dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap()
            .define_symbol(
                "BinOpExpr",
                StructTerm::new(
                    vec![("lhs".into(), Expr{}.into()), ("bin_op".into(), BinOp{}.into()), ("rhs".into(), Expr{}.into())].into()
                ).expect("test").into()
            ).expect("test");
        log::debug!("global_symbol_table: {:#?}", dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap());
        let bin_op_expr = GlobalSymRefTerm::new_unchecked("BinOpExpr".into());
        log::debug!("bin_op_expr: {}", bin_op_expr.stringify());

//         assert!(expr3.inhabits(&bin_op_expr));
    }
}
