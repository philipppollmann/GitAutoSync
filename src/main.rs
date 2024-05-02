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

            for project in project_list {
                println!("{}", project);
            }

            thread::sleep(Duration::from_secs(3600));
        }

    }
