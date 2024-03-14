use actix_web::web;

use crate::driven::repository::recipe::{RepoCreateError, Repository};

use super::{does_recipe_exist_by_name, recipe::Recipe};

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

pub async fn create_recipe<'a, T: Repository<Recipe>>(
    repository: web::Data<T>,
    name: &'a str,
    tags: &'a [&str],
    ingredients: &'a [&str],
    instructions: &'a [&str],
) -> Result<Recipe, CreateError> {
    let tags = tags.iter().map(|item| item.to_string()).collect::<Vec<_>>();

    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>();

    let instructions = instructions
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>();

    let recipe = Recipe::new(
        String::from(""),
        name.to_string(),
        tags.clone(),
        ingredients.clone(),
        instructions.clone(),
    )
    .map_err(CreateError::InvalidData)?;

    if does_recipe_exist_by_name(&repository, name).await {
        return Err(CreateError::Conflict(String::from(
            "Uma receita com esse nome já está presente",
        )));
    }

    repository.create(recipe).await.map_err(|e| {
        return match e {
            RepoCreateError::InvalidData(e) => {
                CreateError::InvalidData(format!("Invalid data: {}", e))
            }
            RepoCreateError::Unknown(e) => CreateError::Unknown(format!("Unknown error: {}", e)),
        };
    })
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         helpers::string_vec_to_vec_str,
//         tests::test_utils::shared::{stub_ingredients, stub_instructions, stub_tags, RECIPE_NAME},
//     };

//     use super::create_recipe;

//     #[test]
//     fn should_create_the_expected_recipe() {
//         let tags = stub_tags();
//         let tags = string_vec_to_vec_str(&tags);
//         let ingredients = stub_ingredients();
//         let ingredients = string_vec_to_vec_str(&ingredients);
//         let instructions = stub_instructions();
//         let instructions = string_vec_to_vec_str(&instructions);

//         let recipe = create_recipe(RECIPE_NAME, &tags, &ingredients, &instructions).unwrap();

//         assert_eq!(recipe.name().value(), RECIPE_NAME);
//         assert_eq!(recipe.tags.value().len(), tags.len());
//         assert_eq!(recipe.ingredients.value().len(), ingredients.len());
//         assert_eq!(recipe.instructions.value().len(), instructions.len());
//         for (i, exp_tags) in recipe.tags.value().iter().enumerate() {
//             assert_eq!(exp_tags, &tags[i])
//         }
//         for (i, exp_ingredients) in recipe.ingredients.value().iter().enumerate() {
//             assert_eq!(exp_ingredients, &ingredients[i])
//         }
//         for (i, exp_instructions) in recipe.instructions.value().iter().enumerate() {
//             assert_eq!(exp_instructions, &instructions[i])
//         }
//     }
// }
