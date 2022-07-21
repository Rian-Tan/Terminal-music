use std::io;
use rustube::VideoFetcher;
use rustube::url::Url;
use rustube::tokio;
use rustube::Video;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

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
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .max_by_key(|stream| stream.quality_label)
        .unwrap()
        .download()
        .await;
    // println!("{:?}",  video_path);
    println!("Done !");
    println!("Currently playing: {title}");
    let path: String = format!("./{}.mp4", &url[32..43]);
    // println!("{}", path);  // testing

    /* symphonia */
    /* music      */
    /* section   */
    /* :D .      */   
    
    let src = std::fs::File::open(path).expect("failed to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("mp4");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }
            Err(err) => {
                // A unrecoverable error occured, halt decoding.
                panic!("{}", err);
            } 
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(_decoded) => {
                // Consume the decoded audio samples (see below).
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occured, halt decoding.
                panic!("{}", err);
            }
        }
    }
}
