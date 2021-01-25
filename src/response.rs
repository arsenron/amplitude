use super::*;


type SerdeMap = serde_json::Map<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AmplitudeResponse {
    Ok(Ok),
    BadRequest(BadRequest),
    PayloadTooLarge(PayloadTooLarge),
    TooManyRequests(TooManyRequests),
    ServerError(ServerError),
    ServiceUnavailable(ServiceUnavailable),
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct Ok {
    code: Option<u16>,
    events_ingested: Option<u16>,
    payload_size_bytes: Option<u64>,
    server_upload_time: Option<u64>,
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct BadRequest {
    code: Option<u16>,
    error: Option<String>,
    missing_field: Option<String>,
    events_with_invalid_fields: Option<SerdeMap>,
    events_with_missing_fields: Option<SerdeMap>,
    events_with_invalid_id_lengths: Option<SerdeMap>,
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct PayloadTooLarge {
    code: Option<u16>,
    error: Option<String>,
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct TooManyRequests {
    code: Option<u16>,
    error: Option<String>,
    eps_threshold: Option<u32>,
    throttled_devices: Option<SerdeMap>,
    throttled_users: Option<SerdeMap>,
    throttled_events: Option<Vec<u32>>,
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct ServerError {
    value: Option<HashMap<String, serde_json::Value>>,  // any value, as it is unknown
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct ServiceUnavailable {
    value: Option<HashMap<String, serde_json::Value>>,  // any value, as it is unknown
}
