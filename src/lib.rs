#![crate_name = "subtitles"]
//! This is the main module of the library.
use std::old_io::File;
use codecs::AVAILABLE_CODECS;
pub mod codecs;

/// The main subtitle structure
///
/// This represents a group of subtitle. The file format is abstracted.
pub struct Subtitle {
    file_path: String
}

impl Subtitle {
    /// Open a subtitle file.
    ///
    /// This function opens a file at the given path and returns a 
    /// subtitle structure.
    pub fn open(path: &str) -> Option<Subtitle> {
        // Try and open the file. It should fail if it can't
        // be opened.
        //
        let f = match File::open(&Path::new(path)) {
            Ok(s) => s,
            Err(why) => {
                panic!("Could not open file: {}", why);
            }
        };
        // ok we have a valid path, try our codec sniffing...
        println!("I get here bro");
        for codec in AVAILABLE_CODECS {
            if codec.detect(&f) {
                println!("Found our codec!");
            }  
        }

        // if we get here, we haven't found our codec...
        None
    }
}

/// A subtitle caption
///
/// This represents a single subtitle caption.
pub struct Caption {
    start_ms: uint,
    end_ms: uint,
    content: String
}


#[cfg(test)]
mod tests {
    use super::Subtitle;
    use std::env;

    #[test]
    #[should_fail]
    fn open_invalid_path() {
        let s = Subtitle::open("/foo");
    }

    #[test]
    fn open_valid_file() {
        let s = Subtitle::open("./src/test.srt");
    }
    
}
