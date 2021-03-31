use alsa::{ Direction, ValueOr };
use alsa::pcm::{PCM, HwParams, Format, Access, State };
pub struct Audio {}

pub const SAMPLE_RATE:usize = 44100;

impl Audio {
    // Open default playback device
    pub fn new_playback() -> PCM {
        PCM::new("default", Direction::Playback, false).unwrap()
    }

    pub fn new_capture() -> PCM {
        PCM::new("default", Direction::Capture, false).unwrap()
    }

    // Set hardware parameters: 44100 Hz / Mono / 16 bit
    pub fn set_hw(pcm: &PCM) {
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(SAMPLE_RATE as u32, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();
    }


    fn set_params(pcm: &PCM) {
        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
        pcm.sw_params(&swp).unwrap();
    }

    pub fn capture(pcm: &PCM, size: usize) -> [i16; 44100*10]{
        // capture sound must be more than 1s.
        assert!(size > SAMPLE_RATE, "Too short sound");

        let io = pcm.io_i16().unwrap();
        Audio::set_params(pcm);

        let mut buf = [0i16; 44100*10];
        
        io.readi(&mut buf[0..size]).unwrap();

        println!("Sound Capture: {:?}", buf);


        // In case the buffer was larger than 5 seconds, start the stream manually.
        if pcm.state() != State::Running { pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        pcm.drain().unwrap();
        buf
    }

    pub fn play(pcm: &PCM, buf: &[i16]){
        let io = pcm.io_i16().unwrap();

        Audio::set_params(pcm);
        io.writei(&buf[..]).unwrap();

        // In case the buffer was larger than 5 seconds, start the stream manually.
        if pcm.state() != State::Running { pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        pcm.drain().unwrap();
    }

    pub fn i16_to_u8(buf: &[i16]) -> &[u8] {
        unsafe {
            let bytes = buf.align_to::<u8>();
            assert_eq!(bytes.0.len() + bytes.2.len(), 0);
            assert_eq!(bytes.1.len(), buf.len()*2);
            bytes.1
        }
    }

    pub fn u8_to_i16(buf: &[u8]) -> &[i16] {
        unsafe {
            let bytes = buf.align_to::<i16>();
            assert_eq!(bytes.0.len()+bytes.2.len(), 0);
            assert_eq!(bytes.1.len() * 2, buf.len());
            bytes.1
        }
    }
}
