use crate::TermTrait;

pub trait TypeTrait: TermTrait {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool;
}
