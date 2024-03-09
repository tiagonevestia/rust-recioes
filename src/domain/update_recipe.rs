#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    NotFound,
    Conflict(String),
}

pub fn update_recipe() -> Result<(), UpdateError> {
    Ok(())
}
