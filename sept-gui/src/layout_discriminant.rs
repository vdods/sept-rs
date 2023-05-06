#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutDiscriminant {
    Expanded,
    BoundaryLevelInline,
    InteriorLevelInline,
}
