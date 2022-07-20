use std::io;
use rustube::VideoFetcher;
use rustube::url::Url;
use rustube::tokio;
use rustube::Video;
use rustube::Id;
//use symphonia::core::io::MediaSourceStream;


#[tokio::main]

async fn main() {
    println!("Terminal music :D");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    println!("{}", url);

    let urll = match Url::parse(&url){
        Err(why) => panic!("error occurred here\nerror: {why}"),
        Ok(urll) => urll, 
    }; 
    let descrambler = VideoFetcher::from_url(&urll)
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let title = descrambler.video_title();
    println!("Currently playing: {}", title);
    println!("{urll}");
    //let test_url = "https://www.youtube.com/watch?v=Z3xPIYHKSoI";
    /* println!("{}",&urll[32..]);
    let id = Id::from_str(&urll[32..]); */
    let path_to_video = match Video::from_url(&urll).await{
        Err(why) => panic!("error occurred here\nerror: {why}"),
        Ok(path_to_video) => path_to_video,
    };
    let video_path = path_to_video
        .streams()
        .iter()
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .max_by_key(|stream| stream.quality_label)
        .unwrap()
        .download()
        .await;
    println!("{:?}",  video_path); 

/*    let path = path_to_video;
    let src = std::fs::File::open(path).expect("failed to open media");
    let _mss = MediaSourceStream::new(Box::new(src), Default::default());
*/
}
