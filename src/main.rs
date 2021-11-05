use std::io::{stdin, stdout, Write};
use std::process::Command;

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
            command => {
                let mut command_result = Command::new(command)
                    .args(args)
                    .spawn();

                match command_result {
                    Ok(mut child) => {
                        child.wait();
                    },
                    Err(e) => {
                        println!("{}", e.to_string());
                    }
                }
            }
        }
    }
}
