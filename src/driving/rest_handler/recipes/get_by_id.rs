use actix_web::web::{self, Json};

use crate::{
    domain::{self, find_one_recipe::FindOneError},
    driving::rest_handler::errors::ApiError,
    helpers::respond_json,
};

use super::RecipeResponse;

pub async fn get_by_id(path: web::Path<String>) -> Result<Json<RecipeResponse>, ApiError> {
    let recipe_id = path.into_inner();

    let result = domain::find_one_recipe::find_one_recipe(
        &recipe_id,
        "",
        vec![].as_ref(),
        vec![].as_ref(),
        vec![].as_ref(),
    );

    result
        .map(|v| respond_json(RecipeResponse::from(v)))
        .map_err(|e| match e {
            FindOneError::Unknown(m) => ApiError::Unknown(m),
            FindOneError::NotFound => ApiError::NotFound(String::from(
                "Nenhuma receita encontrado com os critérios especificados",
            )),
        })?
}
