use std::{thread, time};

fn main() {
    let duration = time::Duration::from_millis(50);
    let char_oaoa: [char;16] = ['O','n','c','e','A','n','d','O','n','c','e','A','g','a','i','n'];
    let bin_oaoa:[u8;16] = char_to_bin(&char_oaoa);


    /* let str_test: String = format!("{:b}",bin_oaoa[0]);
    println!("0{}",str_test); */

    /* let str1 = String::from("aaaa");
    let str2 = String::from("bbbb");
    let str3:String = string_merge(&str1, &str2);
    println!("{}",str3); */
    
    let mut output: String = String::from("");
    for i in 0..16 {
        // if i == 0{
        // output = ele_to_bin_string(bin_oaoa[i]) ;
        // }
        let container:String = ele_to_bin_string(bin_oaoa[i]);
        output =string_merge(&output, &container);
    }
    let mut output_grc:String = cvrt_bin_to_grc(output); 

    let mut shift_num = 1;
    let mut cnt = 0;
    loop{

        output_grc = shift(&output_grc,shift_num);
        // shift_num += 1;
        // if shift_num ==126{
        //     shift_num = 0;
        // }
        println!("{:03}|{}|{}",cnt,shift_num,output_grc);
        cnt +=1;
        if cnt == 128{
            cnt = 0;
        }
        thread::sleep(duration);
    };

}

fn char_to_bin(list:&[char;16])->[u8;16]{
    let mut list_out:[u8;16] = [0;16];
    for i in 0..16 {
        list_out[i] = list[i] as u8;
    }
    list_out
}

fn string_merge(str1:&String, str2:&String)->String{
    let mut ret:String = str1.to_string();
    ret.push_str(&str2);
    ret
}

fn ele_to_bin_string(ch:u8)->String{
    let mut ret:String = format!("{:b}",ch);
    ret.insert(0,'0');
    ret
}

fn cvrt_bin_to_grc(str:String) -> String{
    let mut cnt:usize = 0;
    let mut ret_arr: [char;128] = [' ';128];
    for c in str.chars() {
        if c=='0' {
           ret_arr[cnt] = ' '; 
        }else{
            ret_arr[cnt] = '▇'
        }
        cnt +=1;
    }
    let ret:String = ret_arr.iter().collect();
    ret
}

fn shift(str:&String, n:usize)->String{
    let mut cnt:usize = 0;
    let mut ret_arr:[char;128] = [' ';128];
    // ret_arr = str.chars();
    for c in str.chars() {
        ret_arr[cnt] = c;
        cnt += 1;
    }
    
    for  i in 0..128{
        if i+n >= 128{
            ret_arr[i] = ret_arr[i+n-128];
        }else{
            ret_arr[i] = ret_arr[i+n];

        }
    }
    let ret:String = ret_arr.iter().collect();
    ret
    
}
