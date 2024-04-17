use reqwest::{
    header::{HeaderMap, HeaderName},
    StatusCode,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct HttpClient {
    base_url: String,
    headers: HashMap<HeaderName, String>,
}

impl HttpClient {
    pub fn new(base_url: String) -> HttpClient {
        HttpClient {
            base_url,
            headers: HashMap::new(),
        }
    }

    pub fn set_header(mut self, header_name: HeaderName, value: String) -> Self {
        self.headers.insert(header_name, value);

        return self;
    }

    pub async fn get<R>(&self, path: &str) -> Option<R>
    where
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let headers = self.build_headers();

        let client = reqwest::Client::new();
        let result = client.get(url).headers(headers).send().await;

        if result.is_err() {
            return None;
        }

        let result = result.unwrap();
        let status_code = result.status();

        if status_code != StatusCode::from_u16(200).unwrap() {
            println!("Status code is {}", status_code);
            return None;
        }

        match result.json::<R>().await {
            Ok(res) => return Some(res),
            Err(err) => {
                println!("Unable to parse, error = {}", err);
                return None;
            }
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        for (k, v) in &self.headers {
            headers.insert(k, v.parse().unwrap());
        }

        headers
    }
}
