pub mod parser;
pub mod widgets;

use std::{fs::File, io::prelude::*, io::Error};
use treexml::Document;

fn main() {
    let data = reader("gui.xml");

    match data {
        Ok(data) => match Document::parse(data.as_bytes()) {
            Ok(data) => match data.root {
                Some(r) => parser::xml_parser(r),
                None => eprintln!("No data"),
            },
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

fn reader(path: &str) -> Result<String, Error> {
    let file = File::open(path);
    let mut buf = String::new();

    match file {
        Ok(mut file) => match file.read_to_string(&mut buf) {
            Ok(_) => Ok(buf.to_string()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
