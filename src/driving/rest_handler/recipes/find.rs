use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_qs::actix::QsQuery;

use crate::{
    domain::{self, find_all_recipes::FindAllError, recipe::Recipe},
    driving::rest_handler::errors::ApiError,
    helpers::{respond_json, string_vec_to_vec_str},
};

use super::RecipeResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindRecipeRequest {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub instructions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RecipeListResponse {
    recipes: Vec<RecipeResponse>,
}

impl From<Vec<Recipe>> for RecipeListResponse {
    fn from(value: Vec<Recipe>) -> Self {
        let recipes = value.into_iter().map(RecipeResponse::from).collect();
        RecipeListResponse { recipes }
    }
}

pub async fn find(
    find_req: QsQuery<FindRecipeRequest>,
) -> Result<Json<RecipeListResponse>, ApiError> {
    let name = match &find_req.name {
        Some(n) => n.as_str(),
        None => "",
    };

    let tags = match &find_req.tags {
        Some(t) => string_vec_to_vec_str(t),
        None => vec![],
    };

    let ingredients = match &find_req.ingredients {
        Some(i) => string_vec_to_vec_str(i),
        None => vec![],
    };

    let instructions = match &find_req.instructions {
        Some(i) => string_vec_to_vec_str(i),
        None => vec![],
    };

    let result =
        domain::find_all_recipes::find_all_recipes(name, &tags, &ingredients, &instructions);

    result
        .map(|v| respond_json(RecipeListResponse::from(v)))
        .map_err(|e| match e {
            FindAllError::Unknown(m) => ApiError::Unknown(m),
        })?
}

#[cfg(test)]
mod tests {
    use actix_web::test::TestRequest;
    use actix_web::web;

    use crate::tests::test_utils::execute;

    use super::*;

    async fn should_find_all_recipes() {
        let resp: RecipeListResponse = execute(
            "/",
            None,
            web::get(),
            TestRequest::get(),
            find,
            None::<FindRecipeRequest>,
        )
        .await;

        assert_eq!(resp.recipes.len(), 0)
    }

    fn assert_on_recipe_response(actual: &RecipeResponse, expected: &Recipe) {
        assert_eq!(&actual.name, expected.name().value())
    }
}
