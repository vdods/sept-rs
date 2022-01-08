use sept::{dy::{self, Value}, st::{self, Sint32, VoidType}};

/// This will run once at load time (i.e. presumably before main function is called).
#[ctor::ctor]
fn overall_init() {
    env_logger::try_init().unwrap();
}

#[derive(Clone, Debug, dy::IntoValue, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "VoidType", is_parametric = "false", is_type = "false")]
pub struct FancyTerm;

#[derive(Clone, Debug, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Value")]
#[st_term_trait(abstract_type_expr = "Value::from(Sint32{})")]
#[st_term_trait(is_parametric = "false")]
#[st_term_trait(is_type = "true")]
pub struct DumbType;

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn blah() {
    use sept::st::TermTrait;

    let f = FancyTerm;
    let v = Value::from(f);
    log::debug!("v (as Debug): {:?}", v);

    let x = Sint32{};
    use sept::st::Stringify;
    log::debug!("x: {}", x.stringify());
    let d = DumbType;
    log::debug!("d.abstract_type(): {}", d.abstract_type().stringify());

    let i = sept::st::IntN::<true,32>;
    log::debug!("i: {}", i.stringify());
}
