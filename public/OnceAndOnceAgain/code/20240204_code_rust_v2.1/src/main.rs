use std::{thread,time};

fn main() {
    let cap_o: &str = "01001111";
    let n: &str = "01001110";
    let c: &str = "01000011";
    let e: &str = "01100101";
    let cap_a: &str = "01000001";
    let d: &str = "01100100";
    let a: &str = "01100001";
    let g: &str = "01100111";
    let i: &str = "01101001";
    let grc_cap_o: &str = " ▇  ▇▇▇▇";
    let grc_n: &str = " ▇▇ ▇▇▇ ";    
    let grc_c: &str = " ▇▇   ▇▇";
    let grc_e: &str = " ▇▇  ▇ ▇"; 
    let grc_cap_a: &str = " ▇     ▇"; 
    let grc_d: &str = " ▇▇  ▇  "; 
    let grc_a: &str = " ▇▇    ▇"; 
    let grc_g: &str = " ▇▇  ▇▇▇";    
    let grc_i: &str = " ▇▇ ▇  ▇";    

    let duration = time::Duration:: from_millis(500);

    loop{

    println!("O|{}|{}{}{}{}",cap_o,grc_cap_o,grc_cap_o,grc_cap_o,grc_cap_o);
    thread::sleep(duration);
    println!("n|{}|{}{}{}{}",n,grc_n,grc_n,grc_n,grc_n);
    thread::sleep(duration);
    println!("c|{}|{}{}{}{}",c,grc_c,grc_c,grc_c,grc_c);
    thread::sleep(duration);
    println!("e|{}|{}{}{}{}",e,grc_e,grc_e,grc_e,grc_e);
    thread::sleep(duration);
    println!("A|{}|{}{}{}{}",cap_a,grc_cap_a,grc_cap_a,grc_cap_a,grc_cap_a);
    thread::sleep(duration);
    println!("n|{}|{}{}{}{}",n,grc_n,grc_n,grc_n,grc_n);
    thread::sleep(duration);
    println!("d|{}|{}{}{}{}",d,grc_d,grc_d,grc_d,grc_d);
    thread::sleep(duration);
    println!("O|{}|{}{}{}{}",cap_o,grc_cap_o,grc_cap_o,grc_cap_o,grc_cap_o);
    thread::sleep(duration);
    println!("n|{}|{}{}{}{}",n,grc_n,grc_n,grc_n,grc_n);
    thread::sleep(duration);
    println!("c|{}|{}{}{}{}",c,grc_c,grc_c,grc_c,grc_c);
    thread::sleep(duration);
    println!("e|{}|{}{}{}{}",e,grc_e,grc_e,grc_e,grc_e);
    thread::sleep(duration);
    println!("A|{}|{}{}{}{}",cap_a,grc_cap_a,grc_cap_a,grc_cap_a,grc_cap_a);
    thread::sleep(duration);
    println!("g|{}|{}{}{}{}",g,grc_g,grc_g,grc_g,grc_g);
    thread::sleep(duration);
    println!("a|{}|{}{}{}{}",a,grc_a,grc_a,grc_a,grc_a);
    thread::sleep(duration);
    println!("i|{}|{}{}{}{}",i,grc_i,grc_i,grc_i,grc_i);
    thread::sleep(duration);
    println!("n|{}|{}{}{}{}",n,grc_n,grc_n,grc_n,grc_n);
    thread::sleep(duration);

    }
}

  /* string Ob = "01001111";
  string nb = "01101110";
  string cb = "01100011";
  string eb = "01100101";
  string Ab = "01000001";
  string db = "01100100";
  string ab = "01100001";
  string gb = "01100111";
  string ib = "01101001";
  string O = " ▇  ▇▇▇▇";
  string n = " ▇▇ ▇▇▇ ";
  string c = " ▇▇   ▇▇";
  string e = " ▇▇  ▇ ▇";
  string A = " ▇     ▇";
  string d = " ▇▇  ▇  ";
  string a = " ▇▇    ▇";
  string g = " ▇▇  ▇▇▇";
  string i = " ▇▇ ▇  ▇";
*/
