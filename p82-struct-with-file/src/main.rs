#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File { 
            name: String::from(name),
            data: vec![],
        }
    }

    fn with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(
    f: &File,
    save_to: &mut Vec<u8>,
) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length

}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer = vec![];
    let f2_length = read(&f2, &mut buffer );
    let text = String::from_utf8_lossy(&buffer);
    println!("length : {}", f2_length);
    println!("{}", text);

    let f3_data: Vec<u8> = vec![114, 121, 115, 100, 33];
    let mut f3 = File::with_data("3.txt", &f3_data);

    let mut buffer3 = vec![];
    let f3_length = read(&f3, &mut buffer3 );
    let text3 = String::from_utf8_lossy(&buffer3);
    println!("length : {}", f3_length);
    println!("{}", text3);

}
