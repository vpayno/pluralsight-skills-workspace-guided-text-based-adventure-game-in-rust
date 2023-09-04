use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum GameState {
    AwaitingInput,
    Combat,
    Defeat,
    WinGame,
}

impl GameState {
    pub fn show_menu(&self, room_number: i32, mob_index: usize, curr_room: &mut Vec<Mob>) {
        println!("--------\nCurrent room: {} / 6", room_number);
        println!("Remaining mobs: {}", curr_room.len() - mob_index);
        println!(
            "\nWhat would you like to do?\n1) Initiate Combat\n2) Show Stats\n3) Rest\n4) Quit"
        );
    }

    pub async fn combat(&self, curr_mob: &mut Mob, player: &mut Player, rng: &mut ThreadRng) {
        let mut player_turn = true;

        println!("\nYou started the fight against a {}\n", curr_mob.name);

        while player.current_health >= 0 && curr_mob.current_health >= 0 {
            if player_turn {
                player.player_attack(curr_mob, rng);
            } else {
                curr_mob.mob_attack(player, rng);
            }
            player_turn = !player_turn;
            sleep(Duration::from_millis(1000)).await;
        }
    }

    pub fn player_victory(
        &mut self,
        curr_mob: &mut Mob,
        player: &mut Player,
        mob_index: &mut usize,
    ) {
        //println!("{} has died!", curr_mob.name);
        //println!("\nYou gained {} exp!", curr_mob.exp);
    }

    pub fn proceed_room(
        &mut self,
        room_number: &mut i32,
        mob_index: &mut usize,
        player: &mut Player,
    ) {
    }
}

pub struct GameData {
    pub rng: ThreadRng,
    pub game_state: GameState,
    pub dungeon: Dungeon,
    pub room_number: i32,
    pub mob_index: usize,
}

impl GameData {
    pub fn init() -> Self {
        Self {
            rng: rand::thread_rng(),
            game_state: GameState::AwaitingInput,
            dungeon: Dungeon::build_dungeon(),
            room_number: 1,
            mob_index: 0,
        }
    }
}
