use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub max_health: i32,
    pub current_health: i32,
    pub exp: i32,
    pub armor: i32,
    pub resistance: i32,
    pub min_damage: i32,
    pub max_damage: i32,
    pub dmg_ismagic: bool,
}

impl Mob {
    pub fn spawn_goblin() -> Mob {
        Self {
            name: String::from("Goblin"),
            exp: 10,
            max_health: 8,
            current_health: 8,
            armor: 2,
            resistance: 1,
            min_damage: 3,
            max_damage: 4,
            dmg_ismagic: false,
        }
    }

    pub fn spawn_orc() -> Mob {
        Self {
            name: String::from("Orc"),
            exp: 20,
            max_health: 18,
            current_health: 18,
            armor: 5,
            resistance: 3,
            min_damage: 5,
            max_damage: 9,
            dmg_ismagic: false,
        }
    }

    pub fn spawn_ghost() -> Mob {
        Self {
            name: String::from("Ghost"),
            exp: 15,
            max_health: 10,
            current_health: 10,
            armor: 2,
            resistance: 2,
            min_damage: 3,
            max_damage: 4,
            dmg_ismagic: true,
        }
    }

    pub fn spawn_sorcerer() -> Mob {
        Self {
            name: String::from("Sorcerer"),
            exp: 20,
            max_health: 16,
            current_health: 16,
            armor: 3,
            resistance: 4,
            min_damage: 6,
            max_damage: 8,
            dmg_ismagic: true,
        }
    }

    pub fn spawn_troll() -> Mob {
        Self {
            name: String::from("Troll"),
            exp: 40,
            max_health: 35,
            current_health: 35,
            armor: 8,
            resistance: 5,
            min_damage: 9,
            max_damage: 16,
            dmg_ismagic: false,
        }
    }

    pub fn spawn_demon() -> Mob {
        Self {
            name: String::from("Demon"),
            exp: 70,
            max_health: 40,
            current_health: 40,
            armor: 8,
            resistance: 7,
            min_damage: 10,
            max_damage: 12,
            dmg_ismagic: true,
        }
    }

    pub fn mob_attack(&self, player: &mut Player, rng: &mut ThreadRng) {
        if player.dodge == 0 {
            let base_dmg = rng.gen_range(self.min_damage..=self.max_damage);

            let total_damage = if self.dmg_ismagic {
                (base_dmg - player.resistance).max(1)
            } else {
                (base_dmg - player.armor).max(1)
            };

            player.current_health -= total_damage;

            println!("{} did {} damage to you!\n", self.name, total_damage);
        } else {
            println!("You dodged the attack!\n");
        }
    }
}
