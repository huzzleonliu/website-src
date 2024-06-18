use std::{thread, time};

fn main() {
    let duration = time::Duration::from_millis(50);
    let code:String = String::from("OnceAndOnceAgain");
    let mut code_arr:Vec<char> = make_code_arr(&code);
    loop{
        let code_shift_arr:Vec<char> = shift(&code_arr, 7);
        let code_arr_bin:String = cvrt_char_to_bin(&code_shift_arr);
        println!("{:?}",code_arr_bin);
        thread::sleep(duration);
        code_arr = code_shift_arr;
    }
    
}

// fn cnt_letters(s:&String) -> usize{
//     let mut cnt:usize = 0;
//     for _c in s.chars(){
//         cnt += 1;
//     }
//     cnt
// } 

fn make_code_arr(s:&String ) -> Vec<char> {
    let mut code_arr: Vec<char> = Vec::new();
    for letter in s.chars(){
        code_arr.push(letter);
    }
    code_arr
}

fn cvrt_char_to_bin(ch:&Vec<char>) -> String{
    let mut ret:String = String::from("");
    for c in ch {
        let box1:u8 = *c as u8; 
        // println!("the box1 returns {box1}");
        let box2:String = format!("{:b}",box1);
        let mut box3:String = String::from("0");
        box3.push_str(&box2);
        ret.push_str(&box3);
    }
    ret
}

fn shift(s:&Vec<char>, n:usize) -> Vec<char>{
    let mut ret:Vec<char> = Vec::new();
    for i in 0..s.len(){
        let index = (i+n) % s.len();
        ret.push(s[index]);
    }
    ret
}
