use std::{thread, time};

fn main() {
    // let a = b'O';
    // let b = a + 1;
    // println!("{a},{b}");
    // let c:char = b as char;
    // println!("{c}");
    // let d:u8 = c as u8;
    // println!("{d}");
    // println!("{:#b}",b'O');
    // let c:char = 'O';
    // let chara:u8 = c as u8;
    // println!("{:#b}",chara);
    let oaoa_list:[char;16] = ['O','n','c','e','A','n','d','O','n','c','e','A','g','a','i','n'];
    show(oaoa_list); 
}

fn show(list:[char;16]){

    let duration = time::Duration::from_millis(50);
    let mut i = 0;
    let mut cnt: u64 = 0;
    loop{
        let bin:u8 = list[i] as u8;
        println!("{}|{:02}|{:#b}|{}",list[i],i,bin,cnt);
        thread::sleep(duration);
        i += 1;
        cnt += 1;
        if i==16 {
            i = 0;
        }
    }
}

// fn show(list:[char;16]){
//     let duration = time::Duration::from_millis(100);
//     for i in 0..16{
//         let bin:u8 = list[i] as u8;
//         println!("{}|{}|{:#b}",list[i],i,bin);
//         thread::sleep(duration);
//
//     }
// }


// fn letter(l:char)-> &str{
//     let ret1: &str = "b'";
//     let ret2: &str = "''";
//     let ret: &str = push_str(ret1).l.push_str(ret2);
//     ret
// }
