#[cfg(test)]
pub mod shared {
    use crate::domain::recipe::Recipe;

    pub const RECIPE_NAME: &str = "Oregano Marinated Chicken";

    //
    // ASSERTION HELPERS
    //

    pub fn assert_on_recipe(expected: Recipe, actual: &Recipe) {
        assert_eq!(actual.name().value(), expected.name().value());
        assert_on_vec(expected.tags().value(), actual.tags().value());
        assert_on_vec(expected.ingredients().value(), actual.ingredients().value());
        assert_on_vec(
            expected.instructions().value(),
            actual.instructions().value(),
        );
    }

    pub fn assert_on_vec(expected: &Vec<String>, actual: &Vec<String>) {
        assert_eq!(expected.len(), actual.len());

        for (i, exp_vec) in expected.iter().enumerate() {
            assert_eq!(exp_vec, &actual[i]);
        }
    }

    //
    // STUBBING HELPERS
    //

    pub fn stub_recipe() -> Recipe {
        let recipe_name = RECIPE_NAME;

        let oregano_marinated_chicken = Recipe::new(
            recipe_name.to_string(),
            stub_tags(),
            stub_ingredients(),
            stub_instructions(),
        )
        .unwrap();

        oregano_marinated_chicken
    }

    pub fn stub_tags() -> Vec<String> {
        vec!["main".to_string(), "chicken".to_string()]
    }

    pub fn stub_instructions() -> Vec<String> {
        vec![
            "To marinate the chicken: In a non-reactive dish, combine the lemon juice, olive oil, oregano, salt, and pepper and mix together".to_string(),
            "Add the chicken breasts to the dish and rub both sides in the mixture".to_string()
        ]
    }

    pub fn stub_ingredients() -> Vec<String> {
        vec![
            "4 (6 to 7-ounce) boneless skinless chicken breasts\r".to_string(),
            "10 grinds black pepper\r".to_string(),
        ]
    }
}
