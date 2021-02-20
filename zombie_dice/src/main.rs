mod dice;
mod player;
mod game;

use std::env;
use std::process;
use colored::*;


fn main() {
/*
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
    let p = player::Player::init_players(args);
    println!("{:?}",p);
*/

println!("{} \n", ")❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒".red());
println!("{} \n", ")❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒❒");
}

