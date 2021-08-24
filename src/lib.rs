#![feature(proc_macro_hygiene, decl_macro)]

extern crate actix_http;
extern crate actix_rt;
extern crate actix_web;
extern crate futures;

mod routes;

pub use routes::server;

pub const SERVER: &str = "0.0.0.0:8009";

const DISCORD_TOKEN: &str = env!("DISCORD_TOKEN");
