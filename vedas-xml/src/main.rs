pub mod parser;
pub mod widgets;

use std::{fs::File, io::prelude::*, io::Error, path::Path};
use treexml::Document;

fn main() {
    let data = reader("data.xml");
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

fn reader(fp: &str) -> Result<String, Error> {
    let path = Path::new(fp);
    let f = File::open(path);
    let mut buf = String::new();
    match f {
        Ok(mut file) => match file.read_to_string(&mut buf) {
            Ok(_) => Ok(buf.to_string()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
