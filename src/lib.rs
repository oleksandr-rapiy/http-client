use reqwest::{
    header::{HeaderMap, HeaderName},
    Response, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, future::Future};

pub mod todo;

pub trait Client {
    fn new(base_url: String) -> Self;

    fn set_header(&mut self, header_name: HeaderName, value: String) -> &mut Self;

    fn get<R>(&self, path: &str) -> impl Future<Output = Option<R>>
    where
        R: DeserializeOwned;

    fn post<T, R>(&self, path: &str, body: T) -> impl Future<Output = Option<R>>
    where
        T: Serialize,
        R: DeserializeOwned;

    fn put<T, R>(&self, path: &str, body: T) -> impl Future<Output = Option<R>>
    where
        T: Serialize,
        R: DeserializeOwned;

    fn delete<R>(&self, path: &str) -> impl Future<Output = Option<R>>
    where
        R: DeserializeOwned;
}

pub struct HttpClient {
    base_url: String,
    headers: HashMap<HeaderName, String>,
}

impl Client for HttpClient {
    fn new(base_url: String) -> HttpClient {
        HttpClient {
            base_url,
            headers: HashMap::new(),
        }
    }

    fn set_header(&mut self, header_name: HeaderName, value: String) -> &mut Self {
        self.headers.insert(header_name, value);

        return self;
    }

    async fn get<R>(&self, path: &str) -> Option<R>
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

        return self.parse_response::<R>(result.unwrap()).await;
    }

    async fn post<T, R>(&self, path: &str, body: T) -> Option<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let headers = self.build_headers();

        let client = reqwest::Client::new();
        let result = client.post(url).headers(headers).json(&body).send().await;

        if result.is_err() {
            return None;
        }

        return self.parse_response::<R>(result.unwrap()).await;
    }

    async fn put<T, R>(&self, path: &str, body: T) -> Option<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let headers = self.build_headers();

        let client = reqwest::Client::new();
        let result = client.put(url).headers(headers).json(&body).send().await;

        if result.is_err() {
            return None;
        }

        return self.parse_response::<R>(result.unwrap()).await;
    }

    async fn delete<R>(&self, path: &str) -> Option<R>
    where
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let headers = self.build_headers();

        let client = reqwest::Client::new();
        let result = client.delete(url).headers(headers).send().await;

        if result.is_err() {
            return None;
        }

        return self.parse_response::<R>(result.unwrap()).await;
    }
}

impl HttpClient {
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

    async fn parse_response<R>(&self, response: Response) -> Option<R>
    where
        R: DeserializeOwned,
    {
        let status_code = response.status();

        let body_str = response.text().await.unwrap();

        if status_code != StatusCode::from_u16(200).unwrap() {
            println!("Status code is {}", status_code);
            return None;
        }

        match serde_json::from_str::<R>(body_str.as_str()) {
            Ok(res) => return Some(res),
            Err(err) => {
                println!("Unable to parse, error = {}, body = {}", err, body_str);
                return None;
            }
        }
    }
}
