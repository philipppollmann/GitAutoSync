use std::process::Command;

pub fn status(folder_path: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(folder_path)
        .arg("status")
        .arg(".")
        .status()
        .expect("failed to execute git status");

    if status.success() {
        Ok(())
    } else {
        Err("git add failed".to_string())
    }
}

pub fn add(folder_path: &str) -> Result<(), String> {
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

pub(crate) fn commit(folder_path: &str, commit_message: String) -> Result<(), String> {
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

pub fn push(folder_path: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(folder_path)
        .arg("push")
        .status()
        .expect("failed to execute git push");

    if status.success() {
        Ok(())
    } else {
        Err("git push failed".to_string())
    }
}

pub fn pull(folder_path: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(folder_path)
        .arg("pull")
        .status()
        .expect("failed to execute git pull");

    if status.success() {
        Ok(())
    } else {
        Err("git pull failed".to_string())
    }
}

