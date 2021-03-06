use crate::dice;
use crate::player;
use colored::*;
use std::convert::TryInto;

const NUMBER_OF_DICE_TO_ROLL:u8 = 3;
const SCORES_TO_WIN:u8 = 13;
pub const MAX_NUMBER_OF_PLAYERS:usize = 8;

#[derive(Debug, Copy, Clone)]
pub enum Game_alert {
    Loss,
    WinMaybe, 
    ContinueTheGame,
}

pub fn print_game_alert(alert: Game_alert) -> String {
    match alert {
       Game_alert::Loss => "Unfortunately, you lost all points accumulated during the last turn!".to_string(),
       Game_alert::WinMaybe => "Congratulations, you have 13 points!".to_string(),
       Game_alert::ContinueTheGame => "If you want, you can continue your turn!".to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct Game {

    pub did_end_game: bool,
    pub want_to_continue: bool,

    players: Vec<player::Player>,
    current_player_index: i8,
    winner_index: Option<i8>,
    potential_winners: Vec<i8>,
    did_roll_dice: bool,

    dice_manager: dice::DiceManager,
}

impl Game {

    pub fn new()  -> Game {
        Game {
           did_end_game: false,
           want_to_continue: true,
           players: Vec::new(),
           current_player_index: 0,
           winner_index: None,
           potential_winners: Vec::new(),
           did_roll_dice: false,
           dice_manager: dice::DiceManager::new(),
        }
    }

    fn did_start_game(& self) -> bool {
        return !self.players.is_empty()
    }

    fn end_game_index(&self) -> Option<i8> {
        match self.winner_index {
            Some(x) => {
                if (x - 1) < 0 { 
                   return Some((self.players.len() - 1).try_into().unwrap_or(0))
                 } else {
                     return Some(x - 1)
                 }
            },
            None => None
        }
    }

    fn current_player(&mut self) -> &mut player::Player {
        return &mut self.players[self.current_player_index as usize];
    }
}

impl Game {

    pub fn start(&mut self, players_name: &Vec<String>) {
        self.dice_manager.setup_dices();
        self.players = player::Player::init_players(players_name);
    }

    pub fn start_new_game(&mut self) {
        let n = self.players.len();

        for i in 0..n {
           self.players[i].score = 0;
           self.players[i].end_my_turn();
        }

        self.dice_manager.return_all_dices();
        self.current_player_index = 0;
        self.print_players(false);
        self.did_roll_dice = false;
        self.potential_winners = Vec::new();
        self.winner_index = None;
    }

    fn end_game(&mut self) {
        self.players.sort_by(|a, b| b.score.cmp(&a.score));
        let max = self.players[0].score;
        let mut i = 0;

        while self.players.len() > i && self.players[i].score == max {
            println!("++ Congratulations, {}! You won the game! ++", self.players[i].name.green());
            i += 1
        }

        self.print_players(true);
        println!("Would you like to start a new game with the same players?");
        println!("y / n");
        self.did_end_game = true;
    }
    
    fn increase_index(&mut self) {
        if self.current_player_index == (self.players.len() - 1).try_into().unwrap_or(0) { 
            self.current_player_index = 0;
        } else {
            self.current_player_index += 1;
        }
    }

    fn prepare_for_next_step(&mut self, sides: &Vec<dice::Side>) {

        if self.current_player().current_lifes >= NUMBER_OF_DICE_TO_ROLL {
            println!("{}",print_game_alert(Game_alert::Loss).red());

            self.current_player().scores_of_current_move = 0;
            self.current_player().end_my_turn();
            self.dice_manager.return_all_dices();

            //Check if this player is the last for this game
            if self.end_game_index() != None && self.winner_index != None  {
                if self.current_player_index == self.end_game_index().unwrap(){
                    self.end_game();
                }
            } 
            self.increase_index();
            return 

        } else if self.current_player().score + self.current_player().scores_of_current_move >= SCORES_TO_WIN {
            println!("{}",print_game_alert(Game_alert::WinMaybe));

            if self.winner_index != None && self.end_game_index() != None {
                if self.current_player_index == self.end_game_index().unwrap() {
                    let mut max = self.players.clone().into_iter()
                    .map(|p| { p.score}).max();

                    if self.current_player().score + self.current_player().scores_of_current_move > max.unwrap_or(0) {
                        self.current_player().score += self.current_player().scores_of_current_move;
                        self.potential_winners.push(self.current_player_index);
                        self.end_game();
                        return
                    }

                }
            }
        }

        println!("{}\n",print_game_alert(Game_alert::ContinueTheGame).bold());
    }

    pub fn end_turn(&mut self) {
        if !self.did_roll_dice {
            println!("Wrong command! You must roll the dice first!");
            return
        }

        self.dice_manager.return_all_dices();
        Game::print_separator_line();

        self.print_total_score_of_this_move();
        self.current_player().end_my_turn();

        if self.winner_index == None && self.current_player().score >= SCORES_TO_WIN {
            if self.current_player_index != (self.players.len() - 1).try_into().unwrap_or(0) {
                self.winner_index = Some(self.current_player_index);
            } else {
                self.end_game();
                return;
            }
        }

        if self.end_game_index() != None {
            if self.current_player_index == self.end_game_index().unwrap() {
                self.end_game();
                self.print_players(true);
                return;
            }
        }

       self. increase_index();
       self.print_players(false);
       self.did_roll_dice = false;
       Game::print_separator_line();
    }

    pub fn roll_dice(&mut self) {
        self.did_roll_dice = true;
        let mut side: Vec<dice::Side> = Vec::new();

        side.extend(self.dice_manager.draw_more_dice(NUMBER_OF_DICE_TO_ROLL));
        self.current_player().previous_throws.push(side.to_vec());

        for s in &side {
            match s {
                dice::Side::Brain => self.current_player().scores_of_current_move += 1,
                dice::Side::Shotguns => self.current_player().current_lifes += 1,
                _ => continue,
            }
        }

        Game::print_separator_line();
        self.print_result(&side);
        self.print_previous_throws();
        self.prepare_for_next_step(&side);

        if !self.did_end_game {
            self.print_players(false);
        }

        Game::print_separator_line();
    }
}

//MARK:- Prints
impl Game {

    pub fn print_players(&self, did_end_game: bool) {
        for player in self.players.iter() {
            if !did_end_game && player == &self.players[self.current_player_index as usize] {
                println!("{}: {}  <---- Current player", player.name.green(), (player.score + player.scores_of_current_move).to_string().green()); 
            } else {
                println!("{}: {}", player.name, player.score);
            }
        }
    }

    pub fn print_commands(&self) {
    
        println!("\n{}","Commands available:".green().bold());
        println!("{}","roll_dice/r: use this command to draw and roll the dice".green());
        println!("{}","end_turn/e: use this command to end your turn and get the points you've accumulated so far".green());
    }

    fn print_total_score_of_this_move(& self) {
        let p =  &self.players[self.current_player_index as usize];
        println!("{}  {} {}\n", "In this turn you won:".magenta(), p.scores_of_current_move.to_string().magenta(),"points".magenta());
    }

    fn print_previous_throws(&self) {
        println!( "{} \n","All dice rolls for this turn:".blue().bold());
        let p =  &self.players[self.current_player_index as usize];

        for side in &p.previous_throws {
            self.print_result(side);
        }

        println!("{} \n","Total:".blue().bold());
        println!( "{} : BRAIN \n{} : SHOTGUN\n", p.scores_of_current_move.to_string().blue(), p.current_lifes.to_string().blue());
    }

    fn print_result(&self, sides: &Vec<dice::Side>) {
        let brains: Vec<dice::Side> = sides
        .clone()
        .into_iter()
        .filter(|side| *side == dice::Side::Brain)
        .collect();

        let shotguns: Vec<dice::Side> = sides
        .clone()
        .into_iter()
        .filter(|side| *side == dice::Side::Shotguns)
        .collect();

        let footsteps: Vec<dice::Side> = sides
        .clone()
        .into_iter()
        .filter(|side| *side == dice::Side::Footsteps)
        .collect();
        
        println!("{}x BRAIN  {}x SHOTGUNS  {}x FOOTSTEPS \n", brains.len(), shotguns.len(), footsteps.len());
    }
    
    fn print_separator_line() {
        println!("{} \n", "++++++++++++++++++++++++++++++++++++++++".red()); 
    }

}






