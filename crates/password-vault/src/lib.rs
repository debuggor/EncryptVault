mod model;
mod store;

pub use model::Credential;
use std::collections::HashMap;
use store::SqliteStore;

pub struct Vault {
    credentials: HashMap<String, Credential>,
    store: SqliteStore,
}

impl Vault {
    pub fn open(db_path: &str, key: &[u8; 32]) -> Result<Self, String> {
        let store = SqliteStore::open(db_path, key)?;
        let credentials = store
            .load_all()?
            .into_iter()
            .map(|c| (c.id.clone(), c))
            .collect();
        Ok(Self { credentials, store })
    }

    pub fn add(&mut self, mut cred: Credential) -> Result<String, String> {
        let now = now_secs();
        cred.id = uuid::Uuid::new_v4().to_string();
        cred.created_at = now;
        cred.updated_at = now;
        self.store.upsert(&cred)?;
        let id = cred.id.clone();
        self.credentials.insert(id.clone(), cred);
        Ok(id)
    }

    pub fn update(&mut self, mut cred: Credential) -> Result<(), String> {
        cred.updated_at = now_secs();
        self.store.upsert(&cred)?;
        self.credentials.insert(cred.id.clone(), cred);
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> Result<(), String> {
        self.store.delete(id)?;
        self.credentials.remove(id);
        Ok(())
    }

    pub fn list(&self) -> Vec<&Credential> {
        self.credentials.values().collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Credential> {
        let q = query.to_lowercase();
        self.credentials
            .values()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.url.to_lowercase().contains(&q)
            })
            .collect()
    }
}

fn now_secs() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [7u8; 32]
    }

    fn open_tmp_vault() -> Vault {
        let path = std::env::temp_dir()
            .join(format!("test_vault_{}.db", uuid::Uuid::new_v4()));
        Vault::open(path.to_str().unwrap(), &test_key()).unwrap()
    }

    fn mk_cred(name: &str, url: &str) -> Credential {
        Credential {
            id: String::new(),
            name: name.to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: url.to_string(),
            notes: String::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_add_and_list() {
        let mut v = open_tmp_vault();
        let id = v.add(mk_cred("GitHub", "https://github.com")).unwrap();
        let list = v.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, id);
        assert_eq!(list[0].name, "GitHub");
    }

    #[test]
    fn test_search_by_name() {
        let mut v = open_tmp_vault();
        v.add(mk_cred("GitHub", "https://github.com")).unwrap();
        v.add(mk_cred("GitLab", "https://gitlab.com")).unwrap();
        assert_eq!(v.search("github").len(), 1);
        assert_eq!(v.search("git").len(), 2);
        assert_eq!(v.search("nothing").len(), 0);
    }

    #[test]
    fn test_delete() {
        let mut v = open_tmp_vault();
        let id = v.add(mk_cred("X", "")).unwrap();
        v.delete(&id).unwrap();
        assert!(v.list().is_empty());
    }

    #[test]
    fn test_update() {
        let mut v = open_tmp_vault();
        v.add(mk_cred("Old", "")).unwrap();
        let mut c = v.list()[0].clone();
        c.name = "New".to_string();
        v.update(c).unwrap();
        assert_eq!(v.list()[0].name, "New");
    }

    #[test]
    fn test_persist_and_reload() {
        let path = std::env::temp_dir()
            .join(format!("test_persist_{}.db", uuid::Uuid::new_v4()));
        let key = test_key();
        let id = {
            let mut v = Vault::open(path.to_str().unwrap(), &key).unwrap();
            v.add(mk_cred("Persisted", "https://example.com")).unwrap()
        };
        let v2 = Vault::open(path.to_str().unwrap(), &key).unwrap();
        assert_eq!(v2.list().len(), 1);
        assert_eq!(v2.list()[0].id, id);
        assert_eq!(v2.list()[0].name, "Persisted");
    }

    #[test]
    fn test_wrong_key_on_reload_fails() {
        let path = std::env::temp_dir()
            .join(format!("test_wrongkey_{}.db", uuid::Uuid::new_v4()));
        let key = test_key();
        {
            let mut v = Vault::open(path.to_str().unwrap(), &key).unwrap();
            v.add(mk_cred("X", "")).unwrap();
        }
        assert!(Vault::open(path.to_str().unwrap(), &[99u8; 32]).is_err());
    }
}
