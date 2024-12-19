use std::vec;

use tui::{
    backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget}, Terminal
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

        let options = vec!["Server", "Client", "Quit"];
        let mut selected_option = ListState::default();
        selected_option.select(Some(0));

        loop {
            match self.role {
                Role::None => {

                    terminal.draw(|f| {
                        let size = f.size();
                        let items: Vec<ListItem> = options.iter().map(|i| ListItem::new(i.to_string())).collect();
                        let list_options = List::new(items)
                            .block(Block::default().title("Select Option").borders(Borders::ALL))
                            .highlight_style(
                                Style::default()
                                    .bg(Color::White)
                                    .fg(Color::Black)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .highlight_symbol("> ");
                        f.render_stateful_widget(list_options, size, &mut selected_option);
                    })?;

                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Up => {
                                selected_option.select(Some(selected_option.selected().unwrap_or(0).saturating_sub(1)));
                            }
                            KeyCode::Down => {
                                if selected_option.selected().unwrap_or(0) < options.len() - 1 {
                                    selected_option.select(Some(selected_option.selected().unwrap_or(0).saturating_add(1)));
                                }
                            }
                            KeyCode::Enter => {
                                self.set_role(Role::from(&selected_option));
                                if let Role::None = self.role {
                                    break;
                                }
                            }
                            _ => {}
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

impl From<&ListState> for Role {
/// Converts a `ListState` into a `Role`.
///
/// This function maps the currently selected index of a `ListState` to a corresponding `Role`.
/// If the selected index is 0, it returns `Role::Server`.
/// If the selected index is 1, it returns `Role::Client`.
/// If no index is selected or if the index is out of bounds, it defaults to returning `Role::None`.

    fn from(value: &ListState) -> Self {
        match value.selected().unwrap_or(0) {
            0 => Role::Server,
            1 => Role::Client,
            _ => Role::None
        }
    }
}