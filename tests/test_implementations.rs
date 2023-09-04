use adventure_game::prelude::*;

fn correct_build_dungeon() -> Vec<Vec<Mob>> {
    let mut rooms = Vec::<Vec<Mob>>::new();
    rooms.push(Dungeon::generate_starter_room());
    rooms.push(Dungeon::generate_intermediate_room());
    rooms.push(Dungeon::generate_intermediate_room());
    rooms.push(Dungeon::generate_hard_room());
    rooms.push(Dungeon::generate_boss_room());

    rooms
}

#[test]
fn test_pick_class() {
    let warr = Player::pick_class(1);
    let mage = Player::pick_class(2);
    let ranger = Player::pick_class(3);

    assert_eq!(
        1, warr.class,
        "An input of '1' should give the player the warrior class"
    );
    assert_eq!(
        2, mage.class,
        "An input of '2' should give the player the wizard class"
    );
    assert_eq!(
        3, ranger.class,
        "An input of '3' should give the player the ranger class"
    );
}

#[test]
fn test_level_up() {
    let mut warr = Player::pick_class(1);

    warr.exp = 10;
    warr.level_up();

    assert_eq!(
        2, warr.level,
        "Player should level up when reaching the required amount of exp.(1)"
    );

    warr.exp += 35;
    warr.level_up();

    assert_eq!(
        3, warr.level,
        "Player should level up when reaching the required amount of exp.(2)"
    );
    assert_eq!(5, warr.exp, "Player should keep any leftover exp after reaching the required amount of exp to level up.");

    warr.exp += 130;
    warr.level_up();

    assert_eq!(5, warr.level, "Player should level up to the appropriate level if they receive enough exp for multiple level ups.");
    assert_eq!(15, warr.exp, "Player should keep any leftover exp after receiving the needed exp for multiple level ups.");
}

#[test]
fn test_rest_health() {
    let mut warr = Player::pick_class(1);

    assert_eq!(1, warr.rests, "Player should start with 1 available rest.");

    warr.current_health = 5;
    warr.rest_health();

    assert_eq!(
        15, warr.current_health,
        "Using a rest should heal a player for half their total health."
    );
    assert_eq!(
        0, warr.rests,
        "Using the rest_health() function should consume a rest.(1)"
    );

    warr.rest_health();

    assert_eq!(
        15, warr.current_health,
        "Using rest_health() when the player has 0 rests should not heal them."
    );
    assert_eq!(0, warr.rests, "A player cannot go below 0 rests.");

    warr.rests += 1;
    warr.rest_health();

    assert_eq!(
        20, warr.current_health,
        "Rests can only heal a player up to their maximum health if they have over 50% health."
    );
    assert_eq!(
        0, warr.rests,
        "Using the rest_health() function should consume a rest.(2)"
    );
}

#[test]
fn test_player_attack() {
    let mut warr = Player::spawn_warrior();
    let mut rng = rand::thread_rng();
    let mut test_mob = Mob {
        name: String::from("Test"),
        exp: 70,
        max_health: 100,
        current_health: 100,
        armor: 0,
        resistance: 0,
        min_damage: 0,
        max_damage: 0,
        dmg_ismagic: true,
    };

    //test no crit physical
    warr.min_damage = 8;
    warr.max_damage = 12;
    warr.crit_chance = 0;
    test_mob.armor = 5;

    for _x in 1..=3 {
        warr.player_attack(&mut test_mob, &mut rng);
    }

    assert!(
        test_mob.current_health <= 91,
        "Failed instance of no-crit physical damage minimum."
    );
    assert!(
        test_mob.current_health >= 79,
        "Failed instance of no-crit physical damage maximum."
    );
    test_mob.current_health = test_mob.max_health;

    //test crit physical
    warr.crit_chance = 100;
    for _x in 1..=3 {
        warr.player_attack(&mut test_mob, &mut rng);
    }

    assert!(
        test_mob.current_health <= 67,
        "Failed instance of crit physical damage minimum."
    );
    assert!(
        test_mob.current_health >= 43,
        "Failed instance of crit physical damage maximum."
    );
    test_mob.current_health = test_mob.max_health;

    //test max armor
    test_mob.armor = 100;

    for _x in 1..=3 {
        warr.player_attack(&mut test_mob, &mut rng);
    }

    assert!(
        test_mob.current_health == 97,
        "Failed instance of armor being greater than damage dealt."
    );
    test_mob.current_health = test_mob.max_health;

    //test no crit magical
    warr.crit_chance = 0;
    warr.dmg_ismagic = true;
    test_mob.resistance = 6;

    for _x in 1..=3 {
        warr.player_attack(&mut test_mob, &mut rng);
    }

    assert!(
        test_mob.current_health <= 94,
        "Failed instance of no-crit magic damage minimum."
    );
    assert!(
        test_mob.current_health >= 82,
        "Failed instance of no-crit magic damage maximum."
    );
    test_mob.current_health = test_mob.max_health;

    //test crit magical
    warr.crit_chance = 100;
    warr.min_damage = 15;
    warr.max_damage = 20;
    test_mob.resistance = 10;

    for _x in 1..=3 {
        warr.player_attack(&mut test_mob, &mut rng);
    }

    assert!(
        test_mob.current_health <= 40,
        "Failed instance of crit magic damage minimum."
    );
    assert!(
        test_mob.current_health >= 10,
        "Failed instance of crit magic damage maximum."
    );
    test_mob.current_health = test_mob.max_health;

    //test max resistance
    test_mob.armor = 0;
    test_mob.resistance = 300;

    for _x in 1..=3 {
        warr.player_attack(&mut test_mob, &mut rng);
    }

    assert!(
        test_mob.current_health == 97,
        "Failed instance of resistance being greater than damage dealt."
    );
    test_mob.current_health = test_mob.max_health;
}

