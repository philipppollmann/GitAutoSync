use std::process::Command;

fn add(folder_path: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(folder_path)
        .arg("add")
        .arg(".")
        .status()
        .expect("failed to execute git add");

    if status.success() {
        Ok(())
    } else {
        Err("git add failed".to_string())
    }
}

fn git_commit(folder_path: &str, commit_message: &str) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(folder_path)
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("failed to execute git commit");

    if output.status.success() {
        Ok(())
    } else {
        let err_string = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(format!("Error executing git commit: {}", err_string))
    }
}
fn push(){

}

fn pull(){

}
