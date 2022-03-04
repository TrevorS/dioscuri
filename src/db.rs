use anyhow::anyhow;
use rusqlite::OptionalExtension;

use crate::db::model::Certificate;

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
                [],
            )
            .map_err(|_| anyhow!("failed to prepare database"))?;

        Ok(())
    }

    pub fn get_certificate(&self, hostname: &str) -> anyhow::Result<Option<model::Certificate>> {
        self.connection
            .prepare(
                r#"
            SELECT
                id,
                hostname,
                fingerprint,
                first_seen,
                last_seen
            FROM
                certificates
            WHERE
                hostname = ?1;
            "#,
            )?
            .query_row(rusqlite::params![hostname], |row| row.try_into())
            .optional()
            .map_err(|_| anyhow!("error retrieving certificate from database"))
    }

    pub fn insert_certificate(
        &self,
        hostname: &str,
        fingerprint: &str,
    ) -> anyhow::Result<Certificate> {
        let now = time::OffsetDateTime::now_utc();

        let count = self
            .connection
            .execute(
                r#"
            INSERT INTO
                certificates (
                    hostname,
                    fingerprint,
                    first_seen,
                    last_seen
                )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4
            );
            "#,
                rusqlite::params![hostname, fingerprint, now, now],
            )
            .map_err(|_| anyhow!("failed to insert certificate into database"))?;

        anyhow::ensure!(
            count > 0,
            "insert count was wrong when inserting certificate into database"
        );

        Ok(Certificate {
            id: self.connection.last_insert_rowid(),
            hostname: hostname.to_string(),
            fingerprint: fingerprint.to_string(),
            first_seen: now,
            last_seen: now,
        })
    }

    pub fn update_certificate_timestamp(&self, hostname: &str) -> anyhow::Result<()> {
        let now = time::OffsetDateTime::now_utc();

        self.connection
            .execute(
                r#"
            UPDATE
                certificates
            SET
                last_seen = ?1
            WHERE
                hostname = ?2;
            "#,
                rusqlite::params![now, hostname],
            )
            .map_err(|_| anyhow!("failed to update certificate timestamp"))?;

        Ok(())
    }
}

mod model {
    pub struct Certificate {
        pub id: i64,
        pub hostname: String,
        pub fingerprint: String,
        pub first_seen: time::OffsetDateTime,
        pub last_seen: time::OffsetDateTime,
    }

    impl TryFrom<&rusqlite::Row<'_>> for Certificate {
        type Error = rusqlite::Error;

        fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
            Ok(Self {
                id: row.get(0)?,
                hostname: row.get(1)?,
                fingerprint: row.get(2)?,
                first_seen: row.get(3)?,
                last_seen: row.get(4)?,
            })
        }
    }
}
