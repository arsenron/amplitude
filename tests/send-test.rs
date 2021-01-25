use amplitude::entities::Event;
use amplitude::amp::Amp;


#[tokio::test]
async fn send() {
    let key = "0088b844e9444ba503af88d0b5911deb";
    let amp = Amp::new(key);
    let mut event = Event::new(Some("658"), None, "test").unwrap();
    event.country("BY");
    let response = amp.send(&event).await.unwrap();
    eprintln!("response = {:#?}", response);
    // assert_eq!(1, 2);
}
