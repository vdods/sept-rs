#![allow(unused_imports)]

use sept::{
    ARRAY, ARRAY_TYPE,
    BOOL, Bool, BOOL_TYPE, BoolType, EMPTY_TYPE, FALSE, False, FALSE_TYPE, FalseType,
    FLOAT32, FLOAT32_TYPE, FLOAT64, FLOAT64_TYPE, Result, RUNTIME,
    SINT8, SINT8_TYPE, SINT16, SINT16_TYPE, SINT32, SINT32_TYPE, SINT64, SINT64_TYPE,
    Stringify, TermTrait, TRUE, True, TRUE_TYPE, TrueType, TYPE, TypeTrait,
    UINT8, UINT8_TYPE, UINT16, UINT16_TYPE, UINT32, UINT32_TYPE, UINT64, UINT64_TYPE, VOID, VOID_TYPE,
};
use std::any::Any;

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_term_and_type() -> Result<()> {
    let _ = env_logger::try_init();

    log::debug!("TRUE: {:#?}", TRUE);
    log::debug!("TRUE_TYPE: {:#?}", TRUE_TYPE);

    assert!(VOID.inhabits(&VOID_TYPE));
    assert!(!VOID.inhabits(&FALSE_TYPE));
    assert!(!VOID.inhabits(&TYPE));
    assert!(!VOID.inhabits(&BOOL));
    assert!(!VOID.inhabits(&BOOL_TYPE));

    assert!(VOID_TYPE.inhabits(&TYPE));

    assert!(TRUE.inhabits(&TRUE_TYPE));
    assert!(!TRUE.inhabits(&FALSE_TYPE));
    assert!(TRUE.inhabits(&BOOL));
    assert!(!TRUE.inhabits(&BOOL_TYPE));

    assert!(!FALSE.inhabits(&TRUE_TYPE));
    assert!(FALSE.inhabits(&FALSE_TYPE));
    assert!(FALSE.inhabits(&BOOL));
    assert!(!FALSE.inhabits(&BOOL_TYPE));

    assert!(TRUE_TYPE.inhabits(&BOOL_TYPE));
    assert!(FALSE_TYPE.inhabits(&BOOL_TYPE));
    assert!(BOOL.inhabits(&BOOL_TYPE));
    assert!(!BOOL.inhabits(&TRUE_TYPE));
    assert!(!BOOL.inhabits(&FALSE_TYPE));

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

    assert_eq!(RUNTIME.stringify(&true), "True");
    assert_eq!(RUNTIME.stringify(&false), "False");
    assert_eq!(RUNTIME.stringify(&TRUE), "True");
    assert_eq!(RUNTIME.stringify(&FALSE), "False");
    assert_eq!(RUNTIME.stringify(&TRUE_TYPE), "TrueType");
    assert_eq!(RUNTIME.stringify(&FALSE_TYPE), "FalseType");
    assert_eq!(RUNTIME.stringify(&BOOL), "Bool");
    assert_eq!(RUNTIME.stringify(&BOOL_TYPE), "BoolType");

    log::debug!("RUNTIME.stringify(&123): {:#?}", RUNTIME.stringify(&123));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_eq() -> Result<()> {
    let _ = env_logger::try_init();

    assert!(RUNTIME.eq(&true, &true));
    assert!(!RUNTIME.eq(&true, &false));
    assert!(RUNTIME.eq(&true, &TRUE));
    assert!(!RUNTIME.eq(&true, &FALSE));

    assert!(!RUNTIME.eq(&false, &true));
    assert!(RUNTIME.eq(&false, &false));
    assert!(!RUNTIME.eq(&false, &TRUE));
    assert!(RUNTIME.eq(&false, &FALSE));

    assert!(RUNTIME.eq(&TRUE, &true));
    assert!(!RUNTIME.eq(&TRUE, &false));
    assert!(RUNTIME.eq(&TRUE, &TRUE));
    assert!(!RUNTIME.eq(&TRUE, &FALSE));

    assert!(!RUNTIME.eq(&FALSE, &true));
    assert!(RUNTIME.eq(&FALSE, &false));
    assert!(!RUNTIME.eq(&FALSE, &TRUE));
    assert!(RUNTIME.eq(&FALSE, &FALSE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_runtime_inhabits() -> Result<()> {
    let _ = env_logger::try_init();

    assert!(RUNTIME.inhabits(&true, &BOOL));
    assert!(RUNTIME.inhabits(&false, &BOOL));
    assert!(!RUNTIME.inhabits(&true, &FALSE_TYPE));
    assert!(RUNTIME.inhabits(&false, &FALSE_TYPE));
    assert!(RUNTIME.inhabits(&true, &TRUE_TYPE));
    assert!(!RUNTIME.inhabits(&false, &TRUE_TYPE));
    assert!(RUNTIME.inhabits(&True, &BOOL));
    assert!(RUNTIME.inhabits(&False, &BOOL));
    assert!(RUNTIME.inhabits(&BOOL, &BOOL_TYPE));
    assert!(!RUNTIME.inhabits(&BOOL_TYPE, &BOOL));
    assert!(RUNTIME.inhabits(&VOID, &VOID_TYPE));
    assert!(!RUNTIME.inhabits(&VOID_TYPE, &VOID));

    assert!(!RUNTIME.inhabits(&BOOL, &EMPTY_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_ints() -> Result<()> {
    let _ = env_logger::try_init();

    assert!(RUNTIME.inhabits(&123i8, &SINT8));
    assert!(RUNTIME.inhabits(&123i16, &SINT16));
    assert!(RUNTIME.inhabits(&123i32, &SINT32));
    assert!(RUNTIME.inhabits(&123i64, &SINT64));

    assert!(RUNTIME.inhabits(&123u8, &UINT8));
    assert!(RUNTIME.inhabits(&123u16, &UINT16));
    assert!(RUNTIME.inhabits(&123u32, &UINT32));
    assert!(RUNTIME.inhabits(&123u64, &UINT64));

    assert!(RUNTIME.inhabits(&SINT8, &SINT8_TYPE));
    assert!(RUNTIME.inhabits(&SINT16, &SINT16_TYPE));
    assert!(RUNTIME.inhabits(&SINT32, &SINT32_TYPE));
    assert!(RUNTIME.inhabits(&SINT64, &SINT64_TYPE));

    assert!(RUNTIME.inhabits(&UINT8, &UINT8_TYPE));
    assert!(RUNTIME.inhabits(&UINT16, &UINT16_TYPE));
    assert!(RUNTIME.inhabits(&UINT32, &UINT32_TYPE));
    assert!(RUNTIME.inhabits(&UINT64, &UINT64_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_floats() -> Result<()> {
    let _ = env_logger::try_init();

    assert!(RUNTIME.inhabits(&5.875f32, &FLOAT32));
    assert!(RUNTIME.inhabits(&5.875f64, &FLOAT64));

    assert!(RUNTIME.inhabits(&FLOAT32, &FLOAT32_TYPE));
    assert!(RUNTIME.inhabits(&FLOAT64, &FLOAT64_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_arrays() -> Result<()> {
    let _ = env_logger::try_init();

    let a0: Vec<Box<dyn Any>> = vec![Box::new(3i32), Box::new(5.5f32)];

    log::debug!("a0: {:#?}", a0);
    log::debug!("a0.stringify(): {:#?}", a0.stringify());

    assert!(RUNTIME.inhabits(&a0, &ARRAY));
    assert!(RUNTIME.inhabits(&ARRAY, &ARRAY_TYPE));

    Ok(())
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_abstract_type() -> Result<()> {
    let _ = env_logger::try_init();

    {
        let x = &VOID;
        log::debug!("RUNTIME.abstract_type_of({}): {}", RUNTIME.stringify(x), RUNTIME.stringify(RUNTIME.abstract_type_of(x).as_ref()));
    }

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&TYPE).as_ref(), &TYPE));

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&VOID).as_ref(), &VOID_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&VOID_TYPE).as_ref(), &TYPE));

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&BOOL).as_ref(), &BOOL_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&BOOL_TYPE).as_ref(), &TYPE));

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&TRUE).as_ref(), &TRUE_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&TRUE_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&FALSE).as_ref(), &FALSE_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&FALSE_TYPE).as_ref(), &TYPE));

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT8).as_ref(), &SINT8_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT8_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT16).as_ref(), &SINT16_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT16_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT32).as_ref(), &SINT32_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT32_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT64).as_ref(), &SINT64_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&SINT64_TYPE).as_ref(), &TYPE));

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT8).as_ref(), &UINT8_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT8_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT16).as_ref(), &UINT16_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT16_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT32).as_ref(), &UINT32_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT32_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT64).as_ref(), &UINT64_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&UINT64_TYPE).as_ref(), &TYPE));

    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&FLOAT32).as_ref(), &FLOAT32_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&FLOAT32_TYPE).as_ref(), &TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&FLOAT64).as_ref(), &FLOAT64_TYPE));
    assert!(RUNTIME.eq(RUNTIME.abstract_type_of(&FLOAT64_TYPE).as_ref(), &TYPE));

    Ok(())
}
