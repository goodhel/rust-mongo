use dotenv::dotenv;
use std::env;
use mongodb::{Client};

mod paths;
mod handler;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    // Load the MongoDB connection string from an environment variable:
   let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // Parse a connection string into an options struct.
    let client = Client::with_uri_str(client_uri.as_ref()).await?;
    // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let db = client.database("cobamongo");
    tide::log::start();
    let mut app =  tide::with_state(db.clone());
    let _res = paths::set(&mut app);

    // println!("Databases:");

    // for name in client.list_database_names(None, None).await? {
    //     println!("- {}", name);
    // }
    
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}