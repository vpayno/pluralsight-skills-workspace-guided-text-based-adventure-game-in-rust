use crate::prelude::*;
#[derive(Debug, PartialEq)]
pub struct Dungeon {
    pub rooms: Vec<Vec<Mob>>,
}

impl Dungeon {
    pub fn build_dungeon() -> Self {
        let rooms: Vec<Vec<Mob>> = vec![
            Self::generate_starter_room(),
            Self::generate_intermediate_room(),
            Self::generate_intermediate_room(),
            Self::generate_hard_room(),
            Self::generate_boss_room(),
        ];

        Self { rooms }
    }

    pub fn generate_starter_room() -> Vec<Mob> {
        let room_mobs: Vec<Mob> = vec![Mob::spawn_goblin(), Mob::spawn_ghost()];

        room_mobs
    }

    pub fn generate_intermediate_room() -> Vec<Mob> {
        let room_mobs: Vec<Mob> = vec![
            Mob::spawn_goblin(),
            Mob::spawn_ghost(),
            Mob::spawn_orc(),
            Mob::spawn_sorcerer(),
        ];

        room_mobs
    }

    pub fn generate_hard_room() -> Vec<Mob> {
        let room_mobs: Vec<Mob> = vec![
            Mob::spawn_goblin(),
            Mob::spawn_orc(),
            Mob::spawn_sorcerer(),
            Mob::spawn_orc(),
            Mob::spawn_troll(),
        ];

        room_mobs
    }

    pub fn generate_boss_room() -> Vec<Mob> {
        let room_mobs: Vec<Mob> = vec![Mob::spawn_demon()];

        room_mobs
    }
}
