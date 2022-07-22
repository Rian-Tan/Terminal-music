use std::io;
use rustube::VideoFetcher;
use rustube::url::Url;
use rustube::tokio;
use rustube::Video;
use std::fs;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::process::Stdio;
use std::fs::File;
use ffmpeg_cli::{FfmpegBuilder, File as file, Parameter};
use futures::{future::ready, StreamExt};


#[tokio::main]

async fn main() {
    println!("Terminal music :D");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    //println!("{}", url);

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
    println!("\nCurrently in Queue: {}", title);
    println!("\nloading...");
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
        .filter(|stream| stream.includes_audio_track)
        .max_by_key(|stream| stream.quality_label)
        .unwrap()
        .download_to("a.mp4")
        .await;
    // println!("{:?}",  video_path);
    println!("Done !");
    println!("Currently playing: {title}");
   // let path: String = format!("./{}.mp4", &url[32..43]);
    //let final_Path = format!("./{}.mp3", &url[32..43]);
    File::create("a.mp3")
        .expect("Error encountered while creating file!");
    let builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(Parameter::Single("nostdin"))
        .option(Parameter::Single("y"))
        .input(file::new("./a.mp4"))
        .output(file::new("./a.mp3")
            .option(Parameter::KeyValue("vcodec", "libx265"))
            .option(Parameter::KeyValue("crf", "28")),
    );
            
    
    let ffmpeg = match builder.run().await{
        Err(why) => panic!("{why}"),
        Ok(ffmpeg) => ffmpeg,
    };
    ffmpeg
        .progress
        .for_each(|x| {
            dbg!(x.unwrap());
            ready(())
        })
        .await; 

    let output = ffmpeg.process.wait_with_output().unwrap();
    

    // println!("{}", path);  // testing

    /* symphonia */
    /* music      */
    /* section   */
    /* :D .      */   
   
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("./a.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
    
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
    fs::remove_file("./a.mp4")
        .expect("file could not be removed."); 
    fs::remove_file("./a.mp3")
        .expect("file could not be removed."); 

    
}

//https://www.youtube.com/watch?v=ABvd67kdSzg
