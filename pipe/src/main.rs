extern crate libc;

use std::env;
use std::io::{self, Read};

// ref http://rust-lang-ja.org/rust-by-example/std_misc/arg.html

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Receive args : {:?}", args);

    let isatty = unsafe { libc::isatty(libc::STDIN_FILENO) != 0 };
    if !isatty {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        println!("Receive pipe :\n  {}", input);
    }
}
