mod wordle;

use wordle::*;

fn main() -> Result<(), InvalidWordError> {
    let mut game = GameState::new(wordle::GameWord::new("today")?);

    println!("game = {:?}", game);

    println!("guess = {:?}", game.add_guess(&wordle::GameWord::new("arise")?));

    println!("game = {:?}", game);

    println!("guess = {:?}", game.add_guess(&wordle::GameWord::new("today")?));

    println!("game = {:?}", game);

    Ok(())
}
