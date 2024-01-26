use std::env;
use std::fs;

use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{ChannelId, EmojiId, ReactionType};
use serenity::prelude::*;

#[derive(Deserialize)]
struct Pichu {
    emojis: Vec<String>,
    channels: Vec<u64>,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // does message contain pichu?
        // no -> ignore it
        if !msg.content.to_lowercase().contains("pichu") {
            return;
        }

        // read config from file
        // TODO: do not read file on each message
        let config_path = "pichu.toml";
        let config_string = fs::read_to_string(config_path).expect("Could not read the file!");
        let config_parsed: Pichu = toml::from_str(&config_string).expect("Could not parse toml!");
        println!("emojis: {:?}", config_parsed.emojis);
        println!("channels: {:?}", config_parsed.channels);

        let many_reacts_channel_ids: Vec<ChannelId> = config_parsed
            .channels
            .into_iter()
            .map(ChannelId)
            .collect();

        // is this a pichu spam channel?
        // yes -> react with hearts
        if many_reacts_channel_ids.contains(&msg.channel_id) {
            // read reactions from config
            let heart_strings = config_parsed.emojis.into_iter();
            // convert from unicode string to reaction
            let reactions = heart_strings.map(ReactionType::Unicode);
            // apply reactions
            for heart in reactions {
                // have to clone because async doesn't like references (?)
                if let Err(why) = msg.react(&ctx.http, heart.clone()).await {
                    println!("Could not react with {:?}: {:?}", heart, why);
                }
            }
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
