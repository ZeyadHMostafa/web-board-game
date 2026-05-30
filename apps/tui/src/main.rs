use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod app;
mod handler;
mod ui;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Terminal low-level environment
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Swaps terminal view out to an alternate processing buffer screen
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Instantiate our Elm-style Application State Container
    let mut app = App::new(Some("./packages/python-ml/data/exp1/trained_weights.npy"));

    // 3. Central Application Tick Thread Execution Loop
    while app.running {
        // Redraw user interface based on current app mutations
        terminal.draw(|f| ui::render(f, &app))?;

        // Synchronous keyboard block read listener
        if let Event::Key(key_event) = event::read()? {
            // Filter out internal key release echoes (relevant on Windows environments)
            if key_event.kind == event::KeyEventKind::Press {
                handler::handle_key_events(key_event, &mut app);
            }
        }
    }

    // 4. Graceful Cleanup and Terminal Restructuring Sequence
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}