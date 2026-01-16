use anyhow::Result;

mod app;
mod tui;
mod ui;
mod nm;

use tui::Tui;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize app state
    let mut app = App::new().await?;
    let mut tui = Tui::new()?;

    // Start TUI
    tui.run(&mut app).await?;

    Ok(())
}
