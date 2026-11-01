//! File-backed demo registrations until A7 `users` table.

use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

const STORE_PATH: &str = "storage/framework/demo_users.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoUserEntry {
    pub email: String,
    pub password: String,
    pub user_id: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DemoUsersFile {
    #[serde(default)]
    users: Vec<DemoUserEntry>,
}

#[derive(Debug, thiserror::Error)]
pub enum DemoUserError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("That email is already registered.")]
    DuplicateEmail,
}

pub struct DemoUserStore;

impl DemoUserStore {
    fn load_file() -> Result<DemoUsersFile, io::Error> {
        let path = Path::new(STORE_PATH);
        if !path.is_file() {
            return Ok(DemoUsersFile::default());
        }
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents).unwrap_or_default())
    }

    fn save_file(file: &DemoUsersFile) -> Result<(), io::Error> {
        fs::create_dir_all("storage/framework")?;
        let contents = toml::to_string_pretty(file).map_err(|err| {
            io::Error::new(io::ErrorKind::InvalidData, err.to_string())
        })?;
        fs::write(STORE_PATH, contents)
    }

    #[must_use]
    pub fn find_by_email(email: &str) -> Option<DemoUserEntry> {
        let normalized = email.trim().to_lowercase();
        Self::load_file()
            .ok()?
            .users
            .into_iter()
            .find(|user| user.email.trim().eq_ignore_ascii_case(&normalized))
    }

    pub fn register(email: &str, password: &str, admin_email: &str) -> Result<DemoUserEntry, DemoUserError> {
        let email = email.trim();
        if email.is_empty() {
            return Err(DemoUserError::DuplicateEmail);
        }
        if email.eq_ignore_ascii_case(admin_email.trim()) {
            return Err(DemoUserError::DuplicateEmail);
        }
        let mut file = Self::load_file()?;
        if file
            .users
            .iter()
            .any(|user| user.email.eq_ignore_ascii_case(email))
        {
            return Err(DemoUserError::DuplicateEmail);
        }
        let next_id = file
            .users
            .iter()
            .map(|user| user.user_id)
            .max()
            .unwrap_or(1)
            .saturating_add(1);
        let entry = DemoUserEntry {
            email: email.to_string(),
            password: password.to_string(),
            user_id: next_id,
        };
        file.users.push(entry.clone());
        Self::save_file(&file)?;
        Ok(entry)
    }

    pub fn update_password(email: &str, password: &str) -> Result<(), DemoUserError> {
        let mut file = Self::load_file()?;
        let Some(user) = file
            .users
            .iter_mut()
            .find(|user| user.email.eq_ignore_ascii_case(email.trim()))
        else {
            return Ok(());
        };
        user.password = password.to_string();
        Self::save_file(&file)?;
        Ok(())
    }

    pub fn email_registered(email: &str, admin_email: &str) -> bool {
        email.trim().eq_ignore_ascii_case(admin_email.trim())
            || Self::find_by_email(email).is_some()
    }
}
