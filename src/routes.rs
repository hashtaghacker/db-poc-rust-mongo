use mongodb::Cursor;
use rocket::futures::TryStreamExt;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::MainDatabase, models::Recipe};

#[get("/test", format = "json")]
pub async fn get_recipes(db: Connection<MainDatabase>) -> Json<Vec<Recipe>> {
    let recipes: Cursor<Recipe> = db
        .database("sample_db")
        .collection("samples")
        .find(None, None)
        .await
        .expect("Failed to retrieve recipes");

    Json(recipes.try_collect().await.unwrap())
}
