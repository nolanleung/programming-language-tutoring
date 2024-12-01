use std::env;

macro_rules! exit {
    ($msg:expr) => {
        println!("{}", $msg);
        std::process::exit(1);
    };
}

macro_rules! parse_or_exit {
    ($t:ty, $e:expr) => {
        match $e.parse::<$t>() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse argument");
                std::process::exit(1);
            }
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit!("Please provide a number as an argument");
    }

    let mut i = 0;
    let num = parse_or_exit!(i32, args[1]);

    while i < num {
        println!("{}", i);
        i += 1;
    }
}
