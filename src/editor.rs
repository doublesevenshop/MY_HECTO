use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers, ModifierKeyCode};
mod terminal;
use terminal::Terminal;

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
        if self.should_quit == true {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
            
        } else {
            let _ = Self::draw_rows();
            let _ = Terminal::move_cursor_to(0, 0);
        }
        Ok(())
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        
        Ok(())
    }
    
}