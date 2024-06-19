use std::{thread, time::Duration};

fn main() {
    let mut duration = Duration::from_millis(100);

    println!("OnceAndOnceAgain");
    let code: String = String::from("OnceAndOnceAgain");
    let mut code_vec:Vec<char> = Vec::new();
    code_vec = str_to_vec(&code);
    let code_u8 = vec_char_to_u8(&code_vec);
    let code_bin:Vec<char> = u8_to_bin(&code_u8);
    let mut output_code:Vec<char> = bin_control(&code_bin, 0);
    let mut c = 0;
    let mut cc = 0;
    loop{
        let output = vec_to_str(&output_code);
        output_code = bin_control(&output_code, cc);
        println!("{:03}|{}",cc, output);
        // duration = time_ctl(&duration, c);
        duration = Duration::from_millis(c);
        thread::sleep(duration);
        c += 1;
        cc += 1;
        if c == 64 {
            c = 0;
            cc = 0;
        }

    }

}

fn str_to_vec(code:&str)-> Vec<char>{
    let mut ret:Vec<char> = Vec::new();
    for i in code.chars(){
        ret.push(i);
    }
    ret
}

fn vec_char_to_u8(code:&Vec<char>)->Vec<u8>{
    let mut ret = Vec::new();
    for i in code{
        ret.push(*i as u8);
    }
    ret
}

fn u8_to_bin(code:&Vec<u8>)->Vec<char>{
    let mut str_bin = String::new();
    let mut ret:Vec<char> = Vec::new();
    for i in code {
        let bin = format!("{:b}",i);
        str_bin.push('0');
        str_bin = str_bin + &bin;
    }
    for i in str_bin.chars(){
        ret.push(i);
    }
    ret
}

fn vec_to_str(code:&Vec<char>)->String{
    let mut ret = String::new();
    for i in code {
        ret.push(*i);
    }
    ret
}

fn time_ctl(t:&Duration, c:i32)->Duration{
    let add = Duration::from_millis(c.abs() as u64);
    let mut ret = Duration::new(0,0);
    if c >= 0{
        ret = *t + add;
    }else{
        ret = *t - add;
    }
    ret
}

fn bin_control(code:&Vec<char>, c: usize)->Vec<char>{
    let mut index = 0;
    let mut ret:Vec<char> = Vec::new();
    for i in 0..code.len(){
        index = i + c;
        if index >= code.len(){
            index -= code.len();
        }
        ret.push(code[index]);
    }
    ret
}
