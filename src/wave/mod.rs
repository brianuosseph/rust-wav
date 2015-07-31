//! The Waveform Audio File Format
//!
//! WAVE files use the Resource Interchange File Format (RIFF), a generic
//! file container format that uses chunks to store data. All integers are stored
//! in little-endian format, but identifier bytes are in ASCII, big-endian.
//!
//! References
//! - [McGill University](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html)
//! - [WAVE Spec](http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/Docs/riffmci.pdf)

mod container;
mod chunks;
pub mod decoder;
pub mod encoder;

pub use wave::decoder::Decoder as Decoder;
pub use wave::encoder::Encoder as Encoder;

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::Read;
  use std::path::{Path, PathBuf};
  use ::audio;
  use ::buffer::AudioBuffer;
  use ::codecs::Codec::*;

  #[test]
  fn u8_wave_eq() {
    let mut path = PathBuf::from("tests");
    path.push("wav");
    path.push("empty.wav");
    let files = vec![
      "mono440-u8-44100.wav",
      "stereo440-u8-44100.wav"
    ];

    for file in files.iter() {
      path.set_file_name(file);
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).ok().expect("Couldn't open read file");
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_u8.wav");
      let written = audio::save_as(&write_loc, &audio, LPCM_U8);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn i16_wave_eq() {
    let mut path = PathBuf::from("tests");
    path.push("wav");
    path.push("empty.wav");
    let files = vec![
      "i16-pcm-mono.wav",
      "i16-pcm-stereo.wav"
    ];

    for file in files.iter() {
      path.set_file_name(file);
      println!("{:?}", path.as_path());
      let audio =
        match audio::open(path.as_path()) {
          Ok(a) => a,
          Err(e) => panic!(format!("Error: {:?}", e))
        };
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i16.wav");
      let written = audio::save_as(&write_loc, &audio, LPCM_I16_LE);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn i24_wave_eq() {
    let mut path = PathBuf::from("tests");
    path.push("wav");
    path.push("empty.wav");
    let files = vec![
      "mono440-i24-44100.wav",
      "stereo440-i24-44100.wav"
    ];

    for file in files.iter() {
      path.set_file_name(file);
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).ok().expect("Couldn't open read file");
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i24.wav");
      let written = audio::save_as(&write_loc, &audio, LPCM_I24_LE);
      println!("{:?}", written);
      assert!(written.is_ok());
      
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
      
    }
  }

  #[test]
  fn i32_wave_eq() {
    let mut path = PathBuf::from("tests");
    path.push("wav");
    path.push("empty.wav");
    let files = vec![
      "mono440-i32-44100.wav",
      "stereo440-i32-44100.wav"
    ];

    for file in files.iter() {
      path.set_file_name(file);
      println!("{:?}", path.as_path());
      let audio = audio::open(path.as_path()).ok().expect("Couldn't open read file");
      let total_samples = audio.samples.len();
      let channels = audio.channels;
      let bit_rate = audio.bit_rate;
      let sample_rate = audio.sample_rate;
      let sample_order = audio.order;

      let write_loc = Path::new("tests/results/tmp_i32.wav");
      let written = audio::save_as(&write_loc, &audio, LPCM_I32_LE);
      println!("{:?}", written);
      assert!(written.is_ok());
      let verify: AudioBuffer = audio::open(&write_loc).unwrap();
      assert_eq!(total_samples, verify.samples.len());
      assert_eq!(channels, verify.channels);
      assert_eq!(bit_rate, verify.bit_rate);
      assert_eq!(sample_rate, verify.sample_rate);
      assert_eq!(sample_order, verify.order);

      // File sizes are the same
      let read_file = File::open(path.as_path()).unwrap();
      let written_file = File::open(&write_loc).unwrap();
      let read_meta = read_file.metadata().unwrap();
      let write_meta = written_file.metadata().unwrap();
      assert_eq!(read_meta.len(), write_meta.len());

      // Bytes are the same
      let mut written_file_bytes = written_file.bytes();
      for byte in read_file.bytes() {
        assert_eq!(
          byte.ok().expect("Error reading byte from read file"),
          written_file_bytes.next().expect("End of file").ok().expect("Error reading byte from written file")
        );
      }
    }
  }

  #[test]
  fn read_wave_format_extensible() {
    let mut path = PathBuf::from("tests");
    path.push("wav");
    path.push("empty.wav");
    let files = vec![
      "wavex-mono440-i16-44100.wav",
    ];

    for file in files.iter() {
      path.set_file_name(file);
      println!("{:?}", path.as_path());
      let wavex =
        match audio::open(path.as_path()) {
          Ok(a) => a,
          Err(e) => panic!(format!("Error: {:?}", e))
        };
      let total_samples = wavex.samples.len();
      let channels      = wavex.channels;
      let bit_rate      = wavex.bit_rate;
      let sample_rate   = wavex.sample_rate;
      let sample_order  = wavex.order;
      // Compare to regular wave file with same data
      path.set_file_name("mono440-i16-44100.wav");
      println!("{:?}", path.as_path());
      let wave =
        match audio::open(path.as_path()) {
          Ok(a) => a,
          Err(e) => panic!(format!("Error: {:?}", e))
        };
      assert_eq!(wave.channels, channels);
      assert_eq!(wave.bit_rate, bit_rate);
      assert_eq!(wave.sample_rate, sample_rate);
      assert_eq!(wave.order, sample_order);
      assert_eq!(wave.samples.len(), total_samples);
      for (wave_sample, wavex_sample) in wave.samples.iter().zip(&wavex.samples) {
        assert_eq!(wave_sample, wavex_sample);
      }
    }
  }

  #[test]
  fn wave_with_metadata() {
    let path  = Path::new("tests/wav/Warrior Concerto.wav");
    let audio =
      match audio::open(&path) {
        Ok(a) => a,
        Err(e) => panic!(format!("Error: {:?}", e))
      };
    let total_samples = audio.samples.len();
    let channels      = audio.channels;
    let bit_rate      = audio.bit_rate;
    let sample_rate   = audio.sample_rate;
    let sample_order  = audio.order;
    println!("Read file");
    // Write to file.
    let write_path  = Path::new("tests/results/tmp_i16.wav");
    let written     = audio::save(&write_path, &audio);
    assert!(written.is_ok());
    println!("File written");
    // Read written file and verify read audio is the same.
    let verify: AudioBuffer = audio::open(&write_path).unwrap();
    assert_eq!(channels,      verify.channels);
    assert_eq!(bit_rate,      verify.bit_rate);
    assert_eq!(sample_rate,   verify.sample_rate);
    assert_eq!(sample_order,  verify.order);
    assert_eq!(total_samples, verify.samples.len());
    for (inital_sample, written_sample) in
      audio.samples.iter().zip(&verify.samples) {
      assert_eq!(inital_sample, written_sample);
    }
    println!("Read new file");
    // Check if bytes are the same, but file sizes won't be in this case.
    let read_file     = File::open(&path).unwrap();
    let written_file  = File::open(&write_path).unwrap();
    // Assert every data byte is the same between the two files.
    for (inital_byte, written_byte) in
      read_file.bytes().skip(8).zip(written_file.bytes().skip(8)) {
      assert_eq!(inital_byte.ok(), written_byte.ok());
    }
  }
}
