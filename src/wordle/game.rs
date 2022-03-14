use ansi_term::Colour;
use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;

use crate::wordle::word::*;
use crate::wordle::GameStatus::*;
use crate::wordle::MatchType::*;
use crate::GuessErrors::*;

pub const MAX_GUESSES: usize = 6;

const HINT_TYPES: [MatchType; 2] = [ExactMatch, WrongPosition];
const LETTER_RANGE: RangeInclusive<char> = 'A'..='Z';

#[derive(Debug, PartialEq)]
pub enum GuessErrors {
    DuplicateGuessError,
    UnusedHintsError,
}

type GameLetters = HashMap<char, GameLetter>;

pub struct GameState {
    target: GameWord,
    guesses: Vec<GameGuess>,
    letters: GameLetters,
    status: GameStatus,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameLetter(char, MatchType);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameStatus {
    InProgress,
    Success,
    Failure,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameGuess([GameLetter; WORD_SIZE]);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MatchType {
    Unknown,
    NoMatch,
    WrongPosition,
    ExactMatch,
}

impl GameState {
    pub fn new(target: GameWord) -> GameState {
        GameState {
            target,
            guesses: Vec::with_capacity(MAX_GUESSES),
            letters: initial_letters(),
            status: InProgress,
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status.clone()
    }

    pub fn add_guess(&mut self, proposal: &GameWord) -> Result<GameStatus, GuessErrors> {
        if self.guesses.len() < MAX_GUESSES {
            let guess = evaluate_guess(&self.target, &proposal);
            check_unique_guess(&guess, &self.guesses)?;
            check_all_hints_used(&guess, &self.letters)?;
            self.guesses.push(guess);
            update_letters(&mut self.letters, &guess);
            if guess.perfect_match() {
                self.status = Success
            } else if self.guesses.len() >= MAX_GUESSES {
                self.status = Failure
            };
        }
        Ok(self.status.clone())
    }
}

impl GameGuess {
    fn perfect_match(&self) -> bool {
        self.0.iter().all(|letter| letter.1 == ExactMatch)
    }

    fn includes(&self, c: &char) -> bool {
        self.0.iter().any(|letter| *c == letter.0)
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GameState {{ target: {:?}, guesses: {:?}, status: {:?} }}",
            self.target, self.guesses, self.status
        )
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "guesses:\n")?;
        for (idx, guess) in self.guesses.iter().enumerate() {
            writeln!(f, "{} - {}", idx + 1, guess)?
        }
        writeln!(f, "")?;

        write!(f, "letters: ")?;
        for c in LETTER_RANGE {
            format_game_letter(&self.letters, &c, f)?
        }
        writeln!(f, "")
    }
}

impl fmt::Display for GameGuess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for letter in self.0.iter() {
            write!(f, "{}", letter)?
        }
        write!(f, "")
    }
}

impl fmt::Display for GameLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colour = colour_for_match_type(&self.1);
        write!(f, "{}", colour.paint(format!("{} ", self.0)))
    }
}

fn format_game_letter(letters: &GameLetters, c: &char, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(letter) = letters.get(&c) {
        write!(f, "{}", letter)
    } else {
        write!(f, "")
    }
}

fn evaluate_guess(target: &GameWord, guess: &GameWord) -> GameGuess {
    let mut letters = [GameLetter(NO_LETTER, Unknown); WORD_SIZE];

    target
        .chars_iter()
        .zip(guess.chars_iter())
        .map(|(t, g)| {
            if *t == *g {
                GameLetter(*g, ExactMatch)
            } else if target.chars_contains(g) {
                GameLetter(*g, WrongPosition)
            } else {
                GameLetter(*g, NoMatch)
            }
        })
        .zip(letters.iter_mut())
        .for_each(|(t, r)| *r = t);

    GameGuess(letters)
}

fn check_unique_guess(guess: &GameGuess, guesses: &Vec<GameGuess>) -> Result<(), GuessErrors> {
    if guesses.contains(guess) {
        Err(DuplicateGuessError)
    } else {
        Ok(())
    }
}

fn check_all_hints_used(guess: &GameGuess, letters: &GameLetters) -> Result<(), GuessErrors> {
    if missing_hints(guess, letters) {
        Err(UnusedHintsError)
    } else {
        Ok(())
    }
}

fn missing_hints(guess: &GameGuess, letters: &GameLetters) -> bool {
    letters
        .values()
        .any(|letter| HINT_TYPES.contains(&letter.1) && !guess.includes(&letter.0))
}

