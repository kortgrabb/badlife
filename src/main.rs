mod character;
mod events;
mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
