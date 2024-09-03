#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use rocket_db_pools::Database;

#[launch]
async fn rocket() -> _ {
    let db_client = match db::MainDatabase::init().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {:?}", e);
            std::process::exit(1);
        }
    };

    rocket::build()
        .manage(db_client)
        .mount("/", routes![routes::get_recipes])
}