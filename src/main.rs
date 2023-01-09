mod task;
mod task_list;

use self::task_list::TaskList;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The function to run.
    function: String,

    // The name of a task.
    name: Option<String>,

    // The description of a task.
    description: Option<String>,
}

fn main() {
    // print_title();

    println!("Rust List v0.1.0");
    println!("A simple command line task list written in Rust.");
    println!("Created by: @tharrisondev");
    println!("------------------------------------------------");

    // Print the current directory.
    println!(
        "Current Task List: {}",
        std::env::current_dir().unwrap().display()
    );
    println!("------------------------------------------------");

    let args = Args::parse();

    let mut task_list = TaskList::new();

    match args.function.as_str() {
        "add" => {
            // Check that we have a name and description before proceeding.
            if args.name.is_none() || args.description.is_none() {
                println!("Please provide a name and description.");
                return;
            }

            // Add the task to the list.
            task_list.add(args.name.unwrap(), args.description.unwrap());

            // A friendly message to let the user know that the task was added successfully.
            println!("Task Added Successfully!");
            print_task_list(&task_list);
        }
        "remove" => {
            // Print the list of tasks so the user knows which task to remove.
            println!("Which task would you like to remove?");
            println!("-----------------------------------");
            print_task_list(&task_list);

            let task_num = get_task_number();

            // Check that we have a task number before proceeding.
            if task_num.is_none() {
                return;
            }

            task_list.remove(task_num.unwrap());
            println!("remove");
        }
        "list" => {
            // println!("[{}] | {}: {}", "Done? (✓)", "Name", "Description");

            print_task_list(&task_list);
        }
        "toggle" => {
            // Print the list of tasks so the user knows which task to toggle.
            println!("Which task would you like to toggle?");
            println!("-----------------------------------");
            print_task_list(&task_list);

            // Get the task number from the user.
            let task_num = get_task_number();

            // Check that we have a task number before proceeding.
            if task_num.is_none() {
                return;
            }

            task_list.toggle(task_num.unwrap());
            println!("toggle");
        }
        "clear" => {
            task_list.clear();
            println!("Task List Cleared!");
        }
        _ => {}
    }

    // Save the changes to the task list.
    task_list.save();
}


/// Gets a list of tasks from the user.
///
/// # Panics
///
/// Panics if .
fn get_task_number() -> Option<usize> {
    // Get the task number from the user.
    let mut task_num = String::new();
    std::io::stdin().read_line(&mut task_num).unwrap();

    // Check that the user entered a number.
    if task_num.trim().parse::<usize>().is_err() {
        println!("Please enter a number.");
        return None;
    }

    // Convert to usize.
    let task_num = task_num.trim().parse::<usize>().unwrap();
    return Some(task_num);
}

/// Print the list of tasks located in this directory, if any.
fn print_task_list(task_list: &TaskList) {
    // If there are no tasks, print a message to the user.
    let formatted_tasks = task_list.format();
    if formatted_tasks.len() == 0 {
        println!("There are no tasks in the list! Good job!");
        return;
    }

    // Print the tasks with a number next to each.
    for (i, task) in formatted_tasks.iter().enumerate() {
        println!("{}) {}", i, task);
    }
}
