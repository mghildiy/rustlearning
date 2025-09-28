use std::fs::File;
use std::io;
use std::io::ErrorKind;

pub fn panicking() {
    let file = File::open("non existing file");
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    match File::create("myfile.txt") {
                        Ok(f) => f,
                        Err(e) => panic!("Tried to create file but there was a problem: {e:?}")
                    }
                }
                _ => panic!("Problem opening the file: {e:?}")
            }
        }
    };
}