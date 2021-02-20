use crate::dice;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u8,  /// the number of brains
    pub scores_of_current_move: u8,
    pub current_lifes: u8, /// the number of .guns at the current move
    pub previous_throws: Vec<Vec<dice::Side>>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player{
            name: name,
            score: 0,
            scores_of_current_move: 0,
            current_lifes: 0,
            previous_throws: Vec::new(),
        }
    }

    pub fn end_my_turn(&mut self) {
        self.current_lifes = 0;
        self.score += self.scores_of_current_move;
        self.scores_of_current_move = 0;
        self.previous_throws = Vec::new();
    }
}

impl Player {
    pub fn init_players(names: Vec<String>) -> Vec<Player> {
        let mut res: Vec<Player> = Vec::new();

        for name in names {
            res.push(Player::new(name));
        }

        res
    }
}