
use std::{collections::HashMap, fs::File, io::BufReader};

use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder, source::Source};
use sdl2::{mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag, Sdl2MixerContext, Music}, AudioSubsystem};

use crate::context::Context;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AudioSource {
    pub filename: String,
    pub audio_type: AudioType,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AudioType {
    Music,
    Effect,
}

pub struct AudioManager<'a> {
    audios: HashMap<AudioSource, Option<Box<Music<'a>>>>,
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
            let music = if audio.audio_type == AudioType::Effect {
                Some(Box::new(Music::from_file(filename).unwrap())) }
                else {
                    None
                };
            self.audios.insert(
                audio,
                music,
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

    _stream: OutputStream,
    _stream_handler: OutputStreamHandle,
    _stream_sink: Sink,

}

impl Audio {
    pub fn new(ctx: &Context) -> Self {

        let audio_subsystem = ctx.context.audio().unwrap(); 


       let (stream, stream_handler) = OutputStream::try_default().unwrap();
       let sink = Sink::try_new(&stream_handler).unwrap();

        let mixer = sdl2::mixer::init(InitFlag::all()).unwrap();
        Self {

            _mixer: mixer,
            _sources: HashMap::new(),
            _audio_subsystem: audio_subsystem,
            _opened: false,

            _stream: stream,
            _stream_handler: stream_handler,
            _stream_sink: sink,
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

    pub fn play(&mut self, audio_manager: &AudioManager, filename: &str) {
        
        if let Some(audio_source) = self._sources.get(&filename.to_string()) {
            match audio_source.audio_type {
                AudioType::Music => {
                    self._stream_sink.clear();

                    let file = BufReader::new(File::open(&filename).unwrap());
                    let source = Decoder::new(file).unwrap();
                    self._stream_handler.play_raw(source.convert_samples()).unwrap();

                },
                AudioType::Effect => {
                    if let Some(infos) = audio_manager.audios.get(&audio_source) {
                        if let Some(audio) = infos {
                            audio.play(0).unwrap();
                        }
                    }
                },
            }
        }
    }
}