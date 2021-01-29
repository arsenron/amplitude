use amplitude::amplitude::Amp;
use amplitude::entities::Event;

#[tokio::test]
async fn send() {
    let key = "0088b844e9444ba503af88d0b5911deb";
    let mut amp = Amp::new(key);
    amp
        .single()
        .set_min_id_length(4);
    let mut event = Event::new(Some("6543"), None, "test").unwrap();
    event
        .country("BY")
        .android_id("ewq4tegf")
        .ip("rew");
    let response = amp
        .send(vec![&event])
        .await
        .unwrap();
    eprintln!("response = {:#?}", response);
}

#[tokio::test]
async fn map() {
    use serde_json::json;
    use serde::{Serialize};

    #[derive(Serialize)]
    struct UserProperties {
        age: u8,
        gender: String,
        interests: Vec<String>,
    }

    let mut amp = Amp::from_env().unwrap();
    let mut event = Event::new(Some("6543438"), None, "test").unwrap();
    let up = UserProperties {
        age: 25,
        gender: "female".to_string(),
        interests: vec!["football".to_string(), "hockey".to_string()],
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


#[tokio::test]
async fn event_json() {
    use serde_json::json;
    use serde::{Serialize};
    let key = "0088b844e9444ba503af88d0b5911deb";
    let mut amp = Amp::new(key);
    // let mut amp = Amp::from_env().unwrap();
    let event = Event::from_json(json!(
        {
            "user_id": "43546757",
            "event_type": "json",
            "android_id": "3gfhtey534-647"
        }
    )).unwrap();
    eprintln!("event = {:#?}", event);

}
