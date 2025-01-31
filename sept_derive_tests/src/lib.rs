use sept::{
    dy::{self, Value},
    st::{self, Sint32, VoidType},
};

/// This will run once at load time (i.e. presumably before main function is called).
#[ctor::ctor]
fn overall_init() {
    env_logger::try_init().unwrap();
}

// TODO: Change AbstractTypeType to something that actually makes sense, or comment as to why this type is used.
#[derive(Clone, Debug, dy::IntoValueT, st::TermT)]
#[st_term_t(
    AbstractTypeType = "VoidType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct FancyTerm;

// TODO: Change abstract_type_expr to something that actually makes sense, or comment as to why this value is used.
#[derive(Clone, Debug, st::TermT, st::TypeT)]
#[st_term_t(AbstractTypeType = "Value")]
#[st_term_t(abstract_type_expr = "Value::from(Sint32)")]
#[st_term_t(is_parametric = "false")]
#[st_term_t(is_type = "true")]
pub struct DumbType;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, PartialEq, st::NonParametricTermT, st::TermT,
)]
#[st_term_t(AbstractTypeType = "Value")]
#[st_term_t(abstract_type_expr = "Value::from(Sint32)")]
#[st_term_t(is_parametric = "false")]
#[st_term_t(is_type = "false")]
pub struct Undefined;

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn blah() {
    use sept::st::TermT;

    let f = FancyTerm;
    let v = Value::from(f);
    log::debug!("v (as Debug): {:?}", v);

    let x = Sint32;
    use sept::st::StringifiableT;
    log::debug!("x: {}", x.stringify());
    let d = DumbType;
    log::debug!("d.abstract_type(): {}", d.abstract_type().stringify());
}
