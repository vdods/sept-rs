use sept::{dy::Value, st::{self, Sint32, TermTrait, VoidType}};

/// This will run once at load time (i.e. presumably before main function is called).
#[ctor::ctor]
fn overall_init() {
    env_logger::try_init().unwrap();
}

#[derive(Clone, Debug, st::TermTrait)]
#[term_trait(AbstractTypeType = "VoidType", is_parametric = "false", is_type = "true")]
pub struct FancyType;

#[derive(Clone, Debug, st::TermTrait)]
#[term_trait(AbstractTypeType = "Value")]
#[term_trait(abstract_type_expr = "Value::from(Sint32{})")]
#[term_trait(is_parametric = "false")]
#[term_trait(is_type = "true")]
pub struct DumbType;

#[test]
#[serial_test::serial] // TEMP HACK: Just so the debug spew doesn't collide
fn blah() {
    let x = Sint32{};
    use sept::st::Stringify;
    log::debug!("x: {}", x.stringify());
    let d = DumbType;
    log::debug!("d.abstract_type(): {}", d.abstract_type().stringify());

    let i = sept::st::IntN::<true,32>;
    log::debug!("i: {}", i.stringify());
}
