use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::slice::Iter;

pub const WORD_SIZE: usize = 5;
pub const NO_LETTER: char = '\0';

const VALID_WORD_PATTERN: &'static str = r"^[[:alpha:]]{5}$"; // NOTE - keep RE size == WORD_SIZE
static VALID_WORD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(VALID_WORD_PATTERN).unwrap());

#[derive(PartialEq)]
pub struct GameWord {
    chars: [char; WORD_SIZE]
}

#[derive(Debug, PartialEq)]
pub struct InvalidWordError;

impl GameWord {

    pub fn new(string: &str) -> Result<GameWord, InvalidWordError> {
        is_valid_word(string)?;

        let mut chars = [ NO_LETTER; WORD_SIZE];
        string
            .to_uppercase()
            .chars()
            .zip(chars.iter_mut())
            .for_each(|(c, mc)| *mc = c);

        Ok(GameWord { chars })
    }

    pub fn chars_iter(&self) -> Iter<char> { self.chars.iter() }

    pub fn chars_contains(&self, c: &char) -> bool { self.chars.contains(c) }

    pub fn to_string(&self) -> String { String::from_iter(self.chars) }

}

impl fmt::Debug for GameWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#""{}""#, self.to_string())
    }
}

fn is_valid_word(string: &str) -> Result<(), InvalidWordError> {
    if VALID_WORD_RE.is_match(string) { Ok(()) }
    else { Err(InvalidWordError) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_word_partial_eq() {
        let chars = ['T', 'O', 'D', 'A', 'Y'];
        assert_eq!(GameWord { chars }, GameWord { chars });
    }

    #[test]
    fn length_just_right() {
        assert_eq!(GameWord::new("today").unwrap().chars, ['T', 'O', 'D', 'A', 'Y']);
    }

    #[test]
    fn length_too_short() {
        assert_eq!(GameWord::new("few"), Err(InvalidWordError));
    }

    #[test]
    fn length_too_long() {
        assert_eq!(GameWord::new("toolong"), Err(InvalidWordError));
    }

    #[test]
    fn expected_to_string() {
        assert_eq!(GameWord::new("today").unwrap().to_string(), "TODAY")
    }

    #[test]
    fn expected_debug_output() {
        assert_eq!(format!("{:?}", GameWord::new("today").unwrap()), r#""TODAY""#)
    }

}
