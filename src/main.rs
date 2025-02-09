mod multi;
mod single;
use std::env::args;
use std::fs::{exists, File};
use std::io::{BufRead, BufReader};

fn main() {
    let arr: Vec<String> = args().collect();
    if arr.len() < 2 {
        println!("Please provide a measurements file path");
    } else if !exists(&arr[1]).expect("Error while checking file") {
        println!("File does not exits:- {:#?}", &arr[1]);
    } else {
        let path = &arr[1];
        let file = File::open(path).expect("Unable to open the file");
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        reader
            .read_line(&mut first_line)
            .expect("File does not contain readable content");
        let (_, _) = first_line
            .trim_end()
            .split_once(';')
            .expect("This file does not seems to be a measurements file");

        if arr.len() > 2 && arr[2] == "--single" {
            single::start(path);
        } else {
            multi::start(path);
        }
    }
}
