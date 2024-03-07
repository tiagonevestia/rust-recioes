use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    id: Option<Uuid>,
    name: String,
    tags: Vec<String>,
    ingredients: Vec<String>,
    instructions: Vec<String>,
    published_at: Option<DateTime<Local>>,
}

impl Recipe {
    pub fn new(
        name: String,
        tags: Vec<String>,
        ingredients: Vec<String>,
        instructions: Vec<String>,
    ) -> Result<Self, String> {
        if name.is_empty() {
            return Err("A receita precisa ter um nome".to_string());
        }

        if tags.is_empty() {
            return Err("A receita precisa pelo menos de uma tag".to_string());
        }

        if ingredients.is_empty() {
            return Err("A receita precisa pelo menos de um ingrediente".to_string());
        }

        if instructions.is_empty() {
            return Err("A receita precisa pelo menos de uma instrução".to_string());
        }

        Ok(Recipe {
            id: Some(Uuid::now_v7()),
            name,
            ingredients,
            instructions,
            tags,
            published_at: Some(Local::now()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_expected_recipe() {
        let tags = vec!["main".to_string(), "chicken".to_string()];
        let ingredients = vec!["4 (6 to 7-ounce) boneless skinless chicken breasts\r".to_string()];
        let instructions = vec!["To marinate the chicken: In a non-reactive dish, combine the lemon juice, olive oil, oregano, salt, and pepper and mix together".to_string()];
        let name = "Oregano Marinated Chicken";

        let new_recipe = Recipe::new(
            name.to_string(),
            tags.clone(),
            ingredients.clone(),
            instructions.clone(),
        )
        .unwrap();

        assert_eq!(new_recipe.name, name);
        assert_eq!(ingredients.len(), new_recipe.ingredients.len());
        for (i, exp_ins) in instructions.into_iter().enumerate() {
            assert_eq!(exp_ins, new_recipe.instructions[i])
        }
    }

    #[test]
    fn should_fail_without_a_name_or_ingredients_or_tags_or_instructions() {
        let tags = vec!["main".to_string(), "chicken".to_string()];
        let ingredients = vec!["4 (6 to 7-ounce) boneless skinless chicken breasts\r".to_string()];
        let instructions = vec!["To marinate the chicken: In a non-reactive dish, combine the lemon juice, olive oil, oregano, salt, and pepper and mix together".to_string()];
        let name = "Oregano Marinated Chicken";

        let err_recipe = Recipe::new(
            "".to_string(),
            tags.clone(),
            ingredients.clone(),
            instructions.clone(),
        );
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(err_recipe.unwrap_err(), "A receita precisa ter um nome");

        let err_recipe = Recipe::new(
            name.to_string(),
            vec![],
            ingredients.clone(),
            instructions.clone(),
        );
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(
            err_recipe.unwrap_err(),
            "A receita precisa pelo menos de uma tag"
        );

        let err_recipe = Recipe::new(name.to_string(), tags.clone(), vec![], instructions.clone());
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(
            err_recipe.unwrap_err(),
            "A receita precisa pelo menos de um ingrediente"
        );

        let err_recipe = Recipe::new(name.to_string(), tags.clone(), ingredients.clone(), vec![]);
        assert_eq!(err_recipe.is_err(), true);
        assert_eq!(
            err_recipe.unwrap_err(),
            "A receita precisa pelo menos de uma instrução"
        );
    }
}
