use amplitude::{Event, Amp};
use serde::Serialize;
use serde_json::json;

#[tokio::test]
async fn send() -> Result<(), Box<dyn std::error::Error>> {
    let mut amp = Amp::from_env()?;
    amp.batch().set_min_id_length(4);
    let mut event = Event::new();
    event
        .user_id("34343")
        .event_type("start app")
        .country("BY")
        .android_id("ewq4tegf")
        .time(chrono::Utc::now())
        .ip4(Some(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    eprintln!("event = {:#?}", event);
    let response = amp.send(vec![&event]).await?;
    eprintln!("response = {:#?}", response);
    Ok(())
}

#[tokio::test]
async fn map() {
    use serde_json::json;

    #[derive(Serialize)]
    struct UserProperties {
        age: u8,
        gender: String,
        interests: Vec<String>,
    }

    let mut amp = Amp::from_env().unwrap();
    let mut event = Event::new();
    event
        .user_id("tetd")
        .event_type("loool");
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
    let event = Event::from_json(json!(
        {
            "user_id": "43546757",
            "event_type": "json",
            "android_id": "3gfhtey534-647"
        }
    ))
    .unwrap();
    eprintln!("event = {:#?}", event);
}
