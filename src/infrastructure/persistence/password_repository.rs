use crate::domain::entities::password_entity::PasswordEntity;
use crate::domain::repositories::{IPasswordRepository, RepositoryResult};
use serde_json;
use std::{fs, io::{self, Write}};

pub struct PasswordRepository {
    file_path: String,
}

impl PasswordRepository {
    pub fn new(file_path: &str) -> Self {
        PasswordRepository {
            file_path: file_path.to_string(),
        }
    }

    fn read_all_from_file(&self) -> RepositoryResult<Vec<PasswordEntity>> {
        let content = fs::read_to_string(&self.file_path).unwrap_or_else(|_| "[]".to_string());
        let passwords = serde_json::from_str(&content)?;
        Ok(passwords)
    }

    fn write_all_to_file(&self, passwords: &[PasswordEntity]) -> RepositoryResult<()> {
        let mut file = fs::File::create(&self.file_path)?;
        let content = serde_json::to_string_pretty(passwords)?;
        write!(file, "{}", content)?;
        Ok(())
    }
}

impl IPasswordRepository for PasswordRepository {
    fn save(&self, password: &PasswordEntity) -> RepositoryResult<PasswordEntity> {
        let mut passwords = self.read_all_from_file()?;
        
        if let Some(pos) = passwords.iter().position(|p| p.id == password.id) {
            passwords[pos] = password.clone();
        } else {
            passwords.push(password.clone());
        }

        self.write_all_to_file(&passwords)?;
        Ok(password.clone())
    }

    fn get_by_id(&self, id: &str) -> RepositoryResult<Option<PasswordEntity>> {
        let passwords = self.read_all_from_file()?;
        Ok(passwords.into_iter().find(|p| p.id == id))
    }

    fn get_all(&self) -> RepositoryResult<Vec<PasswordEntity>> {
        self.read_all_from_file()
    }

    fn delete_by_id(&self, id: &str) -> RepositoryResult<()> {
        let mut passwords = self.read_all_from_file()?;
        let initial_len = passwords.len();
        passwords.retain(|p| p.id != id);

        if passwords.len() == initial_len {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Contraseña no encontrada para eliminar").into());
        }

        self.write_all_to_file(&passwords)?;
        Ok(())
    }
}