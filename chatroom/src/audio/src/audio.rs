use alsa::{ Direction, ValueOr };
use alsa::pcm::{PCM, HwParams, Format, Access, State };
pub struct Audio {}

impl Audio {
    // Open default playback device
    pub fn playback() -> PCM {
        PCM::new("default", Direction::Playback, false).unwrap()
    }

    pub fn capture() -> PCM {
        PCM::new("default", Direction::Capture, false).unwrap()
    }

    // Set hardware parameters: 44100 Hz / Mono / 16 bit
    pub fn set_hw(pcm: &PCM) {
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(44100, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();
    }

    pub fn read(pcm: &PCM) -> [i16; 44100*10]{
        let io = pcm.io_i16().unwrap();

        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
        pcm.sw_params(&swp).unwrap();

        let mut buf = [0i16; 44100*10];
        
        // for _ in 0..5*44100/1024 {
        //     assert_eq!(io.readi(&mut buf[..]).unwrap(), 1024);
        // }
        io.readi(&mut buf[..]).unwrap();

        println!("Sound Capture: {:?}", buf);
        // io.readi(&mut buf).unwrap();


        // In case the buffer was larger than 5 seconds, start the stream manually.
        if pcm.state() != State::Running { pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        pcm.drain().unwrap();
        buf
    }

    pub fn write(pcm: &PCM, buf: [i16;44100*10]){
        // println!("Receive Sound: {:?}", buf);
        let io = pcm.io_i16().unwrap();

        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
        pcm.sw_params(&swp).unwrap();


        // Play it back for 5 seconds.
        // for _ in 0..5*44100/1024 {
        //     assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
        // }
        io.writei(&buf[..]).unwrap();

        // In case the buffer was larger than 5 seconds, start the stream manually.
        if pcm.state() != State::Running { pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        pcm.drain().unwrap();
    }
}
