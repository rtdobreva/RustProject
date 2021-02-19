use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
}

#[derive(Debug, Copy, Clone)]
pub enum Side {
    Brain,
    Footsteps,
    Shotguns,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub struct Dice {
     color: Color,
     isThrown: bool, 
}

impl Dice {
    // A public constructor method
    pub fn new(color: Color) -> Dice {
        Dice {
            color: color,
            isThrown: false,
        }
    }
}

impl Dice {

    pub fn get_sides(&self) -> Vec<Side>  {
        let mut args: Vec<Side> = vec![Side::Brain, Side::Brain, Side::Footsteps, Side::Footsteps, Side::Shotguns, Side::Shotguns];

        match self.color {
            Color::Red => {
                args.remove(0);
                args.push(Side::Shotguns);
                return args
            },
            Color::Green => {
                args.pop();
                args.push(Side::Brain);
                return args
            },
            _ => {
                return args
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiceManager {
    pub dices: Vec<Dice>,
}

impl DiceManager {

    pub fn available_dices(&self) -> Vec<Dice> {
        return self.dices.clone().into_iter()
        .filter(|x| !x.isThrown)
        .collect::<Vec<Dice>>();
    }

    //Initial configuration
    //    6 "зелени" - 3х🧠,2х👣,1х💥
    //    4 "жълти" - 2х🧠, 2х👣, 2х💥
    //    3 "червени" - 1х🧠, 2х👣, 3х💥
    pub fn setup_dices(&mut self) {
        for _ in 0..6 {
            let die = Dice::new( Color::Green);
            self.dices.push(die);
        }

        for _ in 0..4 {
            let die = Dice::new( Color::Yellow);
            self.dices.push(die);
        }

        for _ in 0..3 {
            let die = Dice::new( Color::Red);
            self.dices.push(die);
        }
    }

    pub fn roll(&mut self) -> Side {
        if self.available_dices().len() == 0 {
            return Side::Unknown;
        }

        let mut rng = rand::thread_rng();

       let dice_index = rng.gen_range(0..self.dices.len());
       let sides = self.dices[dice_index].get_sides();
       let side_index = rng.gen_range(0..sides.len());

       match sides[side_index] {
        Side::Footsteps => self.dices[dice_index].isThrown = false,
        _ => self.dices[dice_index].isThrown = true
       }

       return  sides[side_index]
    }

    pub fn draw_more_dice(&mut self, count: u8) -> Vec<Side> {
        let mut result: Vec<Side> = Vec::new();

        for _ in 0..count {
           let side = self.roll();
           result.push(side);
        }

        return result;
    }

    pub fn returnAllDice(&mut self) {
        for i in 0..self.dices.len() {
            self.dices[i].isThrown = false
        }
    }
    
}


