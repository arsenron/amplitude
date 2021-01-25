use amplitude::entities::Event;


#[test]
fn e() {
    struct W {}
    let mut event = Event::new(None, Some("5645645"), "qweqwe").unwrap();
    event
        .user_id("gdgfd")
        .user_id("fdgfd");
    eprintln!("event = {:#?}", event);
    assert_eq!(1, 2);
}

