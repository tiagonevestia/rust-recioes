use actix_web::{web, HttpResponse};

use crate::{
    domain::{self, delete_one_recipe::DeleteOneError},
    driving::rest_handler::errors::ApiError,
};

pub async fn delete(path: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let recipe_id = path.into_inner();

    let result = domain::delete_one_recipe::delete_one_recipe(&recipe_id.to_string());

    result
        .map(|_| Ok(HttpResponse::Ok().finish()))
        .map_err(|e| match e {
            DeleteOneError::Unknown(m) => ApiError::Unknown(m),
            DeleteOneError::InvalidData(m) => ApiError::BadRequest(m),
            DeleteOneError::NotFound => ApiError::NotFound(String::from(
                "Nenhuma receita para excluir correspondente ao id recebido",
            )),
        })?
}
