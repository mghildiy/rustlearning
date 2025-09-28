use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn panicking() {
    let file = File::open("non existing file");
    let file = file.unwrap_or_else(|e| handle_error(&e));
    println!("{:?}", file);
}

fn handle_error(error: &io::Error) -> File {
    match error.kind() {
        ErrorKind::NotFound => {
            match File::create("myfile.txt") {
                Ok(f) => f,
                Err(e) => panic!("Tried to create file but there was a problem: {e:?}")
            }
        }
        _ => panic!("Problem opening the file: {error:?}")
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}