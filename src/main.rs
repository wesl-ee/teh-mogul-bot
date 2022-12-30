mod commands;
use dotenv::dotenv;
use serenity::model::prelude::command::Command;

use std::env;

use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::AttachmentType;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let res = match command.data.name.as_str() {
                "waifu" => {
                    command.defer(&ctx.http).await.unwrap();
                    let run_res = commands::waifu::run(&command.data.options)
                        .await;

                    match run_res {
                        Ok((file, seed)) =>
                            command
                            .create_followup_message(&ctx.http, |f| {
                                f.add_file(
                                    AttachmentType::Bytes { data: file.into(),
                                    filename: "out.png".to_string() });
                                f.content(format!("`seed:{}`", seed))
                            }).await,
                        Err(s) =>
                            command
                            .create_followup_message(&ctx.http, |f| {
                                f.content(format!("Error: {}", s))
                            })
                            .await
                    }
                }
                _ => {
                    command
                    .create_followup_message(&ctx.http, |f| {
                        f.content("Not implemented :(")
                    })
                    .await
                }
            };

            if let Err(why) = res {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::create_global_application_command(&ctx.http, |command| {
            commands::waifu::register(command)
        }).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
