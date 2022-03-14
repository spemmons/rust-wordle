use crate::wordle::word::*;
use crate::wordle::GameStatus::{Failure, InProgess, Success};
use crate::wordle::LetterMatch::{ExactMatch, Unknown, NoMatch, WrongPosition};

pub const MAX_GUESSES: usize = 6;

#[derive(Debug)]
pub struct GameState {
    target: GameWord,
    guesses: Vec<GameGuess>,
    status: GameStatus,
}

pub type GameLetter = (char, LetterMatch);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameStatus {
    InProgess,
    Success,
    Failure,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameGuess {
    letters: [GameLetter; WORD_SIZE],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LetterMatch {
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
            status: InProgess,
        }
    }

    pub fn add_guess(&mut self, guess: &GameWord) -> GameStatus {
        if self.guesses.len() < MAX_GUESSES {
            let result = evaluate_guess(&self.target, &guess);
            self.guesses.push(result);
            let success = result.letters.into_iter().all(|value| value.1 == ExactMatch);
            if success { self.status = Success }
            else if self.guesses.len() >= MAX_GUESSES { self.status = Failure };
        }
        self.status
    }

}

pub fn evaluate_guess(target: &GameWord, guess: &GameWord) -> GameGuess {
    let mut letters = [(NO_LETTER, Unknown); WORD_SIZE];

    target.chars_iter().zip(guess.chars_iter())
        .map(|(t, g)|
            if *t == *g { (*g, ExactMatch) }
            else if target.chars_contains(g) { (*g, WrongPosition) }
            else { (*g, NoMatch) }
        )
        .zip(letters.iter_mut()).for_each(|(t, r)| *r = t);

    GameGuess { letters }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_game() {
        let game = GameState::new(GameWord::new("today").unwrap());
        assert_eq!(game.target.to_string(), "TODAY");
        // ...
    }

    #[test]
    fn execute_successful_game() {
        let target = GameWord::new("today").unwrap();
        let guess = GameWord::new("today").unwrap();
        let mut game = GameState::new(target);
        assert_eq!(game.add_guess(&guess), Success);
        assert_eq!(format!("{:?}", game), r#"GameState { target: "TODAY", guesses: [GameGuess { letters: [('T', ExactMatch), ('O', ExactMatch), ('D', ExactMatch), ('A', ExactMatch), ('Y', ExactMatch)] }], status: Success }"#);
    }

    #[test]
    fn execute_failed_game() {
        let target = GameWord::new("today").unwrap();
        let guess = GameWord::new("notit").unwrap();
        let mut game = GameState::new(target);
        assert_eq!(game.add_guess(&guess), InProgess);
        assert_eq!(game.add_guess(&guess), InProgess);
        assert_eq!(game.add_guess(&guess), InProgess);
        assert_eq!(game.add_guess(&guess), InProgess);
        assert_eq!(game.add_guess(&guess), InProgess);
        assert_eq!(game.add_guess(&guess), Failure);
        assert_eq!(game.add_guess(&guess), Failure);
    }

}
