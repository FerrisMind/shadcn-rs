#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FocusStrategy {
    #[default]
    None,
    Trap,
}
