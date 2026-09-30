use std::io::{self};

pub fn show_main_menu(){
    println!("-------------------");
    println!("------MY TODOS-----");
    println!("1. Display todos");
    println!("2. Add Todo");
    println!("3. Complete Tasks");
    println!("4. Pending Tasks");
    println!("5. Delete Tasks");
    println!("6. Exit");
}

pub fn get_initial_choice() -> u8 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
            Ok(_) => input.trim().parse::<u8>().unwrap_or_default(),
            Err(_) => 0
        }
}

pub fn get_todo_title() -> String {
    let mut todo_title = String::new();
    match io::stdin().read_line(&mut todo_title) {
        Ok(_) => todo_title.trim().to_string(),
        Err(e) => {
            println!("{}", e);
            String::new()
        } 
    }
}
