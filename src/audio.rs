use rodio::{Decoder, OutputStream, Sink, Source, OutputStreamHandle};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::collections::HashMap;
use std::path::Path;

pub struct AudioSystem {
    _stream: Option<OutputStream>,
	stream_handle: Option<OutputStreamHandle>,
    music_sink: Option<Sink>,
    sound_effects: HashMap<String, Vec<u8>>,
	sound_volume: f32,
	music_volume: f32,
}

impl AudioSystem {
    pub fn new() -> Self {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Audio initialization failed: {}. Continuing without audio.", e);
                return AudioSystem {
                    _stream: None,
                    stream_handle: None,
                    music_sink: None,
                    sound_effects: HashMap::new(),
					sound_volume: 1.0,
					music_volume: 1.0,
                };
            }
        };

        let music_sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Music sink creation failed: {}. Continuing without audio.", e);
                return AudioSystem {
                    _stream: Some(stream),
                    stream_handle: Some(stream_handle),
                    music_sink: None,
                    sound_effects: HashMap::new(),
					sound_volume: 1.0,
					music_volume: 1.0,
                };
            }
        };

        AudioSystem {
            _stream: Some(stream),
            stream_handle: Some(stream_handle),
            music_sink: Some(music_sink),
            sound_effects: HashMap::new(),
			sound_volume: 1.0,
			music_volume: 1.0,
        }
    }

    // Preload sound effects
    pub fn load_sound(&mut self, name: &str, path: &str) {
        if self.music_sink.is_none() { return; }
        let file = File::open(Path::new(path)).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut buffer).unwrap();
        self.sound_effects.insert(name.to_string(), buffer);
    }

    pub fn play_music(&mut self, path: &str) {
        self.stop_music();
        if let Some(stream_handle) = &self.stream_handle {
            match Sink::try_new(stream_handle) {
                Ok(sink) => {
                    sink.set_volume(self.music_volume);
                    self.music_sink = Some(sink);
                    match File::open(path) {
                        Ok(file) => {
                            let reader = BufReader::new(file);
                            match Decoder::new(reader) {
                                Ok(source) => {
                                    if let Some(sink) = &self.music_sink {
                                        sink.append(source.repeat_infinite());
                                    }
                                }
                                Err(e) => eprintln!("Failed to decode music: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to load music: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to create music sink: {}", e),
            }
        }
    }

    pub fn play_sound(&self, name: &str) {
        if let Some(data) = self.sound_effects.get(name) {
            let cursor = Cursor::new(data.clone());
            match Decoder::new(cursor) {
                Ok(source) => {
                    if let Some(stream_handle) = &self.stream_handle {
                        match Sink::try_new(stream_handle) {
                            Ok(sink) => {
                                sink.set_volume(self.sound_volume);
                                sink.append(source);
                                sink.detach();
                            }
                            Err(e) => eprintln!("Failed to play sound: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Failed to decode sound '{}': {}", name, e),
            }
        }
    }

    pub fn music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.music_sink {
            sink.set_volume(self.music_volume);
        }
    }

	pub fn sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume.clamp(0.0, 1.0);
    }

	pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
        }
    }

    pub fn pause_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.pause();
        }
    }

    pub fn resume_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.play();
        }
    }
}