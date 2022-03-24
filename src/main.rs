#![feature(fmt_internals)]

mod cli;
mod util;
mod wordle;

use cli::*;
use std::*;

fn main() {
    init_and_play_game(env::args().collect(), &mut io::stdin().lock(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_main_success() {
        main();
    }
}