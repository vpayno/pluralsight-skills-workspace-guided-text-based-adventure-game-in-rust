# pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust
Pluralsight Skills Workspace Guided Text-based Adventure Game in Rust

[![actionlint](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/gh-actions.yml/badge.svg?branch=main)](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/gh-actions.yml)
[![spellcheck](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/spellcheck.yml/badge.svg?branch=main)](https://github.com/vpayno/pluralsight-skills-workspace-guided-text-based-adventure-game-in-rust/actions/workflows/spellcheck.yml)

## Repo Introduction

Instead of using the web lab I'm working on this locally with Vim.

## Project Introduction

In this lab, you have been given a text-based adventure game application in Rust. However, the major components have not been fully completed. As such, it will be your responsibility to finish each of their implementations within this lab.

The application can be run at any time by executing the `cargo run` command in the terminal or clicking the Run button in the terminal.

Note: You may notice that when completing tasks for earlier modules(ie. modules 1 and 2) the test output may still output warnings. This is normal as these warnings are just referencing the later modules you have not yet completed and will gradually disappear as you progress

## Steps/Tasks

1. Step `player.rs`

    1. [x] Task `pick_class()`
    2. [x] Task `level_up()`
    3. [x] Task `rest_health()`
    4. [x] Task `player_attack()`

### Step 1: Implementing the Player

The first incomplete component is the Player. The Player module is located within `player.rs`, which has a `Player` struct already defined.
There are also multiple spawning functions for each class, each with their own distinct values.
The `get_class_input()` method is a predefined auxiliary function for reading your input when selecting a class.

You will need to complete the `Player` module by implementing `pick_class()` for selecting your class at the start of the game, `level_up()` for strengthening your character on each level up, `rest_health()` for healing between dungeon rooms, and `player_attack()` to simulate combat.

#### Task 1: `player.rs`

Within `player.rs`, locate the `pick_class()` function.

Complete the implementation using the accompanying instructions.

##### `pick_class()` Instructions

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

##### `pick_class()` Hint #1

Use the `Option` enum in Rust with `Some()` when assigning the player variable.

This is done using `player = Some(input)` which extracts the value of input and assigns it to the `player` if it exists, or `None` if it doesn't.

##### `pick_class()` Hint #2

The functions to spawn a class are defined for you at the top of the `Player` module.
Functions of a module can be called in Rust with the syntax `Module::function()`.
For example, `Player::spawn_warrior()`.

#### Tesk 2: `level_up()`

Within `player.rs`, locate the `level_up()` function.

Complete the implementation with the accompanying instructions.

##### `level_up()` Instructions

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

##### `level_up()` Hint #1

When increasing stat values, use a match statement.
`match` works similar to switch statements from other languages.
The default case of the `match` statement should be to do nothing in the format

```rust
_  => {}
```

#### Tesk 3: `rest_health()`

Within `player.rs`, locate the `rest_health()` function.
Complete the implementation with the accompanying instructions.

##### `rest_health()` Instructions

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

##### `rest_health()` Hint #1

You can use a ternary statement when setting `self.current_health`.

#### Tesk 4: `player_attack()`

Within `player.rs`, locate the `player_attack()` function.
Complete the implementation with the accompanying instructions.

##### `rest_player_attack()` Instructions

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

##### `player_attack()` Hint #1

After completing all the aforementioned tasks, the `Player` module is now complete.
Now the player is able to successfully pick their class at the start of the app, level up when reaching the correct amount of exp to grow stronger, rest to recover health, and simulate an attack against enemies.
Next, we will move on the to `Mob` module.
