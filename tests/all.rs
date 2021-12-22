#![allow(unused_imports)]

use sept::{BOOL, Bool, BOOL_TYPE, BoolType, FALSE, False, FALSE_TYPE, FalseType, Result, TermTrait, TRUE, True, TRUE_TYPE, TrueType, TYPE, TypeTrait, VOID, VOID_TYPE};

#[test]
fn term_and_type() -> Result<()> {
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