#[test]
fn test_mob_attack() {
    let mut warr = Player::spawn_warrior();
    let mut rng = rand::thread_rng();
    let mut test_mob = Mob {
        name: String::from("Test"),
        exp: 70,
        max_health: 100,
        current_health: 100,
        armor: 0,
        resistance: 0,
        min_damage: 8,
        max_damage: 8,
        dmg_ismagic: false,
    };

    //test no dodge physical
    warr.dodge = 0;

    for _x in 1..=3 {
        test_mob.mob_attack(&mut warr, &mut rng);
    }

    assert!(
        warr.current_health <= 11,
        "Failed instance of no dodge physical damage minimum."
    );
    assert!(
        warr.current_health >= 2,
        "Failed instance of no dodge physical damage maximum."
    );
    warr.current_health = warr.max_health;

    //test no dodge max armor
    warr.armor = 100;
    for _x in 1..=3 {
        test_mob.mob_attack(&mut warr, &mut rng);
    }

    assert!(
        warr.current_health == 17,
        "Failed instance of no dodge max armor damage taken."
    );
    warr.current_health = warr.max_health;

    //test dodge physical
    warr.armor = 0;
    warr.dodge = 100;

    for _x in 1..=3 {
        test_mob.mob_attack(&mut warr, &mut rng);
    }

    assert!(
        warr.current_health == 20,
        "Failed instance of full dodge damage taken."
    );

    //test no dodge magical
    warr.dodge = 0;
    warr.resistance = 4;
    test_mob.dmg_ismagic = true;

    for _x in 1..=3 {
        test_mob.mob_attack(&mut warr, &mut rng);
    }

    assert!(
        warr.current_health <= 17,
        "Failed instance of no dodge magical damage minimum."
    );
    assert!(
        warr.current_health >= 8,
        "Failed instance of no dodge magical damage maximum."
    );
    warr.current_health = warr.max_health;

    //test no dodge max resistance
    warr.resistance = 100;

    for _x in 1..=3 {
        test_mob.mob_attack(&mut warr, &mut rng);
    }

    assert!(
        warr.current_health == 17,
        "Failed instance of no dodge max resistance."
    );
    warr.current_health = warr.max_health;

    //test dodge magical
    warr.dodge = 100;
    for _x in 1..=3 {
        test_mob.mob_attack(&mut warr, &mut rng);
    }

    assert!(
        warr.current_health == 20,
        "Failed instance of dodge magical."
    );
}

#[test]
fn test_starter_room() {
    let test_room = Dungeon::generate_starter_room();

    let mut room_mobs = Vec::<Mob>::new();

    room_mobs.push(Mob::spawn_goblin());
    room_mobs.push(Mob::spawn_ghost());

    assert_eq!(test_room, room_mobs);
}

#[test]
fn test_intermediate_room() {
    let test_room = Dungeon::generate_intermediate_room();

    let mut room_mobs = Vec::<Mob>::new();

    room_mobs.push(Mob::spawn_goblin());
    room_mobs.push(Mob::spawn_ghost());
    room_mobs.push(Mob::spawn_orc());
    room_mobs.push(Mob::spawn_sorcerer());

    assert_eq!(test_room, room_mobs);
}

#[test]
fn test_hard_room() {
    let test_room = Dungeon::generate_hard_room();

    let mut room_mobs = Vec::<Mob>::new();

    room_mobs.push(Mob::spawn_goblin());
    room_mobs.push(Mob::spawn_orc());
    room_mobs.push(Mob::spawn_sorcerer());
    room_mobs.push(Mob::spawn_orc());
    room_mobs.push(Mob::spawn_troll());

    assert_eq!(test_room, room_mobs);
}

#[test]
fn test_boss_room() {
    let test_room = Dungeon::generate_boss_room();

    let mut room_mobs = Vec::<Mob>::new();

    room_mobs.push(Mob::spawn_demon());

    assert_eq!(test_room, room_mobs);
}

