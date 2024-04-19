use crate::character::Character;
use crate::events::LifeEvent;
use std::io::{self, Write};

pub struct Game {
    character: Character,
}

impl Game {
    pub fn new() -> Self {
        println!("Welcome to the BitLife clone!");

        let name = String::from("Robby Dobby");

        let character = Character::new(name);
        Game { character }
    }

    pub fn run(&mut self) {
        loop {
            LifeEvent::trigger_random_event(&mut self.character);

            self.character.age();
            self.character.display();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            if choice.trim().eq("q!") {
                break;
            }
        }
    }
}
