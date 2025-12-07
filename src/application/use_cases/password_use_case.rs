use crate::domain::entities::password_entity::PasswordEntity;
use crate::domain::repositories::{IPasswordRepository, RepositoryResult};

pub struct PasswordUseCase {
    repository: Box<dyn IPasswordRepository>,
}

impl PasswordUseCase {
    pub fn new(repository: Box<dyn IPasswordRepository>) -> Self {
        PasswordUseCase { repository }
    }

    pub fn add_password(&self, password: &PasswordEntity) -> RepositoryResult<PasswordEntity> {
        self.repository.save(password)
    }

    pub fn get_all_passwords(&self) -> RepositoryResult<Vec<PasswordEntity>> {
        self.repository.get_all()
    }
    
    pub fn get_password(&self, site: &str, _username: &str) -> RepositoryResult<Option<PasswordEntity>> {
        // En una implementación real, el ID sería una combinación o hash de site y username
        // Aquí, para simplificar, usaremos el 'site' como id para buscar.
        // Se recomienda mejorar este mecanismo de ID.
        let id = self.find_id_by_site_and_username(site, _username)?;
        self.repository.get_by_id(&id)
    }

    pub fn delete_password(&self, site: &str, _username: &str) -> RepositoryResult<()> {
        let id = self.find_id_by_site_and_username(site, _username)?;
        self.repository.delete_by_id(&id)
    }
    
    // Función auxiliar para encontrar el ID
    fn find_id_by_site_and_username(&self, site: &str, username: &str) -> RepositoryResult<String> {
        let passwords = self.repository.get_all()?;
        passwords
            .into_iter()
            .find(|p| p.page_name == site && p.username == username)
            .map(|p| p.id)
            .ok_or_else(|| "No se encontró una entrada con ese sitio y usuario".into())
    }
}