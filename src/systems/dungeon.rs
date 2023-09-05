use crate::prelude::*;
#[derive(Debug, PartialEq)]
pub struct Dungeon {
    pub rooms: Vec<Vec<Mob>>,
}

impl Dungeon {
    pub fn build_dungeon() -> Self {
        let mut rooms = Vec::<Vec<Mob>>::new();
        rooms.push(Self::generate_starter_room());
        rooms.push(Self::generate_intermediate_room());
        rooms.push(Self::generate_intermediate_room());
        rooms.push(Self::generate_hard_room());
        rooms.push(Self::generate_boss_room());

        Self { rooms }
    }

    pub fn generate_starter_room() -> Vec<Mob> {
        let mut room_mobs = Vec::<Mob>::new();

        room_mobs.push(Mob::spawn_goblin());
        room_mobs.push(Mob::spawn_ghost());

        room_mobs
    }

    pub fn generate_intermediate_room() -> Vec<Mob> {
        let mut room_mobs = Vec::<Mob>::new();

        room_mobs.push(Mob::spawn_goblin());
        room_mobs.push(Mob::spawn_ghost());
        room_mobs.push(Mob::spawn_orc());
        room_mobs.push(Mob::spawn_sorcerer());

        room_mobs
    }

    pub fn generate_hard_room() -> Vec<Mob> {
        let mut room_mobs = Vec::<Mob>::new();

        room_mobs.push(Mob::spawn_goblin());
        room_mobs.push(Mob::spawn_orc());
        room_mobs.push(Mob::spawn_sorcerer());
        room_mobs.push(Mob::spawn_orc());
        room_mobs.push(Mob::spawn_troll());

        room_mobs
    }

    pub fn generate_boss_room() -> Vec<Mob> {
        let mut room_mobs = Vec::<Mob>::new();

        room_mobs.push(Mob::spawn_demon());

        room_mobs
    }
}
