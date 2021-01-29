use reqwest::{Client, StatusCode};

use crate::entities::{ApiOptions, Event, UploadBody};
use crate::response::AmplitudeResponse;

use super::*;

pub struct Amp {
    api_key: String,
    client: Client,
    url: String,
    options: Option<ApiOptions>,
}

impl Amp {
    const URL_SINGLE: &'static str = "https://api2.amplitude.com/2/httpapi";
    const URL_BATCH: &'static str = "https://api2.amplitude.com/batch";
    const DEFAULT_SERVER_ERROR: &'static str = r#"{"error": "Some kind of server error"}"#;

    pub fn from_env() -> Result<Self, AmplitudeError> {
        let client = reqwest::Client::new();
        let amplitude_api_key = "AMPLITUDE_API_KEY";
        let api_key = std::env::var(amplitude_api_key);
        if let Ok(api_key) = api_key {
            Ok(Self {
                api_key,
                client,
                url: Self::URL_SINGLE.into(),
                options: None,
            })
        } else {
            let err = "No AMPLITUDE_API_KEY environment variable was found".to_string();
            Err(AmplitudeError::InitializationError(err))
        }
    }

    pub fn new<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        let api_key = api_key.into();
        let client = reqwest::Client::new();
        Self {
            api_key,
            client,
            url: Self::URL_SINGLE.into(),
            options: None,
        }
    }

    /// Sets HTTP API V2 (Single) url to send request to
    pub fn single(&mut self) -> &mut Self {
        self.url = Self::URL_SINGLE.into();
        self
    }

    /// Sets batch url
    pub fn batch(&mut self) -> &mut Self {
        self.url = Self::URL_BATCH.into();
        self
    }

    /// Sets minimum permitted length for user_id & device_id fields
    pub fn set_min_id_length(&mut self, length: u16) -> &mut Self {
        if self.options.is_none() {
            self.options = Some(ApiOptions::default())
        }
        self.options.as_mut().unwrap().min_id_length = Some(length);
        self
    }

    pub async fn send(&self, events: Vec<&Event>) -> Result<AmplitudeResponse, AmplitudeError> {
        let upload_body = UploadBody {
            api_key: self.api_key.clone(),
            events: events.into_iter().cloned().collect(),
            options: self.options.clone(),
        };
        // eprintln!("ser = {:#?}", serde_json::to_string(&upload_body)?);
        let response = self
            .client
            .post(&self.url)
            .json(&upload_body)
            .send()
            .await?;
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or(Self::DEFAULT_SERVER_ERROR.into());
        let amp_response = match status {
            StatusCode::OK => Self::add_tag("Ok", text),
            StatusCode::BAD_REQUEST => Self::add_tag("BadRequest", text),
            StatusCode::PAYLOAD_TOO_LARGE => Self::add_tag("PayloadTooLarge", text),
            StatusCode::TOO_MANY_REQUESTS => Self::add_tag("TooManyRequests", text),
            StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::GATEWAY_TIMEOUT => Self::add_tag("ServerError", text),
            StatusCode::SERVICE_UNAVAILABLE => Self::add_tag("ServiceUnavailable", text),
            _ => {
                // should be unreachable
                Self::add_tag("Ok", text)
            }
        };
        Ok(serde_json::from_str(&amp_response)?)
    }

    /// Adds enum variant's tag so serde can distinguish beetween enum variants
    /// when deserializing
    fn add_tag(tag: &str, text: String) -> String {
        format!(r#"{{"{tag}": {text}}}"#, tag = tag, text = text)
    }
}
