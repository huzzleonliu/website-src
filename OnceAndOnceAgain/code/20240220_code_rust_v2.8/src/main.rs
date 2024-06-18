use std::{thread, time};
fn main() {
    let duration = time::Duration::from_millis(50);
    let mut change = 1;
    let mut cnt = 0;
    let code:String = String::from("OnceAndOnceAgain");
    let code_vec:Vec<char> = str_to_vec(&code);
    let code_bin:Vec<char> = char_to_bin(&code_vec);
    let mut code_o:Vec<char> = shift(&code_bin, change); 
    loop {
        let code_grc:Vec<char> = bin_to_grc(&code_o);
        let code_output:String = vec_to_str(&code_grc);
        println!("{:03}|{:03}|{}",cnt,change,code_output);
        code_o = shift(&code_o, change);
        cnt += 1;
        if cnt >= 128/change {
            cnt = 0;
            change += 1;
        }
        if 128/change == 1 {
            change = 1;
        }

        thread::sleep(duration);

    }
    


}

fn str_to_vec(code:&String)-> Vec<char> {
    let mut ret:Vec<char> = Vec::new();
    for i in code.chars() {
        ret.push(i);
    }
    ret
}

fn vec_to_str(code:&Vec<char>)-> String{
    let mut ret = String::new();
    for i in code{
        ret.push(*i);
    }
    ret
}

fn bin_to_grc(code:&Vec<char>) -> Vec<char> {
    let mut ret:Vec<char> = Vec::new();
    for i in code {
        if *i == '0' {
            ret.push(' ');

        } else {
            ret.push('▇');
        }
    }
    ret
}

fn char_to_bin(code:&Vec<char>) -> Vec<char> {
    let mut ret_bin:String = String::new();
    let mut bin:String = String::new();
    for i in code {
        bin = format!("{:b}",*i as u8);
        ret_bin.push('0');
        ret_bin = ret_bin + &bin;
    }
    let ret = str_to_vec(&ret_bin);
    ret
}

fn shift(code:&Vec<char>, c:usize) -> Vec<char> {
    let mut ret:Vec<char> = Vec::new();
    let mut index = 0;
    for i in 0..code.len(){
        index = i + c;
        if i + c >= code.len() {
            index = index - code.len();
        }
        let bit:char = code[index];
        ret.push(bit);

    }
    ret
}

