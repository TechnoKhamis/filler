pub mod piece;

#[derive(Clone, Debug)]
pub struct Shape {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<(usize, usize)>,
}