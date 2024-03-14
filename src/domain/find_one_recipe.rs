use super::recipe::Recipe;

#[derive(Debug)]
pub enum FindOneError {
    Unknown(String),
    NotFound,
}

pub fn find_one_recipe<'a>(
    id: &'a str,
    name: &'a str,
    tags: &'a [&str],
    ingredients: &'a [&str],
    instructions: &'a [&str],
) -> Result<Recipe, FindOneError> {
    let tags = tags.iter().map(|i| i.to_string()).collect::<Vec<_>>();
    let ingredients = ingredients
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>();
    let instructions = instructions
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>();

    let recipe = Recipe::new(
        id.to_string(),
        name.to_string(),
        tags,
        ingredients,
        instructions,
    )
    .unwrap();

    Ok(recipe)
}

#[cfg(test)]
mod tests {
    use crate::helpers::string_vec_to_vec_str;

    use super::find_one_recipe;

    #[cfg(test)]
    mod tests {

        use super::*;
        use crate::tests::test_utils::shared::{
            assert_on_recipe, stub_ingredients, stub_instructions, stub_recipe, stub_tags,
            RECIPE_NAME,
        };

        #[test]
        fn should_create_the_expected_recipe() {
            let tags = stub_tags().iter().map(|i| i.to_string()).collect();
            let ingredients = stub_ingredients().iter().map(|i| i.to_string()).collect();
            let instructions = stub_instructions().iter().map(|i| i.to_string()).collect();

            let result = find_one_recipe(
                "10",
                RECIPE_NAME,
                &string_vec_to_vec_str(&tags),
                &string_vec_to_vec_str(&ingredients),
                &string_vec_to_vec_str(&instructions),
            );

            assert!(result.is_ok());
            let recipe = result.unwrap();
            assert_on_recipe(stub_recipe(), &recipe);
        }
    }
}
