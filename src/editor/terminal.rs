use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}


impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0})?;
        Self::execute()?;
        Ok(())

    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn hind_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

}