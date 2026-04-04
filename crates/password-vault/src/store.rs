use rusqlite::{params, Connection};
use encrypt::{decrypt_with_key, encrypt_with_key};
use crate::model::Credential;

pub struct SqliteStore {
    conn: Connection,
    key: [u8; 32],
}

impl SqliteStore {
    pub fn open(db_path: &str, key: &[u8; 32]) -> Result<Self, String> {
        let conn = Connection::open(db_path).map_err(|e| format!("db open: {e}"))?;
        #[cfg(unix)]
        {
            use std::fs;
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(db_path, fs::Permissions::from_mode(0o600));
        }
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS credentials \
             (id TEXT PRIMARY KEY, ciphertext BLOB NOT NULL);",
        )
        .map_err(|e| format!("db init: {e}"))?;
        Ok(Self { conn, key: *key })
    }

    pub fn load_all(&self) -> Result<Vec<Credential>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT ciphertext FROM credentials")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, Vec<u8>>(0))
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for row in rows {
            let ct = row.map_err(|e| e.to_string())?;
            let pt = decrypt_with_key(&self.key, &ct)?;
            let cred: Credential = serde_json::from_slice(&pt)
                .map_err(|e| format!("deserialize: {e}"))?;
            out.push(cred);
        }
        Ok(out)
    }

    pub fn upsert(&self, cred: &Credential) -> Result<(), String> {
        let pt = serde_json::to_vec(cred).map_err(|e| e.to_string())?;
        let ct = encrypt_with_key(&self.key, &pt)?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO credentials (id, ciphertext) VALUES (?1, ?2)",
                params![cred.id, ct],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM credentials WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
