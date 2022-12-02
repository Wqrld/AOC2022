use std::fs;

const ROCKVAL: i32 = 1;
const PAPERVAL: i32 = 2;
const SCISSORVAL: i32 = 3;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

pub fn main(){
        let file = fs::read_to_string("data/d2.txt").unwrap();
        let lines = file.lines();

        let mut total = 0;
        for line in lines {
            let mut splitted = line.split(" ");
            let they2 = splitted.next().unwrap();
            let they = they2.replace("A", "X").replace("B", "Y").replace("C", "Z");
            let you = splitted.next().unwrap();
            let mut yourselectedscore = 0;
            if you == "X" {yourselectedscore = ROCKVAL}
            if you == "Y" {yourselectedscore = PAPERVAL}
            if you == "Z" {yourselectedscore = SCISSORVAL}
            // index minus X times 3?

            if you == they {
                total += yourselectedscore + DRAW;
            }else if (you == "X" && they == "Y") || (you == "Y" && they == "Z") || (you == "Z" && they == "X") {
                total += yourselectedscore + LOSS;
            }else{
                total += yourselectedscore + WIN;
            }

        }

    println!("Total: {}", total)
}