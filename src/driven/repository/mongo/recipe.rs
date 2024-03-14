use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::error::Error;
use mongodb::{bson, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::driven::repository::recipe::{RepoDeleteError, RepoFindAllError, RepoUpdateError};
use crate::{
    config::PersistenceConfig,
    domain::recipe::Recipe,
    driven::repository::recipe::{FindRecipe, RepoCreateError, RepoSelectError, Repository},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeMongo {
    _id: ObjectId,
    name: String,
    tags: Vec<String>,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

impl From<Recipe> for RecipeMongo {
    fn from(recipe: Recipe) -> Self {
        let object_id = match recipe.id().value() {
            Some(id) => ObjectId::parse_str(id).unwrap(),
            None => ObjectId::new(),
        };

        let recipe_mongo = RecipeMongo {
            _id: object_id,
            name: recipe.name().value().to_string(),
            tags: recipe.tags().value().clone(),
            ingredients: recipe.ingredients().value().clone(),
            instructions: recipe.instructions().value().clone(),
        };

        recipe_mongo
    }
}

impl TryInto<Recipe> for RecipeMongo {
    type Error = String;

    fn try_into(self) -> Result<Recipe, Self::Error> {
        Recipe::new(
            self._id.to_string(),
            self.name,
            self.tags,
            self.ingredients,
            self.instructions,
        )
    }
}

#[derive(Clone)]
pub struct RecipeMongoRepository {
    database: String,
    collection: String,
    conn_uri: String,
}

impl RecipeMongoRepository {
    async fn open_connection(&self) -> Client {
        let c = Client::with_uri_str(&self.conn_uri);
        c.await
            .expect("Error while opening the connection with MongoDB")
    }

    async fn get_collection(&self) -> Collection<RecipeMongo> {
        let client = self.open_connection().await;
        client.database(&self.database).collection(&self.collection)
    }

    fn compose_document_from_recipe(&self, recipe: FindRecipe) -> Result<Document, Error> {
        if recipe.id.is_some() {
            let id = recipe.id.as_ref();
            let id_str = id.map(|id| id.to_string()).unwrap_or_default();
            Ok(doc! {
                "_id": bson::oid::ObjectId::parse_str(id_str).unwrap()
            })
        } else {
            let mut doc = doc! {};

            if recipe.name.as_ref().map_or(false, |s| !s.is_empty()) {
                doc.insert("name", recipe.name);
            }

            if recipe.tags.as_ref().map_or(true, Vec::is_empty) {
                doc.insert(
                    "tags",
                    doc! {
                        "$all": recipe.tags
                    },
                );
            }

            if recipe.ingredients.as_ref().map_or(true, Vec::is_empty) {
                doc.insert(
                    "ingredients",
                    doc! {
                        "$all": recipe.ingredients
                    },
                );
            }

            if recipe.instructions.as_ref().map_or(true, Vec::is_empty) {
                doc.insert(
                    "instructions",
                    doc! {
                        "$all": recipe.instructions
                    },
                );
            }

            println!("{:?}", doc);

            Ok(doc)
        }
    }
}

#[async_trait]
impl Repository<Recipe> for RecipeMongoRepository {
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized,
    {
        config.validate()?;
        let config = config.clone();
        let conn_uri = create_connection_uri(&config);

        Ok(RecipeMongoRepository {
            database: config.database,
            collection: config.schema_collection,
            conn_uri,
        })
    }

    async fn create(&self, recipe: Recipe) -> Result<Recipe, RepoCreateError> {
        let recipe_mongo = RecipeMongo::from(recipe.clone());
        let recipes_coll = self.get_collection().await;
        let result = recipes_coll.insert_one(recipe_mongo, None).await;
        let inserted_id = match result {
            Ok(e) => e.inserted_id.as_object_id().unwrap(),
            Err(e) => return Err(RepoCreateError::Unknown(e.to_string())),
        };

        let created = Recipe::new(
            inserted_id.to_string(),
            recipe.name().value().to_string(),
            recipe.tags().value().clone(),
            recipe.ingredients().value().clone(),
            recipe.instructions().value().clone(),
        )
        .unwrap();

        Ok(created)
    }

    async fn find_one(&self, recipe: FindRecipe) -> Result<Recipe, RepoSelectError> {
        let recipes_coll = self.get_collection().await;
        let document = self.compose_document_from_recipe(recipe).unwrap();
        let result: Result<Option<RecipeMongo>, Error> =
            recipes_coll.find_one(document, None).await;

        let found = match result {
            Ok(s) => match s {
                Some(s) => s,
                None => return Err(RepoSelectError::NotFound),
            },
            Err(_) => return Err(RepoSelectError::Unknown(String::from("unknown error"))),
        };

        let recipe: Result<Recipe, String> = found.try_into();
        println!("{:?}", recipe);
        Ok(recipe.unwrap())
    }

    async fn find_all(&self, recipe: FindRecipe) -> Result<Vec<Recipe>, RepoFindAllError> {
        todo!()
    }

    async fn update(&self, recipe: Recipe) -> Result<Recipe, RepoUpdateError> {
        todo!()
    }

    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError> {
        todo!()
    }
}

fn create_connection_uri(config: &PersistenceConfig) -> String {
    format!(
        "mongodb://{}:{}@{}/{}",
        config.user,
        config.password,
        match config.port {
            None => config.host.to_string(),
            Some(port) => config.host.clone() + ":" + &port.to_string(),
        },
        config.auth_db
    )
}
