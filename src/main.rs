#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;

use game::{Game, Playable, State};

mod game;
mod utils;

fn main() -> io::Result<()> {
    let mut game = Game::new();

    game.init()?;

    while game.state != State::GameOver {
        game.handle_input()?
    }

    Game::exit()?;

    Ok(())
}
