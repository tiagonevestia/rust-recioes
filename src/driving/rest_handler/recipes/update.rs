use actix_web::web::Json;

use crate::{
    domain::{self, update_recipe::UpdateError},
    driving::rest_handler::{errors::ApiError, validate::validate},
    helpers::{respond_json, string_vec_to_vec_str},
};

use super::{RecipeResponse, UpdateRecipeRequest};

pub async fn update(request: Json<UpdateRecipeRequest>) -> Result<Json<RecipeResponse>, ApiError> {
    validate(&request)?;

    let result = domain::update_recipe::update_recipe(
        &request.id,
        &request.name,
        string_vec_to_vec_str(&request.tags).as_ref(),
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        string_vec_to_vec_str(&request.instructions).as_ref(),
    );

    result
        .map(|v| respond_json(RecipeResponse::from(v)))
        .map_err(|e| match e {
            UpdateError::Unknown(m) => ApiError::Unknown(m),
            UpdateError::InvalidData(m) => ApiError::InvalidData(m),
            UpdateError::NotFound => ApiError::NotFound(String::from(
                "Nenhum sanduíche para atualizar correspondente aos critérios especificados",
            )),
            UpdateError::Conflict(m) => ApiError::Conflict(m),
        })?
}
