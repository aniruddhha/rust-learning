use std::{fs::File, io::Read};

pub fn read_file_01() {
    let file_result = File::open("./hi.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem in Opening file {:?}", error)
    };
}