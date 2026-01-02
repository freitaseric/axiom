use crate::features::surveillance;
use mongodb::Database;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::event::Event;

pub async fn handle(event: Event, http: Arc<Client>, db: Database) -> anyhow::Result<()> {
    match event {
        Event::Ready(_) => {
            println!("✅ AXIOM: Conectado!");

            Ok(())
        }

        Event::MessageCreate(msg) => {
            if msg.author.bot {
                return Ok(());
            }

            Ok(surveillance::scanner::scan(&msg, &http, &db).await?)
        }
        _ => Ok(()),
    }
}
