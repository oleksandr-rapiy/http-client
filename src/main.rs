use http_client::HttpClient;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    const JOKE_URL: &str = "https://icanhazdadjoke.com";

    let http_client = HttpClient::new(JOKE_URL.to_string());

    let http_client = http_client
        .set_header(ACCEPT, String::from("application/json"))
        .set_header(USER_AGENT, String::from("Rust http client"));

    let joke = http_client.get::<Joke>("/").await;

    if let Some(joke) = joke {
        println!("Joke: {}", joke.joke);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Joke {
    id: String,
    joke: String,
    status: i32,
}

