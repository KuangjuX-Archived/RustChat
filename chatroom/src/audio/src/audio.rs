use cpal::traits::{ HostTrait, DeviceTrait, StreamTrait };
use cpal::{ Data, Sample, SampleFormat };
pub struct Audio {}

impl Audio {

}

pub fn setup() {
    let host  = cpal::default_host();
    let device = host.default_output_device().expect("No available output device");

    println!("Sound Device: {}", device.name().unwrap());

    let mut supported_configs_range = device.supported_output_configs()
                                      .expect("errors while querying configs");

    let supported_config = supported_configs_range.next().expect("No expected config")
                           .with_max_sample_rate();

    println!("supported config: {:?}", supported_config);

    
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();

    println!("{:?}", sample_format);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
    }.unwrap();


    // println!("{}", stream);

    stream.play().unwrap();
    // stream.pause().unwrap();
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for (i, sample) in data.iter_mut().enumerate() {
        println!("i: {}", i);
        *sample = Sample::from(& ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0));
    }
}