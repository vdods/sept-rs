use crate::{Stringify, TermTrait};

impl Stringify for bool {
    fn stringify(&self) -> String {
        if *self { "True".into() } else { "False".into() }
    }
}

impl TermTrait for bool {
    fn is_parametric_term(&self) -> bool {
        true
    }
    fn is_type_term(&self) -> bool {
        false
    }
}
