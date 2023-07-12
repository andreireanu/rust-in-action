#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name:&str, data:Vec<u8>) -> Self {
        Self { name: String::from(name), data }
    }
}

fn open(f: &mut File) -> bool {
    true 
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>,) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_data = &f1.data.len();

    println!("{} has {} bytes", f1_name, f1_data);

    let mut f2 = File {
        name: String::from("f2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text2 = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text2);

    let mut f3 = File::new( "f3.txt", Vec::new());
    f3.data.push(78);
    f3.data.push(103);
    f3.data.push(90);
    buffer.clear();
    let f3_length = read(&f3, &mut buffer);
    let text3 = String::from_utf8_lossy(&buffer);
    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text3);


}
