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

/// Sets additional API options
///
/// [The official docs](https://developers.amplitude.com/docs/http-api-v2#schemaRequestOptions)
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[non_exhaustive]
pub(crate) struct ApiOptions {
    pub min_id_length: Option<u16>,
}

/// The main entity to send to the amplitude servers
///
/// [The official docs](https://developers.amplitude.com/docs/http-api-v2#schemaevent)
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct Event {
    event_type: Option<String>,
    user_id: Option<String>,
    device_id: Option<String>,
    time: Option<u64>,
    event_properties: Option<serde_json::Value>,
    user_properties: Option<serde_json::Value>,
    groups: Option<serde_json::Value>,
    app_version: Option<String>,
    platform: Option<String>,
    os_name: Option<String>,
    os_version: Option<String>,
    device_brand: Option<String>,
    device_manufacturer: Option<String>,
    device_model: Option<String>,
    carrier: Option<String>,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
    dma: Option<String>,
    language: Option<String>,
    price: Option<f64>,
    quantity: Option<u32>,
    revenue: Option<f64>,
    #[serde(rename = "productId")]
    product_id: Option<String>,

    #[serde(rename = "revenueType")]
    revenue_type: Option<String>,

    location_lat: Option<f64>,
    location_lng: Option<f64>,
    ip: Option<String>,
    idfa: Option<String>,
    idfv: Option<String>,
    adid: Option<String>,
    android_id: Option<String>,
    event_id: Option<i32>,
    session_id: Option<i64>,
    insert_id: Option<String>,
}

impl Event {
    /// Creates a new empty event
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an event from [json value](https://docs.rs/serde_json/1.0.61/serde_json/value/enum.Value.html)
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
    /// A readable ID specified by you. Must have a minimum length of 5 characters.
    /// Required unless device_id is present.
    pub fn user_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.user_id = Some(val.into());
        self
    }

