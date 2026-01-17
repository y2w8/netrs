use anyhow::Result;
use crate::{nm::client::NmClient, ui::list::StatefulList};

pub struct App {
    pub nm: NmClient,
    pub should_quit: bool,
    pub networks: StatefulList<String>,

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
            networks: StatefulList::with_items(vec!["dwadwa".to_string(), "dwddwadwawa".to_string()]),

            tab: Tabs::KnownNetworks,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
