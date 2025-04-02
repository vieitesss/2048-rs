use crossterm::{
    cursor::{self, Hide, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout, Write};

use crate::utils;

use self::matrix::{Matrix, MatrixTrait};

mod matrix;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Running,
    GameOver,
}

pub trait Playable {
    fn init(&mut self) -> io::Result<()>;
    fn update(&mut self) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<()>;
    fn draw(&self) -> io::Result<()>;
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
        self.draw()?;

        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match modifiers {
                KeyModifiers::NONE => match code {
                    KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                        self.matrix.shift(code);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.state = State::GameOver;
                    }
                    _ => {}
                },
                KeyModifiers::CONTROL => match code {
                    KeyCode::Char('c') | KeyCode::Char('d') => {
                        self.state = State::GameOver;
                    },
                    _ => {}
                }
                _ => {}
            },
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {
                utils::clear_screen()?;
                self.draw()?;
            }
        }

        if self.matrix.changed {
            self.update()?;
            self.matrix.changed = false;
        } else if self.matrix.has_no_moves() {
            self.state = State::GameOver;
        }

        Ok(())
    }

    fn draw(&self) -> io::Result<()> {
        let (columns, rows) = utils::get_window_size();
        let (width, tall) = self.matrix.get_width_on_draw();
        let (width, tall) = (width as u16, tall as u16);
        execute!(
            stdout(),
            cursor::MoveTo(columns / 2 - width / 2, rows / 2 - tall / 2)
        )?;

        writeln!(stdout(), "{}\r", self.matrix).expect("could not write update");

        Ok(())
    }

    fn exit() -> io::Result<()> {
        execute!(stdout(), Show)?;
        disable_raw_mode()?;

        Ok(())
    }
}
