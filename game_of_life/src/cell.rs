#[repr(u8)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Cell {
    Alive,
    Dead
}

impl Cell {
    pub fn toggle(&mut self) -> Cell {
        match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        }
    }
}