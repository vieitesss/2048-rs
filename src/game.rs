use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    fmt::Display,
    io::{self, stdout, Write},
};

use crate::utils;

use self::matrix::{Matrix, MatrixTrait};

mod matrix;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Starting,
    Running,
    GameOver,
}

pub trait Playable {
    fn init(&mut self) -> io::Result<()>;
    fn update(&mut self) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<()>;
    fn draw(&self);
    fn exit() -> io::Result<()>;
}

#[derive(Debug)]
pub struct Game {
    matrix: Matrix,
    pub state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            matrix: Matrix::default(),
            state: State::Running,
        }
    }
}

impl Playable for Game {
    fn init(&mut self) -> io::Result<()> {
        execute!(stdout(), Hide)?;
        enable_raw_mode()?;
        self.update()?;

        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        utils::clear_screen()?;
        self.matrix.spawn_number();
        self.draw();

        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    self.matrix.shift(code);
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.state = State::GameOver;
                }
                _ => {}
            },
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }

        if self.matrix.changed {
            self.update()?;
            self.matrix.changed = false;
        } else if self.matrix.has_no_moves() {
            self.state = State::GameOver;
        }

        Ok(())
    }

    fn draw(&self) {
        writeln!(stdout(), "{}\r", self).expect("could not write update");
    }

    fn exit() -> io::Result<()> {
        execute!(stdout(), Show)?;
        disable_raw_mode()?;

        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}
