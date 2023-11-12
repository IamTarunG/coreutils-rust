use std::{fs::File, io::{BufReader, Read}};

pub fn run(filename : &str,target_word_or_sec: &str ){
    let mut contents = String::new();
    match File::open(filename.trim()) {
        Err(err) =>{
            println!("Error opening file - {}",err);
        }
        Ok(file) =>{
            let mut reader = BufReader::new(file);
            let _ = reader.read_to_string(&mut contents);
            if contents.contains(target_word_or_sec.trim()) {
                println!("{} Exists",target_word_or_sec.trim());
            }
            else {
                println!("{} Does not exists",target_word_or_sec.trim());
            }
            
        }
    }
}