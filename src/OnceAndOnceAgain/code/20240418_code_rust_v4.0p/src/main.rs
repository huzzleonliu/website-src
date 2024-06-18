use std::fs;
use std::{time, thread};

fn main() {
    let duration = time::Duration::from_millis(52);
    loop{
        println!("================================================================================");
        let contents = fs::read_to_string("code.txt")
            .expect("read file fail");
        for line in contents.lines(){
            println!("{}",line);
            thread::sleep(duration);
        }
    }
}
