pub mod glob;

use crate::config::Rule;
use std::path::Path;

pub fn file_matches_rule(path: &Path, rule: &Rule, compiled_rule: &glob::CompiledRule) -> bool {
    // Check if the file name matches the whitelist and blacklist
    // cases:
    // - If the whitelist is empty, it matches all files
    // - If the whitelist is not empty, the file name must contain at least one of the whitelist strings
    // - If the blacklist is not empty, the file name must not contain any of the blacklist strings
    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        if let Some(whitelist) = &compiled_rule.whitelist {
            if !whitelist.is_match(filename) {
                return false;
            }
        }

        if let Some(blacklist) = &compiled_rule.blacklist {
            if blacklist.is_match(filename) {
                return false;
            }
        }
    } else {
        // If the file has no name, it doesn't match
        return false;
    }

    // Check if the file extension matches any of the rule's extensions
    // cases:
    // - If the rule has no extensions, it matches all files
    // - If the rule has extensions, the file must have an extension that matches one of the rule's extensions
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if !rule.extensions.is_empty() && !rule.extensions.iter().any(|e| e == ext) {
            return false;
        }
    } else if !rule.extensions.is_empty() {
        // If the file has no extension but the rule requires extensions, it doesn't match
        return false;
    }

    // Check if the file is in the rule's folder
    // cases:
    // - If the file's parent folder is the same as the rule's folder, it matches
    // - If the file has no parent folder, it doesn't match
    if let Some(parent) = path.parent() {
        if parent != rule.folder {
            return false;
        }
    } else {
        // If the file has no parent, it doesn't match
        return false;
    }

    true
}
