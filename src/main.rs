use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    Result,
};
use std::io::stdout;

fn main() -> Result<()> {
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableBracketedPaste,
        event::EnableFocusChange,
        event::EnableMouseCapture,
    )?;
    terminal::enable_raw_mode()?;

    loop {
        let event = get_event()?;
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => break,
            KeyEvent {
                code: KeyCode::Char(x),
                ..
            } => println!("{}", x),
            _ => {}
        }
    }

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        terminal::Clear(ClearType::All)
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn get_event() -> Result<KeyEvent> {
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            return Ok(event);
        }
    }
}
