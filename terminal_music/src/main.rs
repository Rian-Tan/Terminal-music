use std::io;
use rustube::VideoFetcher;
use rustube::Id;
use rustube::tokio;


#[tokio::main]
async fn main() {
    println!("Terminal music :D");
    let video_url = String::from(get_url());
    println!("{}", video_url);
    let urrl = &video_url;
    let id = Id::from_raw(urrl).unwrap();
    println!("{}", id);
    let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let title = descrambler.video_title();
    println!("Currently playing: {}", title); 

}

fn get_url() -> String {
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    url
}

