use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in the file {}", filename);

    // Read the file

    let mut f = File::open(filename).expect("file not found");
    let mut contents =  String::new();

    f.read_to_string(&mut contents)
        .expect("Error during reading file");
    
    println!("in file text \n {:?}", contents);

}