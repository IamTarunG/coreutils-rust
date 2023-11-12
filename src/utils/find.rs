use std::path::Path;

pub fn run(path : &Path, file_or_dir_name : &str) {
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
                                    if val.file_name() == file_or_dir_name.trim() {

                                        println!("Exists {:?}", val.file_name());
                                        break;
                                    }
                                    run(val.path().as_path(),file_or_dir_name.trim());
                                }
                                else {
                                    if val.file_name() == file_or_dir_name.trim(){

                                        println!("Exists {:?}", val.file_name());
                                        break;
                                    }
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
