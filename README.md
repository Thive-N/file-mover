an application that moves files around with filters
reads a file in userdata that specifies what folders to look in and what files to move and where and any other filters/blacklists

example:
```toml
interval_seconds = 60
[[rules]]
name = "Example Rule"
folder = "/home/user/Downloads"
destination = "/home/user/Documents"
extensions = [".txt"]
```


current state
```
✓ Validation
✓ CLI interaction
→ Matching logic
→ File actions
  Scheduler
  Linux watcher
  Notifications
```