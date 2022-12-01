use std::collections::{BinaryHeap, HashMap};
use std::fs;

// elve calories
pub fn main(){
        println!("hi!");
        let contents = fs::read_to_string("./data/d1_1.txt").expect("Could not read file");
        let splitted = contents.split("\r\n");
        let mut counter = 0;

        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for line in splitted {
                if line == "" {
                        counter += 1;
                        heap.push(sum);
                        sum = 0;

                } else {
                        match line.parse::<i32>() {
                                Ok(n) =>  {sum += n}
                                Err(e) => println!("unknown error: {}, {:?}", e, line)
                        }

                }
        }


        let mut total = 0;
        total += heap.pop().unwrap_or_default();
        println!("The total for the #1 is: {}", total);
        total += heap.pop().unwrap_or_default();
        total += heap.pop().unwrap_or_default();
        println!("the total for the top3 is: {}", total)
}