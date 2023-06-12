pub mod definitions;
pub mod enums;
pub mod manifest;
pub mod schemas;
use crate::bungie::enums::{DestinyComponentType, DestinyRouteParam, PlatformErrorCodes};
use crate::env;
use crate::env::AppVariable;
use levelcrush::serde;
use levelcrush_macros::ExternalAPIResponse;
use reqwest::{Client, Method};
use std::collections::HashMap;

use levelcrush::tokio;
use levelcrush::tracing;

/// The generic response that bungie **normally** sends back in all of their api request.
#[ExternalAPIResponse]
pub struct BungieResponse<T> {
    #[serde(rename = "Response")]
    pub response: Option<T>,

    #[serde(rename = "ErrorCode")]
    pub error_code: i32,

    #[serde(rename = "ErrorStatus")]
    pub error_status: String,

    #[serde(rename = "ThrottleSeconds")]
    pub throttle_seconds: u64,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "MessageData")]
    pub message_data: HashMap<String, String>,
}

impl<T> BungieResponse<T> {
    /// checks to see if we are being throttled
    pub fn is_throttled(&self) -> bool {
        self.error_code != PlatformErrorCodes::Success as i32 && self.throttle_seconds > 0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum BungieRequestMethod {
    Get,
    Post,
}

impl BungieRequestMethod {
    fn to_reqwest(&self) -> Method {
        match self {
            BungieRequestMethod::Get => Method::GET,
            BungieRequestMethod::Post => Method::POST,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum BungieRequestBodyType {
    URLEncoded,
    JSON,
    None,
}

/// Wrapper around making life easier around request to the bungie api
pub struct BungieRequest<B: serde::Serialize> {
    endpoint: String,
    components: Vec<DestinyComponentType>,
    params: HashMap<DestinyRouteParam, String>,
    queries: HashMap<&'static str, String>,
    method: BungieRequestMethod,
    client: Client,
    retries: u16,
    retries_max: u16,
    body: Option<B>,
    body_type: BungieRequestBodyType,
}

impl<'r, B: serde::Serialize> BungieRequest<B> {
    pub fn new<T: Into<String>>(endpoint: T, method: BungieRequestMethod, client: Client) -> BungieRequest<B> {
        BungieRequest {
            endpoint: endpoint.into(),
            components: Vec::new(),
            params: HashMap::new(),
            queries: HashMap::new(),
            method,
            client,
            retries: 0,
            retries_max: 3,
            body: None,
            body_type: BungieRequestBodyType::None,
        }
    }

    /// directly set the body of the request based
    pub fn body(&mut self, new_body: Option<B>, new_body_type: BungieRequestBodyType) -> &mut Self {
        self.body = new_body;
        self.body_type = new_body_type;
        self
    }

    /// Adds a component to the request. not all request require components. Consult bungie documentation
    pub fn component(&mut self, component_type: DestinyComponentType) -> &mut Self {
        self.components.push(component_type);
        self
    }

    /// Add a route param value into the request.
    /// Route params are taken from the doc and when looking at routes they contain things like {mType} {membershipId}
    pub fn param<T: Into<String>>(&mut self, route_param: DestinyRouteParam, value: T) -> &mut Self {
        let value = value.into();
        self.params
            .entry(route_param)
            .and_modify(|v| *v = value.clone())
            .or_insert(value);
        self
    }

    /// inserts a query variable into the request
    pub fn query<T: Into<String>>(&mut self, key: &'static str, value: T) -> &mut Self {
        let value = value.into();

        self.queries
            .entry(key)
            .and_modify(|v| *v = value.clone())
            .or_insert(value);

        self
    }

    /// sends the request off to bungie.net . Does not handle retries. Only sends the request off once!
    pub async fn send_once<T: serde::de::DeserializeOwned>(&mut self) -> BungieResponse<T> {
        // parse things like {membershipId} {mType} {id} in our endpoint with something valid

        let mut endpoint = self.endpoint.clone();
        for (route_param, route_value) in self.params.iter() {
            let encoded_str = urlencoding::encode(route_value);
            endpoint = endpoint.replace(route_param.as_str(), &encoded_str);
        }

        // append components if there are any
        if !self.components.is_empty() {
            let components: Vec<&str> = self.components.iter().map(|v| v.as_str()).collect();
            let components = components.join(",");

            // add this into our query field
            self.queries
                .entry("components")
                .and_modify(|v| *v = components.clone())
                .or_insert(components);
        }

        if !self.queries.is_empty() {
            let mut queries = Vec::new();
            for (key, value) in self.queries.iter() {
                queries.push(format!("{}={}", key, urlencoding::encode(value.as_str())));
            }
            let queries = queries.join("&");
            endpoint = format!("{}?{}", endpoint, queries);
        }

        // construct final endpoint
        endpoint = format!("https://www.bungie.net/Platform{}", endpoint);
        tracing::info!("Requesting: {}", endpoint);

        let api_key = env::get(AppVariable::BungieAPIKey);
        let mut request = self
            .client
            .request(self.method.to_reqwest(), endpoint.as_str())
            .header("X-API-KEY", api_key)
            .header("Accept", "application/json");

        if self.body_type != BungieRequestBodyType::None {
            if let Some(body) = self.body.as_ref() {
                let body = match self.body_type {
                    BungieRequestBodyType::JSON => serde_json::to_string(body).unwrap_or_default(),
                    BungieRequestBodyType::URLEncoded => serde_urlencoded::to_string(body).unwrap_or_default(),
                    _ => "".to_string(), // this should not be possible , but include it just in case
                };

                let content_type = match self.body_type {
                    BungieRequestBodyType::JSON => "application/json",
                    BungieRequestBodyType::URLEncoded => "application/x-www-form-urlencoded",
                    _ => "text/plain",
                };

                // modify request and populate the body and our expected content type
                request = request.body(body).header("Content-Type", content_type);
            }
        }

        // send the request now that we are done building it
        let request = request.send().await;
        if let Ok(request) = request {
            let response = request.json::<BungieResponse<T>>().await;
            if let Ok(response) = response {
                response
            } else {
                let err = response.err().unwrap();
                tracing::error!("{}", err);
                BungieResponse {
                    response: None,
                    error_code: -1,
                    error_status: "Unable to parse the returned api response".to_string(),
                    message: "Unexpected format".to_string(),
                    message_data: HashMap::new(),
                    throttle_seconds: 0,
                }
            }
        } else {
            BungieResponse {
                response: None,
                error_code: -2,
                error_status: "No API Response".to_string(),
                message: "No API Response".to_string(),
                message_data: HashMap::new(),
                throttle_seconds: 0,
            }
        }
    }

    /// sends the request off to bungie.net and handles retries for throttles
    pub async fn send<T: serde::de::DeserializeOwned>(&mut self) -> BungieResponse<T> {
        // set our retries to 0 because every call of send(...) should be treated as a fresh request
        self.retries = 0;
        let mut response = self.send_once().await;

        // check if the response was throttled and if so count how many times we have retried.
        if response.is_throttled() && self.retries < self.retries_max {
            self.retries += 1;
            tracing::warn!(
                "Retrying request (Attempt {}): {}",
                (self.retries + 1),
                self.endpoint.as_str()
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(response.throttle_seconds + 1)).await;
            response = self.send_once().await;
        }
        response
    }
}

#[derive(Clone, Debug)]
pub struct BungieClient {
    http_client: Client,
}

impl BungieClient {
    /// creates a new http client configured for interacting with bungie services
    pub fn new() -> BungieClient {
        let http_client = reqwest::ClientBuilder::new()
            .build()
            .expect("Failed to initialize TLS or get system configuration");

        BungieClient { http_client }
    }

    /// shortcut method for calling a get request
    pub fn get(&self, endpoint: &str) -> BungieRequest<()> {
        BungieRequest::new(endpoint, BungieRequestMethod::Get, self.http_client.clone())
    }

    /// shortcut method for calling a post request
    pub fn post<B: serde::Serialize>(&self, endpoint: &str) -> BungieRequest<B> {
        BungieRequest::new(endpoint, BungieRequestMethod::Post, self.http_client.clone())
    }
}
