//! Session persistence trait.

use async_trait::async_trait;

use super::Session;
use super::SessionId;
use crate::SessionError;

/// Persists sessions (file or database driver).
#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn load(&self, id: &SessionId) -> Result<Option<Session>, SessionError>;
    async fn save(&self, session: &Session) -> Result<(), SessionError>;
    async fn destroy(&self, id: &SessionId) -> Result<(), SessionError>;
}
