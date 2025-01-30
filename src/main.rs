pub mod scanner;
pub mod token_type;

use scanner::scan_tokens;

use std::{cmp::Ordering, env, fs, io::Write, process::exit};

static mut ERROR: bool = false;

fn runfile(path: String) {
    let file = fs::read_to_string(path).expect("File could not be found!");

    //println!("{file}");
    run(file);

    unsafe {
        if ERROR {
            exit(65);
        }
    }
}

fn runprompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Reading error!");
        run(input);

        unsafe {
            ERROR = false;
        }
    }
}

fn run(input: String) {
    //println!("run: {input}");
    let tokens = scan_tokens(&input);
    // We are printing tokens to test, for now.
    println!("{tokens:?}");
}

fn error(line: u16, msg: String) {
    report(line, String::from(""), msg);
}

fn report(line: u16, loc: String, msg: String) {
    eprintln!("[line {line} ] Error {loc}: {msg}");
    unsafe {
        ERROR = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rlox [script]");
            //println!("{}", args[1]);
        }
        Ordering::Equal => {
            runfile(args[1].clone());
        }
        _ => runprompt(),
    };
}
