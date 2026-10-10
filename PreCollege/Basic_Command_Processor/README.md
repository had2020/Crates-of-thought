# 🧠 Basic Command Processor

This challenge explores the use of enums and pattern matching to simulate a simple command engine.

# 📝 Step by step:
1. Define an enum Command with the following variants:
  - Move { x: i32, y: i32 }
  - Say(String)
  - Exit
2. Write a function process_commands(commands: Vec<Command>) -> String that:
  - Starts at position (0, 0)
  - For each Move, update the position
  - For each Say, collect the messages into a log
  - Stops processing when it hits Exit
  - Returns a final string that shows the final position and the collected messages.

# ✅ Example:
```rust
let cmds = vec![
    Command::Move { x: 3, y: 4 },
    Command::Say("Hello".into()),
    Command::Move { x: -1, y: 2 },
    Command::Say("World".into()),
    Command::Exit,
    Command::Say("Ignored".into()), // should not be processed
];

let output = process_commands(cmds);
assert_eq!(output, "Final position: (2, 6), Messages: [\"Hello\", \"World\"]");
```
