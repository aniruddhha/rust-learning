use std::{
    fs::File,
    io::{Error, ErrorKind},
};

pub fn error_handling_non_recoverable() {
    panic!("Some thing bad happened");
}

pub fn error_backtrace() {
    let c = vec![1, 3, 4];

    c[10];
}

pub fn error_recoverable_01() {
    let res: Result<File, Error> = File::open("/hello.txt");
    let fl: File = match res {
        Ok(file) => file,
        Err(error) => panic!("Error in Opening File {:?}", error),
    };
}

pub fn error_recoverable_02() {
    let fl = File::open("/hello.txt").unwrap();
}

pub fn error_recoverable_03() {
    let fl = File::open("/hello.txt").expect("File Not Found");
}

pub fn error_recoverable_04() {
    let fl = File::open("/hello.txt").unwrap_or_else(|err| {
        // if err.kind() == ErrorKind::NotFound {
        //     File::create("hi.txt").unwrap()
        // } else {
        //     panic!("Problem in Creating File")
        // }

        match err.kind() {
            ErrorKind::NotFound => File::create("hello.txt").unwrap(),
            _ =>  panic!("Problem in Creating File")
        }
    });
}
