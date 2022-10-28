use super::Transition;

#[derive(Clone, PartialEq, Eq)]
pub struct Execution {
    pub edition: u16,
    pub transitions: Vec<Transition>,
}
