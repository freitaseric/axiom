mod core;
mod features;
mod services;

use crate::core::database;
use dotenvy::dotenv;
use mongodb::bson::doc;
use std::env;
use std::sync::Arc;
use twilight_gateway::{EventTypeFlags, Shard, StreamExt};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{Intents, ShardId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = database::connect().await?;
    let bosses = db.collection::<features::dungeon::models::Boss>("bosses");
    bosses.delete_many(doc! {}).await?;
    if bosses.count_documents(doc! {}).await? == 0 {
        bosses
            .insert_one(features::dungeon::models::Boss {
                id: None,
                guild_id: "1382177395015946270".to_string(),
                name: "Dummy Target".to_string(),
                hp: 100,
                max_hp: 100,
                active: true,
                thread_id: None,
            })
            .await?;
        println!("🎯 Boss de Treino criado!");
    }

    let token = env::var("DISCORD_TOKEN").expect("faltou o DISCORD_TOKEN no .env");

    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILDS;

    let http = Arc::new(HttpClient::new(token.clone()));

    let mut shard = Shard::new(ShardId::ONE, token, intents);

    println!("👁️  AXIOM: Sistema Inicializando...");

    loop {
        let event = if let Some(event_result) = shard.next_event(EventTypeFlags::all()).await {
            match event_result {
                Ok(event) => event,
                Err(why) => {
                    tracing::warn!(?why, "Erro ao receber o evento");
                    continue;
                }
            }
        } else {
            continue;
        };

        tokio::spawn(core::events::handle(event, http.clone(), db.clone()));
    }
}
