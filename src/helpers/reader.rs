use std::io::prelude::*;
use std::{fs::File, io::Error};

fn reader(path: &str) -> Result<String, Error> {
    let file = File::open(path);
    let mut buffer = String::new();

    match file {
        Ok(mut f) => match f.read_to_string(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
