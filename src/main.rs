use std::io::{self, Write};

use game::{Game, Playable, State};

mod game;
mod utils;

fn main() -> io::Result<()> {
    let mut game = Game::new();

    game.init()?;

    while game.state != State::GameOver {
        game.handle_input()?
    }

    writeln!(io::stdout(), "Game Over!\r")?;
    Game::exit()?;

    Ok(())
}
