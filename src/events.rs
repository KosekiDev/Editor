use crossterm::{cursor, event, QueueableCommand};
use std::io::Stdout;

use crate::application::{Action, Mode};

fn handle_normal_event(
    output: &mut Stdout,
    event: &event::Event,
) -> anyhow::Result<Option<Action>> {
    match event {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char('q') => Ok(Some(Action::Quit)),

            event::KeyCode::Char('i') => {
                output.queue(cursor::SetCursorStyle::SteadyBar)?;
                Ok(Some(Action::ChangeMode(Mode::Insert)))
            }

            event::KeyCode::Left | event::KeyCode::Char('h') => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Right | event::KeyCode::Char('l') => Ok(Some(Action::MoveRight)),
            event::KeyCode::Up | event::KeyCode::Char('k') => Ok(Some(Action::MoveUp)),
            event::KeyCode::Down | event::KeyCode::Char('j') => Ok(Some(Action::MoveDown)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn handle_insert_event(
    output: &mut Stdout,
    event: &event::Event,
) -> anyhow::Result<Option<Action>> {
    match event {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Esc => {
                output.queue(cursor::SetCursorStyle::SteadyBlock)?;
                Ok(Some(Action::ChangeMode(Mode::Normal)))
            }

            event::KeyCode::Left => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Right => Ok(Some(Action::MoveRight)),
            event::KeyCode::Up => Ok(Some(Action::MoveUp)),
            event::KeyCode::Down => Ok(Some(Action::MoveDown)),

            event::KeyCode::Enter => Ok(Some(Action::Return)),
            event::KeyCode::Backspace => Ok(Some(Action::Back)),

            event::KeyCode::Char(char) => Ok(Some(Action::Write(char))),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

pub fn handle_event(
    output: &mut Stdout,
    event: &event::Event,
    mode: &Mode,
) -> anyhow::Result<Option<Action>> {
    match mode {
        Mode::Normal => handle_normal_event(output, event),
        Mode::Insert => handle_insert_event(output, event),
    }
}
