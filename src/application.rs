use crate::{buffer::Buffer, events::handle_event, viewport::Viewport};
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
    buffers: Vec<Buffer>,
    current_viewport: usize,
    viewports: Vec<Viewport>,
}

impl Application {
    pub fn new(output: Stdout, default_mode: Mode, buffers: Vec<Buffer>) -> Self {
        Self {
            mode: default_mode,
            output,
            buffers,

            current_viewport: 0,
            viewports: vec![Viewport::new()],
        }
    }

    pub fn handle_action(&mut self, action: Action) -> anyhow::Result<()> {
        let current_row = cursor::position()?.1;
        let current_column = cursor::position()?.0;
        let current_buffer_id = self.viewports[self.current_viewport].buffer_id;
        let lines = &mut self.buffers[current_buffer_id].lines;

        match action {
            Action::Write(char) => {
                lines[current_row as usize].insert(current_column as usize, char);

                self.output.queue(cursor::MoveRight(1))?;
            }
            Action::Back => {
                if current_column > 0 {
                    lines[current_row as usize].remove(current_column as usize - 1);

                    queue!(
                        self.output,
                        cursor::SavePosition,
                        cursor::MoveToColumn(lines[current_row as usize].len() as u16),
                        style::Print(' '),
                        cursor::RestorePosition,
                        cursor::MoveLeft(1)
                    )?;
                }
            }
            Action::Return => {
                self.output.queue(cursor::MoveToNextLine(1))?;
                lines.push("".to_string());
            }

            Action::MoveLeft => {
                self.output.queue(cursor::MoveLeft(1))?;
            }
            Action::MoveRight => {
                let line_len = lines[current_row as usize].len();

                if (current_column as usize) < line_len {
                    self.output.queue(cursor::MoveRight(1))?;
                } else {
                    self.output.queue(cursor::MoveToColumn(line_len as u16))?;
                }
            }
            Action::MoveUp => {
                self.output.queue(cursor::MoveUp(1))?;
            }
            Action::MoveDown => {
                let lines_count = lines.len();

                if (current_row as usize) < lines_count - 1 {
                    self.output.queue(cursor::MoveDown(1))?;
                }
            }

            Action::ChangeMode(mode) => {
                self.mode = mode;
            }

            Action::Quit => {}
        }

        Ok(())
    }

    pub fn draw_buffer(&mut self) -> anyhow::Result<()> {
        self.output.queue(cursor::SavePosition)?;

        let current_viewport = &mut self.viewports[self.current_viewport];
        current_viewport.resize(terminal::size()?.0, terminal::size()?.1);
        let current_buffer = &mut self.buffers[current_viewport.buffer_id];

        let start_column = current_viewport.start_column as usize;
        let start_rows = if current_buffer.lines.len() > current_viewport.start_rows as usize {
            current_viewport.start_rows as usize
        } else {
            0
        };

        for (index, line) in current_buffer.lines[start_rows..].iter().enumerate() {
            self.output.queue(cursor::MoveTo(0, index as u16))?;
            if line.len() > start_column {
                self.output.queue(style::Print(&line[start_column..]))?;
            } else {
                style::Print("         ");
            }
            self.output.queue(cursor::MoveToNextLine(1))?;
        }

        self.output.queue(cursor::RestorePosition)?;

        Ok(())
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_buffer()?;
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

        self.output.flush()?;
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
        let mut app = Application::new(stdout(), Mode::Normal, vec![Buffer::new(None)]);

        let action = handle_event(&mut app.output, &key_press_event('q'), &Mode::Normal)
            .expect("Error when handling events");

        assert_eq!(Action::Quit, action.expect("No action returned"));
    }

    #[test]
    fn it_should_switch_to_insert_mode() {
        let mut app = Application::new(stdout(), Mode::Normal, vec![Buffer::new(None)]);

        assert_eq!(app.mode, Mode::Normal);

        let action = handle_event(&mut app.output, &key_press_event('i'), &Mode::Normal)
            .expect("Error when handling events");
        let _ = app.handle_action(action.expect("No action returned"));

        assert_eq!(app.mode, Mode::Insert);
    }

    #[test]
    fn it_should_switch_from_insert_to_normal_mode() {
        let mut app = Application::new(stdout(), Mode::Insert, vec![Buffer::new(None)]);

        assert_eq!(app.mode, Mode::Insert);

        let action = handle_event(&mut app.output, &esc_press_event(), &Mode::Insert)
            .expect("Error when handling events");
        let _ = app.handle_action(action.expect("No action returned"));

        assert_eq!(app.mode, Mode::Normal);
    }
}
