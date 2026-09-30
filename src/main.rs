mod todo;

use std::io::{self};
use std::time::SystemTime;

#[derive(Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: SystemTime,
}

fn main() {
    println!("Welcome to your Todo Application");
    let mut todos: Vec<Todo> = Vec::new();

    todos.push(Todo {
        id: 1,
        title: String::from("Learn Rust"),
        completed: false,
        created_at: SystemTime::now(),
    });

    println!("Enter a new todo item ..");
    let mut title = String::new();

    loop {
        match io::stdin().read_line(&mut title) {
            Ok(_) => {
                if title.trim().is_empty() {
                    println!("Enter a proper todo !!");
                    continue;
                }

                println!("Creating new todo ...");
                add_todos(&mut todos, &title);
                break;
            }

            Err(e) => {
                println!("{}", e);
            }
        }
    }
    list_todos(&todos);
}

fn add_todos(todo_list: &mut Vec<Todo>, title: &str) {
    todo_list.push(Todo {
        id: todo_list.len() + 1,
        title: title.to_string(),
        completed: false,
        created_at: SystemTime::now(),
    })
}

fn list_todos(todo_list: &[Todo]) {
    let mut sorted_todos: Vec<&Todo> = todo_list.iter().collect();
    sorted_todos.sort_by_key(|todo| todo.created_at);
    for todo in sorted_todos {
        println!(
            "{}. [{}] {}",
            todo.id,
            if todo.completed { "x" } else { " " },
            todo.title
        );
    }
}
