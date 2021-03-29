use alsa::{ Direction, ValueOr };
use alsa::pcm::{PCM, HwParams, Format, Access, State };
pub struct Audio {}

impl Audio {
    // Open default playback device
    pub fn new() -> PCM {
        PCM::new("default", Direction::Playback, false).unwrap()
    }

    // Set hardware parameters: 44100 Hz / Mono / 16 bit
    pub fn set_hw(pcm: &PCM) -> HwParams {
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(44100, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();

        hwp
    }

    pub fn read(pcm: &mut PCM, hwp: &mut HwParams) {

    }

    pub fn write(pcm: &PCM, hwp: &HwParams){
        let io = pcm.io_i16().unwrap();

        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
        pcm.sw_params(&swp).unwrap();

        // Make a sine wave
        let mut buf = [0i16; 1024];
        for (i, a) in buf.iter_mut().enumerate() {
            *a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
        }

        // Play it back for 2 seconds.
        for _ in 0..2*44100/1024 {
            assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
        }

        // In case the buffer was larger than 2 seconds, start the stream manually.
        if pcm.state() != State::Running { pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        pcm.drain().unwrap();
    }
}
