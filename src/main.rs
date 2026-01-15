use anyhow::Result;

mod app;
mod tui;
mod nm;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize app state
    let mut app = app::App::new().await?;

    // Start TUI
    tui::run(&mut app).await?;

    Ok(())
}
