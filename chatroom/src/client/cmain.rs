mod protocol;
extern crate cpal;

// Localhost with a port in it
pub const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
const MESSAGE_SIZE: usize = 1024 * 1024;

mod client;
use client::*;
use audio::Audio;

fn main() 
{   
    let capture_pcm = Audio::capture();
    let playback_pcm = Audio::playback();
    // let hwp = Audio::set_hw(&pcm);
    Audio::set_hw(&capture_pcm);
    Audio::set_hw(&playback_pcm);
    // Audio::write(&pcm);
    let sound = Audio::read(&capture_pcm);
    Audio::write(&playback_pcm, sound);

    let client = Client::new(LOCAL_HOST);
    let connect = client.connect();
    client.run(connect);

}