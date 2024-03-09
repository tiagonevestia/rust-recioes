use super::recipe::Recipe;

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

pub fn create_recipe<'a>(
    name: &'a str,
    tags: &'a Vec<&str>,
    ingredients: &'a Vec<&str>,
    instructions: &'a Vec<&str>,
) -> Result<Recipe, CreateError> {
    let tags = tags
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    let instructions = instructions
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    let recipe = Recipe::new(
        name.to_string(),
        tags.clone(),
        ingredients.clone(),
        instructions.clone(),
    )
    .map_err(|e| CreateError::InvalidData(e))?;

    Ok(recipe)
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::string_vec_to_vec_str,
        tests::test_utils::shared::{stub_ingredients, stub_instructions, stub_tags, RECIPE_NAME},
    };

    use super::create_recipe;

    #[test]
    fn should_create_the_expected_recipe() {
        let tags = stub_tags();
        let tags = string_vec_to_vec_str(&tags);
        let ingredients = stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);
        let instructions = stub_instructions();
        let instructions = string_vec_to_vec_str(&instructions);

        let recipe = create_recipe(RECIPE_NAME, &tags, &ingredients, &instructions).unwrap();

        assert_eq!(recipe.name().value(), RECIPE_NAME);
        assert_eq!(recipe.tags.value().len(), tags.len());
        assert_eq!(recipe.ingredients.value().len(), ingredients.len());
        assert_eq!(recipe.instructions.value().len(), instructions.len());
        for (i, exp_tags) in recipe.tags.value().iter().enumerate() {
            assert_eq!(exp_tags, &tags[i])
        }
        for (i, exp_ingredients) in recipe.ingredients.value().iter().enumerate() {
            assert_eq!(exp_ingredients, &ingredients[i])
        }
        for (i, exp_instructions) in recipe.instructions.value().iter().enumerate() {
            assert_eq!(exp_instructions, &instructions[i])
        }
    }
}
