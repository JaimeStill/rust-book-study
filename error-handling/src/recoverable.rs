use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn read_with_match(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    }
}

pub fn read_with_unwrap(path: &str) -> File {
    File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}

pub fn run_with_expect(path: &str) -> File {
    File::open(path)
        .expect(&format!("Failed to open {}", path))
}

pub fn read_with_propogation(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(path)?.read_to_string(&mut s)?;

    Ok(s)
}

pub fn read_string(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}