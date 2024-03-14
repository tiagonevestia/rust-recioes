use actix_web::web::{self, Json};

use crate::{
    domain::{self, create_recipe::CreateError, recipe::Recipe},
    driven::repository::recipe::Repository,
    driving::rest_handler::{errors::ApiError, validate::validate},
    helpers::{respond_json, string_vec_to_vec_str},
};

use super::{CreateRecipeRequest, RecipeResponse};

pub async fn create<T: Repository<Recipe>>(
    repository: web::Data<T>,
    request: Json<CreateRecipeRequest>,
) -> Result<Json<RecipeResponse>, ApiError> {
    validate(&request)?;

    let result = domain::create_recipe::create_recipe(
        repository,
        &request.name,
        string_vec_to_vec_str(&request.tags).as_ref(),
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        string_vec_to_vec_str(&request.instructions).as_ref(),
    )
    .await;

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
    use actix_web::web;

    use crate::tests::test_utils::execute;
    use crate::tests::test_utils::shared::{
        stub_ingredients, stub_instructions, stub_recipe, stub_tags, RECIPE_NAME,
    };

    use self::domain::recipe::Recipe;

    use super::CreateRecipeRequest;
    use super::*;

    // #[actix_web::test]
    // async fn should_create_a_recipe() {
    //     let create_req = CreateRecipeRequest {
    //         name: RECIPE_NAME.to_string(),
    //         tags: stub_tags(),
    //         ingredients: stub_ingredients(),
    //         instructions: stub_instructions(),
    //     };

    //     let resp = execute(
    //         "/",
    //         None,
    //         web::post(),
    //         TestRequest::post(),
    //         create,
    //         Some(create_req),
    //     )
    //     .await;

    //     // validate response
    //     assert_on_recipe_response(&resp, &stub_recipe());
    // }

    fn assert_on_recipe_response(actual: &RecipeResponse, expected: &Recipe) {
        assert_eq!(&actual.name, expected.name().value())
    }
}
