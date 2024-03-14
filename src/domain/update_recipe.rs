use super::recipe::Recipe;

#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    NotFound,
    Conflict(String),
}

pub fn update_recipe<'a>(
    id: &'a str,
    name: &'a str,
    tags: &'a [&str],
    ingredients: &'a [&str],
    instructions: &'a [&str],
) -> Result<Recipe, UpdateError> {
    if id.is_empty() {
        return Err(UpdateError::InvalidData(String::from(
            "Cannot update without a target id",
        )));
    }

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
        tags.clone(),
        ingredients.clone(),
        instructions.clone(),
    )
    .unwrap();

    Ok(recipe)
}
