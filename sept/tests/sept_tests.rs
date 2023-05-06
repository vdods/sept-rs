#![allow(unused_imports)]

use sept::{
    dy::{
        self, ArrayTerm, Constructor, Deconstruct, GlobalSymRefTerm, IntoValue, OrderedMapTerm,
        StructTerm, StructTermTerm, SymbolTable, Textifier, TupleTerm, Value, RUNTIME_LA,
    },
    parser, scanner,
    st::{
        self, Array, ArrayType, Bool, BoolType, EmptyType, False, FalseType, Float32, Float32Type,
        Float64, Float64Type, Inhabits, OrderedMap, OrderedMapType, Sint16, Sint16Type, Sint32,
        Sint32Type, Sint64, Sint64Type, Sint8, Sint8Type, Stringifiable, Struct, StructType,
        TermTrait, True, TrueType, Type, TypeTrait, Uint16, Uint16Type, Uint32, Uint32Type, Uint64,
        Uint64Type, Uint8, Uint8Type, Utf8String, Void, VoidType,
    },
};
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

/// This will run once at load time (i.e. presumably before main function is called).
#[ctor::ctor]
fn overall_init() {
    // env_logger::try_init().unwrap();

    use std::io::Write;
    env_logger::Builder::from_default_env()
        .write_style(env_logger::WriteStyle::Auto)
        .format(|buf, record| {
            let bracket_style = {
                let mut style = buf.style();
                style.set_color(env_logger::fmt::Color::Cyan);
                style
            };
            let timestamp_style = {
                let mut style = buf.style();
                style.set_color(env_logger::fmt::Color::Black);
                style.set_intense(true);
                style
            };
            let fileloc_style = {
                let mut style = buf.style();
                style.set_color(env_logger::fmt::Color::Black);
                style.set_intense(true);
                style
            };
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "{}{} {} {}{}{}{} {}",
                bracket_style.value("["),
                timestamp_style.value(chrono::Local::now().format("%Y-%m-%dT%H:%M:%S.%f")),
                level_style.value(record.level()),
                fileloc_style.value(record.file().unwrap_or("unknown")),
                fileloc_style.value(":"),
                fileloc_style.value(record.line().unwrap_or(0)),
                bracket_style.value("]"),
                record.args()
            )
        })
        .init();
}

fn test_replace_substr_in_string_case(
    target_string: &str,
    substr_char_index_start: usize,
    existing_substr: &str,
    replacement: &str,
    expected_result: &str,
) {
    let mut string = target_string.to_string();
    sept::st::replace_substr_in_string(
        &mut string,
        substr_char_index_start,
        existing_substr,
        replacement,
    )
    .expect("pass");
    assert_eq!(string.as_str(), expected_result);
}

fn test_replace_substr_in_string_case_negative(
    target_string: &str,
    substr_char_index_start: usize,
    existing_substr: &str,
    replacement: &str,
) {
    let mut string = target_string.to_string();
    sept::st::replace_substr_in_string(
        &mut string,
        substr_char_index_start,
        existing_substr,
        replacement,
    )
    .expect_err("pass");
}

