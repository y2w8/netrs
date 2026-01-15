use anyhow::Result;
use zbus::{Connection, Proxy};

pub struct NmClient {
    pub connection: Connection,
    pub proxy: Proxy<'static>,
}

impl NmClient {
    pub async fn new() -> Result<Self> {
        // Connect to system D-Bus
        let connection = Connection::system().await?;

        // Create NetworkManager proxy
        let proxy = Proxy::new(
            &connection,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        )
        .await?;

        Ok(Self { connection, proxy })
    }
}
