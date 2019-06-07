use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

use super::conz;
use super::conz::Printable;
use super::data;
use super::astr;
use super::astr::{AStr};
use super::state;
use super::support;
use super::wizard::{Wizardable};
use super::save;

pub fn help_cli(){
    conz::println_type("pplanner is an TUI/CLI program to manage your time.", conz::MsgType::Normal);
    conz::println_type("To use it, start it and type commands in its prompt.", conz::MsgType::Normal);
    conz::print_type("Type ", conz::MsgType::Normal);
    conz::print_type("help", conz::MsgType::Highlight);
    conz::println_type(" in its prompt to get help on commands.", conz::MsgType::Normal);
    conz::println_type("Give a pplanner command as cli argument to run it directly from the terminal.", conz::MsgType::Normal);
    conz::print_type("For example: ", conz::MsgType::Normal);
    conz::println_type("pplanner \'ls todos\'", conz::MsgType::Highlight);
    conz::print_type("pplanner is made by ", conz::MsgType::Normal);
    conz::println_type("Cody Bloemhard.", conz::MsgType::Prompt);
}

pub fn now(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("Today:", conz::MsgType::Normal);
    let dt = data::DT::new();
    conz::print_type(dt.str_datetime(), conz::MsgType::Value);
    conz::print(" ");
    conz::println_type(dt.str_dayname(), conz::MsgType::Value);
}

pub fn license(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let path = save::get_data_dir_path("LICENSE");
    if path.is_none(){
        conz::println_type("Error: Could not find license file.", conz::MsgType::Error);
        return;
    }
    let path = path.unwrap();
    let metatdata = std::fs::metadata(path.as_path());
    if metatdata.is_err(){
        conz::println_type("Error: Could not find license file.", conz::MsgType::Error);
        return;
    }
    let f = File::open(path.as_path());
    if f.is_err(){
        conz::println_type("Error: could not open file.", conz::MsgType::Error);
        return;
    }
    let mut f = f.unwrap();
    let mut string = String::new();
    let ok = f.read_to_string(&mut string);
    if ok.is_err(){
        conz::println_type("Error: could not read file.", conz::MsgType::Error);
        return;
    }
    conz::println_type(string, conz::MsgType::Normal);
}

pub fn help(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_inputs(&inputs);
    if args.len() == 0{
        conz::print_type("Help, type ", conz::MsgType::Normal);
        conz::print_type("help(command) ", conz::MsgType::Highlight);
        conz::println_type("to find help.", conz::MsgType::Normal);
        conz::print_type("For example: ", conz::MsgType::Normal);
        conz::print_type("help (mk point)", conz::MsgType::Highlight);
        conz::println_type(".", conz::MsgType::Normal);
        conz::print_type("To list all commands use ", conz::MsgType::Normal);
        conz::print_type("ls commands", conz::MsgType::Highlight);
        conz::println_type(".", conz::MsgType::Normal);
        return;
    }
    let mut path = std::path::PathBuf::from("./help");
    let mut metatdata = std::fs::metadata(path.as_path());
    if metatdata.is_err(){
        let res = save::get_data_dir_path("help");
        if res.is_some(){
            path = res.unwrap();
            metatdata = std::fs::metadata(path.as_path());
        }
    }
    if metatdata.is_err(){
        conz::println_type("Error: Help directory not found.", conz::MsgType::Error);
        return;
    }
    let res = state.fset.contains(&args[0]);
    if !res {
        conz::println_type("Fail: command does not exist, so help for it neither.", conz::MsgType::Error);
        return;
    }
    path.push(astr::unsplit(&args[0].split_str(&astr::astr_whitespace()), '_' as u8).to_string());
    let res = std::fs::metadata(path.clone());
    if res.is_err(){
        conz::println_type("Error: help file not found.", conz::MsgType::Error);
        return;
    }
    let f = File::open(path.as_path());
    if f.is_err(){
        conz::println_type("Error: could not open file.", conz::MsgType::Error);
        return;
    }
    let mut f = f.unwrap();
    let mut string = String::new();
    let ok = f.read_to_string(&mut string);
    if ok.is_err(){
        conz::println_type("Error: could not read file.", conz::MsgType::Error);
        return;
    }
    conz::print_type("Command: ", conz::MsgType::Normal);
    conz::println_type(astr::unsplit(&args, ' ' as u8).to_string(), conz::MsgType::Highlight);
    conz::println_type(string, conz::MsgType::Normal);
}

