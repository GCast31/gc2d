
use std::collections::HashMap;

use sdl2::{mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag, Sdl2MixerContext, Music, Channel}, AudioSubsystem};

use crate::context::Context;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AudioSource {
    pub filename: String,
    pub audio_type: AudioType,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AudioType {
    Stream,
    Static,
}

pub struct AudioManager<'a> {
    audios: HashMap<AudioSource, Box<Music<'a>>>,
}

impl<'a> AudioManager<'a> {
    pub(crate) fn new() -> Self {
        Self {
            audios: HashMap::new(),
        }
    }

    pub(crate) fn new_audio(&mut self, audio: AudioSource) {

        if !self.audios.contains_key(&audio) {
            let filename = audio.filename.clone();
            let music = Music::from_file(filename).unwrap();
            self.audios.insert(
                audio,
                Box::new(music),
            );
        }
    }
}

impl<'a> AudioManager<'a> {
    pub fn count(&self) -> usize {
        self.audios.capacity() 
    }
}

pub struct Audio {
    _mixer: Sdl2MixerContext,
    _audio_subsystem: AudioSubsystem,
    _sources: HashMap<String, AudioSource>,
    _opened: bool,

}

impl Audio {
    pub fn new(ctx: &Context) -> Self {

        let audio_subsystem = ctx.context.audio().unwrap(); 


        let mixer = sdl2::mixer::init(InitFlag::all()).unwrap();
        Self {

            _mixer: mixer,
            _sources: HashMap::new(),
            _audio_subsystem: audio_subsystem,
            _opened: false,
        }
    }

    pub fn new_source(&mut self, filename: &str, audio_manager: &mut AudioManager, audio_type: AudioType) {

        if !self._opened {
            let frequency = 44_100;
            let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
            let channels = DEFAULT_CHANNELS; // Stereo
            let chunk_size = 1_024;
            sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
            self._opened = true;
        }

        let audio_source = AudioSource {
            filename: filename.to_string(),
            audio_type,
        };

        audio_manager.new_audio(audio_source.clone());
        self._sources.insert(filename.to_string(), audio_source);
    }

    pub fn play(&mut self, audio_manager: &AudioManager, filename: &str, loops: i32) {
        
        if let Some(audio_source) = self._sources.get(&filename.to_string()) {
            match audio_source.audio_type {
                AudioType::Stream => {
                    if let Some(audio) = audio_manager.audios.get(&audio_source) {
                        audio.play(loops).unwrap();
                    }
                },
                AudioType::Static => {
                },
            }
        }
    }
}