use std::{thread, time};
use midir::{MidiInput, MidiInputConnection, Ignore};
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main()  -> Result<(), Box<dyn Error>> {
    let duration = time::Duration::from_millis(50);
    let mut change = 1;
    let mut cnt = 0;
    let code:String = String::from("OnceAndOnceAgain");
    let code_vec:Vec<char> = str_to_vec(&code);
    let code_bin:Vec<char> = char_to_bin(&code_vec);
    let mut code_o:Vec<char> = shift(&code_bin, change); 
    //for Midi

    let mut midi_in = MidiInput::new("My MIDI input")?;
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();

    let in_port = match in_ports.len() {
        0 => return Err("No MIDI input ports available.".into()),
        1 => {
            println!("Choosing the only available MIDI input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("\nAvailable MIDI input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?).ok_or("Invalid input port selected.")?
        }
    };

    let mut number = 0;

    println!("\nOpening connection");
    let conn_in: MidiInputConnection<()> = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            // println!("{}: {:?} (len = {})", stamp, message, message.len());
            // println!("{}",message[1]);
            number = message[1] as isize - 51;
                let code_s = shift(&code_o,number);
                let code_grc:Vec<char> = bin_to_grc(&code_s);
                let code_output:String = vec_to_str(&code_grc);
                println!("{}",code_output);
                

        },
        (),
    )?;

    //

    /* loop {
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

    } */
    
    println!("Connection open. Press Enter to exit.");
    let mut input = String::new();
    stdin().read_line(&mut input)?;  // Wait for user input

    println!("Closing connection");
    drop(conn_in);  // Explicitly close the connection
    Ok(())


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

fn shift(code:&Vec<char>, c:isize) -> Vec<char> {
    let mut ret:Vec<char> = Vec::new();
    let mut index = 0;
    let mut d:usize = c.abs() as usize;
    for i in 0..code.len(){
        index = i + d;
        if i + d >= code.len() {
            index = index - code.len();
        }
        let bit:char = code[index];
        ret.push(bit);

    }
    ret
}

