#![allow(warnings, unused)]

mod dice;
mod player;
mod game;

use std::env;
use std::process;
use colored::*;
use std::io;
use std::io::prelude::*;
use colored::*;
use console::Emoji;


fn main() {

    //TODO:- Better validation
    let mut args: Vec<String> = env::args().collect();
    let args_count = args.len();

    if args_count <= 2 {
        println!("Please, enter the number of players and their names!");
        process::exit(0);
    }

    let number_of_players = args[1].parse::<usize>().unwrap();

    if number_of_players != args_count - 2 {
        println!("Please, enter the number of players and their names!");
        process::exit(0);
    }
    //Remove the program call and the number of players. We only need the players names
    args.remove(0);
    args.remove(0);

    //Create players 
    let p = player::Player::init_players(&args);
    println!("{:?}",p);

    let mut game = game::Game::new();
    game.start(&args);
    println!("Welcome to all!");
    println!("The order of players is:");
    game.print_players(false);

    //Print the commands
    println!("{}","Commands available:".green());
    println!("{}","roll_dice/r: use this command to draw and roll the dice".green());
    println!("{}","end_turn/e: use this command to end your turn and get the points you've accumulated so far".green());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(command) => {
                match command.trim() {
                    "roll_dice" => game.roll_dice(),
                    "r" => game.roll_dice(),
                    "end_turn" => game.end_turn(),
                    "e" => game.end_turn(),
                    "y" => {
                        game.did_end_game = false;
                        game.start_new_game();
                    }
                    "n" => {
                        game.want_to_continue = false;
                        break;
                    }
                    _ => println!("{}","Wrong command!".red()),
                }
            },
            Err(e) => println!("{}","Wrong command!".red())
         }
    }
}

