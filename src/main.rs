use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute, style,
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
        match read_char()? {
            'q' => break,
            x => execute!(stdout, style::Print(x))?,
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

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
