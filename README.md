<img src="https://static.amplitude.com/lightning/46c85bfd91905de8047f1ee65c7c93d6fa9ee6ea/static/media/amplitude-logo-with-text.4fb9e463.svg" alt="Amplitude"/> 


# API overview

## First example

```rust, no_run
use amplitude::{Amp, Event};
use serde_json::json;

#[tokio::main]
async fn main() {
    let amp = Amp::from_env().unwrap(); // api key as env variable must be provided
    let mut event1 = Event::new();
    event1
    	.user_id("some user id")
    	.event_type("some event type")
    	.country("UK")
    	.android_id("slmsung-unlimited-hash")
    	.ip4(Some(Ipv4Addr::new(127, 0, 0, 5)))
    	.time(chrono::Utc::now());
    let event2 = Event::from_json(json!(
    	{
            "user_id": "some user id",
            "event_type": "lay on the beach",
            "android_id": "3gfhtey534-647"
        }
    )).unwrap();
    let response = amp.send(vec![&event1, &event2]).unwrap();
}
```



## Second example

```rust, no_run
use amplitude::{Amp, Event};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut amp = Amp::from_env().unwrap(); // api key as env variable must be provided
    amp
    	.batch() // set batch url
    	.set_min_id_length(4); 
    let mut event = Event::new();
    event
    	.device_id("xxxx")
    	.event_type("register");
    
    #[derive(Serialize)]
    struct UserProperties {
        age: u8,
        gender: String,
        interests: Vec<String>,
    }
    let up = UserProperties {
        age: 25,
        gender: "female".to_string(),
        interests: vec!["football".to_string(), "hockey".to_string()],
    };
    
    event.user_properties(up); // set user properties
    // also instead of creating a struct, you may pass serde_json::Value
    event.event_properties(json!({
        "event_group": "children",
        "foods": [
            {
                "apples": 5
            }
        ]
    }));
    let response = amp.send(vec![&event]).await.unwrap();
}
```

