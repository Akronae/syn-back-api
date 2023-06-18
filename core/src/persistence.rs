

use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::OnceCell;



static DB: OnceCell<Database> = OnceCell::new();

pub async fn get_db() -> Database {
    if DB.get().is_none() {
        let mut client_options = ClientOptions::parse("mongodb://192.168.1.26")
            .await
            .expect("Failed to parse MongoDB URI");
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options).expect("Failed to initialize client.");
        let db = client.database("syn-text-api");
        DB.set(db).expect("Failed to set DB");
    }

    return DB.get().unwrap().clone();
}
