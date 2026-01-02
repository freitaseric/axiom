use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use std::env;

pub async fn connect() -> anyhow::Result<Database> {
    let uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;

    println!("💾 AXIOM: Conectado ao Setor de Memória (MongoDB).");

    Ok(client.database("axiom_db"))
}
