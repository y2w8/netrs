use anyhow::Result;
use crate::nm::client::NmClient;

pub struct App {
    pub nm: NmClient,
    pub should_quit: bool,

    pub tab: Tabs,
}

pub enum Tabs {
    KnownNetworks,
    NewNetworks,
    Device,
}
impl App {
    pub async fn new() -> Result<Self> {
        let nm = NmClient::new().await?;

        Ok(Self {
            nm,
            should_quit: false,

            tab: Tabs::KnownNetworks,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
