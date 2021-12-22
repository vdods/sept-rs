#![allow(unused_imports)]

use sept::{
    BOOL, Bool, BOOL_TYPE, BoolType, EMPTY_TYPE, FALSE, False, FALSE_TYPE, FalseType,
    FLOAT32, FLOAT32_TYPE, FLOAT64, FLOAT64_TYPE, Result, Runtime,
    SINT8, SINT8_TYPE, SINT16, SINT16_TYPE, SINT32, SINT32_TYPE, SINT64, SINT64_TYPE,
    TermTrait, TRUE, True, TRUE_TYPE, TrueType, TYPE, TypeTrait,
    UINT8, UINT8_TYPE, UINT16, UINT16_TYPE, UINT32, UINT32_TYPE, UINT64, UINT64_TYPE, VOID, VOID_TYPE,
};

#[test]
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
fn test_runtime_stringify() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = Runtime::new();

    assert_eq!(rt.stringify(&true), "True");
    assert_eq!(rt.stringify(&false), "False");
    assert_eq!(rt.stringify(&TRUE), "True");
    assert_eq!(rt.stringify(&FALSE), "False");
    assert_eq!(rt.stringify(&TRUE_TYPE), "TrueType");
    assert_eq!(rt.stringify(&FALSE_TYPE), "FalseType");
    assert_eq!(rt.stringify(&BOOL), "Bool");
    assert_eq!(rt.stringify(&BOOL_TYPE), "BoolType");

    log::debug!("rt.stringify(&123): {:#?}", rt.stringify(&123));

    Ok(())
}

#[test]
fn test_runtime_eq() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = Runtime::new();

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
fn test_runtime_inhabits() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = Runtime::new();

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
fn test_ints() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = Runtime::new();

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
fn test_floats() -> Result<()> {
    let _ = env_logger::try_init();

    let rt = Runtime::new();

    assert!(rt.inhabits(&5.875f32, &FLOAT32));
    assert!(rt.inhabits(&5.875f64, &FLOAT64));

    assert!(rt.inhabits(&FLOAT32, &FLOAT32_TYPE));
    assert!(rt.inhabits(&FLOAT64, &FLOAT64_TYPE));

    Ok(())
}
