use super::*;

use reqwest::{Client, StatusCode};
use crate::entities::{Event, UploadBody};
use crate::response::AmplitudeResponse;


pub struct Amp {
    api_key: String,
    client: Client,
}


impl Amp {
    const URL_SINGLE: &'static str = "https://api2.amplitude.com/2/httpapi";
    const URL_BATCH: &'static str = "https://api2.amplitude.com/batch";

    pub fn from_env() -> Self {
        let client = reqwest::Client::new();
        let amplitude_api_key = "AMPLITUDE_API_KEY";
        Self {
            api_key: std::env::var(amplitude_api_key)
                .unwrap_or_else(
                    |_| panic!("Cannot get the {} env variable", amplitude_api_key)
                ),
            client,
        }
    }

    pub fn new<S>(api_key: S) -> Self
        where S: Into<String>
    {
        let api_key = api_key.into();
        let client = reqwest::Client::new();
        Self {
            api_key,
            client,
        }
    }

    pub async fn send(&self, event: &Event) -> Result<AmplitudeResponse, reqwest::Error> {
        let default_server_error = String::from(r#"{"error": "Some kind of server error"}"#);
        let upload_body = UploadBody {
            api_key: self.api_key.clone(),
            events: vec![event.clone()]
        };
        let response = self.client
            .post(Self::URL_SINGLE)
            .json(&upload_body)
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await.unwrap_or(default_server_error);
        let amp_response = match status {
            StatusCode::OK => {
                Self::add_tag("Ok", text)
            },
            StatusCode::BAD_REQUEST => {
                Self::add_tag("BadRequest", text)
            },
            StatusCode::PAYLOAD_TOO_LARGE => {
                Self::add_tag("PayloadTooLarge", text)
            },
            StatusCode::TOO_MANY_REQUESTS => {
                Self::add_tag("TooManyRequests", text)
            },
            StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::BAD_GATEWAY
                | StatusCode::GATEWAY_TIMEOUT =>
            {
                Self::add_tag("ServerError", text)
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                Self::add_tag("ServiceUnavailable", text)
            },
            _ => {
                Self::add_tag("Ok", text)
            }
        };
        eprintln!("amp_response = {:#?}", amp_response);
        Ok(serde_json::from_str(&amp_response).unwrap())
    }

    /// Adds enum variant tag so serde can distinguish beetween enum variants
    /// # Examples
    /// ```
    /// let json = r#"{"field": "value"}"#;
    /// let tagged_json = r#"{"EnumVariant": {"field": "value"}"#;
    /// assert_eq!(add_tag("EnumVariant", json), tagged_json);
    /// ```
    fn add_tag(tag: &str, text: String) -> String {
         format!(r#"{{"{tag}": {text}}}"#, tag=tag, text=text)
    }
}


