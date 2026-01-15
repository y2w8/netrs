use anyhow::Result;
use crate::nm::client::NmClient;

pub struct App {
    pub nm: NmClient,
    pub should_quit: bool,
}

impl App {
    pub async fn new() -> Result<Self> {
        let nm = NmClient::new().await?;

        Ok(Self {
            nm,
            should_quit: false,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
