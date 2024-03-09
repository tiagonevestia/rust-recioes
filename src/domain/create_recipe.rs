use super::recipe::Recipe;

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
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
    use super::create_recipe;

    #[test]
    fn should_create_the_expected_recipe() {
        let tags = vec!["main", "chicken"];
        let ingredients = vec!["10 grinds black pepper\r"];
        let instructions = vec!["Cover the dish with plastic wrap and let marinate in the refrigerator for at least 30 minutes and up to 4 hours"];

        let recipe = create_recipe(
            "Oregano Marinated Chicken",
            &tags,
            &ingredients,
            &instructions,
        )
        .unwrap();

        assert_eq!(recipe.name().value(), "Oregano Marinated Chicken");
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
