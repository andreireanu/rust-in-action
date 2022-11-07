use rand::prelude::*;

#[derive(Debug, PartialEq)]
enum FileState{
    Open,
    Closed,
}
 
fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}


impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file
    }

    fn read(
        self: &File,
        save_to: &mut Vec<u8>,
        ) -> Result<usize, String>   {
            if self.state != FileState::Open {
                return Err(String::from("File must be opened for reading"));
            }
            let mut tmp = self.data.clone();
            let read_length = tmp.len();
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000){
        let err_msg = String::from("Permission denied!");
        return Err(err_msg);
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000){
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    f.state = FileState::Closed;
    Ok(f)
}
    
fn main() {
    let f4_data = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();
    let text = String::from_utf8_lossy(&buffer);
    
    println!("File opened raw {:?}", f4);
    println!("File {} length after reading: {:?}", &f4.name, f4_length);
    println!("File data: {}", text);

}
