use tokio::net::{TcpListener, TcpStream};
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

#[derive(Error, Debug)]
enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

}

async fn tui_loop() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Main Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        };
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let tui_loop = tui_loop();

    tokio::join!(tui_loop);
}
