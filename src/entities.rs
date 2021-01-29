use super::*;
use serde_json::json;
use std::net::{Ipv4Addr, Ipv6Addr};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UploadBody {
    pub api_key: String,
    pub events: Vec<Event>,
    pub options: Option<ApiOptions>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[non_exhaustive]
pub(crate) struct ApiOptions {
    pub min_id_length: Option<u16>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct Event {
    pub event_type: String,
    pub user_id: Option<String>,
    pub device_id: Option<String>,
    pub time: Option<u64>,
    pub event_properties: Option<serde_json::Value>,
    pub user_properties: Option<serde_json::Value>,
    pub groups: Option<serde_json::Value>,
    pub app_version: Option<String>,
    pub platform: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_manufacturer: Option<String>,
    pub device_model: Option<String>,
    pub carrier: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub dma: Option<String>,
    pub language: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<u32>,
    pub revenue: Option<f64>,

    #[serde(rename = "productId")]
    pub product_id: Option<String>,

    #[serde(rename = "revenueType")]
    pub revenue_type: Option<String>,

    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub ip: Option<String>,
    pub idfa: Option<String>,
    pub idfv: Option<String>,
    pub adid: Option<String>,
    pub android_id: Option<String>,
    pub event_id: Option<i32>,
    pub session_id: Option<i64>,
    pub insert_id: Option<String>,
}

impl Event {
    pub fn new<S>(
        user_id: Option<S>,
        device_id: Option<S>,
        event_type: S,
    ) -> Result<Self, AmplitudeError>
    where
        S: Into<String>,
    {
        if user_id.is_none() && device_id.is_none() {
            return Err(AmplitudeError::InitializationError(
                "user_id or device_id must be provided".to_string(),
            ));
        }
        let user_id = user_id.map(|val| val.into());
        let device_id = device_id.map(|val| val.into());
        let mut event = Event::default();
        event.device_id = device_id;
        event.user_id = user_id;
        event.event_type = event_type.into();
        Ok(event)
    }

    pub fn from_json(val: serde_json::Value) -> Result<Self, AmplitudeError> {
        use serde_json::Value::Null;
        let user_id = &val["user_id"];
        let device_id = &val["device_id"];
        if user_id == &Null && device_id == &Null {
            return Err(AmplitudeError::InitializationError(
                "user_id or device_id must be provided".to_string(),
            ));
        }
        Ok(serde_json::from_value(val)?)
    }
}

impl Event {
    pub fn user_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.user_id = Some(val.into());
        self
    }

    pub fn device_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_id = Some(val.into());
        self
    }

    pub fn event_type<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.event_type = val.into();
        self
    }

    pub fn time(&mut self, val: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.time = Some(val.timestamp_millis() as u64);
        self
    }

    pub fn event_properties<T>(&mut self, val: T) -> &mut Self
    where
        T: Serialize,
    {
        self.event_properties = Some(json!(val));
        self
    }

    pub fn user_properties<T>(&mut self, val: T) -> &mut Self
    where
        T: Serialize,
    {
        self.user_properties = Some(json!(val));
        self
    }

    pub fn groups<T>(&mut self, val: T) -> &mut Self
    where
        T: Serialize,
    {
        self.groups = Some(json!(val));
        self
    }

    pub fn app_version<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.app_version = Some(val.into());
        self
    }

    pub fn platform<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.platform = Some(val.into());
        self
    }

    pub fn os_name<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.os_name = Some(val.into());
        self
    }

    pub fn os_version<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.os_version = Some(val.into());
        self
    }

    pub fn device_brand<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_brand = Some(val.into());
        self
    }

    pub fn device_manufacturer<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_manufacturer = Some(val.into());
        self
    }

    pub fn device_model<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_model = Some(val.into());
        self
    }

    pub fn carrier<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.carrier = Some(val.into());
        self
    }

    pub fn country<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.country = Some(val.into());
        self
    }

    pub fn region<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.region = Some(val.into());
        self
    }

    pub fn city<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.city = Some(val.into());
        self
    }

    pub fn dma<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.dma = Some(val.into());
        self
    }

    pub fn language<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.language = Some(val.into());
        self
    }

    pub fn price(&mut self, val: f64) -> &mut Self {
        self.price = Some(val);
        self
    }

    pub fn quantity(&mut self, val: u32) -> &mut Self {
        self.quantity = Some(val);
        self
    }

    pub fn revenue(&mut self, val: f64) -> &mut Self {
        self.revenue = Some(val);
        self
    }

    pub fn product_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.product_id = Some(val.into());
        self
    }

    pub fn revenue_type<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.revenue_type = Some(val.into());
        self
    }

    pub fn location_lat(&mut self, val: f64) -> &mut Self {
        self.location_lat = Some(val);
        self
    }

    pub fn location_lng(&mut self, val: f64) -> &mut Self {
        self.location_lng = Some(val);
        self
    }

    pub fn ip4(&mut self, val: Option<Ipv4Addr>) -> &mut Self {
        let ip = match val {
            None => String::from("$remote"),
            Some(v) => v.to_string(),
        };
        self.ip = Some(ip);
        self
    }

    pub fn ip6(&mut self, val: Option<Ipv6Addr>) -> &mut Self {
        let ip = match val {
            None => String::from("$remote"),
            Some(v) => v.to_string(),
        };
        self.ip = Some(ip);
        self
    }

    pub fn idfa<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.idfa = Some(val.into());
        self
    }

    pub fn idfv<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.idfv = Some(val.into());
        self
    }

    pub fn adid<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.adid = Some(val.into());
        self
    }

    pub fn android_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.android_id = Some(val.into());
        self
    }

    pub fn event_id(&mut self, val: i32) -> &mut Self {
        self.event_id = Some(val);
        self
    }

    pub fn session_id(&mut self, val: i64) -> &mut Self {
        self.session_id = Some(val);
        self
    }

    pub fn insert_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.insert_id = Some(val.into());
        self
    }
}
