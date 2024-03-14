use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use config::parse_local_config;
use domain::recipe::Recipe;
use driven::repository::{mongo::recipe::RecipeMongoRepository, recipe::Repository};
use driving::rest_handler;

mod config;
mod domain;
mod driven;
mod driving;
mod helpers;
mod tests;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = parse_local_config();
    let repo = RecipeMongoRepository::new(&config.persistence).unwrap();

    create_server(repo)
        .await
        .unwrap()
        .await
        .expect("Ocorreu um erro ao iniciar o aplicativo da web");
}

async fn create_server<T: Repository<Recipe> + Send + Sync + 'static + Clone>(
    repo: T,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(repo.clone()))
            .configure(routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    Ok(server)
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::resource("recipes")
                    .route(web::get().to(rest_handler::recipes::find::find))
                    .route(
                        web::post()
                            .to(rest_handler::recipes::create::create::<RecipeMongoRepository>),
                    )
                    .route(web::patch().to(rest_handler::recipes::update::update)),
            )
            .service(
                web::resource("recipes/{id}")
                    .route(web::get().to(rest_handler::recipes::get_by_id::get_by_id))
                    .route(web::get().to(rest_handler::recipes::delete::delete)),
            ),
    );
}
