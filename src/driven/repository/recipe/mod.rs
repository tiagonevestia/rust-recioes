use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{config::PersistenceConfig, domain::Entity};

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoFindAllError {
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoUpdateError {
    InvalidData(String),
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindRecipe {
    pub id: Option<String>,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub instructions: Option<Vec<String>>,
}

#[async_trait]
pub trait Repository<T>
where
    T: Entity,
{
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized;

    async fn create(&self, recipe: T) -> Result<T, RepoCreateError>;

    async fn find_one(&self, recipe: FindRecipe) -> Result<T, RepoSelectError>;

    async fn find_all(&self, recipe: FindRecipe) -> Result<Vec<T>, RepoFindAllError>;

    async fn update(&self, recipe: T) -> Result<T, RepoUpdateError>;

    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError>;
}
