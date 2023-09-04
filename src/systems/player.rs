use crate::prelude::*;

pub struct Player {
    pub class: i32,
    pub level: i32,
    pub max_health: i32,
    pub current_health: i32,
    pub exp: i32,
    pub armor: i32,
    pub resistance: i32,
    pub dodge: i32,
    pub min_damage: i32,
    pub max_damage: i32,
    pub crit_chance: i32,
    pub dmg_ismagic: bool,
    pub rests: i32,
}

impl Player {
    pub fn spawn_warrior() -> Self {
        Self {
            class: 1,
            level: 1,
            max_health: 20,
            current_health: 20,
            exp: 0,
            armor: 2,
            resistance: 2,
            dodge: 4,
            min_damage: 4,
            max_damage: 6,
            dmg_ismagic: false,
            crit_chance: 15,
            rests: 1,
        }
    }

    pub fn spawn_wizard() -> Self {
        Self {
            class: 2,
            level: 1,
            max_health: 13,
            current_health: 13,
            exp: 0,
            armor: 1,
            resistance: 1,
            dodge: 6,
            min_damage: 3,
            max_damage: 5,
            dmg_ismagic: true,
            crit_chance: 15,
            rests: 1,
        }
    }

    pub fn spawn_ranger() -> Self {
        Self {
            class: 3,
            level: 1,
            max_health: 16,
            current_health: 16,
            exp: 0,
            armor: 1,
            resistance: 1,
            dodge: 8,
            min_damage: 3,
            max_damage: 4,
            dmg_ismagic: false,
            crit_chance: 20,
            rests: 1,
        }
    }

    pub fn get_class_input() -> i32 {
        let mut class_choice = 0;

        while class_choice <= 0 {
            let mut input = String::new();

            stdin().read_line(&mut input).expect("Error reading input.");

            class_choice = match input.trim().parse() {
                Ok(1) => 1,
                Ok(2) => 2,
                Ok(3) => 3,
                Ok(_) => {
                    println!("\nPlease select one of the given options.");
                    continue;
                }
                Err(_) => {
                    println!("\nPlease enter a number.");
                    continue;
                }
            }
        }

        class_choice
    }

    pub fn pick_class(selected_class: i32) -> Player {
        let player: Player = if selected_class == 1 {
            println!("\nYou selected the Warrior class.");
            Player::spawn_warrior()
        } else if selected_class == 2 {
            println!("\nYou selected the Wizard class.");
            Player::spawn_wizard()
        } else {
            println!("\nYou selected the Ranger class.");
            Player::spawn_ranger()
        };

        // player.expect("No player class was selected.")
        player
    }

    pub fn level_up(&mut self) {
        let mut enough_xp = true;

        while enough_xp {
            let req_exp = 20 * (self.level - 1) + 10;

            if self.exp >= req_exp {
                self.level += 1;
                self.exp -= req_exp;

                match self.class {
                    1 => {
                        self.max_health += 6;
                        self.current_health = self.max_health;
                        self.armor += 2;
                        self.resistance += 1;
                        self.dodge += 2;
                        self.min_damage += 1;
                        self.max_damage += 3;
                        self.crit_chance += 2;
                    }
                    2 => {
                        self.max_health += 4;
                        self.current_health = self.max_health;
                        self.armor += 1;
                        self.resistance += 1;
                        self.dodge += 3;
                        self.min_damage += 2;
                        self.max_damage += 2;
                        self.crit_chance += 3;
                    }
                    3 => {
                        self.max_health += 5;
                        self.current_health = self.max_health;
                        self.armor += 1;
                        self.resistance += 1;
                        self.dodge += 4;
                        self.min_damage += 2;
                        self.max_damage += 2;
                        self.crit_chance += 5;
                    }
                    _ => {}
                };
                println!("--------\nYou leveled up! You are now level {}", self.level);
            } else {
                enough_xp = false;
            }
        }
    }

    pub fn show_stats(&self) {
        match self.class {
            1 => {
                println!("\nClass: Warrior");
            }
            2 => {
                println!("\nClass: Wizard");
            }
            3 => {
                println!("\nClass: Ranger");
            }
            _ => {}
        }

        println!("Level: {}", self.level);
        println!("Health: {} / {}", self.current_health, self.max_health);
        println!("Rests: {}", self.rests);
        println!("Current Exp: {} / {}", self.exp, 20 * (self.level - 1) + 10);
        println!("Armor: {}", self.armor);
        println!("Magic Resistance: {}", self.resistance);
        println!("Dodge Chance: {}%", self.dodge);
        println!("Damage Range: {} - {}", self.min_damage, self.max_damage);
        println!("Critical Hit Chance: {}%", self.crit_chance);
        println!("Does Magical Damage: {}\n", self.dmg_ismagic);
    }

    pub fn rest_health(&mut self) {
        if self.rests >= 1 {
            let half_hp = self.max_health / 2;
            if self.current_health <= half_hp {
                self.current_health += half_hp
            } else {
                self.current_health = self.max_health;
            }
            self.rests -= 1;

            println!(
                "\nYou used a rest. Your health is now {} / {}.",
                self.current_health, self.max_health
            );
            println!("Remaining rests: {}", self.rests);
        } else {
            println!("\nYou don't have anymore rests to use!");
        }
    }

    pub fn player_attack(&self, mob: &mut Mob, rng: &mut ThreadRng) { 
      
      //let total_damage: i32;

      //println!("Critical Hit! You did {} damage to {}\n", total_damage, mob.name);

      //println!("You did {} damage to {}\n", total_damage, mob.name);

    }

}

