mod todo;
mod cli;


fn main() {
    println!("Welcome to your Todo Application");

    loop {
        cli::show_main_menu();
        let choice = cli::get_initial_choice();         

        match choice {
           1 => println!("Display Todos"),
           2 => println!("Add Todos"),
           3 => println!("Complete Tasks"),
           4 => println!("Pending Tasks"),
           5 => println!("Delete Tasks"),
           6 => {
               println!("Goodbye !!");
                break;
           }
           _ => println!("Invalid choice")
        }
    }
   
}