#[test]
fn test_replace_substr_in_string() {
    test_replace_substr_in_string_case("", 0, "", "a", "a");
    test_replace_substr_in_string_case("", 0, "", "ab", "ab");
    test_replace_substr_in_string_case("", 0, "", "ab", "ab");
    test_replace_substr_in_string_case("a", 0, "", "ab", "aba");
    test_replace_substr_in_string_case("a", 0, "a", "b", "b");
    test_replace_substr_in_string_case("a", 0, "a", "bb", "bb");
    test_replace_substr_in_string_case("a", 0, "a", "bbb", "bbb");
    test_replace_substr_in_string_case("ab", 0, "a", "b", "bb");
    test_replace_substr_in_string_case("ab", 1, "b", "a", "aa");
    test_replace_substr_in_string_case("ab", 1, "b", "aa", "aaa");
    test_replace_substr_in_string_case("ab", 1, "b", "aaa", "aaaa");
    test_replace_substr_in_string_case("abc", 0, "a", "b", "bbc");
    test_replace_substr_in_string_case("abc", 1, "b", "a", "aac");
    test_replace_substr_in_string_case("abc", 2, "c", "b", "abb");
    test_replace_substr_in_string_case("abc", 0, "ab", "c", "cc");
    test_replace_substr_in_string_case("abc", 1, "bc", "a", "aa");
    test_replace_substr_in_string_case("abc", 2, "c", "ab", "abab");
    test_replace_substr_in_string_case("abc", 0, "abc", "d", "d");
    test_replace_substr_in_string_case("abc", 1, "bc", "d", "ad");
    test_replace_substr_in_string_case("abc", 2, "c", "d", "abd");

    test_replace_substr_in_string_case("", 0, "", "日", "日");
    test_replace_substr_in_string_case("", 0, "", "日本", "日本");
    test_replace_substr_in_string_case("", 0, "", "日本", "日本");
    test_replace_substr_in_string_case("日", 0, "", "日本", "日本日");
    test_replace_substr_in_string_case("日", 0, "日", "本", "本");
    test_replace_substr_in_string_case("日", 0, "日", "本本", "本本");
    test_replace_substr_in_string_case("日", 0, "日", "本本本", "本本本");
    test_replace_substr_in_string_case("日本", 0, "日", "本", "本本");
    test_replace_substr_in_string_case("日本", 1, "本", "日", "日日");
    test_replace_substr_in_string_case("日本", 1, "本", "日日", "日日日");
    test_replace_substr_in_string_case("日本", 1, "本", "日日日", "日日日日");
    test_replace_substr_in_string_case("日本語", 0, "日", "本", "本本語");
    test_replace_substr_in_string_case("日本語", 1, "本", "日", "日日語");
    test_replace_substr_in_string_case("日本語", 2, "語", "本", "日本本");
    test_replace_substr_in_string_case("日本語", 0, "日本", "語", "語語");
    test_replace_substr_in_string_case("日本語", 1, "本語", "日", "日日");
    test_replace_substr_in_string_case("日本語", 2, "語", "日本", "日本日本");
    test_replace_substr_in_string_case("日本語", 0, "日本語", "d", "d");
    test_replace_substr_in_string_case("日本語", 1, "本語", "d", "日d");
    test_replace_substr_in_string_case("日本語", 2, "語", "d", "日本d");

    test_replace_substr_in_string_case_negative("abc", 0, "abcd", "d");
    test_replace_substr_in_string_case_negative("abc", 1, "bcd", "d");
    test_replace_substr_in_string_case_negative("abc", 2, "cd", "d");
    test_replace_substr_in_string_case_negative("abc", 0, "abcde", "d");
    test_replace_substr_in_string_case_negative("abc", 1, "bcde", "d");
    test_replace_substr_in_string_case_negative("abc", 1, "x", "d");
    test_replace_substr_in_string_case_negative("abc", 1, "xy", "d");
    test_replace_substr_in_string_case_negative("abc", 1, "a", "d");
    test_replace_substr_in_string_case_negative("abc", 1, "abc", "d");
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

    log::debug!(
        "RUNTIME_LA.stringify(&123): {:#?}",
        runtime_g.stringify(&123)
    );
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
fn test_ordered_maps() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    // Note that Vec<Value> is OrderedMapTerm.
    // Note also that this is constructing a Vec with nonhomogeneous elements, because
    // Value stores Box<dyn Any>.
    let m0 = OrderedMapTerm::from(
        maplit::btreemap! { 3i32.into() => "blah".to_string().into(), 5.5f32.into() => Void.into() },
    );
    log::debug!("m0: {}", m0);
    log::debug!("m0 (as Debug): {:?}", m0);
    log::debug!("m0.stringify(): {}", m0.stringify());

    assert!(runtime_g.inhabits(&m0, &OrderedMap));
    assert!(runtime_g.inhabits(&OrderedMap, &OrderedMapType));

    //     let m1 = vec![100i8, 101i8, 99i8, 10i8];
    //     log::debug!("m1: {:?}", m1);
    //     log::debug!("m1.stringify(): {}", m1.stringify());
    //
    //     assert!(m1.inhabits(&OrderedMap));
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

fn test_prefix_partial_cmp_case(
    lhs: &TupleTerm,
    rhs: &TupleTerm,
    expected_result: Option<std::cmp::Ordering>,
) {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let expected_result_of_reversed_cmp = expected_result.map(|ordering| ordering.reverse());
    assert_eq!(dy::prefix_partial_cmp(lhs, rhs), expected_result);
    assert_eq!(
        dy::prefix_partial_cmp(rhs, lhs),
        expected_result_of_reversed_cmp
    );
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_prefix_partial_cmp() {
    let empty = TupleTerm::from(vec![]);
    let a1 = TupleTerm::from(vec![111u32.into()]);
    let a1_2 = TupleTerm::from(vec![111u32.into(), 222u32.into()]);
    let b1 = TupleTerm::from(vec![true.into()]);
    let b1_2 = TupleTerm::from(vec![true.into(), false.into()]);
    let c = TupleTerm::from(vec![111u32.into(), false.into()]);
    let d = TupleTerm::from(vec![true.into(), 222u32.into()]);

    use std::cmp::Ordering::{Equal, Greater, Less};

    // Check each pair (this checks (lhs, rhs) and (rhs, lhs))

    test_prefix_partial_cmp_case(&empty, &empty, Some(Equal));
    test_prefix_partial_cmp_case(&empty, &a1, Some(Less));
    test_prefix_partial_cmp_case(&empty, &a1_2, Some(Less));
    test_prefix_partial_cmp_case(&empty, &b1, Some(Less));
    test_prefix_partial_cmp_case(&empty, &b1_2, Some(Less));
    test_prefix_partial_cmp_case(&empty, &c, Some(Less));
    test_prefix_partial_cmp_case(&empty, &d, Some(Less));

    test_prefix_partial_cmp_case(&a1, &a1, Some(Equal));
    test_prefix_partial_cmp_case(&a1, &a1_2, Some(Less));
    test_prefix_partial_cmp_case(&a1, &b1, None);
    test_prefix_partial_cmp_case(&a1, &b1_2, None);
    test_prefix_partial_cmp_case(&a1, &c, Some(Less));
    test_prefix_partial_cmp_case(&a1, &d, None);

    test_prefix_partial_cmp_case(&a1_2, &a1_2, Some(Equal));
    test_prefix_partial_cmp_case(&a1_2, &b1, None);
    test_prefix_partial_cmp_case(&a1_2, &b1_2, None);
    test_prefix_partial_cmp_case(&a1_2, &c, None);
    test_prefix_partial_cmp_case(&a1_2, &d, None);

    test_prefix_partial_cmp_case(&b1, &b1, Some(Equal));
    test_prefix_partial_cmp_case(&b1, &b1_2, Some(Less));
    test_prefix_partial_cmp_case(&b1, &c, None);
    test_prefix_partial_cmp_case(&b1, &d, Some(Less));

    test_prefix_partial_cmp_case(&b1_2, &b1_2, Some(Equal));
    test_prefix_partial_cmp_case(&b1_2, &c, None);
    test_prefix_partial_cmp_case(&b1_2, &d, None);

    test_prefix_partial_cmp_case(&c, &c, Some(Equal));
    test_prefix_partial_cmp_case(&c, &d, None);

    test_prefix_partial_cmp_case(&d, &d, Some(Equal));
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_abstract_type() {
    let runtime_g = RUNTIME_LA.read().unwrap();

    {
        let x = &Void;
        log::debug!(
            "runtime_g.abstract_type_of({}): {}",
            runtime_g.stringify(x),
            runtime_g.stringify(runtime_g.abstract_type_of(x).as_ref())
        );
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
    log::debug!(
        "v1.inhabits(&Value::from(Sint32)): {:?}",
        v1.inhabits(&Value::from(Sint32))
    );
    log::debug!(
        "v1.inhabits(&Value::from(Bool)): {:?}",
        v1.inhabits(&Value::from(Bool))
    );
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

    let mut symbol_table = SymbolTable::new_without_parent("fancy".to_string()).expect("test");
    assert!(!symbol_table.symbol_is_defined("test_symbol_table_blah"));
    symbol_table
        .define_symbol("test_symbol_table_blah", Value::from(123i32))
        .expect("test");
    assert!(symbol_table.symbol_is_defined("test_symbol_table_blah"));
    assert_eq!(
        *symbol_table
            .resolved_symbol("test_symbol_table_blah")
            .expect("test")
            .read()
            .unwrap(),
        Value::from(123i32)
    );

    log::debug!("symbol_table: {:#?}", symbol_table);

    // Now check GLOBAL_SYMBOL_TABLE_LA
    {
        assert!(!dy::GLOBAL_SYMBOL_TABLE_LA
            .read()
            .unwrap()
            .symbol_is_defined("test_symbol_table_bleh"));

        // Have to acquire a separate write lock.
        dy::GLOBAL_SYMBOL_TABLE_LA
            .write()
            .unwrap()
            .define_symbol("test_symbol_table_bleh", Value::from(456f32))
            .expect("test");

        // Now acquire a read lock.
        let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
        assert!(global_symbol_table_g.symbol_is_defined("test_symbol_table_bleh"));
        assert_eq!(
            *global_symbol_table_g
                .resolved_symbol("test_symbol_table_bleh")
                .expect("test")
                .read()
                .unwrap(),
            Value::from(456f32)
        );

        log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
    }

    // Test out parent symbol tables.
    let parent_symbol_table_la = Arc::new(RwLock::new(
        SymbolTable::new_without_parent("P".to_string()).expect("test"),
    ));
    let child_symbol_table_la = Arc::new(RwLock::new(
        SymbolTable::new_with_parent("C".to_string(), parent_symbol_table_la.clone())
            .expect("test"),
    ));

    parent_symbol_table_la
        .write()
        .unwrap()
        .define_symbol("test_symbol_table_stuff", Value::from(200u32))
        .expect("test");
    parent_symbol_table_la
        .write()
        .unwrap()
        .define_symbol("test_symbol_table_hippo", Value::from(300u32))
        .expect("test");

    child_symbol_table_la
        .write()
        .unwrap()
        .define_symbol("test_symbol_table_stuff", Value::from(444u32))
        .expect("test");

    assert_eq!(
        *parent_symbol_table_la
            .read()
            .unwrap()
            .resolved_symbol("test_symbol_table_stuff")
            .expect("test")
            .read()
            .unwrap(),
        Value::from(200u32)
    );
    assert_eq!(
        *parent_symbol_table_la
            .read()
            .unwrap()
            .resolved_symbol("test_symbol_table_hippo")
            .expect("test")
            .read()
            .unwrap(),
        Value::from(300u32)
    );

    assert_eq!(
        *child_symbol_table_la
            .read()
            .unwrap()
            .resolved_symbol("test_symbol_table_stuff")
            .expect("test")
            .read()
            .unwrap(),
        Value::from(444u32)
    );
    assert_eq!(
        *child_symbol_table_la
            .read()
            .unwrap()
            .resolved_symbol("test_symbol_table_hippo")
            .expect("test")
            .read()
            .unwrap(),
        Value::from(300u32)
    );

    log::debug!(
        "child_symbol_table:\n{:#?}",
        child_symbol_table_la.read().unwrap()
    );
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_global_sym_ref_term() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    // Write a bunch of stuff into the global_symbol_table
    {
        let mut global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap();
        global_symbol_table_g
            .define_symbol("test_global_sym_ref_term_bleh", Value::from(456f32))
            .expect("test");
        global_symbol_table_g
            .define_symbol("test_global_sym_ref_term_stuff", Value::from(True {}))
            .expect("test");
        global_symbol_table_g
            .define_symbol("test_global_sym_ref_term_andthings", Value::from(Void {}))
            .expect("test");
    }

    // Now check GlobalSymRefTerm.
    let r = GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_bleh".into());
    log::debug!("r (as Debug): {:#?}", r);
    log::debug!("r (as Display): {}", r);
    log::debug!("r: {}", r.stringify());

    let t = TupleTerm::from(vec![
        GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_bleh".into()).into(),
        GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_stuff".into()).into(),
        GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_andthings".into()).into(),
    ]);
    log::debug!("t (as Debug): {:#?}", t);
    log::debug!("t.type_id(): {:?}", t.type_id());
    log::debug!("t (as Display): {}", t);
    log::debug!("t: {}", t.stringify());

    // Test dereferenced
    {
        let r_resolved_la = r.resolved().expect("test");
        let r_resolved_g = r_resolved_la.read().unwrap();
        log::debug!("r_resolved_g (as Debug): {:#?}", r_resolved_g);
        log::debug!(
            "r_resolved_g.as_ref() (as Debug): {:#?}",
            r_resolved_g.as_ref()
        );

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
        log::debug!(
            "r_resolved_g.as_ref() (as Debug): {:#?}",
            r_resolved_g.as_ref()
        );

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
            global_symbol_table_g
                .define_symbol("test_global_sym_ref_term_inner", Value::from(40404u32))
                .expect("test");
            global_symbol_table_g
                .define_symbol(
                    "test_global_sym_ref_term_outer",
                    Value::from(GlobalSymRefTerm::new_unchecked(
                        "test_global_sym_ref_term_inner".into(),
                    )),
                )
                .expect("test");
            global_symbol_table_g
                .define_symbol(
                    "test_global_sym_ref_term_outerer",
                    Value::from(GlobalSymRefTerm::new_unchecked(
                        "test_global_sym_ref_term_outer".into(),
                    )),
                )
                .expect("test");
        }

        {
            let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
            log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
        }

        let inner_ref = GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_inner".into());
        let outer_ref = GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_outer".into());
        let outerer_ref =
            GlobalSymRefTerm::new_unchecked("test_global_sym_ref_term_outerer".into());

        let runtime_g = dy::RUNTIME_LA.read().unwrap();

        {
            let inner_dereferenced_once_la = runtime_g.dereferenced_once(&inner_ref).expect("test");
            let inner_dereferenced_once_g = inner_dereferenced_once_la.read().unwrap();
            log::debug!(
                "inner_dereferenced_once_g (as Debug): {:#?}",
                inner_dereferenced_once_g
            );
            log::debug!(
                "inner_dereferenced_once_g: {}",
                inner_dereferenced_once_g.stringify()
            );
            assert_eq!(*inner_dereferenced_once_g, test_value);
        }

        {
            let outer_dereferenced_once_la = runtime_g.dereferenced_once(&outer_ref).expect("test");
            let outer_dereferenced_once_g = outer_dereferenced_once_la.read().unwrap();
            log::debug!(
                "outer_dereferenced_once_g (as Debug): {:#?}",
                outer_dereferenced_once_g
            );
            log::debug!(
                "outer_dereferenced_once_g: {}",
                outer_dereferenced_once_g.stringify()
            );
            assert_eq!(*outer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        {
            let outerer_dereferenced_once_la =
                runtime_g.dereferenced_once(&outerer_ref).expect("test");
            let outerer_dereferenced_once_g = outerer_dereferenced_once_la.read().unwrap();
            log::debug!(
                "outerer_dereferenced_once_g (as Debug): {:#?}",
                outerer_dereferenced_once_g
            );
            log::debug!(
                "outerer_dereferenced_once_g: {}",
                outerer_dereferenced_once_g.stringify()
            );
            assert_eq!(*outerer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        assert_eq!(
            *inner_ref.resolved().expect("test").read().unwrap(),
            test_value
        );
        assert_eq!(
            *outer_ref.resolved().expect("test").read().unwrap(),
            test_value
        );
        assert_eq!(
            *outerer_ref.resolved().expect("test").read().unwrap(),
            test_value
        );
    }
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_local_sym_ref_term() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

    let local_symbol_table_la = Arc::new(RwLock::new(
        SymbolTable::new_without_parent("fancy".to_string()).expect("test"),
    ));
    local_symbol_table_la
        .write()
        .unwrap()
        .define_symbol("test_local_sym_ref_term_blah", dy::Value::from(123i32))
        .expect("test");
    log::debug!(
        "local_symbol_table_la: {:#?}",
        local_symbol_table_la.read().unwrap()
    );

    let local_sym_ref_term = dy::LocalSymRefTerm::new_checked(
        local_symbol_table_la.clone(),
        "test_local_sym_ref_term_blah".into(),
    )
    .expect("test");
    log::debug!("local_sym_ref_term: (as Debug) {:#?}", local_sym_ref_term);
    log::debug!("local_sym_ref_term: (as Display) {}", local_sym_ref_term);
    log::debug!("local_sym_ref_term: {}", local_sym_ref_term.stringify());

    // Test nested references
    {
        let test_value = Value::from(51515u32);

        // Write more stuff into the local_symbol_table
        {
            let mut local_symbol_table_g = local_symbol_table_la.write().unwrap();
            local_symbol_table_g
                .define_symbol("test_local_sym_ref_term_inner", Value::from(51515u32))
                .expect("test");
            local_symbol_table_g
                .define_symbol(
                    "test_local_sym_ref_term_outer",
                    Value::from(dy::LocalSymRefTerm::new_unchecked(
                        local_symbol_table_la.clone(),
                        "test_local_sym_ref_term_inner".into(),
                    )),
                )
                .expect("test");
            local_symbol_table_g
                .define_symbol(
                    "test_local_sym_ref_term_outerer",
                    Value::from(dy::LocalSymRefTerm::new_unchecked(
                        local_symbol_table_la.clone(),
                        "test_local_sym_ref_term_outer".into(),
                    )),
                )
                .expect("test");
        }

        {
            let local_symbol_table_g = local_symbol_table_la.read().unwrap();
            log::debug!("local_symbol_table_g: {:#?}", local_symbol_table_g);
        }

        let inner_ref = dy::LocalSymRefTerm::new_unchecked(
            local_symbol_table_la.clone(),
            "test_local_sym_ref_term_inner".into(),
        );
        let outer_ref = dy::LocalSymRefTerm::new_unchecked(
            local_symbol_table_la.clone(),
            "test_local_sym_ref_term_outer".into(),
        );
        let outerer_ref = dy::LocalSymRefTerm::new_unchecked(
            local_symbol_table_la.clone(),
            "test_local_sym_ref_term_outerer".into(),
        );

        let runtime_g = dy::RUNTIME_LA.read().unwrap();

        {
            let inner_dereferenced_once_la = runtime_g.dereferenced_once(&inner_ref).expect("test");
            let inner_dereferenced_once_g = inner_dereferenced_once_la.read().unwrap();
            log::debug!(
                "inner_dereferenced_once_g (as Debug): {:#?}",
                inner_dereferenced_once_g
            );
            log::debug!(
                "inner_dereferenced_once_g: {}",
                inner_dereferenced_once_g.stringify()
            );
            assert_eq!(*inner_dereferenced_once_g, test_value);
        }

        {
            let outer_dereferenced_once_la = runtime_g.dereferenced_once(&outer_ref).expect("test");
            let outer_dereferenced_once_g = outer_dereferenced_once_la.read().unwrap();
            log::debug!(
                "outer_dereferenced_once_g (as Debug): {:#?}",
                outer_dereferenced_once_g
            );
            log::debug!(
                "outer_dereferenced_once_g: {}",
                outer_dereferenced_once_g.stringify()
            );
            assert_eq!(*outer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        {
            let outerer_dereferenced_once_la =
                runtime_g.dereferenced_once(&outerer_ref).expect("test");
            let outerer_dereferenced_once_g = outerer_dereferenced_once_la.read().unwrap();
            log::debug!(
                "outerer_dereferenced_once_g (as Debug): {:#?}",
                outerer_dereferenced_once_g
            );
            log::debug!(
                "outerer_dereferenced_once_g: {}",
                outerer_dereferenced_once_g.stringify()
            );
            assert_eq!(*outerer_dereferenced_once_g, Value::from(inner_ref.clone()));
        }

        assert_eq!(
            *inner_ref.resolved().expect("test").read().unwrap(),
            test_value
        );
        assert_eq!(
            *outer_ref.resolved().expect("test").read().unwrap(),
            test_value
        );
        assert_eq!(
            *outerer_ref.resolved().expect("test").read().unwrap(),
            test_value
        );
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
    dy::GLOBAL_SYMBOL_TABLE_LA
        .write()
        .unwrap()
        .define_symbol(
            "Hippo",
            StructTerm::new(
                vec![
                    ("age".into(), Uint8.into()),
                    ("gravity".into(), Float64.into()),
                ]
                .into(),
            )
            .expect("test")
            .into(),
        )
        .expect("test");

    let global_symbol_table_g = dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
    log::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
    let hippo = GlobalSymRefTerm::new_unchecked("Hippo".into());
    log::debug!("hippo: {}", hippo.stringify());

    let x = global_symbol_table_g
        .resolved_symbol("Hippo")
        .expect("test")
        .read()
        .unwrap()
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![23u8.into(), 999.0f64.into()].into())
        .expect("test");
    let y = global_symbol_table_g
        .resolved_symbol("Hippo")
        .expect("test")
        .read()
        .unwrap()
        .downcast_ref::<StructTerm>()
        .unwrap()
        .construct(vec![100u8.into(), (-3.0f64).into()].into())
        .expect("test");
    log::debug!("x: {}", x.stringify());
    log::debug!("y: {}", y.stringify());
    log::debug!("x == y: {}", x == y);

    assert_eq!(x, x);
    assert_eq!(y, y);
    assert!(x != y);
    assert!(y != x);

    let x2 = dy::StructTermTerm::new_checked(
        hippo.clone().into(),
        vec![23u8.into(), 999.0f64.into()].into(),
    )
    .expect("test");
    let y2 = dy::StructTermTerm::new_checked(
        hippo.clone().into(),
        vec![100u8.into(), (-3.0f64).into()].into(),
    )
    .expect("test");

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
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();

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
        let s = StructTerm::new(vec![
            ("name".into(), Utf8String.into()),
            ("age".into(), Uint8.into()),
        ])
        .expect("test");
        log::debug!("s (stringify): {}", s.stringify());
        {
            let deconstruction = s.deconstructed();
            log::debug!("s.deconstructed(): {:#?}", deconstruction);
        }
        {
            let deconstruction = s.clone().deconstruct();
            log::debug!("s.deconstruct(): {:#?}", deconstruction);
        }

        let s_term = StructTermTerm::new_checked(
            s.clone().into(),
            TupleTerm::from((String::from("Hippo"), 99u8)),
        )
        .expect("test");
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

    {
        dy::GLOBAL_SYMBOL_TABLE_LA
            .write()
            .unwrap()
            .define_symbol("thingy", 1234.5678f64.into())
            .expect("test");
        let g = GlobalSymRefTerm::new_unchecked("thingy".into());
        let deconstruction = g.deconstructed();
        log::debug!("g.deconstructed(): {:#?}", deconstruction);
    }
}

fn test_deconstruct_reconstruct_roundtrip<T, C>(x: T)
where
    T: Deconstruct + Stringifiable + PartialEq,
    C: Constructor + Stringifiable,
    <C as Constructor>::ConstructedType: std::fmt::Display + PartialEq<T>,
{
    log::debug!("x: {}", x.stringify());
    log::debug!("x (as Debug): {:#?}", x);
    let x_deconstruction = x.deconstructed();
    log::debug!("x_deconstruction (as Debug): {:#?}", x_deconstruction);
    assert_eq!(x_deconstruction.kind(), dy::DeconstructionKind::Parametric);
    let parameterization = x_deconstruction.clone().into_parametric().unwrap();
    log::debug!("parameterization (as Debug): {:#?}", parameterization);
    assert!(parameterization
        .constructor_d
        .reconstruct()
        .expect("test")
        .is::<C>());
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

    test_deconstruct_reconstruct_roundtrip::<TupleTerm, st::Tuple>(TupleTerm::from((
        123i8,
        99u32,
        100.25f32,
        String::from("HIPPO"),
    )));
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
    test_textify_case(
        String::from("Hippos and Hippas"),
        "Utf8String(\"Hippos and Hippas\")",
    );
    test_textify_case(
        TupleTerm::from((123i8, 99u32, 100.25f32, String::from("HIPPO"))),
        "Tuple(Sint8(123), Uint32(99), Float32(100.25), Utf8String(\"HIPPO\"))",
    );
    test_textify_case(
        StructTerm::new(vec![
            ("name".into(), Utf8String.into()),
            ("score".into(), Uint64.into()),
        ])
        .expect("test"),
        "Struct(Tuple(Utf8String(\"name\"), Utf8String), Tuple(Utf8String(\"score\"), Uint64))",
    );
    test_textify_case(
        GlobalSymRefTerm::new_unchecked("fancyfancy".into()),
        "GlobalSymRef(Utf8String(\"fancyfancy\"))",
    );
}

fn test_try_scanning_case(
    token_kind: scanner::TokenKind,
    input: &str,
    expected_token_o: Option<scanner::Token>,
) {
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

    test_try_scanning_case(
        TokenKind::Whitespace,
        " ",
        Some(Token::Whitespace(" ".into())),
    );
    test_try_scanning_case(
        TokenKind::Whitespace,
        "\t",
        Some(Token::Whitespace("\t".into())),
    );
    test_try_scanning_case(
        TokenKind::Whitespace,
        "\n",
        Some(Token::Whitespace("\n".into())),
    );
    test_try_scanning_case(
        TokenKind::Whitespace,
        "    \t  \n\n\n",
        Some(Token::Whitespace("    \t  \n\n\n".into())),
    );
    test_try_scanning_case(
        TokenKind::Whitespace,
        "    \t  \n\n\n!",
        Some(Token::Whitespace("    \t  \n\n\n".into())),
    );
    test_try_scanning_case(TokenKind::Whitespace, "x", None);
    test_try_scanning_case(TokenKind::Whitespace, "", None);
    test_try_scanning_case(TokenKind::Whitespace, "\x7F", None);

    test_try_scanning_case(
        TokenKind::CIdentifier,
        "xyz",
        Some(Token::CIdentifier("xyz".into())),
    );
    test_try_scanning_case(
        TokenKind::CIdentifier,
        "ABC__123",
        Some(Token::CIdentifier("ABC__123".into())),
    );
    test_try_scanning_case(
        TokenKind::CIdentifier,
        "_x_100y_z",
        Some(Token::CIdentifier("_x_100y_z".into())),
    );
    test_try_scanning_case(
        TokenKind::CIdentifier,
        "_x_100y_z.",
        Some(Token::CIdentifier("_x_100y_z".into())),
    );
    test_try_scanning_case(TokenKind::CIdentifier, ".", None);
    test_try_scanning_case(TokenKind::CIdentifier, "1", None);
    test_try_scanning_case(TokenKind::CIdentifier, "(", None);
    test_try_scanning_case(TokenKind::CIdentifier, ")", None);
    test_try_scanning_case(TokenKind::CIdentifier, "", None);
    test_try_scanning_case(TokenKind::CIdentifier, "\x7F", None);

    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "0.",
        Some(Token::DecimalPointLiteral("0.".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        ".0",
        Some(Token::DecimalPointLiteral(".0".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "1.",
        Some(Token::DecimalPointLiteral("1.".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        ".2",
        Some(Token::DecimalPointLiteral(".2".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "3.4",
        Some(Token::DecimalPointLiteral("3.4".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "-3.4",
        Some(Token::DecimalPointLiteral("-3.4".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "-3.",
        Some(Token::DecimalPointLiteral("-3.".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "-.4",
        Some(Token::DecimalPointLiteral("-.4".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+3.4",
        Some(Token::DecimalPointLiteral("+3.4".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+3.",
        Some(Token::DecimalPointLiteral("+3.".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+.4",
        Some(Token::DecimalPointLiteral("+.4".into())),
    );

    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "0.e0",
        Some(Token::DecimalPointLiteral("0.e0".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        ".0e1",
        Some(Token::DecimalPointLiteral(".0e1".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "1.e100",
        Some(Token::DecimalPointLiteral("1.e100".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        ".2e-0",
        Some(Token::DecimalPointLiteral(".2e-0".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "3.4e-10",
        Some(Token::DecimalPointLiteral("3.4e-10".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "-3.4e8",
        Some(Token::DecimalPointLiteral("-3.4e8".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "-3.e8",
        Some(Token::DecimalPointLiteral("-3.e8".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "-.4e8",
        Some(Token::DecimalPointLiteral("-.4e8".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+3.4e8",
        Some(Token::DecimalPointLiteral("+3.4e8".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+3.e8",
        Some(Token::DecimalPointLiteral("+3.e8".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+.4e8",
        Some(Token::DecimalPointLiteral("+.4e8".into())),
    );

    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+.4e8xyz",
        Some(Token::DecimalPointLiteral("+.4e8".into())),
    );
    test_try_scanning_case(
        TokenKind::DecimalPointLiteral,
        "+.4e8)",
        Some(Token::DecimalPointLiteral("+.4e8".into())),
    );

    test_try_scanning_case(TokenKind::DecimalPointLiteral, ".", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "0", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "1", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "4e100", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "abcxyz", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "__", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "(", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, ")", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "", None);
    test_try_scanning_case(TokenKind::DecimalPointLiteral, "\x7F", None);

    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "0",
        Some(Token::IntegerLiteral("0".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "1",
        Some(Token::IntegerLiteral("1".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "1234",
        Some(Token::IntegerLiteral("1234".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "99999999999999999999999",
        Some(Token::IntegerLiteral("99999999999999999999999".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "-0",
        Some(Token::IntegerLiteral("-0".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "-1",
        Some(Token::IntegerLiteral("-1".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "-1234",
        Some(Token::IntegerLiteral("-1234".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "-99999999999999999999999",
        Some(Token::IntegerLiteral("-99999999999999999999999".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "+0",
        Some(Token::IntegerLiteral("+0".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "+1",
        Some(Token::IntegerLiteral("+1".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "+1234",
        Some(Token::IntegerLiteral("+1234".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "+99999999999999999999999",
        Some(Token::IntegerLiteral("+99999999999999999999999".into())),
    );
    // Note that this doesn't try to match the whole string.
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "1.",
        Some(Token::IntegerLiteral("1".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "1x",
        Some(Token::IntegerLiteral("1".into())),
    );
    test_try_scanning_case(
        TokenKind::IntegerLiteral,
        "1)",
        Some(Token::IntegerLiteral("1".into())),
    );
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
        r#""\0""#,
        r#""\a""#,
        r#""\b""#,
        r#""\t""#,
        r#""\n""#,
        r#""\v""#,
        r#""\f""#,
        r#""\r""#,
        r#""\"""#,
        r#""\\""#,
        r#""\x00""#,
        r#""\xAA""#,
        r#""\x93""#,
        r#""\x7f""#,
        r#""\xee""#,
        r#""\xFf""#,
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
    scanner::scan(input).expect_err("pass");
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_scan() {
    use scanner::Token;
    test_scan_case("", vec![]);
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
    for input in vec![
        "3",
        "+8",
        "-9",
        "10",
        "0",
        "+0",
        "-0",
        "-1000",
        "999999999999999999999999999999999999999999999999999999999999999999",
    ] {
        test_scan_case(input, vec![Token::IntegerLiteral(input.into())]);
    }
    for input in vec![
        "4.",
        ".5",
        "6.7",
        "0.0",
        "+8.",
        "-9.",
        "4.e0",
        "4.e+0",
        "4.e-0",
        ".5e1",
        "1.6e10",
        "8.05e-20",
        "-999999.1010101010e+666666666",
    ] {
        test_scan_case(input, vec![Token::DecimalPointLiteral(input.into())]);
    }
    for input in vec![
        r#""""#,
        r#""\0""#,
        r#""\a""#,
        r#""\b""#,
        r#""\t""#,
        r#""\n""#,
        r#""\v""#,
        r#""\f""#,
        r#""\r""#,
        r#""\"""#,
        r#""\\""#,
        r#""\x00""#,
        r#""\xAA""#,
        r#""\x93""#,
        r#""\x7f""#,
        r#""\xee""#,
        r#""\xFf""#,
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

fn test_parse_value_case(input: &str, expected_value: dy::Value) {
    log::debug!("input: {:?}", input);
    let actual_value = parser::parse_value(input).expect("test");
    log::debug!("actual_value: {:?}", actual_value);
    // TODO: Also do an equality check that prevents referential transparency, so that symbolic
    // refs can be tested on a non-referenced level.
    assert_eq!(actual_value, expected_value);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_parse_value() {
    // Have to clear the global_symbol_table, since we don't know what order the tests will run in.
    dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap().clear();
    dy::GLOBAL_SYMBOL_TABLE_LA
        .write()
        .unwrap()
        .define_symbol("thingy", true.into())
        .expect("test");
    dy::GLOBAL_SYMBOL_TABLE_LA
        .write()
        .unwrap()
        .define_symbol("bloppy", 123u32.into())
        .expect("test");
    dy::GLOBAL_SYMBOL_TABLE_LA
        .write()
        .unwrap()
        .define_symbol("Tuple", st::Tuple.into())
        .expect("test");

    test_parse_value_case("Tuple", st::Tuple.into());
    test_parse_value_case(
        "Tuple(thingy, bloppy)",
        dy::TupleTerm::from(vec![dy::Value::from(true), dy::Value::from(123u32)]).into(),
    );
    //     test_parse_value_case("Tuple(Sint8(123), Bool(true), Void, Utf8String)");

    //     test_parse_value_case(
    //         "ArrayES(Float64, 4)(100.0, 8.9, 0.0, 1.0)",
    //         st::ArrayES
    //             .construct(dy::TupleTerm::from(vec![st::Float64, 4]))
    //             .construct(dy::TupleTerm::from(vec![100.0f64, 8.9f64, 0.0f64, 1.0f64])),
    //     );
    test_parse_value_case(
        "Tuple(Float64, Uint64)(100.0, 89)",
        dy::TupleTerm::from(vec![dy::Value::from(100.0f64), dy::Value::from(89u64)]).into(),
    );

    //     test_parse_value_case("GlobalSymRef(Utf8String(\"weewoo\"))");
    test_parse_value_case(
        "thingy",
        dy::GlobalSymRefTerm::new_unchecked("thingy".into()).into(),
    );
}

fn test_parse_deconstruction_case(input: &str, expected_value: dy::Value) {
    log::debug!("test_parse_deconstruction_case; input: {}", input);
    let deconstruction = parser::parse_deconstruction(input).expect("test");
    log::debug!("deconstruction: {:?}", deconstruction);
    let reconstructed_value = deconstruction.reconstruct().expect("test");
    log::debug!("reconstructed_value: {:?}", reconstructed_value);
    assert_eq!(reconstructed_value, expected_value);
}

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn test_parse_deconstruction() {
    test_parse_deconstruction_case("true", dy::Value::from(true));
    test_parse_deconstruction_case("false", dy::Value::from(false));
    test_parse_deconstruction_case("True", dy::Value::from(st::True));
    test_parse_deconstruction_case("False", dy::Value::from(st::False));
    test_parse_deconstruction_case("Tuple", dy::Value::from(st::Tuple));
    test_parse_deconstruction_case("Tuple()", dy::Value::from(dy::TupleTerm::from(vec![])));
    test_parse_deconstruction_case(
        "Tuple(True, False)",
        dy::Value::from(dy::TupleTerm::from((true, false))),
    );

    test_parse_deconstruction_case("Float64(4.5)", dy::Value::from(4.5f64));
    test_parse_deconstruction_case("Float64(4.e10)", dy::Value::from(4.0e10f64));
    test_parse_deconstruction_case("Float64(-.01)", dy::Value::from(-0.01f64));

    //     test_parse_deconstruction_case("Float32(4.5)", dy::Value::from(4.5f32));
    //     test_parse_deconstruction_case("Float32(4.e10)", dy::Value::from(4.0e10f32));
    //     test_parse_deconstruction_case("Float32(-.01)", dy::Value::from(-0.01f32));

    // TODO: Add test cases for UnicodeChar

    test_parse_deconstruction_case(
        "Utf8String(\"blah\\n\\thh\")",
        dy::Value::from(String::from("blah\n\thh")),
    );

    test_parse_deconstruction_case(
        "Struct(Tuple(Utf8String(\"name\"), Utf8String), Tuple(Utf8String(\"score\"), Uint64))",
        dy::Value::from(
            StructTerm::new(vec![
                ("name".into(), Utf8String.into()),
                ("score".into(), Uint64.into()),
            ])
            .expect("test"),
        ),
    );
}

fn test_runtime_registration_case<T: st::TermTrait>() {
    assert!(
        dy::RUNTIME_LA.read().unwrap().is_registered_term::<T>(),
        "type {} is not a registered term",
        std::any::type_name::<T>()
    );
}

#[test]
#[serial_test::serial]
fn test_runtime_registrations() {
    sept::for_each_non_parametric_term!(T, test_runtime_registration_case::<T>());
    sept::for_each_parametric_term!(T, test_runtime_registration_case::<T>());
}

fn test_serialize_deserialize_case<
    T: PartialEq + st::Deserializable + st::Serializable + st::TermTrait,
>(
    x: &T,
) {
    log::debug!("test_serialize_deserialize_case; x: {:#?}", x);
    let mut serialized_byte_v = Vec::new();
    x.serialize(&mut serialized_byte_v).expect("pass");
    let x_deserialized = T::deserialize(&mut serialized_byte_v.as_slice()).expect("pass");
    assert_eq!(x_deserialized, *x);
    log::debug!("test_serialize_deserialize_case; PASSED: x: {:#?}", x);
}

fn test_serialize_deserialize_case_as_value<
    T: PartialEq + st::Deserializable + dy::IntoValue + st::Serializable + st::TermTrait,
>(
    x: &T,
) {
    log::debug!("test_serialize_deserialize_case_as_value; x: {:#?}", x);
    let x_as_value = dy::Value::from(x.clone());
    log::debug!("x_as_value.type_id(): {:?}", x_as_value.type_id());
    let mut serialized_byte_v = Vec::new();
    use st::Serializable;
    x_as_value.serialize(&mut serialized_byte_v).expect("pass");
    use st::Deserializable;
    let x_deserialized = dy::Value::deserialize(&mut serialized_byte_v.as_slice()).expect("pass");
    assert_eq!(x_deserialized, x_as_value);
    log::debug!(
        "test_serialize_deserialize_case_as_value; PASSED: x: {:#?}",
        x
    );
}

fn test_serialize_deserialize_test_values<
    T: PartialEq
        + st::Deserializable
        + dy::IntoValue
        + st::Serializable
        + st::TermTrait
        + st::TestValues,
>() {
    // TODO: Also generate random test values
    use st::TestValues;
    for x in T::fixed_test_values() {
        test_serialize_deserialize_case(&x);
        test_serialize_deserialize_case_as_value(&x);
    }
}

#[test]
#[serial_test::serial]
fn test_serialize_deserialize() {
    sept::for_each_non_parametric_term!(T, test_serialize_deserialize_test_values::<T>());

    test_serialize_deserialize_test_values::<bool>();
    test_serialize_deserialize_test_values::<i8>();
    test_serialize_deserialize_test_values::<i16>();
    test_serialize_deserialize_test_values::<i32>();
    test_serialize_deserialize_test_values::<i64>();
    test_serialize_deserialize_test_values::<u8>();
    test_serialize_deserialize_test_values::<u16>();
    test_serialize_deserialize_test_values::<u32>();
    test_serialize_deserialize_test_values::<u64>();
    test_serialize_deserialize_test_values::<f32>();
    test_serialize_deserialize_test_values::<f64>();
    test_serialize_deserialize_test_values::<String>();

    test_serialize_deserialize_test_values::<ArrayTerm>();
    test_serialize_deserialize_test_values::<OrderedMapTerm>();
    // {
    //     // For this one, need to ensure that the appropriate symbol is defined in the global symbol table.
    //     {
    //         let mut global_symbol_table_g = GLOBAL_SYMBOL_TABLE_LA.write().unwrap();
    //         global_symbol_table_g.clear();
    //         global_symbol_table_g
    //             .define_symbol(
    //                 "fixed_test_value",
    //                 "hippos and ostriches are natural enemies"
    //                     .to_string()
    //                     .into(),
    //             )
    //             .expect("pass");
    //     }
    //     test_serialize_deserialize_test_values::<GlobalSymRefTerm>();
    // }
    test_serialize_deserialize_test_values::<StructTerm>();
    test_serialize_deserialize_test_values::<StructTermTerm>();
}

// Queryable is going defunct.
// #[test]
// #[serial_test::serial]
// fn test_queryable_array_term() {
//     let a = ArrayTerm::from(vec![
//         "abc".to_string().into(),
//         123.456f32.into(),
//         true.into(),
//         Void.into(),
//     ]);

//     use dy::Queryable;

//     assert_eq!(
//         a.query(&[])
//             .expect("pass")
//             .downcast_ref::<ArrayTerm>()
//             .expect("pass"),
//         &a
//     );
//     assert_eq!(
//         a.query(&[0u32.into()])
//             .expect("pass")
//             .downcast_ref::<String>()
//             .expect("pass"),
//         a[0].downcast_ref::<String>().expect("pass")
//     );
//     assert_eq!(
//         a.query(&[1u32.into()])
//             .expect("pass")
//             .downcast_ref::<f32>()
//             .expect("pass"),
//         a[1].downcast_ref::<f32>().expect("pass")
//     );
//     assert_eq!(
//         a.query(&[2u32.into()])
//             .expect("pass")
//             .downcast_ref::<bool>()
//             .expect("pass"),
//         a[2].downcast_ref::<bool>().expect("pass")
//     );
//     assert_eq!(
//         a.query(&[3u32.into()])
//             .expect("pass")
//             .downcast_ref::<Void>()
//             .expect("pass"),
//         a[3].downcast_ref::<Void>().expect("pass")
//     );
//     a.query(&[4u32.into()]).expect_err("pass");
// }

// NOTE: dy::Diffable is defunct, so there's no point in testing it.

// fn test_diffable_case<T: dy::Diffable + PartialEq, D: dy::Diff>(
//     target: &T,
//     diff: &D,
//     expected_intermediate: &T,
// ) {
//     use dy::Diffable;

//     assert!(target.diff_is_mutation_in_place(diff).expect("pass"));

//     let mut t = target.clone();
//     t.apply_diff_in_place(&[], diff).expect("pass");
//     assert_eq!(t, *expected_intermediate);

//     let diff_inverse = diff.inverse();
//     assert!(t.diff_is_mutation_in_place(&diff_inverse).expect("pass"));
//     t.apply_diff_in_place(&[], &diff_inverse).expect("pass");
//     assert_eq!(t, *target);
// }

// #[test]
// #[serial_test::serial]
// fn test_diffable() {
//     let s = "ab 日本語 ab".to_string();

//     type Ins = dy::ElementInsertionTerm;

//     let case_v = [
//         (
//             Ins {
//                 index: 0u32.into(),
//                 data: 'X'.into(),
//             },
//             "Xab 日本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 0u32.into(),
//                 data: '字'.into(),
//             },
//             "字ab 日本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 1u32.into(),
//                 data: 'X'.into(),
//             },
//             "aXb 日本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 1u32.into(),
//                 data: '字'.into(),
//             },
//             "a字b 日本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 3u32.into(),
//                 data: 'X'.into(),
//             },
//             "ab X日本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 3u32.into(),
//                 data: '字'.into(),
//             },
//             "ab 字日本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 4u32.into(),
//                 data: 'X'.into(),
//             },
//             "ab 日X本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 4u32.into(),
//                 data: '字'.into(),
//             },
//             "ab 日字本語 ab".to_string(),
//         ),
//         (
//             Ins {
//                 index: 8u32.into(),
//                 data: 'X'.into(),
//             },
//             "ab 日本語 aXb".to_string(),
//         ),
//         (
//             Ins {
//                 index: 8u32.into(),
//                 data: '字'.into(),
//             },
//             "ab 日本語 a字b".to_string(),
//         ),
//         (
//             Ins {
//                 index: 9u32.into(),
//                 data: 'X'.into(),
//             },
//             "ab 日本語 abX".to_string(),
//         ),
//         (
//             Ins {
//                 index: 9u32.into(),
//                 data: '字'.into(),
//             },
//             "ab 日本語 ab字".to_string(),
//         ),
//     ];
//     for (diff, expected_intermediate_term) in case_v.iter() {
//         test_diffable_case(&s, diff, expected_intermediate_term);
//     }
// }

fn test_diff_case<'a, T: PartialEq + st::TermTrait, Address, D: st::DiffTrait>(
    target: &T,
    address_i: Address,
    diff: &D,
    expected_intermediate: &T,
) where
    T: st::Diffable<D>,
    T: st::Diffable<D::Inverse>,
    Address: Iterator<Item = &'a dy::Value> + Clone,
{
    use st::DiffTrait;

    let mut t = target.clone();
    log::trace!("test_diff_case; -- start ----------------------------------------");
    log::trace!("test_diff_case; target: {:?}", target);
    // TODO: Figure out how to print address_i reasonably.
    log::trace!("test_diff_case; diff: {:?}", diff);
    t.apply_diff(address_i.clone(), diff).expect("pass");
    log::trace!("test_diff_case; after apply_diff; t: {:?}", t);
    assert_eq!(t, *expected_intermediate);
    let diff_inv = diff.inverse();
    log::trace!("test_diff_case; diff_inv: {:?}", diff_inv);
    t.apply_diff(address_i, &diff_inv).expect("pass");
    log::trace!("test_diff_case; after apply_diff with diff_inv; t: {:?}", t);
    assert_eq!(t, *target);
    log::trace!("test_diff_case; -- end ----------------------------------------");
}

fn test_nonterminal_editable_case<
    'a,
    T: PartialEq + st::TermTrait,
    A: st::TermTrait,
    E: st::DiffTrait,
>(
    target: &T,
    address_head: &A,
    edit: &E,
    expected_intermediate: &T,
) where
    T: st::NonterminalEditable<A, E>,
    T: st::NonterminalEditable<A, E::Inverse>,
{
    use st::DiffTrait;

    let mut t = target.clone();
    log::trace!(
        "test_nonterminal_editable_case; -- start ----------------------------------------"
    );
    log::trace!("test_nonterminal_editable_case; target: {:?}", target);
    log::trace!(
        "test_nonterminal_editable_case; address_head: {:?}",
        address_head
    );
    log::trace!("test_nonterminal_editable_case; edit: {:?}", edit);
    t.apply_nonterminal_edit(address_head, std::iter::empty(), edit)
        .expect("pass");
    log::trace!(
        "test_nonterminal_editable_case; after apply_diff; t: {:?}",
        t
    );
    assert_eq!(t, *expected_intermediate);
    let edit_inv = edit.inverse();
    log::trace!("test_nonterminal_editable_case; edit_inv: {:?}", edit_inv);
    t.apply_nonterminal_edit(address_head, std::iter::empty(), &edit_inv)
        .expect("pass");
    log::trace!(
        "test_nonterminal_editable_case; after apply_diff with edit_inv; t: {:?}",
        t
    );
    assert_eq!(t, *target);
    log::trace!("test_nonterminal_editable_case; -- end ----------------------------------------");
}

#[test]
#[serial_test::serial]
fn test_nonterminal_editable_utf8string() {
    let s = "ab 日本語 ab".to_string();

    // type Ins = st::ElementInsertionTerm<String, u32, char>;
    type Ins = dy::InsertionTerm;

    test_nonterminal_editable_case(
        &s,
        &0u32,
        &Ins::new('X'.into()),
        &"Xab 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &0u32,
        &Ins::new('字'.into()),
        &"字ab 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &1u32,
        &Ins::new('X'.into()),
        &"aXb 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &1u32,
        &Ins::new('字'.into()),
        &"a字b 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &3u32,
        &Ins::new('X'.into()),
        &"ab X日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &3u32,
        &Ins::new('字'.into()),
        &"ab 字日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &4u32,
        &Ins::new('X'.into()),
        &"ab 日X本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &4u32,
        &Ins::new('字'.into()),
        &"ab 日字本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &8u32,
        &Ins::new('X'.into()),
        &"ab 日本語 aXb".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &8u32,
        &Ins::new('字'.into()),
        &"ab 日本語 a字b".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &9u32,
        &Ins::new('X'.into()),
        &"ab 日本語 abX".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &9u32,
        &Ins::new('字'.into()),
        &"ab 日本語 ab字".to_string(),
    );

    type Repl = dy::ReplacementTerm;

    test_nonterminal_editable_case(
        &s,
        &0u32,
        &Repl::new('a'.into(), 'X'.into()),
        &"Xb 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &0u32,
        &Repl::new('a'.into(), '字'.into()),
        &"字b 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &1u32,
        &Repl::new('b'.into(), 'X'.into()),
        &"aX 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &1u32,
        &Repl::new('b'.into(), '字'.into()),
        &"a字 日本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &3u32,
        &Repl::new('日'.into(), 'X'.into()),
        &"ab X本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &3u32,
        &Repl::new('日'.into(), '字'.into()),
        &"ab 字本語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &4u32,
        &Repl::new('本'.into(), 'X'.into()),
        &"ab 日X語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &4u32,
        &Repl::new('本'.into(), '字'.into()),
        &"ab 日字語 ab".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &8u32,
        &Repl::new('b'.into(), 'X'.into()),
        &"ab 日本語 aX".to_string(),
    );
    test_nonterminal_editable_case(
        &s,
        &8u32,
        &Repl::new('b'.into(), '字'.into()),
        &"ab 日本語 a字".to_string(),
    );
}

// fn test_diff_case_as_value<
//     T: dy::IntoValue + PartialEq + st::TermTrait,
//     D: st::DiffTrait<T> + dy::IntoValue,
// >(
//     target: &T,
//     diff: &D,
//     expected_intermediate: &T,
// ) {
//     use st::DiffTrait;

//     let diff_value = dy::Value::from(diff.clone());
//     let mut target_value = dy::Value::from(target.clone());
//     log::trace!("test_diff_case_as_value; -- start ----------------------------------------");
//     log::trace!(
//         "test_diff_case_as_value; target_value: {}",
//         target_value.stringify()
//     );
//     log::trace!(
//         "test_diff_case_as_value; diff_value: {}",
//         diff_value.stringify()
//     );
//     diff_value.apply_in_place(&mut target_value).expect("pass");
//     log::trace!(
//         "test_diff_case_as_value; target_value after diff apply_in_place: {}",
//         target_value.stringify()
//     );

//     assert_eq!(
//         *target_value.downcast_ref::<T>().unwrap(),
//         *expected_intermediate
//     );
//     let diff_inv_value = diff_value.inverse();
//     log::trace!(
//         "test_diff_case_as_value; diff_inv_value: {}",
//         diff_inv_value.stringify()
//     );
//     diff_inv_value
//         .apply_in_place(&mut target_value)
//         .expect("pass");
//     log::trace!(
//         "test_diff_case_as_value; target_value after diff's inverse apply_in_place: {}",
//         target_value.stringify()
//     );
//     assert_eq!(*target_value.downcast_ref::<T>().unwrap(), *target);
//     log::trace!("test_diff_case_as_value; -- done ----------------------------------------");
// }

#[test]
#[serial_test::serial]
fn test_diff_utf8string() {
    let s = "ab 日本語 ab".to_string();

    // type Ins = st::ElementInsertionTerm<String, u32, char>;
    type Ins = dy::InsertionTerm;

    test_diff_case(
        &s,
        [0u32.into_value()].iter(),
        &Ins {
            new_data: 'X'.into(),
        },
        &"Xab 日本語 ab".to_string(),
    );
    test_diff_case(
        &s,
        [0u32.into_value()].iter(),
        &Ins {
            new_data: '字'.into(),
        },
        &"字ab 日本語 ab".to_string(),
    );
    // test_diff_case(&s, &Ins::new(0u32, '字'), &"字ab 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(1u32, 'X'), &"aXb 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(1u32, '字'), &"a字b 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(3u32, 'X'), &"ab X日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(3u32, '字'), &"ab 字日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(4u32, 'X'), &"ab 日X本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(4u32, '字'), &"ab 日字本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(8u32, 'X'), &"ab 日本語 aXb".to_string());
    // test_diff_case(&s, &Ins::new(8u32, '字'), &"ab 日本語 a字b".to_string());
    // test_diff_case(&s, &Ins::new(9u32, 'X'), &"ab 日本語 abX".to_string());
    // test_diff_case(&s, &Ins::new(9u32, '字'), &"ab 日本語 ab字".to_string());

    // test_diff_case(&s, &Ins::new(0u32, 'X'), &"Xab 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(0u32, '字'), &"字ab 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(1u32, 'X'), &"aXb 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(1u32, '字'), &"a字b 日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(3u32, 'X'), &"ab X日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(3u32, '字'), &"ab 字日本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(4u32, 'X'), &"ab 日X本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(4u32, '字'), &"ab 日字本語 ab".to_string());
    // test_diff_case(&s, &Ins::new(8u32, 'X'), &"ab 日本語 aXb".to_string());
    // test_diff_case(&s, &Ins::new(8u32, '字'), &"ab 日本語 a字b".to_string());
    // test_diff_case(&s, &Ins::new(9u32, 'X'), &"ab 日本語 abX".to_string());
    // test_diff_case(&s, &Ins::new(9u32, '字'), &"ab 日本語 ab字".to_string());

    // test_diff_case_as_value(&s, &Ins::new(0u32, 'X'), &"Xab 日本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(0u32, '字'), &"字ab 日本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(1u32, 'X'), &"aXb 日本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(1u32, '字'), &"a字b 日本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(3u32, 'X'), &"ab X日本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(3u32, '字'), &"ab 字日本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(4u32, 'X'), &"ab 日X本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(4u32, '字'), &"ab 日字本語 ab".to_string());
    // test_diff_case_as_value(&s, &Ins::new(8u32, 'X'), &"ab 日本語 aXb".to_string());
    // test_diff_case_as_value(&s, &Ins::new(8u32, '字'), &"ab 日本語 a字b".to_string());
    // test_diff_case_as_value(&s, &Ins::new(9u32, 'X'), &"ab 日本語 abX".to_string());
    // test_diff_case_as_value(&s, &Ins::new(9u32, '字'), &"ab 日本語 ab字".to_string());
}

fn test_query_trait_case<Q: dy::QueryTrait>(
    q_b: Box<Q>,
    address_v: &[dy::Value],
    expected_result: dy::Value,
) {
    let query_view = q_b.run_query(&mut address_v.iter()).expect("pass");
    match query_view.queried_value().expect("pass") {
        dy::MaybeDereferencedValue::Ref(x) => {
            log::debug!(
                "query_result: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(x)
            );
            assert!(dy::RUNTIME_LA
                .read()
                .unwrap()
                .eq(x, expected_result.as_ref()));
        }
        dy::MaybeDereferencedValue::ValueLA(x) => {
            log::debug!(
                "query_result: {}",
                dy::RUNTIME_LA
                    .read()
                    .unwrap()
                    .stringify(x.read().unwrap().as_ref())
            );
            assert_eq!(*x.read().unwrap(), expected_result);
        }
    }
}

#[test]
#[serial_test::serial]
fn test_query_trait_utf8_string_term() {
    let s = "ab 日本語 hippo\nOSTRICH".to_string();
    test_query_trait_case(dy::Utf8StringTermView::new(&s), &[], s.clone().into_value());
    test_query_trait_case(
        dy::Utf8StringTermView::new(&s),
        &["char".to_string().into_value(), 0u32.into_value()],
        'a'.into_value(),
    );
    test_query_trait_case(
        dy::Utf8StringTermView::new(&s),
        &["char".to_string().into_value(), 1u32.into_value()],
        'b'.into_value(),
    );
    test_query_trait_case(
        dy::Utf8StringTermView::new(&s),
        &["char".to_string().into_value(), 2u32.into_value()],
        ' '.into_value(),
    );
    test_query_trait_case(
        dy::Utf8StringTermView::new(&s),
        &["char".to_string().into_value(), 3u32.into_value()],
        '日'.into_value(),
    );

    test_query_trait_case(
        dy::Utf8StringTermView::new(&s),
        &["line".to_string().into_value(), 0u32.into_value()],
        "ab 日本語 hippo\n".to_string().into_value(),
    );
    test_query_trait_case(
        dy::Utf8StringTermView::new(&s),
        &["line".to_string().into_value(), 1u32.into_value()],
        "OSTRICH".to_string().into_value(),
    );
}

#[test]
#[serial_test::serial]
fn test_query_trait_ordered_map_term() {
    let ordered_map_term = dy::OrderedMapTerm::from(maplit::btreemap! {
        st::Void.into() => dy::OrderedMapTerm::from(maplit::btreemap! { 4u32.into() => "blah".to_string().into() }).into(),
        st::EmptyType.into() => dy::OrderedMapTerm::from(maplit::btreemap! { 4u32.into() => "blah".to_string().into(), 5u32.into() => "FWEEE".to_string().into() }).into(),
        dy::OrderedMapTerm::from(maplit::btreemap! { 889u32.into() => "woooooo".to_string().into() }).into() => st::Bool.into(),
    });
    test_query_trait_case(
        dy::OrderedMapTermView::new(&ordered_map_term),
        &[],
        ordered_map_term.clone().into_value(),
    );
    test_query_trait_case(
        dy::OrderedMapTermView::new(&ordered_map_term),
        &['k'.into_value(), st::Void.into_value()],
        st::Void.into_value(),
    );
    test_query_trait_case(
        dy::OrderedMapTermView::new(&ordered_map_term),
        &[
            'k'.into_value(),
            dy::OrderedMapTerm::from(
                maplit::btreemap! { 889u32.into() => "woooooo".to_string().into() },
            )
            .into_value(),
        ],
        dy::OrderedMapTerm::from(
            maplit::btreemap! { 889u32.into() => "woooooo".to_string().into() },
        )
        .into(),
    );
    test_query_trait_case(
        dy::OrderedMapTermView::new(&ordered_map_term),
        &[
            'k'.into_value(),
            dy::OrderedMapTerm::from(
                maplit::btreemap! { 889u32.into() => "woooooo".to_string().into() },
            )
            .into_value(),
            'k'.into_value(),
            889u32.into_value(),
        ],
        889u32.into_value(),
    );
    test_query_trait_case(
        dy::OrderedMapTermView::new(&ordered_map_term),
        &['v'.into_value(), st::Void.into_value()],
        dy::OrderedMapTerm::from(maplit::btreemap! { 4u32.into() => "blah".to_string().into() })
            .into_value(),
    );
}

// fn test_query_mut_trait_case<'a, T: dy::QueryableMutDynTrait + PartialEq + st::TermTrait>(
fn test_query_mut_trait_case<'a, T: dy::Editable + PartialEq + st::TermTrait>(
    mut x: T,
    address_v: &[dy::Value],
    edit: dy::Value,
    expected_x: T,
) {
    // x.make_and_run_query_mut(&mut address_v.iter())
    //     .expect("pass")
    //     .apply_edit(edit)
    //     .expect("pass");
    x.query_mut_and_apply_edit(&mut address_v.iter(), edit)
        .expect("pass");
    assert_eq!(x, expected_x);
}

#[test]
#[serial_test::serial]
fn test_query_mut_trait_utf8_string_term() {
    let test_case_data_v = vec![
        (
            "ab 日本語 hippo\nOSTRICH",
            vec![],
            dy::ReplacementTerm {
                old_data: "ab 日本語 hippo\nOSTRICH".to_string().into(),
                new_data: "something totally different".to_string().into(),
            }
            .into_value(),
            "something totally different",
        ),
        (
            "ab 日本語 hippo\nOSTRICH",
            vec!["char".to_string().into_value(), 0u32.into_value()],
            dy::ReplacementTerm {
                old_data: 'a'.into(),
                new_data: '字'.into(),
            }
            .into_value(),
            "字b 日本語 hippo\nOSTRICH",
        ),
        (
            "ab 日本語 hippo\nOSTRICH",
            vec!["char".to_string().into_value(), 12u32.into_value()],
            dy::ReplacementTerm {
                old_data: '\n'.into(),
                new_data: '字'.into(),
            }
            .into_value(),
            "ab 日本語 hippo字OSTRICH",
        ),
        (
            "ab 日本語 hippo\nOSTRICH",
            vec!["line".to_string().into_value(), 0u32.into_value()],
            dy::ReplacementTerm {
                old_data: "ab 日本語 hippo\n".to_string().into(),
                new_data: "a big, fancy ".to_string().into(),
            }
            .into_value(),
            "a big, fancy OSTRICH",
        ),
        (
            "ab 日本語 hippo\nOSTRICH",
            vec![
                "line".to_string().into_value(),
                0u32.into_value(),
                "char".to_string().into_value(),
                5u32.into_value(),
            ],
            dy::ReplacementTerm {
                old_data: '語'.into(),
                new_data: 'X'.into(),
            }
            .into_value(),
            "ab 日本X hippo\nOSTRICH",
        ),
        (
            "ab 日本語 hippo\nOSTRICH",
            vec![
                "line".to_string().into_value(),
                1u32.into_value(),
                "char".to_string().into_value(),
                5u32.into_value(),
            ],
            dy::ReplacementTerm {
                old_data: 'C'.into(),
                new_data: '語'.into(),
            }
            .into_value(),
            "ab 日本語 hippo\nOSTRI語H",
        ),
    ];
    for (initial_value, address_v, edit, expected_value) in test_case_data_v.into_iter() {
        test_query_mut_trait_case(
            initial_value.to_string(),
            address_v.as_slice(),
            edit,
            expected_value.to_string(),
        );
    }
}

#[test]
#[serial_test::serial]
fn test_query_mut_trait_value() {
    let test_case_data_v = vec![
        (
            123u32.into_value(),
            vec![],
            st::NoOp.into_value(),
            123u32.into_value(),
        ),
        (
            123u32.into_value(),
            vec![],
            dy::ReplacementTerm {
                old_data: 123u32.into_value(),
                new_data: st::Bool.into_value(),
            }
            .into_value(),
            st::Bool.into_value(),
        ),
    ];
    for (initial_value, address_v, edit, expected_value) in test_case_data_v.into_iter() {
        test_query_mut_trait_case(initial_value, address_v.as_slice(), edit, expected_value);
    }
}

#[test]
#[serial_test::serial]
fn test_query_mut_trait_array_term() {
    let test_case_data_v = vec![
        (
            dy::ArrayTerm::from(vec![]),
            vec![],
            dy::ReplacementTerm {
                old_data: dy::ArrayTerm::from(vec![]).into(),
                new_data: dy::ArrayTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
            }
            .into_value(),
            dy::ArrayTerm::from(vec![123i64.into(), st::Bool.into()]),
        ),
        (
            dy::ArrayTerm::from(vec![456u32.into(), 101010u32.into()]),
            vec![0u32.into()],
            dy::ReplacementTerm {
                old_data: 456u32.into(),
                new_data: dy::ArrayTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
            }
            .into_value(),
            dy::ArrayTerm::from(vec![
                dy::ArrayTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
                101010u32.into(),
            ]),
        ),
        (
            dy::ArrayTerm::from(vec![
                456u32.into(),
                dy::ArrayTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
                101010u32.into(),
            ]),
            vec![1u32.into(), 0u32.into()],
            dy::ReplacementTerm {
                old_data: 123i64.into(),
                new_data: 123000000i64.into(),
            }
            .into_value(),
            dy::ArrayTerm::from(vec![
                456u32.into(),
                dy::ArrayTerm::from(vec![123000000i64.into(), st::Bool.into()]).into(),
                101010u32.into(),
            ]),
        ),
    ];
    for (initial_value, address_v, edit, expected_value) in test_case_data_v.into_iter() {
        test_query_mut_trait_case(initial_value, address_v.as_slice(), edit, expected_value);
    }
}

#[test]
#[serial_test::serial]
fn test_query_mut_trait_tuple_term() {
    let test_case_data_v = vec![
        (
            dy::TupleTerm::from(vec![]),
            vec![],
            dy::ReplacementTerm {
                old_data: dy::TupleTerm::from(vec![]).into(),
                new_data: dy::TupleTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
            }
            .into_value(),
            dy::TupleTerm::from(vec![123i64.into(), st::Bool.into()]),
        ),
        (
            dy::TupleTerm::from(vec![456u32.into(), 101010u32.into()]),
            vec![0u32.into()],
            dy::ReplacementTerm {
                old_data: 456u32.into(),
                new_data: dy::TupleTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
            }
            .into_value(),
            dy::TupleTerm::from(vec![
                dy::TupleTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
                101010u32.into(),
            ]),
        ),
        (
            dy::TupleTerm::from(vec![
                456u32.into(),
                dy::TupleTerm::from(vec![123i64.into(), st::Bool.into()]).into(),
                101010u32.into(),
            ]),
            vec![1u32.into(), 0u32.into()],
            dy::ReplacementTerm {
                old_data: 123i64.into(),
                new_data: 123000000i64.into(),
            }
            .into_value(),
            dy::TupleTerm::from(vec![
                456u32.into(),
                dy::TupleTerm::from(vec![123000000i64.into(), st::Bool.into()]).into(),
                101010u32.into(),
            ]),
        ),
    ];
    for (initial_value, address_v, edit, expected_value) in test_case_data_v.into_iter() {
        test_query_mut_trait_case(initial_value, address_v.as_slice(), edit, expected_value);
    }
}

#[test]
#[serial_test::serial]
fn test_query_mut_trait_ordered_map_term_val() {
    test_query_mut_trait_case(
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => 123u32.into(),
        }),
        vec!['v'.into(), st::Void.into()].as_slice(),
        dy::ReplacementTerm {
            old_data: 123u32.into(),
            new_data: st::Bool.into(),
        }
        .into_value(),
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => st::Bool.into(),
        }),
    );

    test_query_mut_trait_case(
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => dy::OrderedMapTerm::from(maplit::btreemap! {
                4u32.into() => "blah".to_string().into()
            }).into(),
        }),
        vec!['v'.into(), st::Void.into(), 'v'.into(), 4u32.into()].as_slice(),
        dy::ReplacementTerm {
            old_data: "blah".to_string().into(),
            new_data: st::Bool.into(),
        }
        .into_value(),
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => dy::OrderedMapTerm::from(maplit::btreemap! {
                4u32.into() => st::Bool.into()
            }).into(),
        }),
    );
}

#[test]
#[serial_test::serial]
fn test_query_mut_trait_ordered_map_term_key() {
    test_query_mut_trait_case(
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => 123u32.into(),
        }),
        vec!['k'.into(), st::Void.into()].as_slice(),
        dy::ReplacementTerm {
            old_data: st::Void.into(),
            new_data: st::Bool.into(),
        }
        .into_value(),
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Bool.into() => 123u32.into(),
        }),
    );

    test_query_mut_trait_case(
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => 123u32.into(),
            st::EmptyType.into() => 123u32.into(),
        }),
        vec!['k'.into(), st::Void.into()].as_slice(),
        dy::ReplacementTerm {
            old_data: st::Void.into(),
            new_data: st::Bool.into(),
        }
        .into_value(),
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Bool.into() => 123u32.into(),
            st::EmptyType.into() => 123u32.into(),
        }),
    );

    {
        // Negative test -- key collision.
        use dy::QueryableMutDynTrait;
        dy::OrderedMapTerm::from(maplit::btreemap! {
            st::Void.into() => 123u32.into(),
            st::EmptyType.into() => 123u32.into(),
        })
        .make_and_run_query_mut(&mut ['k'.into(), st::Void.into()].iter())
        .expect("pass")
        .apply_edit(
            dy::ReplacementTerm {
                old_data: st::Void.into(),
                new_data: st::EmptyType.into(),
            }
            .into(),
        )
        .expect_err("pass");
    }

    // Editing of nested keys.
    // TODO: This is not supported yet.
    // test_query_mut_trait_case(
    //     dy::OrderedMapTerm::from(maplit::btreemap! {
    //         dy::OrderedMapTerm::from(maplit::btreemap! {
    //             4u32.into() => "blah".to_string().into()
    //         }).into() => st::Void.into(),
    //     }),
    //     vec![
    //         'k'.into(),
    //         dy::OrderedMapTerm::from(maplit::btreemap! {
    //             4u32.into() => "blah".to_string().into()
    //         })
    //         .into(),
    //         'k'.into(),
    //         4u32.into(),
    //     ]
    //     .as_slice(),
    //     dy::ReplacementTerm {
    //         old_data: 4u32.into(),
    //         new_data: st::Bool.into(),
    //     }
    //     .into_value(),
    //     dy::OrderedMapTerm::from(maplit::btreemap! {
    //         dy::OrderedMapTerm::from(maplit::btreemap! {
    //             st::Bool.into() => "blah".to_string().into()
    //         }).into() => st::Void.into(),
    //     }),
    // );
}

//
// TEMP TESTING
//

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct BinOp;

impl st::Inhabits<Type> for BinOp {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct UnOp;

impl st::Inhabits<Type> for UnOp {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

trait BinOpTermTrait {
    // TODO: A BinOp whose character is defined at runtime (analogous to NonParametricTermCode) would need
    // a &self parameter.  Could distingish this by having st::BinOpTermTrait and dy::BinOpTermTrait
    // or actually, maybe static vs dynamic isn't exactly right.. nonparametric vs parametric?
    fn is_commutative() -> bool;
}

trait UnOpTermTrait {}

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Add;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Sub;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Mul;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Div;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "BinOp", is_parametric = "false", is_type = "false")]
pub struct Pow;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
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

#[derive(
    Clone, Copy, Debug, Eq, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait,
)]
#[st_non_parametric_term_trait(code = "Undefined")]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Expr;

impl st::Inhabits<Type> for Expr {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for Expr {}

impl Inhabits<Expr> for f64 {
    fn inhabits(&self, _rhs: &Expr) -> bool {
        true
    }
}

impl Inhabits<Expr> for TupleTerm {
    fn inhabits(&self, _rhs: &Expr) -> bool {
        // TODO: Expr should really be Union(BinOpExpr, LiteralExpr, UnOpExpr)
        // TODO: Either register this with the runtime or make a const
        let bin_op_expr = TupleTerm::from(vec![Expr {}.into(), BinOp {}.into(), Expr {}.into()]);
        // TODO: Left and right unary ops
        self.inhabits(&bin_op_expr)
    }
}

fn eval_expr(expr: &Value) -> f64 {
    use std::ops::Deref;

    // TODO: Either register this with the runtime or make a const
    let bin_op_expr = TupleTerm::from(vec![Expr {}.into(), BinOp {}.into(), Expr {}.into()]);

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
        runtime_g.register_inhabits::<f64, Expr>().unwrap();
        runtime_g.register_inhabits::<TupleTerm, Expr>().unwrap();
    }

    let expr1 = TupleTerm::from(vec![123.0f64.into(), Add {}.into(), 456.0f64.into()]);
    let bin_op_expr = TupleTerm::from(vec![Expr {}.into(), BinOp {}.into(), Expr {}.into()]);
    log::debug!("expr1: {}", expr1);
    log::debug!("bin_op_expr: {}", bin_op_expr);
    log::debug!("Expr{{}}: {}", Expr {}.stringify());
    assert!(expr1.inhabits(&bin_op_expr));
    assert!(expr1.inhabits(&Expr {}));

    let expr2 = TupleTerm::from(vec![
        TupleTerm::from(vec![77.75f64.into(), Mul {}.into(), 900.125f64.into()]).into(),
        Add {}.into(),
        1.0f64.into(),
    ]);
    log::debug!("expr2: {}", expr2);
    assert!(expr2.inhabits(&Expr {}));
    assert!(expr2.inhabits(&bin_op_expr));

    log::debug!(
        "eval_expr({}): {}",
        expr2.stringify(),
        eval_expr(&Value::from(expr2))
    );

    let expr3 = TupleTerm::from((
        TupleTerm::from((77.75f64, Mul {}, 900.125f64)),
        Add {},
        1.0f64,
    ));
    log::debug!("expr3: {}", expr3);

    // TEMP TESTING
    {
        // Create the BinOpExpr struct
        dy::GLOBAL_SYMBOL_TABLE_LA
            .write()
            .unwrap()
            .define_symbol(
                "BinOpExpr",
                StructTerm::new(
                    vec![
                        ("lhs".into(), Expr {}.into()),
                        ("bin_op".into(), BinOp {}.into()),
                        ("rhs".into(), Expr {}.into()),
                    ]
                    .into(),
                )
                .expect("test")
                .into(),
            )
            .expect("test");
        log::debug!(
            "global_symbol_table: {:#?}",
            dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap()
        );
        let bin_op_expr = GlobalSymRefTerm::new_unchecked("BinOpExpr".into());
        log::debug!("bin_op_expr: {}", bin_op_expr.stringify());

        //         assert!(expr3.inhabits(&bin_op_expr));
    }

    // Now test parsing something involving the defined symbols, and then evaluating it.
    {}
}
