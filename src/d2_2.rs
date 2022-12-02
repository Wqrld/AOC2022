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
    if what == SCISSORS {return ROCK}
    if what == PAPER {return SCISSORS}
    if what == ROCK {return PAPER}
    else {
        return ROCK; // TODO cleanup
    }
}

fn loses(what: &str) -> &str{
    if what == SCISSORS {return PAPER}
    if what == PAPER {return ROCK}
    if what == ROCK {return SCISSORS}
    else {
        return ROCK; // TODO cleanup
    }
}

fn movescore(what: &str) -> i32 {
    if what == SCISSORS {return SCISSORVAL}
    if what == PAPER {return PAPERVAL}
    if what == ROCK {return ROCKVAL}
    else { return 0 };
}

pub fn main(){
        let file = fs::read_to_string("data/d2.txt").unwrap();
        let lines = file.lines();

        let mut total = 0;
        for line in lines {
            let mut splitted = line.split(" ");
            let they: &str = splitted.next().unwrap();
            let outcome: &str = splitted.next().unwrap();

            // Paper beats Rock beats scissors beats paper

            if outcome == WIN {
                total += movescore(beats(they)) + WINSCORE;
                println!("WIN | They was {}, doing {}, total: {}", they, beats(they), total)
            }else if outcome == DRAW {
                total += movescore(they) + DRAWSCORE;
                println!("DRAW | They was {}, doing {}, total: {}", they, beats(they), total)
            }else if outcome == LOSS{
                total += movescore(loses(they)) + LOSSSCORE;
                println!("LOSS | They was {}, doing {}, total: {}", they, beats(they), total)
            }
        }

    println!("Total: {}", total)
}