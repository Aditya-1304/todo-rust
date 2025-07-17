use std::{env, fs, io::{self, Write}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool
}
fn main() {
    let arguments : Vec<String> = env::args().collect();

    let command = arguments.get(1).expect("Provide a command Nigga: add, list, or complete.");

    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    match command.as_str() {
        "add" => {
            let description = arguments.get(2).expect("Add a Fucking description dumbass who makes a todo with out description. What are u a retard??").to_string();
            add_task(&mut tasks, description);
        }
        "list" => {
            list_task(&tasks);
        }
        "complete" => {
            let id_str = arguments.get(2).expect("Nigga! Provide the ID of the task u want to complete, Fucking retard");
            let id = id_str.parse::<u32>().expect("Are you serious, ID SHOULD BE A NUMBER for fuck sake");
            complete_task(&mut tasks, id);
        }
        "delete" => {
            let id_str = arguments.get(2).expect("Nigga! Provide the ID of the task u want to complete, Fucking retard");
            let id = id_str.parse::<u32>().expect("Are you serious, ID SHOULD BE A NUMBER for fuck sake");
            delete_task(&mut tasks, id);
        }
        _=> {
            writeln!(io::stderr(), "MF, give the correct arguments what is {} are u dumb??",command).unwrap();
        }
    }
    save_tasks(&tasks).expect("FAILED TO SAVE STUPID, ASS PROGRAM")
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
    if let Some(position) = tasks.iter().position(|t| t.id == id) {
        let deleted_task = tasks.remove(position);
        println!("Deleted task {}:{}", id, deleted_task.description);
    }else {
        println!("Stupid ass!, give me the correct ID, What the is this {} ??", id);
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

