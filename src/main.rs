use std::path::Path;
use std::{env, fs};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{EmojiId, ReactionType};
use serenity::prelude::*;

struct Handler;

#[derive(Debug, Deserialize, Serialize)]
struct Reaction {
    string: String,
    id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // TODO: do not read the config file on each message
        // read config file
        let file_path = Path::new("config.ron");
        let contents = fs::read_to_string(file_path).expect("Failed to read file");
        let reactions: Vec<Reaction> = ron::from_str(&contents).unwrap();

        for reaction in reactions {
            // see https://regex101.com/r/Egqt9E/1
            let re_string = format!("(\\W|^){}(\\W|$)", reaction.string);
            let re = Regex::new(&re_string).unwrap();

            if !re.is_match(&msg.content.to_lowercase()) {
                continue;
            }

            // react to any message containing substring
            let discord_reaction = ReactionType::Custom {
                animated: false,
                id: EmojiId(reaction.id),
                name: Some(reaction.string.to_string()),
            };
            if let Err(why) = msg.react(&ctx.http, discord_reaction).await {
                println!(
                    "Could not react with {:?}: {:?}\nmessage:{:?}",
                    reaction.string, why, msg
                );
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
