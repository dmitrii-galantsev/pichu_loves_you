use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{EmojiId, ReactionType};
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() != "gm" {
            return;
        }

        // react to any message containing "gm" substring
        // gm emote is from from GM Fanclub server
        // TODO: make this configurable
        let gm = ReactionType::Custom {
            animated: false,
            id: EmojiId(1195006722675323031),
            name: Some("gm".to_string()),
        };
        if let Err(why) = msg.react(&ctx.http, gm).await {
            println!(
                "Could not react with gm: {:?}\nmessage:{:?}",
                why, msg
            );
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // get DISCORD_TOKEN from ENV
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // start a single shard, and start listening to events.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
