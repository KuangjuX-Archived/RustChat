use cpal::traits::{ HostTrait, DeviceTrait, StreamTrait };
use cpal::{ Data, Sample, SampleFormat };
pub struct Audio {}

impl Audio {

}

pub fn setup() {
    let host  = cpal::default_host();
    let device = host.default_output_device().expect("No available output device");

    let mut supported_configs_range = device.supported_output_configs()
                                      .expect("errors while querying configs");

    let supported_config = supported_configs_range.next().expect("No expected config")
                           .with_max_sample_rate();

    // let config = StreamConfig{
    //     channels: 1u16,
    //     sample_rate: SampleRate(8000),
    //     buffer_size: BufferSize::Default
    // };
    // let config  = supported_config.into();
    
    // let stream  = device.build_output_stream(
    //     &config,
    //     move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    //         // react to stream events and read or write stream data here.
    //     },
    //     move |err| {
    //         // react to errors here.
    //     },
    // );  
    
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
    }.unwrap();

    stream.play().unwrap();
    stream.pause().unwrap();
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&10.0);
    }
}