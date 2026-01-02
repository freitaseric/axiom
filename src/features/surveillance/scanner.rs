use crate::features::dungeon::models::Boss;
use crate::services::image_gen;
use mongodb::Database;
use mongodb::bson::doc;
use regex::Regex;
use std::sync::OnceLock;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_model::http::attachment::Attachment;

static LAUGH_REGEX: OnceLock<Regex> = OnceLock::new();

pub async fn scan(msg: &MessageCreate, http: &Client, db: &Database) -> anyhow::Result<()> {
    let regex = LAUGH_REGEX.get_or_init(|| Regex::new(r"(?i)(k{3,}|ha{2,}|rs{2,}|😂)").unwrap());

    if regex.is_match(&msg.content) {
        let collection = db.collection::<Boss>("bosses");
        let guild_id = msg.guild_id.unwrap().to_string();

        let filter = doc! { "guild_id": &guild_id, "active": true };
        let update = doc! { "$inc": { "hp": -10 } };

        let result = collection.find_one_and_update(filter, update).await?;

        if let Some(boss_doc) = result {
            let current_hp = boss_doc.hp - 10;

            let png_data = image_gen::generate_hp_bar(current_hp, boss_doc.max_hp, &boss_doc.name)?;

            let attachment = Attachment::from_bytes("status.png".to_string(), png_data, 1);

            http.create_message(msg.channel_id)
                .content("⚔️ **Ataque Confirmado!** (Dano: 10)")
                .attachments(&[attachment])
                .await
                .expect("Erro ao manda msg pro canal");
        } else {
            http.create_message(msg.channel_id)
                .content("erro")
                .await
                .expect("Erro ao manda msg pro canal");
        }
    }

    Ok(())
}
