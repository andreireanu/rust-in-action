#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            FileState::Open => write!(f, "--OPEN--"),
            FileState::Closed => write!(f, "--CLOSED--"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn main() {
    let f6 = File {
        name : String::from("f6.txt"),
        data: vec![],
        state: FileState::Closed,
    };
    println!("{:?}", f6);
    println!("{}", f6);
}
