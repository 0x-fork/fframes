use crate::frame::Frame;
use std::fs;
use webvtt_parser::{self, parse_vtt, Vtt};

#[derive(Debug)]
pub struct Subtitles {
    pub(crate) subtitles: Vtt,
}

impl Subtitles {
    pub fn from_str(content: &str) -> Self {
        Subtitles {
            subtitles: parse_vtt(content).unwrap(),
        }
    }

    pub fn from_file(path: &str) -> Self {
        let file = fs::read_to_string(path).unwrap();
        Self::from_str(file.as_str())
    }

    pub fn get_phrase_for_frame(&self, frame: &Frame) -> Option<&str> {
        self.subtitles.cues.iter().rev().find_map(|cue| {
            match (frame.get_current_second() * 1000.0) as u64 {
                milliseconds if milliseconds >= cue.start.0 && milliseconds <= cue.end.0 => {
                    Some(cue.text.as_str())
                }
                _ => None,
            }
        })
    }
}
