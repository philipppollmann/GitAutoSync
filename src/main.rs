use std::thread;
use std::time::Duration;

mod message_generator;
mod project_folders;
mod git_commands;


fn main() {
    loop {
        let message = message_generator::generate_message();
        println!("{}", message);

        let project_list = project_folders::get_project_list();

        for project in &project_list {
            println!("Checking status for: {}", project);
            // printing the git status of the directory
            match git_commands::status(project) {
                Ok(_) => println!("Status check for {} successful", project),
                Err(e) => println!("Error checking status for {}: {}", project, e),
            }

            // pull new changes
            println!("Pull newest version");
            match git_commands::pull(project) {
                Ok(_) => println!("Pull successful for {}", project),
                Err(e) => println!("Pull failed for {}due to: {}",project, e),
            }

            // add all files
            println!("add all files");
            match git_commands::add(project) {
                Ok(_) => println!("Added successful for {}", project),
                Err(e) => println!("Failed to add files for: {} with error {}", project, e),
            }

            // commit newest version
            match git_commands::commit(project, message_generator::generate_message()) {
                Ok(_) => println!("Commit was successfully"),
                Err(e) => println!("Failed to commit due to: {} in project: {}", e, project),
            }

            // push the newest version
            match git_commands::push(project) {
                Ok(_) => println!("Push was successfully"),
                Err(e) => println!("Failed to push due to: {} in project: {}", e, project),
            }
        }

        thread::sleep(Duration::from_secs(3600));
    }
}