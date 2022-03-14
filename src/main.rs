#![feature(fmt_internals)]

mod cli;
mod util;
mod wordle;

use cli::*;
use std::*;
use wordle::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <target>", args[0]);
    } else {
        match wordle::GameWord::new(&args[1]) {
            Ok(target) => play_game(target, &mut io::stdin().lock(), &mut io::stdout()),
            Err(_) => println!("invalid target word: {}", args[1]),
        }
    }
}
