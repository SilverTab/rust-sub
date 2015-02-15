//! The Codecs module contains all the different form of subtitles we can
//! support as submodules. The module itself provides a interface to interact
//! with these submodules.
use std::old_io::File;


mod subrip;

/// Available codecs should be added here!
pub const AVAILABLE_CODECS: &'static [SubtitleCodec] = &[subrip::CODEC];

pub struct SubtitleCodec {
    detect: fn (file: &File) -> bool
}

impl SubtitleCodec {
    pub fn detect(&self, file: &File) -> bool {
        (self.detect)(file)
    }
}

