use std::path::Path;
use std::path::PathBuf;

pub fn get_project_list() -> Vec<PathBuf> {
    let mut projects = Vec::new();
    projects.push(PathBuf::from("/Users/philipppollmann/Library/Mobile Documents/iCloud~md~obsidian/Documents/Obsidian"));
    projects.push(PathBuf::from("/Users/philipppollmann/Library/Mobile Documents/iCloud~com~logseq~logseq/Documents/Logseq"));
    projects.push(PathBuf::from(r"C:\github\Logseq"));
    projects.push(PathBuf::from(r"C:\github\Obsidian"));
    projects.push(PathBuf::from(r"/Users/A200017104/GitHub/Obsidian"));
    projects.push(PathBuf::from(r"/Users/A200017104/GitHub/Logseq"));
    projects
}

pub fn check_directory_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_dir()
}