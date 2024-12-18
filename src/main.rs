use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc};

use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Widget, Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Alignment}
};

use crossterm::{
    event::{self, Event, KeyCode, DisableMouseCapture, EnableMouseCapture},
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use thiserror::Error;

mod app;
use app::App;

#[derive(Error, Debug)]
enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let mut app = App::default();
    app.set_terminal(terminal).await;
    app.run_app().await.unwrap();

    let mut terminal = app.terminal.unwrap();
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}