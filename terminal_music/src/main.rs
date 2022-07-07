use std::io;
use rustube::VideoFetcher;
use rustube::url::Url;
use rustube::tokio;

#[tokio::main]

async fn main() {
    println!("Terminal music :D");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
 
    println!("{}", url);

    let urll = Url::parse(&url).unwrap();
    let descrambler = VideoFetcher::from_url(&urll)
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let title = descrambler.video_title();
    println!("Currently playing: {}", title); 
    
}

/* fn get_url() -> String {
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    url
} */
//https://www.youtube.com/watch?v=PEnJbjBuxnw
