use super::SubtitleCodec;
use Caption;
use std::old_io;
use std::old_io::File;

pub const CODEC: SubtitleCodec = {
    fn detect(file: &File) -> bool {
        println!("Trying to detect if the file is subrip.");
        parse_one(file);
        return false;
    }

    fn parse_one(file: &File) -> Option<Caption> {
        let mut vec: Vec<&str> = Vec::new();
        let reader = old_io::BufferedReader::new(*file);
        loop {
            match reader.read_line() {
                Ok(s) => {
                    let ss = s.as_slice();
                    println!("Line LOLOL read: {}", ss);
                    if ss.trim() == "" {
                        // We have an empty line...
                        println!("empty line, stopping");
                        break;
                    }
                    vec.push(ss.trim());
                },
                Err(old_io::IoError { kind: old_io::EndOfFile, .. }) => {
                    break;
                },
                Err(why) => {
                    panic!("Could not read file: {}", why);
                }
            }
        }
        print_vec(&vec);

        None
    }

    fn print_vec(v: &[&str]) {
        for &l in v.iter() {
            println!("Line: {}", l);
        }
    }

    SubtitleCodec {
        detect: detect
    }
};
