use std::path::Path;

pub fn run(path : &Path) {
    let read_directory = std::fs::read_dir(path);
    match read_directory {
        Ok(dir) => {
            let list : Vec<_> = dir.collect();
            for item in list {
                match item {
                    Ok(val) =>{
                        
                        let metadata = val.metadata();
                        match metadata {
                            Err(err) =>{
                                println!("{}",err);
                            }
                            Ok(data) =>{
                                if data.is_dir(){
                                    println!("Dir {:?}", val.file_name());
                                    run(val.path().as_path());
                                }
                                else {
                                    println!("File {:?}", val.file_name());
                                }
                            }
                        }
                        
                    },
                    Err(err) =>{
                        println!("Item error - {err}");
                    }
                }
            }
        }
        Err(err) => {
            println!("Dir - {err}")
        }
    }
}
