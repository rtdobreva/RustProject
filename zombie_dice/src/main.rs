mod dice;

use std::env;
use std::process;


fn main() {

    //TODO:- Better validation
    let args: Vec<String> = env::args().collect();
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

    for i in 0..args.len(){
       let name = &args[i];
       println!("{} \n", name);
    }

}
