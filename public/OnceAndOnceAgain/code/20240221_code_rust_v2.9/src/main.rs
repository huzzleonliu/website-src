use std::{thread, time};
// #[derive(Debug)]
#[derive(Clone)]
#[allow(unused_variables)]
struct Letter {
    code:char,
    bin:Vec<char>,
    grc:Vec<char>,
}

impl Letter {
    fn new(letter:char) -> Self {
        let bin_vec = vec![letter];
        let bin = char_to_bin(&bin_vec);
        Self {
            code:letter,
            bin: bin.clone(),
            grc: bin_to_grc(bin),
        }
    }
}

fn main() {
    let code:String = String::from("OnceAndOnceAgain");
    let code_vec:Vec<char> = str_to_vec(&code);
    let code_bin:Vec<char> = char_to_bin(&code_vec);
    let letters:Vec<Letter> = build_letter_vec(&code_vec);
    // println!("{:#?}",letters);
    play_picture(&letters);
}

fn str_to_vec(code:&String) -> Vec<char> {
    let mut ret = Vec::new();
    for i in code.chars() {
        ret.push(i);
    }
    ret
}

fn bin_to_grc(code:Vec<char>) -> Vec<char> {
    let mut ret = Vec::new();
    for i in code {
        if i == '0' {
            ret.push(' ');
        }else if i == '1' {
            ret.push('▇');
        }else {
            ret.push(i);
        }
    }
    ret
}

fn vec_to_str(code:&Vec<char>)-> String{
    let mut ret = String::new();
    for i in code {
        ret.push(*i);
    }
    ret
}

pub fn char_to_bin(code:&Vec<char>)-> Vec<char>{
    let mut ret_bin:String = String::new();
    for i in code {
        ret_bin.push('0');
        let bin:String = format!("{:b}",*i as u8);
        ret_bin = ret_bin + &bin;
    }
    let ret:Vec<char> = str_to_vec(&ret_bin);
    ret
}

fn build_letter_vec(code:&Vec<char>)->Vec<Letter> {
    let mut letters:Vec<Letter> = Vec::new();
    for i in code {
        let letter = Letter::new(*i);
        letters.push(letter);
    }
    letters
}

fn play_picture(code_r:&Vec<Letter>) {
    let duration = time::Duration::from_millis(70);
    let mut code:Vec<Letter> = code_r.clone();
    let mut cnt = 0;
    for i in 0..3 {
        let letter_p:Letter = code[i].clone();
        code.push(letter_p);
    }
    loop{
        let s1;
        let s2;
        let s3;
        let s4;
        if cnt % 2 == 0 {
            s1 = vec_to_str(&code[cnt].bin);
            s2 = vec_to_str(&code[cnt+1].grc);
            s3 = vec_to_str(&code[cnt+2].bin);
            s4 = vec_to_str(&code[cnt+3].grc);

        }else{
            s1 = vec_to_str(&code[cnt].grc);
            s2 = vec_to_str(&code[cnt+1].bin);
            s3 = vec_to_str(&code[cnt+2].grc);
            s4 = vec_to_str(&code[cnt+3].bin);

        }
        println!("{}||{}||{}||{}||{}||{}||{}||{}",code[cnt].code,s1,code[cnt+1].code,s2,code[cnt+2].code,s3,code[cnt+3].code,s4);
        // println!("{}|{}|{}|{}|{}|{}|{}|{}",code[cnt].code,s1,code[cnt+1].code,s2,code[cnt+2].code,s3,code[cnt+3].code,s4);
        cnt += 1;
        if cnt + 3 >= code.len() {
            cnt = 0;
        }
        thread::sleep(duration);

    }

}
