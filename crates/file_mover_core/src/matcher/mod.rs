use crate::config::Rule;
use std::path::Path;

pub fn file_matches_rule(path: &Path, rule: &Rule) -> bool {
    // Check if the file extension matches any of the rule's extensions
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_with_dot = format!(".{}", ext);
        return rule
            .extensions
            .iter()
            .any(|rule_ext| rule_ext == &ext_with_dot);
    }

    false
}
