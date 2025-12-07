use crate::domain::entities::PasswordEntity;
use std::error::Error;


// encapsular el error que recibe de rust
pub type RepositoryResult<T> = Result<T, Box<dyn Error>>;

pub trait IPasswordRepository {
    fn save(&self, password: &PasswordEntity) -> RepositoryResult<PasswordEntity>;

    fn get_by_id(&self, id: &str) -> RepositoryResult<Option<PasswordEntity>>;

    fn get_all(&self) -> RepositoryResult<Vec<PasswordEntity>>;

    fn delete_by_id(&self, id: &str) -> RepositoryResult<()>;
}