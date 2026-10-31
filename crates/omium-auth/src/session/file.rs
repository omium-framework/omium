//! File-based session driver (`storage/framework/sessions`).

use std::path::PathBuf;

use async_trait::async_trait;
use tokio::fs;

use super::Session;
use super::SessionId;
use super::store::SessionStore;
use crate::SessionError;

/// One JSON file per session id under a base directory.
#[derive(Clone, Debug)]
pub struct FileSessionStore {
    base_dir: PathBuf,
}

impl FileSessionStore {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }

    fn path_for(&self, id: &SessionId) -> PathBuf {
        self.base_dir.join(id.as_str())
    }
}

#[async_trait]
impl SessionStore for FileSessionStore {
    async fn load(&self, id: &SessionId) -> Result<Option<Session>, SessionError> {
        let path = self.path_for(id);
        if !path.is_file() {
            return Ok(None);
        }
        let bytes = fs::read(&path).await?;
        let session: Session = serde_json::from_slice(&bytes)?;
        Ok(Some(session))
    }

    async fn save(&self, session: &Session) -> Result<(), SessionError> {
        if session.is_destroyed() {
            return self.destroy(session.id()).await;
        }
        fs::create_dir_all(&self.base_dir).await?;
        let path = self.path_for(session.id());
        let tmp = path.with_extension("tmp");
        let payload = serde_json::to_vec(session)?;
        fs::write(&tmp, payload).await?;
        fs::rename(&tmp, &path).await?;
        Ok(())
    }

    async fn destroy(&self, id: &SessionId) -> Result<(), SessionError> {
        let path = self.path_for(id);
        if path.is_file() {
            fs::remove_file(path).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::model::Session;

    #[tokio::test]
    async fn file_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = FileSessionStore::new(dir.path());

        let mut session = Session::new();
        session.put("user", "demo");
        store.save(&session).await.expect("save");

        let loaded = store
            .load(session.id())
            .await
            .expect("load")
            .expect("some");
        assert_eq!(loaded.get::<String>("user"), Some("demo".to_string()));
    }

    #[tokio::test]
    async fn destroy_removes_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = FileSessionStore::new(dir.path());
        let mut session = Session::new();
        store.save(&session).await.expect("save");
        session.destroy();
        store.save(&session).await.expect("destroy save");
        assert!(store.load(session.id()).await.expect("load").is_none());
    }
}
