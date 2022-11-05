use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    println!("{:?}",f);

    let mut reader = BufReader::new(f);

    let mut line = String::new();

    // Reading using loop
    loop {
        let len = reader.read_line(&mut line)
                            .unwrap();
        if len == 0 {
            break
        }

        println!("READING USING LOOP");
        println!("{} ({} bytes long)", line, len);
        line.truncate(0);
    }

    // Reading using reader iterator
    println!("READING USING READER ITERATOR");
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);
    for line_ in reader.lines(){
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
