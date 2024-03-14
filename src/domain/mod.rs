use actix_web::web;

use crate::driven::repository::recipe::{FindRecipe, Repository};

use self::recipe::Recipe;

pub mod recipe;
// use case
pub mod create_recipe;
pub mod delete_one_recipe;
pub mod find_all_recipes;
pub mod find_one_recipe;
pub mod update_recipe;

pub trait Entity {}

async fn does_recipe_exist<'a, T: Repository<Recipe>>(repository: &web::Data<T>, id: &str) -> bool {
    let r = FindRecipe {
        id: Some(String::from(id)),
        name: None,
        tags: None,
        ingredients: None,
        instructions: None,
    };

    repository.find_one(r).await.is_ok()
}

async fn does_recipe_exist_by_name<'a, T: Repository<Recipe>>(
    repository: &web::Data<T>,
    name: &str,
) -> bool {
    let s = FindRecipe {
        id: None,
        name: Some(String::from(name)),
        tags: None,
        ingredients: None,
        instructions: None,
    };

    println!("{:?}", s);

    repository.find_one(s).await.is_ok()
}
