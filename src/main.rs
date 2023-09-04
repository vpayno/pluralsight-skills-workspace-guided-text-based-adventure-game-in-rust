use adventure_game::prelude::*;

#[tokio::main]
async fn main() {    
    println!("Welcome to your Rust Adventure Game!\nPick your desired class using their corresponding numbers.\n");
    println!("1) Warrior\n2) Wizard\n3) Ranger");

    let mut game_data = GameData::init();
    let mut dungeon_rooms = game_data.dungeon.rooms.iter_mut();
    let mut curr_room = dungeon_rooms.next().unwrap();

    let mut player = Player::pick_class(Player::get_class_input());

    'game_loop: loop {

        match game_data.game_state {
            GameState::AwaitingInput => {
                loop {
                    game_data.game_state.show_menu(game_data.room_number, game_data.mob_index, curr_room);

                    let mut input = String::new();

                    stdin().read_line(&mut input).expect("Error reading input.");

                    match input.trim().parse() {
                        Ok(1) => {
                            game_data.game_state = GameState::Combat;
                            break;
                        },
                        Ok(2) => {
                            player.show_stats();
                        },
                        Ok(3) => {
                            player.rest_health();
                        },
                        Ok(4) => {
                            println!("\nYou quit the game.");
                            break 'game_loop;
                        },
                        Ok(_) => {
                            println!("\nPlease enter a valid choice");
                        },
                        Err(_) => {
                            println!("\nPlease enter a number");
                        }
                    }
                }
            },
            GameState::Combat => {
                let curr_mob = &mut curr_room[game_data.mob_index];

                game_data.game_state.combat(curr_mob, &mut player, &mut game_data.rng).await;

                if player.current_health <= 0 {
                    game_data.game_state = GameState::Defeat;
                    continue;
                }
                else{
                    game_data.game_state.player_victory(curr_mob, &mut player, &mut game_data.mob_index);
                }

                if game_data.mob_index == curr_room.len() {
                    let next_room = dungeon_rooms.next();
                    if let None = next_room {
                        game_data.game_state = GameState::WinGame;
                    }
                    else {
                        curr_room = next_room.unwrap();
                        game_data.game_state.proceed_room(&mut game_data.room_number, &mut game_data.mob_index, &mut player);
                    }
                    continue;
                }
            },
            GameState::Defeat => {
                println!("\nOh no! You have died.");
                break;
            },
            GameState::WinGame => {
                println!("\nCongratulations, you have conquered the Dungeon!.");
                break;
            }
        }
    }
}
