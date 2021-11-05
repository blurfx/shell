use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
use std::path::Path;

use std::env;

fn main() {
    loop {
        print!("$ ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {
            let mut args = command.trim().split_whitespace();
            if args.clone().count() == 0usize {
                continue
            }
            let command = args.next().unwrap();

            match command {
                "exit" => {
                    return
                }
                "pwd" => {
                    let dir = env::current_dir();
                    match dir {
                        Ok(dir) => {
                            println!("{}", dir.display().to_string());
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
                "cd" => {
                    let dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                command => {
                    let stdin = prev_command.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(mut output) => {
                            prev_command = Some(output);
                        },
                        Err(e) => {
                            prev_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }
    }
}
