use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use rocket_db_pools::{mongodb, Database};

#[derive(Database)]
#[database("mongodb")]
pub struct MainDatabase(mongodb::Client);

impl MainDatabase {
    pub async fn init() -> mongodb::error::Result<mongodb::Client> {
        let uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");

        println!("Connecting to MongoDB at: {}", uri);

        let mut client_options = ClientOptions::parse(&uri).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        client_options.connect_timeout = Some(std::time::Duration::from_secs(30));
        client_options.server_selection_timeout = Some(std::time::Duration::from_secs(30));

        let client = mongodb::Client::with_options(client_options)?;

        client
            .database("sample_db")
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await?;

        println!("Connected to MongoDB successfully!");
        Ok(client)
    }
}