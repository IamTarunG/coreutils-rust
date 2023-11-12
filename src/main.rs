use std::{io, env::current_dir};

use crate::utils::{echo,cat,ls,find,grep};

mod utils;
fn main() {
    loop{
        let mut choice = String::new();
        print!("\n");
        print!("\n");
        println!("echo");
        println!("cat");
        println!("ls");
        println!("find");
        println!("grep");
        println!("q : Exit");
        println!();
        let _ = io::stdin().read_line(&mut choice);
        match choice.trim() {
            "echo" => {
                println!("Enter something");
                let mut echo_inp = String::new();
                let _ = io::stdin().read_line(&mut echo_inp);
                echo::run(echo_inp)
            },
            "cat" =>{
                let mut filename1 = String::new();
                let mut filename2 = String::new();
                println!("Enter first filename");
                let _f1 = io::stdin().read_line(&mut filename1);
                println!("Enter second filename");
                let _f2 = io::stdin().read_line(&mut filename2);
             
                cat::run(vec![&filename1,&filename2]);
                
            },
            "ls" =>{
                let curent_dir = current_dir();
                match curent_dir {
                    Err(err) =>{
                        println!("{}",err);
                    }
                    Ok(dir) =>{
                        ls::run(&dir)
                    }
                }
                
            },
            "find" =>{
                let curent_dir = current_dir();
                match curent_dir {
                    Err(err) =>{
                        println!("{}",err);
                    }
                    Ok(dir) =>{
                        println!("Enter file or dir name to find");
                        let mut file_name_or_dir_name = String::new();
                        let _ = io::stdin().read_line(&mut file_name_or_dir_name);
                        find::run(&dir,&file_name_or_dir_name);
                    }
                }
            },
            "grep" =>{
                let mut filename = String::new();
                let mut target = String::new();
                println!("Enter filename");
                let _ = io::stdin().read_line(&mut filename);
                println!("Enter target");
                let _t = io::stdin().read_line(&mut target);
                grep::run(filename.as_str(), target.as_str());
                
                
            }
            "q" =>{
                break;
            }
            _ => println!("Not in the given choice")
        }
    }

    
    
}
