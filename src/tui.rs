use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
};
use std::io::stdout;

use crate::{app::App, ui::table};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        stdout
            .execute(terminal::EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Tui { terminal })
    }
    pub async fn run(&mut self, app: &mut App) -> Result<()> {
        while !app.should_quit {
            self.terminal.draw(|f| {
                let size = f.area();
                f.render_widget(
                    ratatui::widgets::Paragraph::new("netrs — NetworkManager TUI"),
                    size,
                );
                let layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(30),
                        Constraint::Percentage(20),
                    ],
                )
                .split(size);
                table::draw(f, layout[0], app);
            })?;

            if event::poll(std::time::Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        self.terminal
            .backend_mut()
            .execute(terminal::LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
