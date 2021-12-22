#![allow(unused_imports)]

use sept::{BOOL, Bool, BOOL_TYPE, BoolType, FALSE, False, FALSE_TYPE, FalseType, Result, Runtime, TermTrait, TRUE, True, TRUE_TYPE, TrueType, TYPE, TypeTrait, VOID, VOID_TYPE};

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

    Ok(())
}
