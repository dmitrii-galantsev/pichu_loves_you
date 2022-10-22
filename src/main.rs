use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{ChannelId, EmojiId, ReactionType};
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // does message contain pichu?
        // no -> ignore it
        if !msg.content.to_lowercase().contains("pichu") {
            return;
        }

        let many_reacts_channel_ids = [
            ChannelId(198924507472199680),  // pichu-enthusiasts
            ChannelId(1033459352373313568), // testing_2
        ];

        // is this a pichu spam channel?
        // yes -> react with hearts
        if many_reacts_channel_ids.contains(&msg.channel_id) {
            // discord supports unicode reactions
            let heart_strings = [
                "❤️", "💘", "💓", "💕", "💖", "💗", "💙", "💚", "💛", "💜", "🖤", "💝", "💞", "💟",
                "❣️",
            ];
            // convert from unicode string to reaction
            let reactions = heart_strings.map(|x| ReactionType::Unicode(x.to_string()));
            // apply reactions
            for heart in reactions {
                // have to clone because async doesn't like references (?)
                if let Err(why) = msg.react(&ctx.http, heart.clone()).await {
                    println!("Could not react with {:?}: {:?}", heart, why);
                }
            }
        }

        // react to any message containing "pichu" substring
        // pichuYAY emote is from from Pichu Fanclub server
        let pichu = ReactionType::Custom {
            animated: false,
            id: EmojiId(308324956373254147),
            name: Some("pichuYAY".to_string()),
        };
        if let Err(why) = msg.react(&ctx.http, pichu).await {
            println!(
                "Could not react with pichuYAY: {:?}\nmessage:{:?}",
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

    // finally, start a single shard, and start listening to events.
    //
    // shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
