use std::fs;
use std::path::PathBuf;

use crate::config::Rule;
use crate::matcher::file_matches_rule;

pub struct ExecutionResult {
    pub moved: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
    pub errors: Vec<(PathBuf, String)>,
}

pub fn execute_rule(rule: &Rule) -> std::io::Result<ExecutionResult> {
    let mut result = ExecutionResult {
        moved: vec![],
        skipped: vec![],
        errors: vec![],
    };

    let entries = fs::read_dir(&rule.folder)?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                if file_matches_rule(&path, rule) {
                    let filename = match path.file_name() {
                        Some(f) => f,
                        None => continue,
                    };

                    let dest = rule.destination.join(filename);

                    match fs::rename(&path, &dest) {
                        Ok(_) => result.moved.push(dest),
                        Err(e) => result.errors.push((path, e.to_string())),
                    }
                } else {
                    result.skipped.push(path);
                }
            }
            Err(e) => {
                result.errors.push((rule.folder.clone(), e.to_string()));
            }
        }
    }

    Ok(result)
}
