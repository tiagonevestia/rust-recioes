use uuid::Uuid;

#[derive(Debug)]
pub enum DeleteOneError {
    Unknown(String),
}

fn delete_one_recipe(id: Uuid) -> Result<(), DeleteOneError> {
    Ok(())
}
