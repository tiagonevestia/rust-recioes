#[derive(Debug)]
pub enum FindOneError {
    Unknown(String),
    NotFound,
}

pub fn find_one_recipe<'a>(id: &'a str) -> Result<(), FindOneError> {
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_create_the_expected_recipe() {}
}
