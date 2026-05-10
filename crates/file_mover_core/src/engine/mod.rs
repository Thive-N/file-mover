use crate::config::Rule;
use crate::matcher::file_matches_rule;
use crate::matcher::glob;
use std::fs;
use std::path::PathBuf;

// struct to hold the results of executing a rule
pub struct ExecutionResult {
    pub moved: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
    pub errors: Vec<(PathBuf, String)>,
}

// Executes the given rules and returns a vector of results for each rule
pub fn execute_rules(rules: &[Rule]) -> Vec<(String, std::io::Result<ExecutionResult>)> {
    let mut results = vec![];

    for rule in rules {
        let res = execute_rule(rule);
        results.push((rule.name.clone(), res));
    }

    results
}

// Executes a single rule and returns the result
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

                if file_matches_rule(&path, rule, &glob::compile_rule(rule)) {
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
