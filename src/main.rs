use rand::Rng;
use std::cmp::{Ord, Ordering};
use std::io;

#[derive(Debug)]
enum RpsMoves {
    Rock,
    Paper,
    Scissors,
}

fn get_player_value() -> Result<RpsMoves, String> {
    let mut typed_move = String::new();
    io::stdin().read_line(&mut typed_move).unwrap();
    let typed_move = typed_move.trim().to_lowercase();

    match typed_move.as_ref() {
        "r" => Ok(RpsMoves::Rock),
        "p" => Ok(RpsMoves::Paper),
        "s" => Ok(RpsMoves::Scissors),
        "q" => std::process::exit(0),
        _ => Err("Wrong input".to_string()),
    }
}
fn get_computer_value() -> RpsMoves {
    let computer_move_number = rand::thread_rng().gen_range(0, 3);

    match computer_move_number {
        0 => RpsMoves::Rock,
        1 => RpsMoves::Paper,
        _ => RpsMoves::Scissors,
    }
}

impl Ord for RpsMoves {
    fn cmp(&self, other: &RpsMoves) -> Ordering {
        match self {
            RpsMoves::Rock => match other {
                RpsMoves::Rock => Ordering::Equal,
                RpsMoves::Paper => Ordering::Less,
                RpsMoves::Scissors => Ordering::Greater,
            },
            RpsMoves::Paper => match other {
                RpsMoves::Rock => Ordering::Greater,
                RpsMoves::Paper => Ordering::Equal,
                RpsMoves::Scissors => Ordering::Less,
            },
            RpsMoves::Scissors => match other {
                RpsMoves::Rock => Ordering::Less,
                RpsMoves::Paper => Ordering::Greater,
                RpsMoves::Scissors => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for RpsMoves {
    fn partial_cmp(&self, other: &RpsMoves) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RpsMoves {
    fn eq(&self, other: &RpsMoves) -> bool {
        self == other
    }
}

impl Eq for RpsMoves {}

//Generate new move for the computer

fn main() {

    println!("Type r/p/s to select a move or q to exit the application.");

    let player_move = get_player_value().unwrap();
    let computer_move = get_computer_value();

    match player_move.cmp(&computer_move) {
        Ordering::Less => println!(
            "Computer wins, player_move {:?} computer_move {:?}",
            player_move, computer_move
        ),
        Ordering::Equal => println!(
            "Nobody wins, player_move {:?} computer_move {:?}",
            player_move, computer_move
        ),
        Ordering::Greater => println!(
            "You win, player_move {:?} computer_move {:?}",
            player_move, computer_move
        ),
    }
}
