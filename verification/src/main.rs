use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    student_id: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let student_id = lines.next().unwrap().to_string();

        Config { name, student_id }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("What is your name? {}", config.name);
    println!("What is your student ID? {}", config.student_id);
    println!("Hi {}! Your student ID is {}.", config.name, config.student_id);
}

fn main(){
    reading_from_file();
}