#[derive(PartialEq, Eq)]
pub struct State<T> {
    pub state: T,
    pub cost: u128,
}
