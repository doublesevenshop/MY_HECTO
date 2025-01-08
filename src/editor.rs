use crossterm::event::{
    read, 
    Event::{self, Key}, 
    KeyCode::Char, 
    KeyEvent, 
    KeyModifiers, 
};
mod terminal;
use terminal::{Terminal, Position, Size};

pub struct Editor {
    should_quit: bool,  // If someone push ctrl+o, should quit. 
}

impl Editor {
    // open to normal user
    pub const fn default() -> Self {
        Self {should_quit: false}
    }
    // start run, 3 step: init, repl, terminate. 
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    // used by yourself, you need to refreash the screen and checkout if quit.
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            let _ = self.evaluate_event(&event);
        }

        Ok(())

    }
    fn evaluate_event(&mut self, event: &Event) {
        // monitor the keyboard the match the code and modifiers
        if let Key(KeyEvent{
            code, modifiers, ..
        }) = event 
        {
            match code {
                Char('o') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hind_cursor()?;

        if self.should_quit == true {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
            
        } else {
            let _ = Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:0, y:0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let Size{height, ..} = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    
}