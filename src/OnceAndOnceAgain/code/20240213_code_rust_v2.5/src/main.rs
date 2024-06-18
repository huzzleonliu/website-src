use std::{time, thread};

fn main() {
    let duration = time::Duration::from_millis(100);
    let mut code = String::new();
    let code_init = "OnceAndOnceAgain";
    code = code_init.to_string();
    let code_char = code_to_char(&code);
    let code_bin:String = code_char_to_bin(&code_char);
    // let code_shifted:String = shift_str(&code_bin,1);
    // println!("{:?}",code_shifted);
    let mut code_shifted:String = code_bin;
    let mut shift:usize = 0;
    loop{
        println!("{:?}",code_shifted);
        code_shifted = shift_str(&code_shifted, shift);
        thread::sleep(duration);
        shift += 1;
        if shift >= 16 {
            shift = 0;
        }
    }

}

fn code_to_char(code:&str)->Vec<char> {

    let mut ret:Vec<char> = Vec::new();
    for i in code.chars(){
        ret.push(i);
    }
    ret

}

fn code_char_to_bin(code_char:&Vec<char>)->String{
    let mut code_bin = String::new();
    for c in code_char{
        let u = *c as u8;
        let bin = format!("{:b}",u);
        code_bin.push('0');
        code_bin = code_bin + &bin;
    }
    code_bin
}

fn shift_str(str1:&str, s:usize)->String{
    let mut ret:String = String::new();
    let str1_vec:Vec<char> = str1.chars().collect();
    // println!("{}",str1_vec.len());
    let mut ret_vec:Vec<char> = Vec::new();
    for i in 0..str1_vec.len(){
        let mut index = i + s;
        if index >= str1_vec.len(){
            index = i + s -str1_vec.len();
        }
        ret_vec.push(str1_vec[index]);
    }
    ret = ret_vec.into_iter().collect();
    ret
}


