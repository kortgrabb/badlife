use crate::character::Character;
use rand::Rng;

pub enum LifeEvent {
    JobPromotion,
    Cancer,
    Cold,
}

impl LifeEvent {
    pub fn trigger(&self, character: &mut Character) {
        match self {
            LifeEvent::JobPromotion => println!("Congratulations! You got promoted."),
            LifeEvent::Cancer => {
                println!("Unfortunately, you've been diagnosed with cancer.");
                character.sickness.death_chance = 0.3;
            }
            LifeEvent::Cold => println!("You caught a cold."),
        }
    }

    pub fn event_chance(&self) -> f32 {
        match self {
            LifeEvent::JobPromotion => 0.01,
            LifeEvent::Cancer => 0.001,
            LifeEvent::Cold => 0.05,
        }
    }

    pub fn trigger_random_event(character: &mut Character) {
        let events = [
            LifeEvent::JobPromotion,
            LifeEvent::Cancer,
            LifeEvent::Cold,
        ];
        let mut rng = rand::thread_rng();
        let event_chance: f32 = rng.gen();

        for event in events.iter() {
            if event_chance < event.event_chance() {
                event.trigger(character);
                break;
            }
        }
    }
}