fn initial_letters() -> GameLetters {
    let mut map = HashMap::new();
    for c in LETTER_RANGE {
        map.insert(c, GameLetter(c, Unknown));
    }
    map
}

fn update_letters(letters: &mut GameLetters, guess: &GameGuess) {
    for guess_letter in guess.0 {
        let GameLetter(c, guess_type) = guess_letter;
        if let Some(GameLetter(_, letter_type)) = letters.get(&c) {
            if needs_letter_update(&guess_type, &letter_type) {
                letters.insert(c, guess_letter.clone());
            }
        }
    }
}

fn needs_letter_update(guess_type: &MatchType, letter_type: &MatchType) -> bool {
    match (guess_type, letter_type) {
        (_, ExactMatch) => false,
        (ExactMatch, _) => true,
        (WrongPosition, _) => *letter_type != ExactMatch,
        (NoMatch, Unknown) => true,
        _ => false,
    }
}

fn colour_for_match_type(match_type: &MatchType) -> Colour {
    match match_type {
        Unknown => Colour::White,
        NoMatch => Colour::Red,
        WrongPosition => Colour::Yellow,
        ExactMatch => Colour::Green,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_match;
    use regex::Regex;

    #[test]
    fn create_new_game() {
        let game = GameState::new(GameWord::new("today").unwrap());
        assert_eq!(game.target.to_string(), "TODAY");
        assert_eq!(game.guesses.len(), 0);
        assert_eq!(game.letters.len(), 26);
        assert_eq!(game.status(), InProgress);
    }

    #[test]
    fn guess_in_one() {
        let target = GameWord::new("today").unwrap();
        let guess = GameWord::new("today").unwrap();
        let mut game = GameState::new(target);
        assert_eq!(game.add_guess(&guess), Ok(Success));
        assert_match!(
            format!("{:?}", game),
            r#"GameState \{ target: "TODAY", guesses: \[.*\], status: Success \}"#
        );
        assert_match!(
            format!("{}", game),
            r"^guesses:\n1 - [^\n]+T [^\n]+O [^\n]+D [^\n]+A [^\n]+Y [^\n]+\n\nletters: [^\n]+\n$"
        );
    }

    #[test]
    fn execute_failed_game() {
        let target = GameWord::new("today").unwrap();
        let guess = GameWord::new("txday").unwrap();
        let all_x = GameWord::new("xxxxx").unwrap();
        let mut game = GameState::new(target);
        assert_eq!(game.add_guess(&guess), Ok(InProgress));
        assert_eq!(game.add_guess(&guess), Err(DuplicateGuessError));
        assert_eq!(game.add_guess(&all_x), Err(UnusedHintsError));
        assert_eq!(
            game.add_guess(&GameWord::new("ytxda").unwrap()),
            Ok(InProgress)
        );
        assert_eq!(
            game.add_guess(&GameWord::new("aytxd").unwrap()),
            Ok(InProgress)
        );
        assert_eq!(
            game.add_guess(&GameWord::new("daytx").unwrap()),
            Ok(InProgress)
        );
        assert_eq!(
            game.add_guess(&GameWord::new("xdayt").unwrap()),
            Ok(InProgress)
        );
        assert_eq!(
            game.add_guess(&GameWord::new("tyday").unwrap()),
            Ok(Failure)
        );
        assert_eq!(
            game.add_guess(&GameWord::new("today").unwrap()),
            Ok(Failure)
        );
    }

    #[test]
    fn use_right_colour_for_match_type() {
        assert_eq!(colour_for_match_type(&Unknown), Colour::White);
        assert_eq!(colour_for_match_type(&NoMatch), Colour::Red);
        assert_eq!(colour_for_match_type(&WrongPosition), Colour::Yellow);
        assert_eq!(colour_for_match_type(&ExactMatch), Colour::Green);
    }

    #[test]
    fn skip_update_for_missing_game_letters() {
        let target = GameWord::new("today").unwrap();
        let guess = GameWord::new("tadxy").unwrap();
        let mut game = GameState::new(target);
        let mut letters: GameLetters = HashMap::new();

        assert_eq!(game.add_guess(&guess), Ok(InProgress));
        update_letters(&mut letters, &game.guesses[0]);
        assert_eq!(letters.len(), 0);
    }

    #[test]
    fn skip_output_for_missing_game_letters() {
        let mut test_output = String::new();
        let mut formatter = fmt::Formatter::new(&mut test_output);
        let letters: GameLetters = HashMap::new();
        assert_eq!(
            format_game_letter(&letters, &'A', &mut formatter),
            Result::Ok(())
        );
        assert_eq!(test_output.len(), 0);
    }
}
