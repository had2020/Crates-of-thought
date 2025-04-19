enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Exit,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn process_commands(commands: Vec<Command>) -> &'static str {
    let mut log: Vec<String> = vec![];
    let mut cur_pos = Position { x: 0, y: 0 };
    let mut exit_flag = false;

    for cmd in commands {
        if !exit_flag {
            match cmd {
                Command::Move { x, y } => {
                    cur_pos = Position {
                        x: cur_pos.x + x,
                        y: cur_pos.y + y,
                    }
                }
                Command::Say(message) => log.push(message),
                Command::Exit => exit_flag = true,
            }
        }
    }

    Box::leak(format!("Final position: {:?}, Messages: {:?}", cur_pos, log).into_boxed_str())
}

fn main() {
    let cmds = vec![
        Command::Move { x: 3, y: 4 },
        Command::Say("Hello".into()),
        Command::Move { x: -1, y: 2 },
        Command::Say("World".into()),
        Command::Exit,
        Command::Say("Ignored".into()),
    ];

    let output = process_commands(cmds);
    println!("{:?}", output);
}
