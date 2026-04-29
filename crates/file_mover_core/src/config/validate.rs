use crate::config::error::ValidationError;
use crate::config::model::Config;
use std::collections::HashSet;
use std::path::Path;

pub fn validate_config(config: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // no rules
    if config.rules.is_empty() {
        errors.push(ValidationError::NoRules);
    }

    if config.interval_seconds.unwrap_or(0) == 0 {
        errors.push(ValidationError::InvalidInterval);
    }

    let mut seen = HashSet::new();

    for rule in &config.rules {
        if rule.name.trim().is_empty() {
            errors.push(ValidationError::EmptyRuleName {
                rule: "<unknown>".into(),
            });
        }

        if !seen.insert(rule.name.clone()) {
            errors.push(ValidationError::DuplicateRuleName(rule.name.clone()));
        }

        if !rule.folder.exists() {
            errors.push(ValidationError::FolderMissing {
                rule: rule.name.clone(),
                path: rule.folder.clone(),
            });
        } else if !rule.folder.is_dir() {
            errors.push(ValidationError::FolderNotDirectory {
                rule: rule.name.clone(),
                path: rule.folder.clone(),
            });
        }

        let has_filters =
            !rule.extensions.is_empty() || !rule.whitelist.is_empty() || !rule.blacklist.is_empty();

        if !has_filters {
            errors.push(ValidationError::NoFilters {
                rule: rule.name.clone(),
            });
        }

        for ext in &rule.extensions {
            if !ext.starts_with('.') {
                errors.push(ValidationError::InvalidExtension {
                    rule: rule.name.clone(),
                    extension: ext.clone(),
                });
            }
        }

        for pat in &rule.whitelist {
            if glob::Pattern::new(pat).is_err() {
                errors.push(ValidationError::InvalidGlob {
                    rule: rule.name.clone(),
                    pattern: pat.clone(),
                });
            }
        }

        for pat in &rule.blacklist {
            if glob::Pattern::new(pat).is_err() {
                errors.push(ValidationError::InvalidGlob {
                    rule: rule.name.clone(),
                    pattern: pat.clone(),
                });
            }
        }

        let _ = Path::new(&rule.destination);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
