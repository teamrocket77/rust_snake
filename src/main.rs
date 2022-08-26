use crate::game::Game;
use std::io::stdout;

mod snake;
mod direction;
mod game;
mod point;
mod command{
    use crate::direction::{Direction, Speed};

    pub enum Command {
        Quit,
        Adjust(Speed),
        Turn(Direction),
        }
}

fn main() {
    Game::new(stdout(), 10, 10).run();
}

