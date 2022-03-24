use crate::wordle::*;
use std::io::{BufRead, Write};


pub fn init_and_play_game(args: Vec<String>, input: &mut impl BufRead, output: &mut impl Write) {
    if args.len() != 2 {
        writeln!(output, "usage: {} <target>", args[0]).ok();
    } else {
        match GameWord::new(&args[1]) {
            Ok(target) => play_game(target, input, output),
            Err(_) => {
                writeln!(output, "invalid target word: {}", args[1]).ok();
            },
        }
    }
}

fn play_game(target: GameWord, input: &mut impl BufRead, output: &mut impl Write) {
    let mut game = GameState::new(target);
    write!(output, "{}", game).ok();
    while game.status() == GameStatus::InProgress {
        loop {
            match read_guess(input, output) {
                None => return,
                Some(guess) => match game.add_guess(&guess) {
                    Ok(_) => break,
                    Err(GuessErrors::DuplicateGuessError) => {
                        writeln!(output, "guess already used").ok();
                    }
                    Err(GuessErrors::UnusedHintsError) => {
                        writeln!(output, "guess missing known hints").ok();
                    }
                },
            }
        }
        write!(output, "{}", game).ok();
    }
}

fn read_guess(input: &mut impl BufRead, output: &mut impl Write) -> Option<GameWord> {
    loop {
        let string = read_line(input, output)?;
        match GameWord::new(&string) {
            Ok(word) => return Some(word),
            Err(_) => {
                write!(output, "invalid word: {}", string).ok();
            }
        }
    }
}

fn read_line(input: &mut impl BufRead, output: &mut impl Write) -> Option<String> {
    write!(output, "> ").ok();
    output.flush().ok();
    let mut raw = "".to_string();
    input.read_line(&mut raw).ok();
    let trim = raw.trim().to_string();
    if trim.len() > 0 {
        Some(trim)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_match;
    use regex::Regex;

    #[test]
    fn read_trimmed_line() {
        let mut test_input: &[u8] = b" test \n";
        let mut test_output: Vec<u8> = Vec::new();

        assert_eq!(
            read_line(&mut test_input, &mut test_output),
            Some("test".to_string())
        );
        assert_eq!(test_output, "> ".as_bytes());
    }

    #[test]
    fn read_empty_line() {
        let mut test_input: &[u8] = b"  \n";
        let mut test_output: Vec<u8> = Vec::new();

        assert_eq!(read_line(&mut test_input, &mut test_output), None);
        assert_eq!(test_output, "> ".as_bytes());
    }

    #[test]
    fn read_trimmed_word_after_skipping_invalid_word() {
        let mut test_input: &[u8] = b"bad\n today \n";
        let mut test_output: Vec<u8> = Vec::new();

        assert_eq!(
            read_guess(&mut test_input, &mut test_output),
            Some(GameWord::new("TODAY").unwrap())
        );
        assert_eq!(
            String::from_utf8(test_output),
            Ok("> invalid word: bad> ".to_string())
        );
    }

    #[test]
    fn trigger_all_guess_errors() {
        let mut test_input: &[u8] = b"arise\narise\nxxxxx\n";
        let mut test_output: Vec<u8> = Vec::new();
        let target = GameWord::new("TODAY").unwrap();

        play_game(target, &mut test_input, &mut test_output);
        assert_match!(
            String::from_utf8(test_output).unwrap(),
            r"guesses:\n\nletters: [^\n]+\n> guesses:\n1 - [^\n]+\n\nletters: [^\n]*\n> "
        );
    }

    #[test]
    fn no_arguments_prints_usage() {
        let mut test_input: &[u8] = b"\n";
        let mut test_output: Vec<u8> = Vec::new();
        let mut test_args: Vec<String> = Vec::new();

        test_args.insert(0,"test".to_string());
        init_and_play_game(test_args, &mut test_input, &mut test_output);
        assert_eq!(test_output, "usage: test <target>\n".as_bytes());
    }

    #[test]
    fn no_game_with_invalid_target() {
        let mut test_input: &[u8] = b"\n";
        let mut test_output: Vec<u8> = Vec::new();
        let mut test_args: Vec<String> = Vec::new();

        test_args.insert(0,"test".to_string());
        test_args.insert(0,"test".to_string());
        init_and_play_game(test_args, &mut test_input, &mut test_output);
        assert_eq!(test_output, "invalid target word: test\n".as_bytes());
    }

    #[test]
    fn terminate_game_with_empty_line() {
        let mut test_input: &[u8] = b"\n";
        let mut test_output: Vec<u8> = Vec::new();
        let mut test_args: Vec<String> = Vec::new();

        test_args.insert(0,"test".to_string());
        test_args.insert(1,"today".to_string());
        init_and_play_game(test_args, &mut test_input, &mut test_output);
        assert_match!(
            String::from_utf8(test_output).unwrap(),
            r"guesses:\n\nletters: [^\n]+\n> "
        );
    }

}
