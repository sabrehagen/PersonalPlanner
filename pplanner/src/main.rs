extern crate termcolor;
extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;
extern crate num_derive;

#[macro_use]
mod conz;
mod parser;
mod data;
mod astr;
mod save;
mod wizard;
mod state;
mod commands;
mod misc;
mod support;

use conz::PrinterFunctions;

fn main() {
    let ok = save::setup_config_dir();
    if !ok {return;}
    let state = state::State::new();
    if state.is_none() {
        pprintln_type!(&"Error: Could not create state.", conz::MsgType::Error);
        return;
    }
    let mut parser = parser::Parser::new(state.unwrap());
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        parser::process_cli_args(args, &mut parser);
    }else{
        parser.start_loop();
    }
}
