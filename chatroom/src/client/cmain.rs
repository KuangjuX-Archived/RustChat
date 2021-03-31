mod protocol;
extern crate cpal;

// Localhost with a port in it
pub const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
pub const MESSAGE_SIZE: usize = 1024 * 1024;

// Help manual
pub const HELP: &'static str = include_str!("help.txt");

mod client;
use client::*;
use audio::Audio;

fn main() 
{   
    // get linux default capture device
    // let capture_pcm = Audio::new_capture();
    // get linux default playback device
    // let playback_pcm = Audio::new_playback();

    // set device hardware: 44100HZ; Mono; 16bits
    // Audio::set_hw(&capture_pcm);
    // Audio::set_hw(&playback_pcm);

    // capture sound by default device
    // let sound = Audio::capture(&capture_pcm, 10);

    // playback sound which has been captured
    // Audio::play(&playback_pcm, &sound);

    let client = Client::new(LOCAL_HOST);
    let connect = client.connect();
    client.run(connect);

}