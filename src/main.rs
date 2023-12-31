#[macro_use] extern crate rocket;

use std::env;
use dotenv::dotenv;
use redis::Commands;

#[get("/")]
fn index() -> &'static str {
    "Use this API to send messages to a redis pub/sub channel: GET publish/<channel>/<message>"
}

#[get("/publish/<channel>/<message>")]
fn publish(channel : &str,message : &str) {
    let client: redis::Client = redis::Client::open(redis_url()).unwrap();
    let mut conn: redis::Connection = client.get_connection().unwrap();

    let _: () = conn.publish(channel, message).unwrap();
}

fn redis_url() -> String {
    env::var("REDIS").unwrap_or_default()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index,publish])
}