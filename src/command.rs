use crate::direction::{Direction, Speed};

pub enum Command {
    Quit, 
    Adjust(Speed),
    Turn(Direction),
}
