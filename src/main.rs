use std::time::SystemTime;

#[derive(Debug)]
struct Todo{
    id: usize,
    title: String,
    completed: bool,
    created_at: SystemTime,
}

fn main(){
   println!("Welcome to your Todo Application");     
    let mut todos: Vec<Todo> =  Vec::new();

    todos.push(Todo {
        id: 1, 
        title: String::from("Learn Rust"),
        completed: false,
        created_at: SystemTime::now() 
    });

    add_todos(&mut todos, "New Task");
    list_todos(&todos);
} 

fn add_todos(todo_list: &mut Vec<Todo>, title: &str){
    todo_list.push( Todo{
        id: todo_list.len() + 1,
        title: title.to_string(), 
        completed: false,
        created_at: SystemTime::now() 
    }
    )
}

fn list_todos(todo_list: &Vec<Todo>){
    for todo in todo_list {
        println!(
            "{}. [{}] {}",
            todo.id,
            if todo.completed {"x"} else {" "},
            todo.title
        );
    }
}
