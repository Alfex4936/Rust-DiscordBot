use discordbot::DISCORD_TOKEN;

use actix_web::{middleware, web, App, HttpServer};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Client, Context, EventHandler},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "`test" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = Client::builder(DISCORD_TOKEN)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let (_, _) = futures::future::join(
        client.start(),
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .service(discordbot::server::hello)
        })
        .bind(discordbot::SERVER)
        .expect("Error binding")
        .run(),
    )
    .await;

    Ok(())
}
