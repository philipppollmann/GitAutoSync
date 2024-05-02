mod git_commands;
mod message_generator;


fn main() {
    let message = message_generator::generate_message();
    format!("{}", message);
}
