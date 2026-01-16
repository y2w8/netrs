use anyhow::Result;
use crate::nm::client::NmClient;

pub struct App {
    pub nm: NmClient,
    pub should_quit: bool,

    pub focus: Focus,
}

pub enum Focus {
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
            focus: Focus::KnownNetworks,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
