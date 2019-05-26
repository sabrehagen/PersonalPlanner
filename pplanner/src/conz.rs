use std::io;
use std::io::Write; //flush stdout
use std::collections::VecDeque;
use std::io::Read;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use super::astr;
use super::astr::{TOSTRING};

#[derive(Clone)]
pub enum MsgType {
    Normal,
    Error,
    Prompt,
    Highlight,
    Value,
}

pub trait Printable{
    fn print(&self);
}

pub trait PrettyPrintable{
    type ArgType;
    fn pretty_print(&self, arg: &Self::ArgType) -> (astr::AstrVec,Vec<MsgType>);
    fn lengths(arg: &Self::ArgType) -> Vec<u16>;
    fn titles(arg: &Self::ArgType) -> Vec<astr::Astr>;
}

fn set_colour(msgtype: MsgType){
    let colorcode = match msgtype {
        MsgType::Normal => "\x1B[32m",
        MsgType::Error => "\x1B[31m",
        MsgType::Prompt => "\x1B[36m",
        MsgType::Highlight => "\x1B[37m",
        MsgType::Value => "\x1B[33m",
    };
    print!("{}", colorcode);
}

pub fn print<T: astr::TOSTRING>(msg: T){
    set_colour(MsgType::Normal);
    print!("{}", msg.tostring());
}

pub fn print_type<T: astr::TOSTRING>(msg: T, msgtype: MsgType){
    set_colour(msgtype);
    print!("{}", msg.tostring());
}

pub fn print_error<T: astr::TOSTRING>(pre: T, mid: T, pos: T){
    set_colour(MsgType::Error);
    print!("{}", pre.tostring());
    set_colour(MsgType::Highlight);
    print!("{}", mid.tostring());
    set_colour(MsgType::Error);
    print!("{}", pos.tostring());
}

pub fn println<T: astr::TOSTRING>(msg: T){
    set_colour(MsgType::Normal);
    println!("{}", msg.tostring());
}

pub fn println_type<T: astr::TOSTRING>(msg: T, msgtype: MsgType){
    set_colour(msgtype);
    println!("{}", msg.tostring());
}

pub fn println_error<T: astr::TOSTRING>(pre: T, mid: T, pos: T){
    set_colour(MsgType::Error);
    print!("{}", pre.tostring());
    set_colour(MsgType::Highlight);
    print!("{}", mid.tostring());
    set_colour(MsgType::Error);
    println!("{}", pos.tostring());
}

fn getch() -> u8{
    //https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();
    return buffer[0];
}

/*fn test_chars(){
    loop {
        print!("{:?}", getch());
    }
}*/

fn custom_inp() -> astr::Astr{
    fn typed_char(ch: u8, buff: &mut Vec<u8>, astate: &mut u8){
        print!("{}", ch as char);
        buff.push(ch);
        *astate = 0;
    }
    let mut res = astr::new();
    let mut arrow_state: u8 = 0;
    set_colour(MsgType::Normal);
    loop {
        match getch(){
            10 => { print!("\n"); break; } //enter
            127 => {  //backspace
                if res.len() <= 0 { continue; }
                res.pop();
                for _ in 0..res.len() + 1 {
                    print!("{}", 8 as char);
                }
                let mut printres = res.clone();
                printres.push(' ' as u8);
                print(printres);
                print!("{}", 8 as char);
                //print!("\x1B[1D"); //also works
                arrow_state = 0;
            }
            27 => { arrow_state = 1; } //first char in arrow code
            91 => { if arrow_state == 1 { arrow_state = 2; } } //2nd char in arrow code
            65 => { //up arrow 
                if arrow_state == 2 {}
                else { typed_char(65, &mut res, &mut arrow_state); }
            }
            66 => { //down arrow 
                if arrow_state == 2 {}
                else { typed_char(66, &mut res, &mut arrow_state); }
            }
            67 => {  //right arrow
                if arrow_state == 2 { print!("\x1B[1C"); arrow_state = 0; }
                else { typed_char(67, &mut res, &mut arrow_state); }
            }
            68 => {  //left arrow
                if arrow_state == 2 { print!("{}", 8 as char); arrow_state = 0; }
                else { typed_char(68, &mut res, &mut arrow_state); }
            }
            x => { typed_char(x, &mut res, &mut arrow_state); }
        }
    }
    return res;
}

pub fn prompt(msg : &str) -> String{
    print_type(msg, MsgType::Prompt);
    std::io::stdout().flush().expect("Error: stdout flush failed.");
    return custom_inp().tostring();
}

pub fn read_bool(msg: &str, inputs: &mut Option<VecDeque<astr::Astr>>) -> bool{
    let line;
    if inputs.is_none(){line = prompt(&msg);}
    else{
        let res = inputs.as_mut().unwrap().pop_front();
        if res.is_none(){line = prompt(&msg);}
        else {line = res.unwrap().tostring();}
    }
    match line.as_ref(){
        "y" => true,
        "ye" => true,
        "yes" => true,
        "ok" => true,
        "+" => true,
        _ => false,
    }
}
