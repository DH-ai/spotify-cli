use crossterm::{
    cursor, ExecutableCommand, Result, terminal, event::{self, KeyCode, KeyEvent, Event}
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    // Setup terminal
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    // Main loop to capture input
    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => break, // Quit on 'q'
                Event::Key(KeyEvent { code, .. }) => {
                    stdout.execute(cursor::MoveTo(0, 0))?;
                    println!("You pressed: {:?}", code);
                    stdout.flush()?;
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

