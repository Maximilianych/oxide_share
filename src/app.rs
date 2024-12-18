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

use crate::Error;

/// Application state
///
/// This struct contains the state of the application, including which role (client or server) the
/// application is currently playing.
///
/// The role defaults to `Role::None`, which is used to indicate that the application is not
/// currently connected to a server.
#[derive(Default)]
pub struct App {
    /// The role the application is currently playing
    pub role: Role,
    pub terminal: Option<Terminal<CrosstermBackend<std::io::Stdout>>>,
}

impl App {
    pub async fn set_terminal(&mut self, terminal: Terminal<CrosstermBackend<std::io::Stdout>>) {
        self.terminal = Some(terminal);
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    pub fn tui_choose_role(&mut self) {
        // tui for choosing role
        todo!()
    }
    
    pub async fn run_app(&mut self) -> Result<(), Error>{
        let mut terminal = self.terminal.take().unwrap();

        loop {
            match self.role {
                Role::None => {
                    terminal.draw(|f| {
                        let size = f.size();
                        let block = Block::default().title("None Role").borders(Borders::ALL);
                        f.render_widget(block, size);
                    })?;
            
                    if event::poll(std::time::Duration::from_millis(200))? {
                        if let Event::Key(key) = event::read()? {
                            match key.code {
                                KeyCode::Char('q') => break,
                                KeyCode::Char('a') => self.set_role(Role::Server),
                                KeyCode::Char('b') => self.set_role(Role::Client),
                                _ => {}
                            }
                        }
                    }
                }

                Role::Server => {
                    terminal.draw(|f| {
                        let size = f.size();
                        let block = Block::default().title("Server Role").borders(Borders::ALL);
                        f.render_widget(block, size);
                    })?;

                    if event::poll(std::time::Duration::from_millis(200))? {
                        if let Event::Key(key) = event::read()? {
                            match key.code {
                                KeyCode::Char('q') => self.set_role(Role::None),
                                _ => {}
                            }
                        }
                    }
                }

                Role::Client => {
                    terminal.draw(|f| {
                        let size = f.size();
                        let block = Block::default().title("Client Role").borders(Borders::ALL);
                        f.render_widget(block, size);
                    })?;

                    if event::poll(std::time::Duration::from_millis(200))? {
                        if let Event::Key(key) = event::read()? {
                            match key.code {
                                KeyCode::Char('q') => self.set_role(Role::None),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        self.terminal = Some(terminal);
        Ok(())
    }

}

/// An enumeration of the roles the application can play
///
/// This enumeration specifies the roles the application can play, including client, server, and
/// none.
#[derive(Debug, Default)]
pub enum Role {
    Client,
    Server,
    #[default]
    None
}
