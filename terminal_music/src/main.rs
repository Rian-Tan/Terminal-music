use std::io;
use rustube::VideoFetcher;
use rustube::url::Url;
use rustube::tokio;
use symphonia::core::io::MediaSourceStream;


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
    let path_to_video = rustube::download_worst_quality(&url).await;

    let path = path_to_video.expect("file path not provided");
    let src = std::fs::File::open(path).expect("failed to open media");
    let _mss = MediaSourceStream::new(Box::new(src), Default::default());
}
