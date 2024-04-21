use std::env;

use log::info;
use mongodb::Database;

use mongodb::Client;
pub async fn create_db(db_name: &str) -> Database {
    let db_pass =
        env::var("MONGO_INITDB_ROOT_PASSWORD").expect("MONGO_INITDB_ROOT_PASSWORD must be set");
    let db_user_name =
        env::var("MONGO_INITDB_ROOT_USERNAME").expect("MONGO_INITDB_ROOT_USERNAME must be set");

    let uri = String::from(format!(
        "mongodb://{db_user_name}:{db_pass}@localhost:27017"
    ));
    info!("{uri}");
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let db = client.database(&db_name);
    db
}
