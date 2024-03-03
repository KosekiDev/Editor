use crate::events::handle_event;
use crossterm::{cursor, event, queue, style, terminal, ExecutableCommand, QueueableCommand};
use std::io::{Stdout, Write};

#[derive(Debug, PartialEq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,

    Back,
    Return,

    Write(char),

    ChangeMode(Mode),

    Quit,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct Application {
    mode: Mode,
    output: Stdout,
}

impl Application {
    pub fn new(output: Stdout, default_mode: Mode) -> Self {
        Self {
            mode: default_mode,
            output,
        }
    }

    pub fn handle_action(&mut self, action: Action) -> anyhow::Result<()> {
        match action {
            Action::Write(char) => {
                self.output.queue(style::Print(char))?;
            }
            Action::Back => {
                queue!(
                    self.output,
                    cursor::MoveLeft(1),
                    style::Print(' '),
                    cursor::MoveLeft(1)
                )?;
            }
            Action::Return => {
                self.output.queue(cursor::MoveToNextLine(1))?;
            }

            Action::MoveLeft => {
                self.output.queue(cursor::MoveLeft(1))?;
            }
            Action::MoveRight => {
                self.output.queue(cursor::MoveRight(1))?;
            }
            Action::MoveUp => {
                self.output.queue(cursor::MoveUp(1))?;
            }
            Action::MoveDown => {
                self.output.queue(cursor::MoveDown(1))?;
            }

            Action::ChangeMode(mode) => {
                self.mode = mode;
            }

            Action::Quit => {}
        }

        Ok(())
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.output.flush()?;

        Ok(())
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;

        self.output.execute(terminal::EnterAlternateScreen)?;
        self.output
            .execute(terminal::Clear(terminal::ClearType::All))?;

        self.output.queue(cursor::MoveTo(0, 0))?;

        loop {
            self.draw()?;

            let event = event::read()?;

            if let Some(action) = handle_event(&mut self.output, &event, &self.mode)? {
                match action {
                    Action::Quit => break,
                    _ => self.handle_action(action)?,
                };
            }
        }

        self.output.execute(terminal::LeaveAlternateScreen)?;

        terminal::disable_raw_mode()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::application::*;
    use std::io::stdout;

    fn key_press_event(key_press: char) -> event::Event {
        event::Event::Key(event::KeyEvent::new(
            event::KeyCode::Char(key_press),
            event::KeyModifiers::empty(),
        ))
    }
    fn esc_press_event() -> event::Event {
        event::Event::Key(event::KeyEvent::new(
            event::KeyCode::Esc,
            event::KeyModifiers::empty(),
        ))
    }

    #[test]
    fn it_should_quit_application() {
        let mut app = Application::new(stdout(), Mode::Normal);

        let action = handle_event(&mut app.output, &key_press_event('q'), &Mode::Normal)
            .expect("Error when handling events");

        assert_eq!(Action::Quit, action.expect("No action returned"));
    }

    #[test]
    fn it_should_switch_to_insert_mode() {
        let mut app = Application::new(stdout(), Mode::Normal);

        assert_eq!(app.mode, Mode::Normal);

        let action = handle_event(&mut app.output, &key_press_event('i'), &Mode::Normal)
            .expect("Error when handling events");
        let _ = app.handle_action(action.expect("No action returned"));

        assert_eq!(app.mode, Mode::Insert);
    }

    #[test]
    fn it_should_switch_from_insert_to_normal_mode() {
        let mut app = Application::new(stdout(), Mode::Insert);

        assert_eq!(app.mode, Mode::Insert);

        let action = handle_event(&mut app.output, &esc_press_event(), &Mode::Insert)
            .expect("Error when handling events");
        let _ = app.handle_action(action.expect("No action returned"));

        assert_eq!(app.mode, Mode::Normal);
    }
}
