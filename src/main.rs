use std::env;
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;

use interrogator; 

fn main() {
    let argv: Vec<String> = env::args().collect();
    let config = Config::new(argv);

    loop {
        println!("Press 'Enter' to for a question or type 'exit' to exit");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            process::exit(0);
        }

        let question = match config.mode {
            Mode::SDE => interrogator::pick_rand_sde(), 
            Mode::PM => interrogator::pick_rand_pm(), 
            Mode::COMMON => interrogator::pick_rand_common(), 
            Mode::ALL => interrogator::pick_rand_all(), 
        };

        println!("{}", question);

        let mut child = Command::new("say")
            .arg(question)
            .spawn()
            .expect("Failed to execute process");

        let _ = child.wait();
    }
}

struct Config {
    mode: Mode,
}

enum Mode {
    SDE,
    PM,
    COMMON,
    ALL,
}

impl Config {
    fn new(argv: Vec<String>) -> Config {
        if argv.len() < 2 {
            eprintln!(r#"
usage: cargo run [mode]

mode: sde | pm | common |all"#);
            process::exit(1);
        }

        let mode = match argv[1].as_ref() {
            "sde" => Mode::SDE,
            "pm" => Mode::PM,
            "common" => Mode::COMMON,
            "all" => Mode::ALL,
            _ => {
                eprintln!("modes are: sde, pm, common, all");
                process::exit(1);
            },
        };

        Config { mode }
    }
}
