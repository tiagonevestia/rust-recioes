#[derive(Debug)]
pub enum DeleteOneError {
    InvalidData(String),
    Unknown(String),
    NotFound,
}

pub fn delete_one_recipe(id: &str) -> Result<(), DeleteOneError> {
    Ok(())
}