#[test]
fn test_game_state() {
    let test_game_state = GameState::AwaitingInput;

    let test_dungeon = correct_build_dungeon();
    let mut test_data = GameData::init();

    assert_eq!(test_data.game_state, test_game_state);

    test_data.game_state = GameState::Combat;
    test_data.game_state = GameState::Defeat;
    test_data.game_state = GameState::WinGame;

    let test_range =
        if test_data.rng.gen_range(0..=10) >= 0 && test_data.rng.gen_range(0..=10) <= 10 {
            true
        } else {
            false
        };
    assert_eq!(test_range, true);

    assert_eq!(test_data.dungeon.rooms, test_dungeon);
    assert_eq!(test_data.room_number, 1);
    assert_eq!(test_data.mob_index, 0);
}

#[test]
fn test_spawn_mobs() {
    let goblin = Mob {
        name: String::from("Goblin"),
        exp: 10,
        max_health: 8,
        current_health: 8,
        armor: 2,
        resistance: 1,
        min_damage: 3,
        max_damage: 4,
        dmg_ismagic: false,
    };

    let orc = Mob {
        name: String::from("Orc"),
        exp: 20,
        max_health: 18,
        current_health: 18,
        armor: 5,
        resistance: 3,
        min_damage: 5,
        max_damage: 9,
        dmg_ismagic: false,
    };

    let ghost = Mob {
        name: String::from("Ghost"),
        exp: 15,
        max_health: 10,
        current_health: 10,
        armor: 2,
        resistance: 2,
        min_damage: 3,
        max_damage: 4,
        dmg_ismagic: true,
    };

    let sorcerer = Mob {
        name: String::from("Sorcerer"),
        exp: 20,
        max_health: 16,
        current_health: 16,
        armor: 3,
        resistance: 4,
        min_damage: 6,
        max_damage: 8,
        dmg_ismagic: true,
    };

    let troll = Mob {
        name: String::from("Troll"),
        exp: 40,
        max_health: 35,
        current_health: 35,
        armor: 8,
        resistance: 5,
        min_damage: 9,
        max_damage: 16,
        dmg_ismagic: false,
    };

    let demon = Mob {
        name: String::from("Demon"),
        exp: 70,
        max_health: 40,
        current_health: 40,
        armor: 8,
        resistance: 7,
        min_damage: 10,
        max_damage: 12,
        dmg_ismagic: true,
    };

    let test_mobs = vec![goblin, ghost, orc, sorcerer, troll, demon];

    assert_eq!(Mob::spawn_goblin(), test_mobs[0]);
    assert_eq!(Mob::spawn_ghost(), test_mobs[1]);
    assert_eq!(Mob::spawn_orc(), test_mobs[2]);
    assert_eq!(Mob::spawn_sorcerer(), test_mobs[3]);
    assert_eq!(Mob::spawn_troll(), test_mobs[4]);
    assert_eq!(Mob::spawn_demon(), test_mobs[5]);
}

#[test]
fn test_player_victory() {
    let mut game_data = GameData::init();
    let mut player = Player::spawn_warrior();
    let mut demon = Mob::spawn_demon();

    game_data.game_state = GameState::WinGame;

    game_data
        .game_state
        .player_victory(&mut demon, &mut player, &mut game_data.mob_index);

    assert_eq!(player.level, 3);
    assert_eq!(game_data.mob_index, 1);
    assert_eq!(game_data.game_state, GameState::AwaitingInput);

    game_data.game_state = GameState::Defeat;

    game_data
        .game_state
        .player_victory(&mut demon, &mut player, &mut game_data.mob_index);

    assert_eq!(player.level, 4);
    assert_eq!(game_data.mob_index, 2);
    assert_eq!(game_data.game_state, GameState::AwaitingInput);
}

#[test]
fn test_proceed_room() {
    let mut game_data = GameData::init();
    let mut player = Player::spawn_warrior();

    game_data.game_state = GameState::Defeat;
    game_data.mob_index = 5;

    game_data.game_state.proceed_room(
        &mut game_data.room_number,
        &mut game_data.mob_index,
        &mut player,
    );

    assert_eq!(player.rests, 2);
    assert_eq!(game_data.room_number, 2);
    assert_eq!(game_data.mob_index, 0);
    assert_eq!(game_data.game_state, GameState::AwaitingInput);

    game_data.game_state = GameState::Defeat;
    game_data.mob_index = 5;

    game_data.game_state.proceed_room(
        &mut game_data.room_number,
        &mut game_data.mob_index,
        &mut player,
    );

    assert_eq!(player.rests, 3);
    assert_eq!(game_data.room_number, 3);
    assert_eq!(game_data.mob_index, 0);
    assert_eq!(game_data.game_state, GameState::AwaitingInput);
}
