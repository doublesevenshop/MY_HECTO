use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::stdout;

pub struct Terminal {}


impl Terminal {
    // App has finished, we need to free the resource(raw_mode)
    pub fn terminate() -> Result<(), std::io::Error> {
        let _ = disable_raw_mode();
        Ok(())
    }
    // init, start the raw_mode, and clear the screen and move the cursor to (0, 0)
    pub fn initialize() -> Result<(), std::io::Error> {
        let _ = enable_raw_mode();
        Self::clear_screen()?;
        let _ = Self::move_cursor_to(0, 0);
        Ok(())

    }
    // clear the screen, execute the execute!
    pub fn clear_screen() -> Result<(), std::io::Error> {
        let _ = execute!(stdout(), Clear(ClearType::All));
        Ok(())
    }
    // move the cursor to the coordinate 
    pub fn move_cursor_to(x: u16, y:u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

}