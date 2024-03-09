use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{self, create_recipe::CreateError, recipe::Recipe},
    helpers::{respond_json, string_vec_to_vec_str},
};

use super::{errors::ApiError, validate::validate};

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

//
// RESPONSES
//
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RecipeResponse {
    pub id: Uuid,
    pub name: String,
    pub tags: Vec<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

impl From<Recipe> for RecipeResponse {
    fn from(r: Recipe) -> Self {
        RecipeResponse {
            id: r.id.clone().unwrap_or(Uuid::now_v7()),
            name: r.name().value().to_string(),
            tags: r.tags().value().clone(),
            ingredients: r.ingredients().value().clone(),
            instructions: r.instructions().value().clone(),
        }
    }
}

pub async fn create_recipe(
    request: Json<CreateRecipeRequest>,
) -> Result<Json<RecipeResponse>, ApiError> {
    validate(&request)?;

    let result = domain::create_recipe::create_recipe(
        &request.name,
        string_vec_to_vec_str(&request.tags).as_ref(),
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        string_vec_to_vec_str(&request.instructions).as_ref(),
    );

    result
        .map(|v| respond_json(RecipeResponse::from(v)))
        .map_err(|e| match e {
            CreateError::Unknown(m) => ApiError::Unknown(m),
            CreateError::InvalidData(m) => ApiError::InvalidData(m),
            CreateError::Conflict(m) => ApiError::Conflict(m),
        })?
}

#[cfg(test)]
mod tests {
    use actix_web::test::TestRequest;
    use actix_web::{test, web, App};

    use crate::tests::test_utils::shared::{
        stub_ingredients, stub_instructions, stub_recipe, stub_tags, RECIPE_NAME,
    };

    use super::CreateRecipeRequest;
    use super::*;

    #[actix_web::test]
    async fn should_create_a_recipe() {
        let create_req = CreateRecipeRequest {
            name: RECIPE_NAME.to_string(),
            tags: stub_tags(),
            ingredients: stub_ingredients(),
            instructions: stub_instructions(),
        };
        // init service
        let app = test::init_service(App::new().route("/", web::post().to(create_recipe))).await;
        // create request
        let req = TestRequest::post().set_json(create_req);
        // execute request
        let resp = test::call_and_read_body_json(&app, req.to_request()).await;
        // validate response
        assert_on_recipe_response(&resp, &stub_recipe());
    }

    fn assert_on_recipe_response(actual: &RecipeResponse, expected: &Recipe) {
        assert_eq!(&actual.name, expected.name().value())
    }
}
