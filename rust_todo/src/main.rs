use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    let mut completed_tasks: Vec<bool> = Vec::new();
    loop {
        println!(
            "
===== TODO MANAGER =====

1. Add task
2. List tasks
3. Complete task
4. Delete task
5. Exit
"
        );

        let mut choice = String::new();
        println!("Enter your choice:");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let mut task = String::new();
                println!("Enter the task:");
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read input");
                add_task(&mut tasks, task, &mut completed_tasks);
            }
            "2" => {
                list_tasks(&tasks, &completed_tasks);
            }
            "3" => {
                let mut index = String::new();
                println!("Enter the task number to complete:");
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                let index: usize = index.trim().parse().expect("Failed to parse input");
                complete_task(&mut completed_tasks, index - 1);
            }
            "4" => {
                let mut index = String::new();
                println!("Enter the task number to delete:");
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                let index: usize = index.trim().parse().expect("Failed to parse input");
                delete_task(&mut tasks, &mut completed_tasks, index - 1);
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice! Please try again.");
            }
        }
    }
}

fn add_task(tasks: &mut Vec<String>, task: String, completed_tasks: &mut Vec<bool>) {
    tasks.push(task);
    completed_tasks.push(false);
    println!("Task added successfully!");
}

fn list_tasks(tasks: &Vec<String>, completed_tasks: &Vec<bool>) {
    for (index, task) in tasks.iter().enumerate() {
        if completed_tasks[index] == false {
            println!("{}: [ ] {} ", index + 1, task);
        } else {
            println!("{}: [✓] {} ", index + 1, task);
        }
    }
}

fn complete_task(completed_tasks: &mut Vec<bool>, index: usize) {
    if index < completed_tasks.len() {
        completed_tasks[index] = true;
        println!("Task marked as completed!");
    } else {
        println!("Invalid task number!");
    }
}

fn delete_task(tasks: &mut Vec<String>, completed_tasks: &mut Vec<bool>, index: usize) {
    if index < tasks.len() {
        tasks.remove(index);
        completed_tasks.remove(index);
        println!("Task deleted successfully!");
    } else {
        println!("Invalid task number!");
    }
}
