use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode},
    execute, queue,
    terminal::{self, ClearType},
    Result,
};
use std::io::{self, stdout, Write};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to retrieve original terminal state!");
        Output::clear_screen().expect("Unable to clear screen");
    }
}

struct Reader;

impl Reader {
    fn new() -> Self {
        Self
    }

    fn read_key(&self) -> Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }
}

struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader::new(),
            output: Output::new(),
        }
    }

    fn process(&self) -> Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                // modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                return Ok(false);
            }
            _ => Ok(true),
        }
    }

    fn run(&mut self) -> Result<bool> {
        self.output.refresh_screen()?;
        self.process()
    }
}

struct Size {
    x: usize,
    y: usize,
}

struct Output {
    window_size: Size,
    editor_contents: EditorContents,
}

impl Output {
    fn new() -> Self {
        let window_size = terminal::size()
            .map(|(x, y)| Size {
                x: x as usize,
                y: y as usize,
            })
            .unwrap();
        Self {
            window_size,
            editor_contents: EditorContents::new(),
        }
    }

    fn clear_screen() -> Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.window_size.y;
        for i in 0..screen_rows {
            self.editor_contents.push('~');
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r");
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<()> {
        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        queue!(self.editor_contents, cursor::MoveTo(0, 0))?;
        self.editor_contents.flush()
    }
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableBracketedPaste,
        event::EnableFocusChange,
        event::EnableMouseCapture,
    )?;

    let mut editor = Editor::new();
    while editor.run()? {}

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        terminal::Clear(ClearType::All)
    )?;

    Ok(())
}
