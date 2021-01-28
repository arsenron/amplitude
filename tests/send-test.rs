use amplitude::amplitude::Amp;
use amplitude::entities::Event;

#[tokio::test]
async fn send() {
    let key = "0088b844e9444ba503af88d0b5911deb";
    let mut amp = Amp::new(key);
    amp.single().set_min_id_length(4);
    let mut event = Event::new(Some("6543"), None, "test").unwrap();
    event.country("BY").android_id("ewq4tegf");
    let response = amp.send(vec![&event]).await.unwrap();
    eprintln!("response = {:#?}", response);
    // assert_eq!(1, 2);
}

#[tokio::test]
async fn map() {
    use serde_json::json;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct UserProperties {
        age: u8,
        gender: String,
        interests: Vec<String>,
    }

    let key = "0088b844e9444ba503af88d0b5911deb";
    let mut amp = Amp::new(key);
    let mut event = Event::new(Some("6543438"), None, "test").unwrap();
    let up = UserProperties {
        age: 25,
        gender: "female".to_string(),
        interests: vec!["football".to_string(), "hockey".to_string()]
    };
    event.user_properties(up);
    event.event_properties(json!({
        "test-event": "property228",
        "arr": [
            {
                "a": "t"
            }
        ]
    }));
    eprintln!("event = {:#?}", event);
    let response = amp.send(vec![&event]).await.unwrap();
    eprintln!("response = {:#?}", response);
}
