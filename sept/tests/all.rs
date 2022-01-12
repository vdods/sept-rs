#![allow(unused_imports)]

use sept::{
    dy::{
        self, ArrayTerm, GlobalSymRefTerm, IntoValue, RUNTIME_LA, StructTerm, SymbolTable, TupleTerm, Value,
    },
    st::{
        self,
        Array, ArrayType, Bool, BoolType, EmptyType, False, FalseType,
        Float32, Float32Type, Float64, Float64Type, Inhabits, Result,
        Sint8, Sint8Type, Sint16, Sint16Type, Sint32, Sint32Type, Sint64, Sint64Type, Stringify,
        Struct, StructType,
        TermTrait, True, TrueType, Type, TypeTrait,
        Uint8, Uint8Type, Uint16, Uint16Type, Uint32, Uint32Type, Uint64, Uint64Type, Void, VoidType,
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
fn test_term_and_type() -> Result<()> {
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

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_stringify() -> Result<()> {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert_eq!(runtime_g.stringify(&true), "True");
    assert_eq!(runtime_g.stringify(&false), "False");
    assert_eq!(runtime_g.stringify(&True), "True");
    assert_eq!(runtime_g.stringify(&False), "False");
    assert_eq!(runtime_g.stringify(&TrueType), "TrueType");
    assert_eq!(runtime_g.stringify(&FalseType), "FalseType");
    assert_eq!(runtime_g.stringify(&Bool), "Bool");
    assert_eq!(runtime_g.stringify(&BoolType), "BoolType");

    log::debug!("RUNTIME_LA.stringify(&123): {:#?}", runtime_g.stringify(&123));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_eq() -> Result<()> {
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

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_inhabits() -> Result<()> {
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

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_ints() -> Result<()> {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert!(runtime_g.inhabits(&123i8, &Sint8{}));
    assert!(runtime_g.inhabits(&123i16, &Sint16{}));
    assert!(runtime_g.inhabits(&123i32, &Sint32{}));
    assert!(runtime_g.inhabits(&123i64, &Sint64{}));

    assert!(runtime_g.inhabits(&123u8, &Uint8{}));
    assert!(runtime_g.inhabits(&123u16, &Uint16{}));
    assert!(runtime_g.inhabits(&123u32, &Uint32{}));
    assert!(runtime_g.inhabits(&123u64, &Uint64{}));

    assert!(runtime_g.inhabits(&Sint8{}, &Sint8Type{}));
    assert!(runtime_g.inhabits(&Sint16{}, &Sint16Type{}));
    assert!(runtime_g.inhabits(&Sint32{}, &Sint32Type{}));
    assert!(runtime_g.inhabits(&Sint64{}, &Sint64Type{}));

    assert!(runtime_g.inhabits(&Uint8{}, &Uint8Type{}));
    assert!(runtime_g.inhabits(&Uint16{}, &Uint16Type{}));
    assert!(runtime_g.inhabits(&Uint32{}, &Uint32Type{}));
    assert!(runtime_g.inhabits(&Uint64{}, &Uint64Type{}));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_floats() -> Result<()> {
    let runtime_g = RUNTIME_LA.read().unwrap();

    assert!(runtime_g.inhabits(&5.875f32, &Float32{}));
    assert!(runtime_g.inhabits(&5.875f64, &Float64{}));

    assert!(runtime_g.inhabits(&Float32{}, &Float32Type{}));
    assert!(runtime_g.inhabits(&Float64{}, &Float64Type{}));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_arrays() -> Result<()> {
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

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_tuples() -> Result<()> {
    let t1 = TupleTerm::from(vec![3i32.into(), 5.5f32.into()]);
    let t2 = TupleTerm::from(vec![Sint32{}.into(), Float32{}.into()]);
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

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_abstract_type() -> Result<()> {
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

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint8{}).as_ref(), &Sint8Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint8Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint16{}).as_ref(), &Sint16Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint16Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint32{}).as_ref(), &Sint32Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint32Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint64{}).as_ref(), &Sint64Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Sint64Type{}).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint8{}).as_ref(), &Uint8Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint8Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint16{}).as_ref(), &Uint16Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint16Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint32{}).as_ref(), &Uint32Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint32Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint64{}).as_ref(), &Uint64Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Uint64Type{}).as_ref(), &Type));

    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float32{}).as_ref(), &Float32Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float32Type{}).as_ref(), &Type));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float64{}).as_ref(), &Float64Type{}));
    assert!(runtime_g.eq(runtime_g.abstract_type_of(&Float64Type{}).as_ref(), &Type));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_value() -> Result<()> {
    let v1 = Value::from(3i32);
    let v2 = Value::from(7i32);

    log::debug!("Value::label(): {:?}", Value::label());
    log::debug!("v1.stringify(): {:?}", v1.stringify());
    log::debug!("v2.stringify(): {:?}", v2.stringify());
    log::debug!("v1.abstract_type(): {:?}", v1.abstract_type());

    log::debug!("v1.inhabits(&Sint32): {:?}", v1.inhabits(&Sint32{}));
    log::debug!("v1.inhabits(&Bool): {:?}", v1.inhabits(&Bool));
    log::debug!("v1.inhabits(&Value::from(Sint32)): {:?}", v1.inhabits(&Value::from(Sint32{})));
    log::debug!("v1.inhabits(&Value::from(Bool)): {:?}", v1.inhabits(&Value::from(Bool)));
    log::debug!("v1.inhabits(&v2): {:?}", v1.inhabits(&v2));
    let v3 = Value::from(Sint32{});
    log::debug!("v1.inhabits(&v3): {:?}", v1.inhabits(&v3));

    log::debug!("v1: {}", v1);
    log::debug!("v2: {}", v2);

    log::debug!("v1 (as Debug): {:?}", v1);
    log::debug!("v2 (as Debug): {:?}", v2);

    log::debug!("v1 == v1: {:?}", v1 == v1);
    log::debug!("v1 == v2: {:?}", v1 == v2);
    log::debug!("v2 == v1: {:?}", v2 == v1);
    log::debug!("v2 == v2: {:?}", v2 == v2);

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_symbol_table() -> Result<()> {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    let mut symbol_table = SymbolTable::new_with_parent(None);
    assert!(!symbol_table.symbol_is_defined("blah"));
    symbol_table.define_symbol("blah", Value::from(123i32))?;
    assert!(symbol_table.symbol_is_defined("blah"));
    assert_eq!(*symbol_table.resolved_symbol("blah")?.read().unwrap(), Value::from(123i32));

    log::debug!("symbol_table: {:#?}", symbol_table);

    // Now check GLOBAL_SYMBOL_TABLE_LA
    {
        assert!(!dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap().symbol_is_defined("bleh"));

        // Have to acquire a separate write lock.
        dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().define_symbol("bleh", Value::from(456f32))?;

        // Now acquire a read lock.
        let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
        assert!(global_symbol_table_g.symbol_is_defined("bleh"));
        assert_eq!(*global_symbol_table_g.resolved_symbol("bleh")?.read().unwrap(), Value::from(456f32));

        log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
    }

    // Test out parent symbol tables.
    let parent_symbol_table_la = Arc::new(RwLock::new(SymbolTable::new_with_parent(None)));
    let child_symbol_table_la = Arc::new(RwLock::new(SymbolTable::new_with_parent(Some(parent_symbol_table_la.clone()))));

    parent_symbol_table_la.write().unwrap().define_symbol("stuff", Value::from(200u32))?;
    parent_symbol_table_la.write().unwrap().define_symbol("hippo", Value::from(300u32))?;

    child_symbol_table_la.write().unwrap().define_symbol("stuff", Value::from(444u32))?;

    assert_eq!(*parent_symbol_table_la.read().unwrap().resolved_symbol("stuff")?.read().unwrap(), Value::from(200u32));
    assert_eq!(*parent_symbol_table_la.read().unwrap().resolved_symbol("hippo")?.read().unwrap(), Value::from(300u32));

    assert_eq!(*child_symbol_table_la.read().unwrap().resolved_symbol("stuff")?.read().unwrap(), Value::from(444u32));
    assert_eq!(*child_symbol_table_la.read().unwrap().resolved_symbol("hippo")?.read().unwrap(), Value::from(300u32));

    log::debug!("child_symbol_table:\n{:#?}", child_symbol_table_la.read().unwrap());

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_global_sym_ref_term() -> Result<()> {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    // Write a bunch of stuff into the global_symbol_table
    {
        let mut global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap();
        global_symbol_table_g.define_symbol("bleh", Value::from(456f32))?;
        global_symbol_table_g.define_symbol("stuff", Value::from(True{}))?;
        global_symbol_table_g.define_symbol("andthings", Value::from(Void{}))?;
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
        let r_resolved_la = r.resolved()?;
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
        let r_resolved_la = r.resolved()?;
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
            global_symbol_table_g.define_symbol("inner", Value::from(40404u32))?;
            global_symbol_table_g.define_symbol("outer", Value::from(GlobalSymRefTerm::new_unchecked("inner".into())))?;
            global_symbol_table_g.define_symbol("outerer", Value::from(GlobalSymRefTerm::new_unchecked("outer".into())))?;
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
            let inner_dereferenced_once_la = runtime_g.dereferenced_once(&inner_ref)?;
            let inner_dereferenced_once_g = inner_dereferenced_once_la.read().unwrap();
            log::debug!("inner_dereferenced_once_g (as Debug): {:#?}", inner_dereferenced_once_g);
            log::debug!("inner_dereferenced_once_g: {}", inner_dereferenced_once_g.stringify());
            assert_eq!(*inner_dereferenced_once_g, test_value);
        }

        {
            let outer_dereferenced_once_la = runtime_g.dereferenced_once(&outer_ref)?;
            let outer_dereferenced_once_g = outer_dereferenced_once_la.read().unwrap();
            log::debug!("outer_dereferenced_once_g (as Debug): {:#?}", outer_dereferenced_once_g);
            log::debug!("outer_dereferenced_once_g: {}", outer_dereferenced_once_g.stringify());
            assert_eq!(*outer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        {
            let outerer_dereferenced_once_la = runtime_g.dereferenced_once(&outerer_ref)?;
            let outerer_dereferenced_once_g = outerer_dereferenced_once_la.read().unwrap();
            log::debug!("outerer_dereferenced_once_g (as Debug): {:#?}", outerer_dereferenced_once_g);
            log::debug!("outerer_dereferenced_once_g: {}", outerer_dereferenced_once_g.stringify());
            assert_eq!(*outerer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        assert_eq!(*inner_ref.resolved()?.read().unwrap(), test_value);
        assert_eq!(*outer_ref.resolved()?.read().unwrap(), test_value);
        assert_eq!(*outerer_ref.resolved()?.read().unwrap(), test_value);
    }

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_local_sym_ref_term() -> Result<()> {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    let local_symbol_table_la = Arc::new(RwLock::new(SymbolTable::new_with_parent(None)));
    local_symbol_table_la.write().unwrap().define_symbol("blah", dy::Value::from(123i32))?;
    log::debug!("local_symbol_table_la: {:#?}", local_symbol_table_la.read().unwrap());

    let local_sym_ref_term = dy::LocalSymRefTerm::new_checked(local_symbol_table_la.clone(), "blah".into())?;
    log::debug!("local_sym_ref_term: (as Debug) {:#?}", local_sym_ref_term);
    log::debug!("local_sym_ref_term: (as Display) {}", local_sym_ref_term);
    log::debug!("local_sym_ref_term: {}", local_sym_ref_term.stringify());

    // Test nested references
    {
        let test_value = Value::from(51515u32);

        // Write more stuff into the local_symbol_table
        {
            let mut local_symbol_table_g = local_symbol_table_la.write().unwrap();
            local_symbol_table_g.define_symbol("inner", Value::from(51515u32))?;
            local_symbol_table_g.define_symbol("outer", Value::from(dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "inner".into())))?;
            local_symbol_table_g.define_symbol("outerer", Value::from(dy::LocalSymRefTerm::new_unchecked(local_symbol_table_la.clone(), "outer".into())))?;
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
            let inner_dereferenced_once_la = runtime_g.dereferenced_once(&inner_ref)?;
            let inner_dereferenced_once_g = inner_dereferenced_once_la.read().unwrap();
            log::debug!("inner_dereferenced_once_g (as Debug): {:#?}", inner_dereferenced_once_g);
            log::debug!("inner_dereferenced_once_g: {}", inner_dereferenced_once_g.stringify());
            assert_eq!(*inner_dereferenced_once_g, test_value);
        }

        {
            let outer_dereferenced_once_la = runtime_g.dereferenced_once(&outer_ref)?;
            let outer_dereferenced_once_g = outer_dereferenced_once_la.read().unwrap();
            log::debug!("outer_dereferenced_once_g (as Debug): {:#?}", outer_dereferenced_once_g);
            log::debug!("outer_dereferenced_once_g: {}", outer_dereferenced_once_g.stringify());
            assert_eq!(*outer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        {
            let outerer_dereferenced_once_la = runtime_g.dereferenced_once(&outerer_ref)?;
            let outerer_dereferenced_once_g = outerer_dereferenced_once_la.read().unwrap();
            log::debug!("outerer_dereferenced_once_g (as Debug): {:#?}", outerer_dereferenced_once_g);
            log::debug!("outerer_dereferenced_once_g: {}", outerer_dereferenced_once_g.stringify());
            assert_eq!(*outerer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        assert_eq!(*inner_ref.resolved()?.read().unwrap(), test_value);
        assert_eq!(*outer_ref.resolved()?.read().unwrap(), test_value);
        assert_eq!(*outerer_ref.resolved()?.read().unwrap(), test_value);
    }

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_structs() -> Result<()> {
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
                "Hippo".into(),
                vec![("age".into(), Uint8{}.into()), ("gravity".into(), Float64{}.into())].into()
            ).into()
        )?;

    let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
    log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
    let hippo = GlobalSymRefTerm::new_unchecked("Hippo".into());
    log::debug!("hippo: {}", hippo.stringify());

    let x = global_symbol_table_g
        .resolved_symbol("Hippo")?
        .read().unwrap()
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![23u8.into(), 999.0f64.into()].into())?;
    let y = global_symbol_table_g
        .resolved_symbol("Hippo")?
        .read().unwrap()
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![100u8.into(), (-3.0f64).into()].into())?;
    log::debug!("x: {}", x.stringify());
    log::debug!("y: {}", y.stringify());
    log::debug!("x == y: {}", x == y);

    assert_eq!(x, x);
    assert_eq!(y, y);
    assert!(x != y);
    assert!(y != x);

    let x2 = dy::StructTermTerm::new_checked(hippo.clone().into(), vec![23u8.into(), 999.0f64.into()].into())?;
    let y2 = dy::StructTermTerm::new_checked(hippo.clone().into(), vec![100u8.into(), (-3.0f64).into()].into())?;

    log::debug!("x2: {}", x2.stringify());
    log::debug!("y2: {}", y2.stringify());
    log::debug!("x2 == y2: {}", x2 == y2);

    assert_eq!(x2, x2);
    assert_eq!(y2, y2);
    assert!(x2 != y2);
    assert!(y2 != x2);

    assert_eq!(x, x2);
    assert_eq!(y, y2);

    Ok(())
}

//
// TEMP TESTING
//

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct BinOp;

impl st::Inhabits<Type> for BinOp {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct UnOp;

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
//     static BIN_OP_EXPR: TupleTerm = TupleTerm::from(vec![Sint32{}.into(), BinOp{}.into(), Sint32{}.into()]);
// }
// std::thread_local!{
//     pub static BIN_OP_EXPR: TupleTerm = TupleTerm::from(vec![Sint32{}.into(), BinOp{}.into(), Sint32{}.into()]);
// }


#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Expr;

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
    if expr.inhabits(&Float64{}) {
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
fn test_ast() -> Result<()> {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    {
        let mut runtime_g = RUNTIME_LA.write().unwrap();

        runtime_g.register_term::<Add>()?;
        runtime_g.register_term::<Sub>()?;
        runtime_g.register_term::<Mul>()?;
        runtime_g.register_term::<Div>()?;
        runtime_g.register_term::<Pow>()?;
        runtime_g.register_term::<Neg>()?;

        runtime_g.register_type::<BinOp>()?;
        runtime_g.register_type::<UnOp>()?;
        runtime_g.register_type::<Expr>()?;

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
                    "BinOpExpr".into(),
                    vec![("lhs".into(), Expr{}.into()), ("bin_op".into(), BinOp{}.into()), ("rhs".into(), Expr{}.into())].into()
                ).into()
            )?;
        log::debug!("global_symbol_table: {:#?}", dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap());
        let bin_op_expr = GlobalSymRefTerm::new_unchecked("BinOpExpr".into());
        log::debug!("bin_op_expr: {}", bin_op_expr.stringify());

//         assert!(expr3.inhabits(&bin_op_expr));
    }

    Ok(())
}
