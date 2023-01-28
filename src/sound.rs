use sdl2::audio::{AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT};
use sdl2::AudioSubsystem;

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

pub fn play_wav(wav_path: &str, audio_subsystem: &AudioSubsystem) {
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None      // default
    };
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_path)
            .expect("Could not load test WAV file");
        let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        // initialize the audio callback
        Sound {
            data,
            volume: 1.0,
            pos: 0,
        }
    }).unwrap();
    // Start playback
    device.resume();
    // Play for a second
    // std::thread::sleep_ms(1000);
}
