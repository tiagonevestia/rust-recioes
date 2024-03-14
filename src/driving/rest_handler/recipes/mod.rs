use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::domain::recipe::Recipe;

pub mod create;
pub mod delete;
pub mod find;
pub mod get_by_id;
pub mod update;

//
// REQUESTS
//
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateRecipeRequest {
    #[validate(length(
        min = 3,
        message = "o nome é obrigatório e deve ter pelo menos 3 caracteres"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        message = "tags são obrigatórias e devem ter pelo menos 1 item"
    ))]
    pub tags: Vec<String>,

    #[validate(length(
        min = 1,
        message = "ingredientes são obrigatórios e devem ter pelo menos 1 item"
    ))]
    pub ingredients: Vec<String>,

    #[validate(length(
        min = 1,
        message = "instruções são obrigatórias e devem ter pelo menos 1 item"
    ))]
    pub instructions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateRecipeRequest {
    pub id: String,
    #[validate(length(
        min = 3,
        message = "o nome é obrigatório e deve ter pelo menos 3 caracteres"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        message = "tags são obrigatórias e devem ter pelo menos 1 item"
    ))]
    pub tags: Vec<String>,
    #[validate(length(
        min = 1,
        message = "ingredientes são obrigatórios e devem ter pelo menos 1 item"
    ))]
    pub ingredients: Vec<String>,
    #[validate(length(
        min = 1,
        message = "instruções são obrigatórias e devem ter pelo menos 1 item"
    ))]
    pub instructions: Vec<String>,
}

//
// RESPONSES
//
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RecipeResponse {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

impl From<Recipe> for RecipeResponse {
    fn from(r: Recipe) -> Self {
        RecipeResponse {
            id: r
                .id()
                .value()
                .clone()
                .unwrap_or(String::from(""))
                .to_string(),
            name: r.name().value().to_string(),
            tags: r.tags().value().clone(),
            ingredients: r.ingredients().value().clone(),
            instructions: r.instructions().value().clone(),
        }
    }
}
