use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PasswordEntity {
    pub id: String,
    pub page_url: String,
    pub page_name: String,
    pub username: String,
    pub email: String,
    pub password: String
}

impl PasswordEntity {
    pub fn new(id: String, page_url: String, page_name: String, username: String, email: String, password: String) -> Self {
        Self {
            id,
            page_url,
            page_name,
            username,
            email,
            password
        }
    }
}
