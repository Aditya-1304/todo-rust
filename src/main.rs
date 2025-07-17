use std::{ fs, io::{self, Write}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool
}
fn main() {
    println!("");
    println!("");
    println!("Welcome to the ToDoz App!");
    print_help();
    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    ctrlc::set_handler(move || {
        println!("\nCtrl+C detected. Type 'exit' or 'quit' to close the application.");
    }).expect("NIGGA EVEN THE CTRL+C HANDLER AIN'T WORKING!!");

    loop {
        print!("\n>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't even read a fcking line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        let command = match parts.get(0) {
            Some(cmd) => *cmd,
            None => continue,
        };

        let arguments = &parts[1..];

        match command {
            "add" => {
                if arguments.is_empty() {
                    println!("Add a Fucking description dumbass who makes a todo with out description. What are u a retard??");
                    continue;
                }
                let description = arguments.join(" ");
                add_task(&mut tasks, description);
                save_tasks(&tasks).expect("This program sucks, It failed to save the task");
            }
            "list" => {
                list_task(&tasks);
            }
            "complete" => {
                if let Some(id_str) = arguments.get(0) {
                    match id_str.parse::<u32>() {
                        Ok(id) => {
                            complete_task(&mut tasks, id);
                            save_tasks(&tasks).expect("This program sucks, It failed to save the task");
                        }
                        Err(_) => println!("Are you serious, ID SHOULD BE A NUMBER for fuck sake!!"),
                    }
                } else {
                    println!("Nigga! Provide the ID of the task u want to complete, Fucking retard")
                }
                
            }
            "delete" => {
                if let Some(id_str) = arguments.get(0) {
                    match id_str.parse::<u32>() {
                        Ok(id) => {
                            delete_task(&mut tasks, id);
                            save_tasks(&tasks).expect("This program sucks, It failed to save the task");
                        }
                        Err(_) => println!("Are you serious, ID SHOULD BE A NUMBER for fuck sake")
                    }
                } else {
                    println!("Nigga! Provide the ID of the task u want to complete, Fucking retard");
                }
            }
            "help" => {
                print_help();
            }
            "exit" | "quit" => {
                println!("Exiting.");
                break;
            }        
            _=> {
                writeln!(io::stderr(), "MF, give the correct arguments what is {} are u dumb??",command).unwrap();
            }
        }
    }

    
}

fn print_help() {
    println!("\nAvailable commands:");
    println!("  add <description>  - Add a new task");
    println!("  list               - Show all tasks");
    println!("  complete <id>      - Mark a task as complete");
    println!("  delete <id>        - Delete a task");
    println!("  help               - Show this help message");
    println!("  exit / quit        - Close the application");
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) +1;
    let new_task = Task {
        id: new_id,
        description,
        completed: false,
    };
    tasks.push(new_task);
    println!("Added task with ID: {}", new_id)
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    let task_to_delete = tasks.iter().position(|t| t.id == id);

    if let Some(position) = task_to_delete {
        if !tasks[position].completed {
            print!("MF, This task is still incomplete. Do your bitch ass really want to delete it?? (y/n) ");
            io::stdout().flush().unwrap();

            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation).expect("IT AGAIN FAILED TO READ Confirmation.");

            if confirmation.trim().eq_ignore_ascii_case("y") {

            } else {
                println!("Deletion canceled.");
                return;
            }
        }
        let deleted_task = tasks.remove(position);
        println!("Deleted task {}: {}", id, deleted_task.description);
        save_tasks(tasks).expect("Failed to save tasks.");
    }  else {
        println!("Error: Task with ID {} not found.", id);
    }

}


fn list_task(tasks: &[Task]){
    if tasks.is_empty() {
        println!("No tasks yet!");
        return;
    }
    println!("{:<4} {:<6} {}", "ID", "Done", "Description");
    println!("{:-<4} {:-<6} {:-<20}", "", "", "");

    for task in tasks {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("{:<4} {:<6} {}", task.id, status, task.description)
    }
}

fn complete_task(tasks: &mut [Task], id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Completed task {}: {}", id , task.description)
    } else {
        println!("Give me the correct ID or I'll fuck u up, {} this is wrong mf", id)
    }
}

fn load_tasks() -> Result<Vec<Task>, io::Error> {
    let data = fs::read_to_string("tasks.json")?;
    let tasks: Vec<Task> = serde_json::from_str(&data).expect("fucking dumbass program failed to parse the tasks");
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<(), io::Error> {
    let data = serde_json::to_string_pretty(tasks).expect("This fucker again failed to serialize tasks.");
    fs::write("tasks.json", data)?;
    Ok(())
}

