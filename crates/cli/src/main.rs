use clap::{Parser, Subcommand};
use file_mover_core::config::{Rule, load_or_create, save_config, validate_config};
use file_mover_core::matcher::glob::compile_rule;

#[derive(Parser)]
#[command(name = "file-mover")]
#[command(version)]
#[command(about = "File mover utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate,
    AddRule {
        #[arg(long)]
        name: String,

        #[arg(long)]
        folder: String,

        #[arg(long)]
        destination: String,

        #[arg(long, num_args = 0..)]
        extensions: Vec<String>,
    },
    DeleteRule {
        name: String,
    },
    ListRules,
    TestRule {
        name: String,
        file: String,
    },
    Run,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate => validate(),
        Commands::AddRule {
            name,
            folder,
            destination,
            extensions,
        } => add_rule(name, folder, destination, extensions),
        Commands::DeleteRule { name } => delete_rule(name),
        Commands::ListRules => list_rules(),
        Commands::TestRule { name, file } => test_rule(name, file),
        Commands::Run => run(),
    }
}

fn validate() {
    match load_or_create() {
        Ok(config) => match validate_config(&config) {
            Ok(()) => {
                println!("✔ Config is valid");
            }
            Err(errors) => {
                eprintln!("✘ Config validation failed:");

                for err in errors {
                    eprintln!("  - {}", err);
                }

                std::process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            std::process::exit(1);
        }
    }
}
fn add_rule(name: String, folder: String, destination: String, extensions: Vec<String>) {
    let mut config = load_or_create().unwrap();

    let rule = Rule {
        name,
        folder: folder.into(),
        destination: destination.into(),
        whitelist: Vec::new(),
        blacklist: Vec::new(),

        extensions: extensions
            .into_iter()
            .map(|ext| {
                if ext.starts_with('.') {
                    ext
                } else {
                    format!(".{}", ext)
                }
            })
            .collect(),
    };

    config.add_rule(rule);

    match validate_config(&config) {
        Ok(_) => {
            save_config(&config).unwrap();
            println!("Rule added");
        }
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
        }
    }
}
fn delete_rule(name: String) {
    let mut config = load_or_create().unwrap();

    if config.delete_rule(&name) {
        save_config(&config).unwrap();
        println!("Deleted rule '{}'", name);
    } else {
        println!("Rule not found");
    }
}

fn list_rules() {
    let config = load_or_create().unwrap();

    if config.rules.is_empty() {
        println!("No rules defined");
        return;
    }

    for rule in config.rules {
        println!(
            "{}: {} -> {}",
            rule.name,
            rule.folder.display(),
            rule.destination.display()
        );
    }
}

fn test_rule(name: String, file: String) {
    let config = load_or_create().unwrap();

    let rule = match config.rules.iter().find(|r| r.name == name) {
        Some(r) => r,
        None => {
            eprintln!("Rule not found");
            std::process::exit(1);
        }
    };

    let path = std::path::Path::new(&file);

    if file_mover_core::matcher::file_matches_rule(path, rule, &compile_rule(rule)) {
        println!("File matches the rule");
    } else {
        println!("File does not match the rule");
    }
}

fn run() {
    let config = load_or_create().unwrap();

    match validate_config(&config) {
        Ok(()) => {
            println!("Config is valid, running...");
            let results = file_mover_core::engine::execute_rules(&config.rules);

            for (rule_name, res) in results {
                match res {
                    Ok(result) => {
                        println!("Rule '{}':", rule_name);
                        println!("  Moved: {}", result.moved.len());
                        println!("  Skipped: {}", result.skipped.len());
                        println!("  Errors: {}", result.errors.len());
                    }
                    Err(e) => {
                        eprintln!("Error executing rule '{}': {}", rule_name, e);
                    }
                }
            }
        }
        Err(errors) => {
            eprintln!("Config validation failed:");

            for err in errors {
                eprintln!("  - {}", err);
            }

            std::process::exit(1);
        }
    }
}
