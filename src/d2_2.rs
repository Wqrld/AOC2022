use std::fs;

// This is the most horrible code i've ever written. Don't even look at it.

const ROCKVAL: i32 = 1;
const PAPERVAL: i32 = 2;
const SCISSORVAL: i32 = 3;

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

const WIN: &str = "Z";
const DRAW: &str = "Y";
const LOSS: &str = "X";

const WINSCORE: i32 = 6;
const DRAWSCORE: i32 = 3;
const LOSSSCORE: i32 = 0;

fn beats(what: &str) -> &str{
    return match what {
        SCISSORS => ROCK,
        PAPER => SCISSORS,
        ROCK => PAPER,
        _ => panic!("Invalid type")
    };
}

fn loses(what: &str) -> &str{
    return match what {
        SCISSORS => PAPER,
        PAPER => ROCK,
        ROCK => SCISSORS,
        _ => panic!("Invalid type")
    };
}

fn movescore(what: &str) -> i32 {
    return match what {
        SCISSORS => SCISSORVAL,
        PAPER => PAPERVAL,
        ROCK => ROCKVAL,
        _ => panic!("Invalid type")
    };
}

pub fn main(){
        let file = fs::read_to_string("data/d2.txt").unwrap();
        let lines = file.lines();

        let mut total = 0;
        for line in lines {
            let mut splitted = line.split(" ");
            let they: &str = splitted.next().unwrap();
            let outcome: &str = splitted.next().unwrap();

            match outcome {
                WIN => { total += movescore(beats(they)) + WINSCORE; }
                DRAW => { total += movescore(they) + DRAWSCORE; }
                LOSS => { total += movescore(loses(they)) + LOSSSCORE; }
                _ => panic!("invalid outcome")
            }

        }

        println!("Total: {}", total)
}