pub fn ls_commands(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("All commands: ", conz::MsgType::Normal);
    for f in state.fset.clone(){
        conz::println_type(f, conz::MsgType::Normal);
    }
}

pub fn mk_point(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Add point: ", conz::MsgType::Normal);
    let fields = data::Point::get_fields(false);
    let res = fields.execute(&mut inputs);
    if res.is_none() {return;}
    let mut res = res.unwrap();
    let point = data::Point::extract(&mut res);
    if point.is_none() {return;}
    state.points.add_item(point.unwrap());
    if !state.points.write() {return;}
    conz::println_type("Success: Point saved.", conz::MsgType::Highlight);
}

pub fn rm_points(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    let items = state.points.get_items().clone();
    support::rm_items(items, &mut state.points, &mut state.points_archive, &mut inputs);
}

pub fn clean_points(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Remove all points that are in the past: ", conz::MsgType::Normal);
    match conz::read_bool("Sure to remove them?: ", &mut inputs){
        true =>{}
        false =>{return;}
    }
    let points = state.points.get_items().clone();
    let mut vec = Vec::new();
    let now = data::DT::new();
    for i in 0..points.len(){
        if !now.diff(&points[i].dt).neg{
            break;
        }
        vec.push(i);
    }
    support::remove_and_archive(&mut state.points, &mut state.points_archive, vec, &points);
}

pub fn edit_points(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    check_unsupported_inputs!(inputs);
    support::edit_items(&mut state.points);
}

pub fn ls_points(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    support::pretty_print(state.points.get_items(), &data::DT::new());
}

pub fn ls_points_archive(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let res = state.points_archive.read();
    support::pretty_print(&res, &data::DT::new());
}

pub fn inspect_point(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Inspect point(search first): ", conz::MsgType::Normal);
    loop{
        let points = state.points.get_items();
        let (match_res, vec) = support::get_matches(&points,&mut inputs);
        if match_res == support::MatchResult::None || vec.len() > 1{
            if vec.len() > 1{
                conz::println_type("Fail: more than one result.", conz::MsgType::Error);
            }else{
                conz::println_type("Fail: no results found.", conz::MsgType::Error);
            }
            if inputs.is_some() {return;}
            match conz::read_bool("Try again?: ", &mut Option::None){
                true =>{continue;}
                false =>{return;}
            }
        }
        points[vec[0]].print();
        let now = data::DT::new();
        let diff = now.diff(&points[vec[0]].dt);
        diff.print();
        return;
    }
}

pub fn mk_todo(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Add todo: ", conz::MsgType::Normal);
    let fields = data::Todo::get_fields(false);
    let res = fields.execute(&mut inputs);
    if res.is_none() {return;}
    let mut res = res.unwrap();
    let todo = data::Todo::extract(&mut res);
    if todo.is_none() {return;}
    state.todos.add_item(todo.unwrap());
    if !state.todos.write() {return;}
    conz::println_type("Success: Todo saved.", conz::MsgType::Highlight);
}

pub fn rm_todos(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    let items = state.todos.get_items().clone();
    support::rm_items(items, &mut state.todos, &mut state.todos_archive, &mut inputs);
}

pub fn edit_todos(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    check_unsupported_inputs!(inputs);
    support::edit_items(&mut state.todos);
}

pub fn ls_todos(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let (to,lo,id) = support::split_todos(state.todos.get_items());
    conz::print_type("Todo: ", conz::MsgType::Normal);
    support::pretty_print(&to, &false);
    conz::print_type("Longterm: ", conz::MsgType::Normal);
    support::pretty_print(&lo, &false);
    conz::print_type("Idea: ", conz::MsgType::Normal);
    support::pretty_print(&id, &false);
}

pub fn ls_todos_archive(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let res = state.todos_archive.read();
    support::pretty_print(&res, &true);
}

pub fn status(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    now(state, args.clone(), inputs.clone());
    ls_points(state, args.clone(), inputs.clone());
    ls_todos(state, args.clone(), inputs.clone());
}

pub fn flush_files(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    if state.is_clean() {
        conz::println_type("All files clean, nothing to do.", conz::MsgType::Highlight);
        return;
    }
    let res = state.flush_files();
    if res {
        conz::println_type("Success: Flushed all dirty files.", conz::MsgType::Highlight);
    }else{
        conz::println_type("Error: Could not flush all dirty files.", conz::MsgType::Error);
    }
}

pub fn test_keys(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("Testing keys, press any key to get id, exit program to stop.", conz::MsgType::Normal);
    conz::test_chars();
}
