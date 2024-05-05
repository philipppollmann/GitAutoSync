use std::path::PathBuf;
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

        for project_path in &project_list {
            println!("Checking directory for: {}", project_path.display());

            // Überprüfe, ob das Verzeichnis existiert
            if project_folders::check_directory_exists(project_path) {
                println!("Directory exists: {}", project_path.display());

                println!("Checking status for: {}", project_path.display());
                match git_commands::status(project_path) {
                    Ok(_) => println!("Status check for {} successful", project_path.display()),
                    Err(e) => println!("Error checking status for {}: {}", project_path.display(), e),
                }

                // Pull new changes
                println!("Pull newest version for {}", project_path.display());
                match git_commands::pull(project_path) {
                    Ok(_) => println!("Pull successful for {}", project_path.display()),
                    Err(e) => println!("Pull failed for {} due to: {}", project_path.display(), e),
                }

                // Add all files
                println!("Adding all files in {}", project_path.display());
                match git_commands::add(project_path) {
                    Ok(_) => println!("Added successful for {}", project_path.display()),
                    Err(e) => println!("Failed to add files for: {} with error {}", project_path.display(), e),
                }

                // Commit newest version
                println!("Committing changes in {}", project_path.display());
                match git_commands::commit(project_path, &message) {
                    Ok(_) => println!("Commit was successfully for {}", project_path.display()),
                    Err(e) => println!("Failed to commit due to: {} in project: {}", e, project_path.display()),
                }

                // Push the newest version
                println!("Pushing changes for {}", project_path.display());
                match git_commands::push(project_path) {
                    Ok(_) => println!("Push was successfully for {}", project_path.display()),
                    Err(e) => println!("Failed to push due to: {} in project: {}", e, project_path.display()),
                }
            } else {
                println!("Directory does not exist: {}, skipping...", project_path.display());
            }
        }

        thread::sleep(Duration::from_secs(3600));
    }
}
