# File Mover

A Linux-first file organization utility written in Rust.

The application monitors folders and moves files based on configurable rules defined in a TOML configuration file.

The project is designed around:

* Zero external runtime dependencies
* Native Linux support first
* Background daemon execution
* Config-driven automation
* Extensible architecture for future GUI/TUI support

---

# Features

## Current Features

* Rust workspace architecture
* Shared core crate (`file_mover_core`)
* Config file creation/loading
* XDG-compatible config paths
* TOML config parsing
* Rule validation
* CLI commands

  * validate
  * add
  * delete
  * list
  * run

* File matching engine
* Wildcard glob support

---

# Example Config

```toml
interval_seconds = 60

[[rules]]
name = "Move Text"
folder = "/home/user/Downloads"
destination = "/home/user/txts"
extensions = [".txt"]
whitelist = ["*.txt"]
blacklist = ["secret*.txt"]
```

---

# Architecture

```text
CLI / Daemon
    ↓
Execution Engine
    ↓
Matcher
    ↓
Config System
```

## Workspace Structure

```text
file-mover/
├── Cargo.toml
└── crates/
    ├── file_mover_core/
    ├── daemon/
    ├── cli/
    └── platform-linux/
```

---

# Planned Components

## file_mover_core

Shared business logic.

Contains:

* Config parsing
* Validation
* Matching engine
* File execution engine
* Shared errors

## daemon

Background process responsible for:

* Scheduled execution
* File watching
* Notifications
* Logging

## cli

User-facing command line interface.

## platform-linux

Linux-specific functionality.

Planned:

* inotify integration
* systemd support
* desktop notifications

---

# CLI Commands

## Validate Config

```bash
file-mover validate
```

## Add Rule

```bash
file-mover add
```

## Delete Rule

```bash
file-mover delete
```

## List Rules

```bash
file-mover list
```

---

# Current Development Status

## Completed

* [x] Cargo workspace setup
* [x] Core crate architecture
* [x] Config loading
* [x] Config creation
* [x] XDG config path handling
* [x] TOML parsing
* [x] Validation system
* [x] CLI command structure
* [x] Rule add/delete/list support
* [x] Basic file matching engine
* [x] Extension filtering
* [x] Initial whitelist/blacklist support

---

## In Progress

* [ ] Replace string matching with glob matching
* [ ] Compile globsets for performance
* [ ] Implement execution engine
* [ ] Implement dry-run mode

---

## Planned

### Core Engine

* [ ] File moving logic
* [ ] File copying support
* [ ] Recursive directory support
* [ ] Conflict handling
* [ ] Case-insensitive matching
* [ ] Symlink handling

### Daemon

* [ ] Background scheduler
* [ ] Config hot reload
* [ ] Graceful shutdown handling
* [ ] Periodic rescanning

### Linux Features

* [ ] inotify watcher integration
* [ ] systemd service installation
* [ ] Desktop notifications
* [ ] journald logging

### Logging

* [ ] Structured logging
* [ ] Rotating log files
* [ ] Error reporting

### Testing

* [ ] Unit tests for matcher
* [ ] Config validation tests
* [ ] Integration tests
* [ ] End-to-end filesystem tests

### Future Features

* [ ] TUI configuration editor
* [ ] Web UI
* [ ] Cross-platform support
* [ ] Rule priorities
* [ ] Scheduling per rule
* [ ] Age-based filters
* [ ] File size filters

---

# Development

## Build

```bash
cargo build
```

## Run CLI

```bash
cargo run -p file-mover -- validate
```

---

# Configuration Location

## Linux

```text
~/.config/file-mover/config.toml
```

## Windows

```text
AppData/Roaming/file-mover/config.toml
```

---

# License

TBD

