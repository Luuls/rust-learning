use std::fs;
use std::io::*;

fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;

    // // Cannot read s1 since it is now invalid because s2 took ownership (move) of its resource.
    // println!("{s1}");

    let filepath = String::from("name.txt");
    let mut file = match fs::File::open(filepath) {
        Ok(f) => f,
        Err(e) => panic!("Error while opening the file: {e:?}")
    };

    let mut name = String::new();
    let bytes = file.read_to_string(&mut name).expect("Error while opening the file: ");
    println!("bytes read from the file: {bytes}");
    println!("Name: {name}");
}
