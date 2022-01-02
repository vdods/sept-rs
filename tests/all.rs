#![allow(unused_imports)]

use sept::{
    dy::{
        self, ArrayTerm, GlobalSymRefTerm, IntoValue, RUNTIME, StructTerm, SymbolTable, TupleTerm, Value,
    },
    st::{
        ARRAY, ARRAY_TYPE,
        BOOL, Bool, BOOL_TYPE, BoolType, EMPTY_TYPE, FALSE, False, FALSE_TYPE, FalseType,
        FLOAT32, FLOAT32_TYPE, FLOAT64, Float64, FLOAT64_TYPE, Inhabits, Result,
        SINT8, SINT8_TYPE, SINT16, SINT16_TYPE, SINT32, Sint32, SINT32_TYPE, SINT64, SINT64_TYPE, Stringify,
        STRUCT, Struct, STRUCT_TYPE, StructType,
        TermTrait, TRUE, True, TRUE_TYPE, TrueType, TYPE, Type, TypeTrait,
        UINT8, Uint8, UINT8_TYPE, UINT16, UINT16_TYPE, UINT32, UINT32_TYPE, UINT64, UINT64_TYPE, VOID, Void, VOID_TYPE,
    },
};
use std::any::Any;

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_term_and_type() -> Result<()> {
    let _ = env_logger::try_init();

    log::debug!("TRUE: {:#?}", TRUE);
    log::debug!("TRUE_TYPE: {:#?}", TRUE_TYPE);

    // NOTE: The commented out ones asserting non-inhabitation, if uncommented, would produce
    // compile errors to the effect of "Void doesn't implement Inhabits<FalseType>", which
    // is correct and desired, since these types are known at compile time.

    assert!(VOID.inhabits(&VOID_TYPE));
//     assert!(!VOID.inhabits(&FALSE_TYPE));
//     assert!(!VOID.inhabits(&TYPE));
//     assert!(!VOID.inhabits(&BOOL));
//     assert!(!VOID.inhabits(&BOOL_TYPE));

    assert!(VOID_TYPE.inhabits(&TYPE));

    assert!(TRUE.inhabits(&TRUE_TYPE));
//     assert!(!TRUE.inhabits(&FALSE_TYPE));
    assert!(TRUE.inhabits(&BOOL));
//     assert!(!TRUE.inhabits(&BOOL_TYPE));

//     assert!(!FALSE.inhabits(&TRUE_TYPE));
    assert!(FALSE.inhabits(&FALSE_TYPE));
    assert!(FALSE.inhabits(&BOOL));
//     assert!(!FALSE.inhabits(&BOOL_TYPE));

    assert!(TRUE_TYPE.inhabits(&BOOL_TYPE));
    assert!(FALSE_TYPE.inhabits(&BOOL_TYPE));
    assert!(BOOL.inhabits(&BOOL_TYPE));
//     assert!(!BOOL.inhabits(&TRUE_TYPE));
//     assert!(!BOOL.inhabits(&FALSE_TYPE));


    assert!(!TRUE.is_parametric_term());
    assert!(!TRUE.is_type_term());
    assert!(!TRUE_TYPE.is_parametric_term());
    assert!(TRUE_TYPE.is_type_term());

    assert!(!FALSE.is_parametric_term());
    assert!(!FALSE.is_type_term());
    assert!(!FALSE_TYPE.is_parametric_term());
    assert!(FALSE_TYPE.is_type_term());

    assert!(true.is_parametric_term());
    assert!(!true.is_type_term());
    assert!(false.is_parametric_term());
    assert!(!false.is_type_term());
    assert!(!BOOL.is_parametric_term());
    assert!(BOOL.is_type_term());
    assert!(!BOOL_TYPE.is_parametric_term());
    assert!(BOOL_TYPE.is_type_term());

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_stringify() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    assert_eq!(rt.stringify(&true), "True");
    assert_eq!(rt.stringify(&false), "False");
    assert_eq!(rt.stringify(&TRUE), "True");
    assert_eq!(rt.stringify(&FALSE), "False");
    assert_eq!(rt.stringify(&TRUE_TYPE), "TrueType");
    assert_eq!(rt.stringify(&FALSE_TYPE), "FalseType");
    assert_eq!(rt.stringify(&BOOL), "Bool");
    assert_eq!(rt.stringify(&BOOL_TYPE), "BoolType");

    log::debug!("RUNTIME.stringify(&123): {:#?}", rt.stringify(&123));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_eq() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    assert!(rt.eq(&true, &true));
    assert!(!rt.eq(&true, &false));
    assert!(rt.eq(&true, &TRUE));
    assert!(!rt.eq(&true, &FALSE));

    assert!(!rt.eq(&false, &true));
    assert!(rt.eq(&false, &false));
    assert!(!rt.eq(&false, &TRUE));
    assert!(rt.eq(&false, &FALSE));

    assert!(rt.eq(&TRUE, &true));
    assert!(!rt.eq(&TRUE, &false));
    assert!(rt.eq(&TRUE, &TRUE));
    assert!(!rt.eq(&TRUE, &FALSE));

    assert!(!rt.eq(&FALSE, &true));
    assert!(rt.eq(&FALSE, &false));
    assert!(!rt.eq(&FALSE, &TRUE));
    assert!(rt.eq(&FALSE, &FALSE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_inhabits() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    assert!(rt.inhabits(&true, &BOOL));
    assert!(rt.inhabits(&false, &BOOL));
    assert!(!rt.inhabits(&true, &FALSE_TYPE));
    assert!(rt.inhabits(&false, &FALSE_TYPE));
    assert!(rt.inhabits(&true, &TRUE_TYPE));
    assert!(!rt.inhabits(&false, &TRUE_TYPE));
    assert!(rt.inhabits(&True, &BOOL));
    assert!(rt.inhabits(&False, &BOOL));
    assert!(rt.inhabits(&BOOL, &BOOL_TYPE));
    assert!(!rt.inhabits(&BOOL_TYPE, &BOOL));
    assert!(rt.inhabits(&VOID, &VOID_TYPE));
    assert!(!rt.inhabits(&VOID_TYPE, &VOID));

    assert!(!rt.inhabits(&BOOL, &EMPTY_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_ints() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    assert!(rt.inhabits(&123i8, &SINT8));
    assert!(rt.inhabits(&123i16, &SINT16));
    assert!(rt.inhabits(&123i32, &SINT32));
    assert!(rt.inhabits(&123i64, &SINT64));

    assert!(rt.inhabits(&123u8, &UINT8));
    assert!(rt.inhabits(&123u16, &UINT16));
    assert!(rt.inhabits(&123u32, &UINT32));
    assert!(rt.inhabits(&123u64, &UINT64));

    assert!(rt.inhabits(&SINT8, &SINT8_TYPE));
    assert!(rt.inhabits(&SINT16, &SINT16_TYPE));
    assert!(rt.inhabits(&SINT32, &SINT32_TYPE));
    assert!(rt.inhabits(&SINT64, &SINT64_TYPE));

    assert!(rt.inhabits(&UINT8, &UINT8_TYPE));
    assert!(rt.inhabits(&UINT16, &UINT16_TYPE));
    assert!(rt.inhabits(&UINT32, &UINT32_TYPE));
    assert!(rt.inhabits(&UINT64, &UINT64_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_floats() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    assert!(rt.inhabits(&5.875f32, &FLOAT32));
    assert!(rt.inhabits(&5.875f64, &FLOAT64));

    assert!(rt.inhabits(&FLOAT32, &FLOAT32_TYPE));
    assert!(rt.inhabits(&FLOAT64, &FLOAT64_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_arrays() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    // Note that Vec<Value> is ArrayTerm.
    // Note also that this is constructing a Vec with nonhomogeneous elements, because
    // Value stores Box<dyn Any>.
    let a0 = ArrayTerm::from(vec![3i32.into(), 5.5f32.into()]);
    log::debug!("a0: {}", a0);
    log::debug!("a0 (as Debug): {:?}", a0);
    log::debug!("a0.stringify(): {}", a0.stringify());

    assert!(rt.inhabits(&a0, &ARRAY));
    assert!(rt.inhabits(&ARRAY, &ARRAY_TYPE));

//     let a1 = vec![100i8, 101i8, 99i8, 10i8];
//     log::debug!("a1: {:?}", a1);
//     log::debug!("a1.stringify(): {}", a1.stringify());
//
//     assert!(a1.inhabits(&ARRAY));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_tuples() -> Result<()> {
    let _ = env_logger::try_init();

    let t1 = TupleTerm::from(vec![3i32.into(), 5.5f32.into()]);
    let t2 = TupleTerm::from(vec![SINT32.into(), FLOAT32.into()]);
    log::debug!("t1: {}", t1);
    log::debug!("t2: {}", t2);
    log::debug!("t1.abstract_type(): {}", t1.abstract_type());
    log::debug!("t2.abstract_type(): {}", t2.abstract_type());

    assert!(t1.inhabits(&t2));
    assert!(t1.is_parametric_term());
    assert!(t2.is_parametric_term());
    assert!(!t1.is_type_term());
    assert!(t2.is_type_term());

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_abstract_type() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = RUNTIME.read().unwrap();

    {
        let x = &VOID;
        log::debug!("rt.abstract_type_of({}): {}", rt.stringify(x), rt.stringify(rt.abstract_type_of(x).as_ref()));
    }

    assert!(rt.eq(rt.abstract_type_of(&TYPE).as_ref(), &TYPE));

    assert!(rt.eq(rt.abstract_type_of(&VOID).as_ref(), &VOID_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&VOID_TYPE).as_ref(), &TYPE));

    assert!(rt.eq(rt.abstract_type_of(&BOOL).as_ref(), &BOOL_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&BOOL_TYPE).as_ref(), &TYPE));

    assert!(rt.eq(rt.abstract_type_of(&TRUE).as_ref(), &TRUE_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&TRUE_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&FALSE).as_ref(), &FALSE_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&FALSE_TYPE).as_ref(), &TYPE));

    assert!(rt.eq(rt.abstract_type_of(&SINT8).as_ref(), &SINT8_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT8_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT16).as_ref(), &SINT16_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT16_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT32).as_ref(), &SINT32_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT32_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT64).as_ref(), &SINT64_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&SINT64_TYPE).as_ref(), &TYPE));

    assert!(rt.eq(rt.abstract_type_of(&UINT8).as_ref(), &UINT8_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT8_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT16).as_ref(), &UINT16_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT16_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT32).as_ref(), &UINT32_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT32_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT64).as_ref(), &UINT64_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&UINT64_TYPE).as_ref(), &TYPE));

    assert!(rt.eq(rt.abstract_type_of(&FLOAT32).as_ref(), &FLOAT32_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&FLOAT32_TYPE).as_ref(), &TYPE));
    assert!(rt.eq(rt.abstract_type_of(&FLOAT64).as_ref(), &FLOAT64_TYPE));
    assert!(rt.eq(rt.abstract_type_of(&FLOAT64_TYPE).as_ref(), &TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_value() -> Result<()> {
    let _ = env_logger::try_init();

    let v1 = Value::from(3i32);
    let v2 = Value::from(7i32);

    log::debug!("Value::label(): {:?}", Value::label());
    log::debug!("v1.stringify(): {:?}", v1.stringify());
    log::debug!("v2.stringify(): {:?}", v2.stringify());
    log::debug!("v1.abstract_type(): {:?}", v1.abstract_type());

    log::debug!("v1.inhabits(&SINT32): {:?}", v1.inhabits(&SINT32));
    log::debug!("v1.inhabits(&BOOL): {:?}", v1.inhabits(&BOOL));
    log::debug!("v1.inhabits(&Value::from(SINT32)): {:?}", v1.inhabits(&Value::from(SINT32)));
    log::debug!("v1.inhabits(&Value::from(BOOL)): {:?}", v1.inhabits(&Value::from(BOOL)));
    log::debug!("v1.inhabits(&v2): {:?}", v1.inhabits(&v2));
    let v3 = Value::new(SINT32);
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
    let _ = env_logger::try_init();

    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    RUNTIME.write().unwrap().global_symbol_table.clear();

    let mut symbol_table = SymbolTable::default();
    assert!(!symbol_table.symbol_is_defined("blah"));
    symbol_table.define_symbol("blah", Value::from(123i32))?;
    assert!(symbol_table.symbol_is_defined("blah"));
    assert_eq!(*symbol_table.resolve_symbol("blah")?, Value::from(123i32));

    log::debug!("symbol_table: {:#?}", symbol_table);

    // Now check RUNTIME.global_symbol_table
    {
        assert!(!RUNTIME.read().unwrap().global_symbol_table.symbol_is_defined("bleh"));

        // Have to acquire a separate write lock.
        RUNTIME.write().unwrap().global_symbol_table.define_symbol("bleh", Value::from(456f32))?;

        // Now acquire a read lock.
        let global_symbol_table = &RUNTIME.read().unwrap().global_symbol_table;
        assert!(global_symbol_table.symbol_is_defined("bleh"));
        assert_eq!(*global_symbol_table.resolve_symbol("bleh")?, Value::from(456f32));

        log::debug!("global_symbol_table: {:#?}", global_symbol_table);
    }

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_global_sym_ref_term() -> Result<()> {
    let _ = env_logger::try_init();

    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    RUNTIME.write().unwrap().global_symbol_table.clear();

    // Write a bunch of stuff into the global_symbol_table
    {
        let mut write_lock = RUNTIME.write().unwrap();
        write_lock.global_symbol_table.define_symbol("bleh", Value::from(456f32))?;
        write_lock.global_symbol_table.define_symbol("stuff", Value::from(True{}))?;
        write_lock.global_symbol_table.define_symbol("andthings", Value::from(Void{}))?;
    }

    // Now check GlobalSymRefTerm.
    let r = GlobalSymRefTerm::from("bleh");
    log::debug!("r (as Debug): {:#?}", r);
    log::debug!("r (as Display): {}" , r);
    log::debug!("r: {}" , r.stringify());

    let t = TupleTerm::from(vec![
        GlobalSymRefTerm::from("bleh").into(),
        GlobalSymRefTerm::from("stuff").into(),
        GlobalSymRefTerm::from("andthings").into(),
    ]);
    log::debug!("t (as Debug): {:#?}", t);
    log::debug!("t (as Display): {}", t);
    log::debug!("t: {}", t.stringify());

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_structs() -> Result<()> {
    let _ = env_logger::try_init();

    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    RUNTIME.write().unwrap().global_symbol_table.clear();

    log::debug!("STRUCT: {}", STRUCT.stringify());
    log::debug!("STRUCT_TYPE: {}", STRUCT_TYPE.stringify());

    assert!(STRUCT.inhabits(&STRUCT_TYPE));

    // Create the Hippo struct
    RUNTIME.write().unwrap()
        .global_symbol_table
        .define_symbol(
            "Hippo",
            StructTerm::new(
                "Hippo".into(),
                vec![("age".into(), Uint8{}.into()), ("gravity".into(), Float64{}.into())].into()
            ).into()
        )?;
    log::debug!("global_symbol_table: {:#?}", RUNTIME.read().unwrap().global_symbol_table);
    let hippo = GlobalSymRefTerm::from("Hippo");
    log::debug!("hippo: {}", hippo.stringify());

    let x = RUNTIME.read().unwrap()
        .global_symbol_table
        .resolve_symbol("Hippo")?
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![23u8.into(), 999.0f64.into()].into())?;
    let y = RUNTIME.read().unwrap()
        .global_symbol_table
        .resolve_symbol("Hippo")?
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

    Ok(())
}

//
// TEMP TESTING
//

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinOp;

impl dy::IntoValue for BinOp {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnOp;

impl dy::IntoValue for UnOp {}

trait BinOpTermTrait {
    // TODO: A BinOp whose character is defined at runtime (analogous to DynNPTerm) would need
    // a &self parameter.  Could distingish this by having st::BinOpTermTrait and dy::BinOpTermTrait
    // or actually, maybe static vs dynamic isn't exactly right.. nonparametric vs parametric?
    fn is_commutative() -> bool;
}

trait UnOpTermTrait {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sub;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mul;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Div;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Neg;

impl dy::IntoValue for Add {}
impl dy::IntoValue for Sub {}
impl dy::IntoValue for Mul {}
impl dy::IntoValue for Div {}
impl dy::IntoValue for Pow {}
impl dy::IntoValue for Neg {}

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

impl TermTrait for BinOp {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "BinOp"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for UnOp {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "UnOp"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for Add {
    type AbstractTypeFnReturnType = BinOp;

    fn label() -> &'static str {
        "Add"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for Sub {
    type AbstractTypeFnReturnType = BinOp;

    fn label() -> &'static str {
        "Sub"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for Mul {
    type AbstractTypeFnReturnType = BinOp;

    fn label() -> &'static str {
        "Mul"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for Div {
    type AbstractTypeFnReturnType = BinOp;

    fn label() -> &'static str {
        "Div"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for Pow {
    type AbstractTypeFnReturnType = BinOp;

    fn label() -> &'static str {
        "Pow"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for Neg {
    type AbstractTypeFnReturnType = UnOp;

    fn label() -> &'static str {
        "Neg"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TypeTrait for BinOp {}
impl TypeTrait for UnOp {}

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


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Expr;

impl dy::IntoValue for Expr {}

impl Stringify for Expr {
    fn stringify(&self) -> String {
        "Expr".into()
    }
}

impl TermTrait for Expr {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "Expr"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TypeTrait for Expr {}

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
    if expr.inhabits(&FLOAT64) {
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
    let _ = env_logger::try_init();

    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    RUNTIME.write().unwrap().global_symbol_table.clear();

    {
        let mut rt = RUNTIME.write().unwrap();

        rt.register_label::<Add>().unwrap();
        rt.register_label::<Sub>().unwrap();
        rt.register_label::<Mul>().unwrap();
        rt.register_label::<Div>().unwrap();
        rt.register_label::<Pow>().unwrap();
        rt.register_label::<Neg>().unwrap();
        rt.register_label::<BinOp>().unwrap();
        rt.register_label::<UnOp>().unwrap();
        rt.register_label::<Expr>().unwrap();

        rt.register_stringify::<Add>().unwrap();
        rt.register_stringify::<Sub>().unwrap();
        rt.register_stringify::<Mul>().unwrap();
        rt.register_stringify::<Div>().unwrap();
        rt.register_stringify::<Pow>().unwrap();
        rt.register_stringify::<Neg>().unwrap();
        rt.register_stringify::<BinOp>().unwrap();
        rt.register_stringify::<UnOp>().unwrap();
        rt.register_stringify::<Expr>().unwrap();

        rt.register_inhabits_fn::<Add,BinOp>().unwrap();
        rt.register_inhabits_fn::<Sub,BinOp>().unwrap();
        rt.register_inhabits_fn::<Mul,BinOp>().unwrap();
        rt.register_inhabits_fn::<Div,BinOp>().unwrap();
        rt.register_inhabits_fn::<Pow,BinOp>().unwrap();
        rt.register_inhabits_fn::<Neg,UnOp>().unwrap();
        rt.register_inhabits_fn::<f64,Expr>().unwrap();
        rt.register_inhabits_fn::<TupleTerm,Expr>().unwrap();
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
        RUNTIME.write().unwrap()
            .global_symbol_table
            .define_symbol(
                "BinOpExpr",
                StructTerm::new(
                    "BinOpExpr".into(),
                    vec![("lhs".into(), Expr{}.into()), ("bin_op".into(), BinOp{}.into()), ("rhs".into(), Expr{}.into())].into()
                ).into()
            )?;
        log::debug!("global_symbol_table: {:#?}", RUNTIME.read().unwrap().global_symbol_table);
        let bin_op_expr = GlobalSymRefTerm::from("BinOpExpr");
        log::debug!("bin_op_expr: {}", bin_op_expr.stringify());

//         assert!(expr3.inhabits(&bin_op_expr));
    }

    Ok(())
}
