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

fn main() {

    let mut args: Vec<String> = validate_input();

    let mut game = game::Game::new();
    game.start(&args);

    println!("{}","+------------------------------------------------------------------------------------------+");
    println!("| Welcome to all!                                                                          |");
    println!("{}","+------------------------------------------------------------------------------------------+");
    println!("\n{}","The order of players is:".bold());
    game.print_players(false);

    game.print_commands();

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

//MARK:- Validation
fn validate_input() -> Vec<String> {
      let mut args: Vec<String> = env::args().collect();
      let args_count = args.len();
  
      if args_count <= 2 {
          println!("Please, enter the number of players and their names!");
          process::exit(0);
      }
  
      //Cast the number of players
      let mut number_of_players:usize = 0;
      match args[1].parse::<usize>() {
          Ok(n) => number_of_players = n,
          Err(..) => println!("Invalid command!"),
      };
  
      if number_of_players <= 1 || number_of_players >= game::MAX_NUMBER_OF_PLAYERS {
          println!("Sorry, this game is for 2-8 players only!");
          process::exit(0);
      }
  
      if number_of_players != args_count - 2 {
          println!("Please, enter the number of players and their names!");
          process::exit(0);
      }

      //Remove the program call and the number of players. We only need the players names
        args.remove(0);
        args.remove(0);

      return args;
}

