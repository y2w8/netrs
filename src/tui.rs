use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Write, stdout};

use crate::app::App;

pub struct Tui {
    stdout: std::io::Stdout,
    backend: CrosstermBackend,
    terminal: Terminal,
}
impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        stdout
            .execute(terminal::EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        Ok(Tui {
            stdout,
            backend,
            terminal,
        })
    }
    pub async fn run(&mut self, app: &mut App) -> Result<()> {

        while !app.should_quit {
            self.terminal.draw(|f| {
                let size = f.area();
                f.render_widget(
                    ratatui::widgets::Paragraph::new("netrs — NetworkManager TUI"),
                    size,
                );
            })?;

            if event::poll(std::time::Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Char('c') => app.quit(),
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        self.stdout.flush();
        self.stdout.execute(terminal::LeaveAlternateScreen);
        Ok(())
    }
}
