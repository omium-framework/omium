use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub _token: String,
}
