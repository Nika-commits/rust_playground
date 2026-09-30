mod todo;
mod cli;


fn main() {
    println!("Welcome to your Todo Application");
    let mut todos: Vec<todo::Todo> = Vec::new();

    loop {
        cli::show_main_menu();
        let choice = cli::get_initial_choice();         

        match choice {
           1 => println!("Display Todos"),
           2 => {
               println!("Enter todo title.");
               let title = cli::get_todo_title();
               if title.is_empty(){
                   println!("Title cannot be is_empty");
                   continue;
               }
               
               let todo_id = todos.len() + 1; 
               let new_todo = todo::Todo::new(todo_id, &title);
               todos.push(new_todo);

               println!("Todo added successfully.")
           }, 
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



