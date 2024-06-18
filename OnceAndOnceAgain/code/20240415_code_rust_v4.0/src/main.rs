use OaOA_20240415::Letter;
use OaOA_20240415::outputString;
use std::{thread, time};

fn main() {
    run3();

}

fn run1(){
    let code = "OnceAndOnceAgain";
    let mut sentence:Vec<Letter> = code.chars()
        .map(|c| Letter::new(&c))
        .collect();

    let duration = time::Duration::from_millis(300);
    loop {
        for letter in &sentence {
            for i in (0..8) {
                let mut letterOut = Letter::derefer(letter);
                letterOut = Letter::indexedLetter(letterOut, i);
                println!("   INDEX: {} |{:12}| ASCII: {:3} | BINARY: {} | PATTERN: {}",
                         letterOut.shift,
                         letterOut.image,
                         letterOut.uint8,
                         outputString(&letterOut.bin),
                         outputString(&letterOut.bin_graphic)
                        );
                thread::sleep(duration);
            }
            println!("--------------------------------------------------------------------------------");
        }
    }
}

fn run2(){

    let code = "OnceAndOnceAgain";
    let mut sentence:Vec<Letter> = code.chars()
        .map(|c| Letter::new(&c))
        .collect();

    let duration = time::Duration::from_millis(10);
    loop {
        for letter in &sentence {
            for i in (0..8) {
                let mut letterOut = Letter::derefer(letter);
                letterOut = Letter::indexedLetter(letterOut, i);
                print!("{}",
                         outputString(&letterOut.bin),
                        );
                thread::sleep(duration);
            }
            print!("\n");
        }
    }
}


fn run3(){

    let code = "OnceAndOnceAgain";
    let mut sentence:Vec<Letter> = code.chars()
        .map(|c| Letter::new(&c))
        .collect();

    let duration = time::Duration::from_millis(10);
    loop {
        for letter in &sentence {
            print!("{} |",letter.letter);
            for i in (0..8) {
                let mut letterOut = Letter::derefer(letter);
                letterOut = Letter::indexedLetter(letterOut, i);
                print!("{}",
                         outputString(&letterOut.bin_graphic),
                        );
                thread::sleep(duration);
            }
            print!("\n");
        }
    }
}
