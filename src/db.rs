pub struct Db {
    connection: rusqlite::Connection,
}

impl Db {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            connection: rusqlite::Connection::open(path)?,
        })
    }
}

impl Db {
    pub fn prepare(&self) -> anyhow::Result<()> {
        self.connection
            .execute(
                r#"
                CREATE TABLE IF NOT EXISTS certificates (
                    id INTEGER PRIMARY KEY,
                    hostname TEXT NOT NULL,
                    fingerprint TEXT NOT NULL,
                    first_seen TEXT NOT NULL,
                    last_seen TEXT NOT NULL
                );
            "#,
                rusqlite::params![],
            )
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("failed to prepare database"))
    }
}
