use crate::TermTrait;

impl TermTrait for bool {
    fn is_parametric_term(&self) -> bool {
        true
    }
    fn is_type_term(&self) -> bool {
        false
    }
}
