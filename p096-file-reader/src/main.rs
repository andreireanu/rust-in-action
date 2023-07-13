use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Opened,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FileState::Opened => {
                return write!(f, "OPENED");
            }
            FileState::Closed => {
                return write!(f, "CLOSED");
            }
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "<{} ({})>", self.name, self.state);
    }
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data,
            state: FileState::Closed,
        }
    }

    fn open(mut self) -> Result<File, String> {
        self.state = FileState::Opened;
        Ok(self)
    }

    fn close(mut self) -> Result<File, String> {
        self.state = FileState::Closed;
        Ok(self)
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state == FileState::Closed {
            return Err(String::from("File is closed and can't be read from"));
        }
        let data_length = self.data.len();
        let mut data = self.data.clone();
        save_to.reserve(data_length);
        save_to.append(&mut data);
        Ok(data_length)
    }
}

fn main() {
    let mut f1 = File::new("f1");
    println!(
        "{} has {:?} bytes and is {:?}",
        &f1.name,
        &f1.data.len(),
        &f1.state
    );

    let data = vec![134, 98, 101, 121];
    let mut f2 = File::new_with_data("f2", data);
    println!(
        "{} has {:?} bytes and is {:?}",
        &f2.name,
        &f2.data.len(),
        &f2.state
    );

    let f3 = File::new_with_data("f3", vec![97, 90, 78, 101, 111]);
    let f3 = f3.open().unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    let length = &f3.read(&mut buffer).unwrap();
    let f3 = f3.close().unwrap();

    println!("{} has {:?} bytes and is { }", &f3.name, length, &f3.state);
    println!("{}", f1);
    println!("{}", f2);
    println!("{}", f3);
}
