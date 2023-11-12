use std::fs::File;
use std::io::{BufReader, Read, Write};

pub fn run(filesnames : Vec<&str>){
    let mut appended_content = String::new();
    for file in filesnames{
        let mut content = String::new();
        let file_open = File::open(file.trim());
        match file_open {
            Err(err) =>{
                println!("Err opening file {err}");
            }
            Ok(f)=>{
                let mut buf = BufReader::new(f);
                let _ = buf.read_to_string(&mut content);

                appended_content.push_str(&content.to_string());
                let write_file = File::create("contents.txt");
                match write_file {
                    Err(err) =>{
                        println!("Error writing file {}",err);
                    }
                    Ok(mut file) =>{
                        let _ = file.write_all(&appended_content.as_bytes());
                    }
                }
            }
        }
    }
}

