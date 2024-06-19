use std::{thread, time };
fn main() {
    let mut duration = time::Duration::from_millis(100);

    let code: String = String::from("OnceAndOnceAgain");
    let code_vec:Vec<char> = str_to_vec(&code);
    let code_bin:Vec<char> = str_to_bin(&code_vec);
    let code_bin = add_space(&code_bin);
    // Display
    let mut cnt_l = 0;
    let mut cnt_b = 0;
    let mut t = 0.0;
    /* let a = (190000.0 * f32::sin(t)).abs();
    println!("{a}"); */
    loop{
        println!("{}|{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}|{:02}",
                 code_vec[cnt_l],
                 code_bin[cnt_b],
                 code_bin[cnt_b+1],
                 code_bin[cnt_b+2],
                 code_bin[cnt_b+3],
                 code_bin[cnt_b+4],
                 code_bin[cnt_b+5],
                 code_bin[cnt_b+6],
                 code_bin[cnt_b+7],
                 code_bin[cnt_b+8],
                 code_bin[cnt_b+9],
                 code_bin[cnt_b+10],
                 code_bin[cnt_b+11],
                 code_bin[cnt_b+12],
                 code_bin[cnt_b+13],
                 code_bin[cnt_b+14],
                 code_bin[cnt_b+15],
                 duration.as_millis(),
                 );
        cnt_b += 1;
        cnt_l += 1;
        if cnt_l >= code_vec.len(){
            cnt_l = 0;
        }
        if cnt_b + 16 >= code_bin.len(){
            cnt_b = 0;
        }

        /* // Sin()test
        let faf:f32 = f32::sin(t);
        println!("{t}|{}",faf);
        t += 10.0; */
        
        duration = time::Duration::from_millis((100.0 * f32::sin(t)).abs() as u64);
        t += 0.02;

        thread::sleep(duration);
    }
}

fn str_to_vec(code:&String)-> Vec<char>{
    let mut ret:Vec<char> = Vec::new();
    for i in code.chars() {
        ret.push(i);
    }
    ret
}

fn str_to_bin(code:&Vec<char>) -> Vec<char> {
    let mut code_u8:Vec<u8> = Vec::new();
    let mut code_bin:String = String::new();
    let mut ret:Vec<char> = Vec::new();
    for i in code{
        code_u8.push(*i as u8);
    }

    for i in code_u8{
        let bin:String = format!("{:b}",i);
        code_bin.push('0');
        code_bin = code_bin + &bin;
    }

    ret = str_to_vec(&code_bin);

    ret
}

fn add_space(code:&Vec<char>) -> Vec<char>{
    let mut ret:Vec<char> = code.to_vec();
    for _i in 0..16{
        ret.insert(0,' ');
        ret.push(' ');
    }
    ret

}
