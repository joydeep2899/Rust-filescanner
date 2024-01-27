use chrono::Local;
mod modules;
use modules::scanner;
use std::path::Path;
use std::env;

fn main() {
    let env_args = env::args();
    let mut n_args = env_args.count();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    n_args -= 1;

    let mut now = Local::now();
    println!("Time of scan Start: {}", now);

    if n_args > 0 {
        for arg in args {
            scanner::scan(Path::new(&arg));
        }
    } else {
        println!("pass atleast 1 argument")
    }

    now = Local::now();
    println!("Time of scan End: {}", now);
}
