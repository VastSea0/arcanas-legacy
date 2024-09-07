// players.rs

use crate::level::Level;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Class {
    Gladius,
    Arcana,
    Aegis,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub level: Level,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

impl Player {
    pub fn new(class: Class) -> Self {
        match class {
            Class::Gladius => Player {
                name: "Gladius".to_string(),
                class,
                level: Level::new(1),
                hp: 120,
                attack: 15,
                defense: 10,
            },
            Class::Arcana => Player {
                name: "Arcana".to_string(),
                class,
                level: Level::new(1),
                hp: 80,
                attack: 20,
                defense: 5,
            },
            Class::Aegis => Player {
                name: "Aegis".to_string(),
                class,
                level: Level::new(1),
                hp: 100,
                attack: 10,
                defense: 20,
            },
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Name: {}, Class: {:?}, Level: {}, HP: {}, Attack: {}, Defense: {}",
            self.name, self.class, self.level, self.hp, self.attack, self.defense
        )
    }
}
