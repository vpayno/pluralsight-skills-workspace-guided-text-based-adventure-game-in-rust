# pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust

Pluralsight Skills Workspace Guided Text-based Adventure Game in Rust

[![actionlint](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/gh-actions.yml/badge.svg?branch=main)](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/gh-actions.yml)
[![spellcheck](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/spellcheck.yml/badge.svg?branch=main)](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/spellcheck.yml)
[![rust](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/rust.yml)

[![gitlab pipline](https://gitlab.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/badges/main/pipeline.svg)](https://gitlab.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/-/pipelines)

## Repo Introduction

Instead of using the web lab I'm working on this locally with Vim.

## Project Introduction

In this lab, you have been given a text-based adventure game application in Rust. However, the major components have not been fully completed. As such, it will be your responsibility to finish each of their implementations within this lab.

The application can be run at any time by executing the `cargo run` command in the terminal or clicking the Run button in the terminal.

Note: You may notice that when completing tasks for earlier modules(ie. modules 1 and 2) the test output may still output warnings.
This is normal as these warnings are just referencing the later modules you have not yet completed and will gradually disappear as you progress

## Steps/Tasks

1. Step `player.rs`

    1. [x] Task `pick_class()`
    2. [x] Task `level_up()`
    3. [x] Task `rest_health()`
    4. [x] Task `player_attack()`

2. Step `mobs.rs`

    1. [x] Task `mob_attack()`

3. Step `dungeon.rs`

    1. [x] Task `generate_starter_room()`
    2. [x] Task `generate_intermediate_room()`
    3. [x] Task `generate_hard_room()`
    4. [x] Task `generate_boss_room()`

4. Step `game_state.rs`

    1. [x] Task `player_victor()`
    2. [ ] Task `proceed_room()`

### Step 1: Implementing the Player

The first incomplete component is the Player. The Player module is located within `player.rs`, which has a `Player` struct already defined.
There are also multiple spawning functions for each class, each with their own distinct values.
The `get_class_input()` method is a predefined auxiliary function for reading your input when selecting a class.

You will need to complete the `Player` module by implementing `pick_class()` for selecting your class at the start of the game,
`level_up()` for strengthening your character on each level up, `rest_health()` for healing between dungeon rooms, and `player_attack()` to simulate combat.

#### Task 1: `player.rs`

Within `player.rs`, locate the `pick_class()` function.

Complete the implementation using the accompanying instructions.

##### Instructions `pick_class()`

```rust
let player;

if selected_class is warrior(value of 1)
    set player to spawn_warrior()
    print warrior class selection

else if selected class is wizard(value of 2)
    set player to spawn_wizard()
    print wizard class selection
else
    set player to spawn_ranger()
    print ranger class selection

player.expect("No player class was selected.")
```

##### Hint #1 `pick_class()`

Use the `Option` enum in Rust with `Some()` when assigning the player variable.

This is done using `player = Some(input)` which extracts the value of input and assigns it to the `player` if it exists, or `None` if it doesn't.

##### Hint #2 `pick_class()`

The functions to spawn a class are defined for you at the top of the `Player` module.
Functions of a module can be called in Rust with the syntax `Module::function()`.
For example, `Player::spawn_warrior()`.

#### Task 2: `level_up()`

Within `player.rs`, locate the `level_up()` function.

Complete the implementation with the accompanying instructions.

##### Instructions `level_up()`

```rust
let mut enough_xp = true;

while enough_xp is true
    define req_exp for level up

    if self.exp >= req_exp
        increment self.level
        subract req_exp from self.exp

        increase player stat values based off self.class
        print level up message
    else
        set enough_xp to false
```

##### Hint #1 `level_up()`

When increasing stat values, use a match statement.
`match` works similar to switch statements from other languages.
The default case of the `match` statement should be to do nothing in the format

```rust
_  => {}
```

#### Task 3: `rest_health()`

Within `player.rs`, locate the `rest_health()` function.
Complete the implementation with the accompanying instructions.

##### Instructions `rest_health()`

```rust
if self.rests >= 1
    calculate half_hp(self.max_health/2)
    if self.current_health <= half_hp, self.current_health += half_hp
    else self.current_health = self.max_health
    decrement self.rests by 1

    print current health
    print remaining rests
else
 print no more rests message
```

##### Hint #1 `rest_health()`

You can use a ternary statement when setting `self.current_health`.

#### Task 4: `player_attack()`

Within `player.rs`, locate the `player_attack()` function.
Complete the implementation with the accompanying instructions.

##### Instructions `rest_player_attack()`

```rust
let mut base_dmg = rng.gen_range(self.min_damage to self.max_damage)
let total_damage

check if attack is a critical hit, if it is then double base_dmg

check if dmg_ismagic
    if it is, set total_damage to base_damage - mob.resistance or 1 if the value is less than 1

if dmg_ismagic is false
    set total_damage to base_damage - mob.armor or 1 if the value is less than 1

subtract total_damage from mob.current_health

if attack is critical hit, print critical hit message. Else print normal damage message.
```

##### Hint #1 `player_attack()`

After completing all the aforementioned tasks, the `Player` module is now complete.
Now the player is able to successfully pick their class at the start of the app, level up when reaching the correct amount of exp to grow stronger, rest to recover health, and simulate an attack against enemies.
Next, we will move on the to `Mob` module.

### Step 2: Implementing the Mobs

The `Mob` module contains all the data regarding the monsters you will encounter in this game.
It is very similar to the `Player` module as it has a Mob struct with multiple spawning functions for mobs with their own distinct values.
The only function that you will need to complete is the `mob_attack()` function to simulate combat against the player.

#### Task 1: `mob_attack()`

Within `mobs.rs`, locate the `mob_attack()` function at the bottom.
It works similarly to `player_attack()` from the previous module, except mobs cannot critically hit the player.
Complete the implementation with the accompanying instructions.

##### Instructions `mob_attack()`

```rust
check if player dodge rolled successfully

if player didnt dodge
    calculate base damage similar to the previous task
    let mut total_damage = 1 #Minimum damage a mob does to a player is 1

    if self.dmg_ismagic
        calculate total damage as base damage - player.resistance or 1
    else
        calculate total damage as base damage - player.armor or 1

    subtract total damage from player.health

    print damage done to player

else
    print dodge message #no damage should have been done to player
```

##### Hint #1 `mob_attack()`

After completing the aforementioned task, the `Mob` module is complete.
Now, both the `Player` and `Mob` can engage in combat by attacking each other.

### Step 3: Dungeon Generation

Now we will need to populate the dungeon with the completed `Mob` creatures.
It would be a good idea to familiarize yourself with all the mob spawning functions that were already predefined for you in the `Mob` module as they'll be used extensively here.

#### Task 1: `generate_starter_room()`

Within `dungeon.rs`, locate the `generate_starter_room()` function. Complete the implementation by spawning in a goblin followed by a ghost.

##### Hint #1 `generate_starter_room()`

Since the functions to spawn the mobs are in the separate `Mob` module, you will need to spawn them with the `Module::function()` syntax.
Since `room_mobs` is just a vector, you can simply `push()` the spawned mob into `room_mobs`.

#### Task 2: `generate_intermediate_room()`

Within `dungeon.rs`, locate the `generate_intermediate_room()` function.
Complete the implementation by spawning in a goblin, a ghost, an orc, and a sorcerer in that order.

#### Task 3: `generate_hard_room()`

Within `dungeon.rs`, locate the `generate_hard_room()` function.
Complete the implementation by spawning mobs in the following order: goblin, orc, sorcerer, another orc, and a troll.

#### Task 4: `generate_boss_room()`

Within `dungeon.rs`, locate the `generate_boss_room()` function. Complete the implementation by spawning in a demon.

### Step 4: Game State

The last module to complete is the `GameState` which stores data on the current progression of the game.
`player_victory()` and `proceed_room()` are the two functions that you will need to implement here.

#### Task 1: `player_victory()`

Within `game_state.rs,` locate the `player_victory()` function. Complete the implementation with the accompanying instructions.

##### Instructions `player_victory()`

```rust
print statements

add curr_mob.exp to player.exp
call player.level_up() #use player parameter, not Player module
increment mob_index #Dereference pointer
set self to GameState::AwaitingInput #Dereference pointer
```

##### Hint #1 `player_victory()`

#### Task 2 `proceed_room()`

Within `game_state.rs,` locate the `proceed_room()` function. Complete the implementation with the accompanying instructions.

##### Instructions `proceed_room()`

```rust
increment room_number #Derefence pointer
increment player.rests #No need to dereference
set mob_index to 0 #Dereference pointer
set self to GameState::AwaitingInput #Dereference pointer
```

With these tasks completed, the app should be fully functional. Great job!