    /// A device-specific identifier, such as the Identifier for Vendor on iOS. Required unless user_id is present.
    /// If a device_id is not sent with the event, it will be set to a hashed version of the user_id.
    pub fn device_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_id = Some(val.into());
        self
    }

    /// A unique identifier for your event.
    pub fn event_type<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.event_type = Some(val.into());
        self
    }

    /// The timestamp of the event (DateTime converts to milliseconds since epoch).
    /// If time is not sent with the event, it will be set to the request upload time.
    pub fn time(&mut self, val: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.time = Some(val.timestamp_millis() as u64);
        self
    }

    /// [A dictionary of key-value pairs](https://docs.rs/serde_json/1.0.61/serde_json/value/enum.Value.html)
    /// that represent additional data to be sent along with the event.
    /// You can store property values in an array. Date values are transformed into string values.
    /// Object depth may not exceed 40 layers.
    pub fn event_properties<T>(&mut self, val: T) -> &mut Self
    where
        T: Serialize,
    {
        self.event_properties = Some(json!(val));
        self
    }

    /// [A dictionary of key-value pairs](https://docs.rs/serde_json/1.0.61/serde_json/value/enum.Value.html)
    /// that represent additional data tied to the user.
    /// You can store property values in an array.
    /// Date values are transformed into string values. Object depth may not exceed 40 layers.
    pub fn user_properties<T>(&mut self, val: T) -> &mut Self
    where
        T: Serialize,
    {
        self.user_properties = Some(json!(val));
        self
    }

    /// This feature is only available to Enterprise customers who have purchased the Accounts add-on. This field adds
    /// [a dictionary of key-value pairs](https://docs.rs/serde_json/1.0.61/serde_json/value/enum.Value.html)
    /// that represent groups of users
    /// to the event as an event-level group. You can only track up to 5 groups.
    pub fn groups<T>(&mut self, val: T) -> &mut Self
    where
        T: Serialize,
    {
        self.groups = Some(json!(val));
        self
    }

    /// The current version of your application.
    pub fn app_version<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.app_version = Some(val.into());
        self
    }

    /// Platform of the device.
    pub fn platform<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.platform = Some(val.into());
        self
    }

    /// The name of the mobile operating system or browser that the user is using.
    pub fn os_name<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.os_name = Some(val.into());
        self
    }

    /// The version of the mobile operating system or browser the user is using.
    pub fn os_version<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.os_version = Some(val.into());
        self
    }

    /// The device brand that the user is using.
    pub fn device_brand<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_brand = Some(val.into());
        self
    }

    /// The device manufacturer that the user is using.
    pub fn device_manufacturer<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_manufacturer = Some(val.into());
        self
    }

    /// The device model that the user is using.
    pub fn device_model<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.device_model = Some(val.into());
        self
    }

    /// The carrier that the user is using.
    pub fn carrier<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.carrier = Some(val.into());
        self
    }

    /// The current country of the user.
    pub fn country<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.country = Some(val.into());
        self
    }

    /// The current region of the user.
    pub fn region<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.region = Some(val.into());
        self
    }

    /// The current city of the user.
    pub fn city<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.city = Some(val.into());
        self
    }

    /// The current Designated Market Area of the user.
    pub fn dma<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.dma = Some(val.into());
        self
    }

    /// The language set by the user.
    pub fn language<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.language = Some(val.into());
        self
    }

    /// The price of the item purchased. Required for revenue data if the revenue field is not sent.
    /// You can use negative values to indicate refunds.
    pub fn price(&mut self, val: f64) -> &mut Self {
        self.price = Some(val);
        self
    }

    /// The quantity of the item purchased. Defaults to 1 if not specified.
    pub fn quantity(&mut self, val: u32) -> &mut Self {
        self.quantity = Some(val);
        self
    }

    /// revenue = price quantity. If you send all 3 fields of price, quantity, and revenue,
    /// then (price quantity) will be used as the revenue value.
    /// You can use negative values to indicate refunds.
    pub fn revenue(&mut self, val: f64) -> &mut Self {
        self.revenue = Some(val);
        self
    }

    /// An identifier for the item purchased.
    /// You must send a price and quantity or revenue with this field.
    pub fn product_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.product_id = Some(val.into());
        self
    }

    /// The type of revenue for the item purchased.
    /// You must send a price and quantity or revenue with this field.
    pub fn revenue_type<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.revenue_type = Some(val.into());
        self
    }

    /// The current Latitude of the user.
    pub fn location_lat(&mut self, val: f64) -> &mut Self {
        self.location_lat = Some(val);
        self
    }

    /// The current Longitude of the user.
    pub fn location_lng(&mut self, val: f64) -> &mut Self {
        self.location_lng = Some(val);
        self
    }

    /// Sets ip4 address on the event. If None, sets "$remote"
    pub fn ip4(&mut self, val: Option<Ipv4Addr>) -> &mut Self {
        let ip = match val {
            None => String::from("$remote"),
            Some(v) => v.to_string(),
        };
        self.ip = Some(ip);
        self
    }

    /// Sets ip6 address on the event. If None, sets "$remote"
    pub fn ip6(&mut self, val: Option<Ipv6Addr>) -> &mut Self {
        let ip = match val {
            None => String::from("$remote"),
            Some(v) => v.to_string(),
        };
        self.ip = Some(ip);
        self
    }

    /// (iOS) Identifier for Advertiser.
    pub fn idfa<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.idfa = Some(val.into());
        self
    }

    /// (iOS) Identifier for Vendor.
    pub fn idfv<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.idfv = Some(val.into());
        self
    }

    /// (Android) Google Play Services advertising ID
    pub fn adid<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.adid = Some(val.into());
        self
    }

    /// (Android) Android ID (not the advertising ID)
    pub fn android_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.android_id = Some(val.into());
        self
    }

    /// (Optional) An incrementing counter to distinguish events with the same user_id
    /// and timestamp from each other. We recommend you send an event_id, increasing
    /// over time, especially if you expect events to occur simultanenously.
    pub fn event_id(&mut self, val: i32) -> &mut Self {
        self.event_id = Some(val);
        self
    }

    /// (Optional) The start time of the session in milliseconds since epoch (Unix Timestamp),
    /// necessary if you want to associate events with a particular system.
    /// A session_id of -1 is the same as no session_id specified.
    pub fn session_id(&mut self, val: i64) -> &mut Self {
        self.session_id = Some(val);
        self
    }

    /// (Optional) A unique identifier for the event. We will deduplicate subsequent events sent
    /// with an insert_id we have already seen before within the past 7 days.
    /// We recommend generation a UUID or using some combination of
    /// device_id, user_id, event_type, event_id, and time.
    pub fn insert_id<S>(&mut self, val: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.insert_id = Some(val.into());
        self
    }
}
