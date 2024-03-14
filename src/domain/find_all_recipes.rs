use super::recipe::Recipe;

#[derive(Debug)]
pub enum FindAllError {
    Unknown(String),
}

pub fn find_all_recipes<'a>(
    name: &'a str,
    tags: &'a [&str],
    ingredients: &'a [&str],
    instructions: &'a [&str],
) -> Result<Vec<Recipe>, FindAllError> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_all_recipes() {
        // let recipe_list = find_all_recipes().unwrap();

        // assert_eq!(recipe_list.len(), 0)
    }
}
