use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    loop {
        print!("$ ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut args = input.trim().split_whitespace();
        let command = args.next().unwrap();

        match command {
            "exit" => {
                return;
            }
            "cd" => {
                let dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let mut command_result = Command::new(command)
                    .args(args)
                    .spawn();

                match command_result {
                    Ok(mut child) => {
                        child.wait();
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
}
