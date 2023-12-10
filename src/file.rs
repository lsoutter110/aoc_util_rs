use crate::cspace::Plane;

pub fn read_to_string(filename: &str) -> String {
    use std::{fs::File, path::Path, io::Read};
    let path = Path::new(filename);

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open {}: {}", filename, e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't read {} to string: {}", filename, e),
    }
    
    return s;
}

pub fn read_to_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename).split("\n").map(|line| line.to_string()).collect();
}

pub fn read_to_byte_plane(filename: &str) -> Plane<u8> {
    
}