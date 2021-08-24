use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn get_users_from_voice(ctx: &Context) {
    let channel = ctx.http.get_channel(834776536489656345).await.unwrap();

    let channel = channel.guild().unwrap();

    let members = channel.members;
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
