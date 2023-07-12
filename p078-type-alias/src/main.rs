use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}



#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {

    fn new(name: &str) -> File {
        File { 
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name:&str, data:&Vec<u8>) -> Self {
        Self { name: String::from(name), data: data.clone() }
    }

    fn read(&self, save_to: &mut Vec<u8>,) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
    
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }


}

fn open(f: File) -> Result<File, String> {
    if one_in(2) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String>  {
    if one_in(2) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
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

    f2 = open(f2).unwrap();
    let f2_length = &f2.read(&mut buffer).unwrap();
    f2 = close(f2).unwrap();

    let text2 = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text2);

    let mut f3 = File::new_with_data( "f3.txt", &Vec::new());
    f3.data.push(78);
    f3.data.push(103);
    f3.data.push(90);
    buffer.clear();
    f3 = open(f3).unwrap();
    let f3_length = &f3.read(&mut buffer).unwrap();
    f3 = close(f3).unwrap();
    let text3 = String::from_utf8_lossy(&buffer);
    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text3);


}
