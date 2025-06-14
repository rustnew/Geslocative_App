use crate::domain::models::user::{Users, CreateUser};
use async_trait::async_trait;
use uuid::Uuid;



#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Users>, String>;
    async fn create(&self, user: CreateUser) -> Result<Users, String>;
    async fn update(&self, user: Users) -> Result<Users, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}