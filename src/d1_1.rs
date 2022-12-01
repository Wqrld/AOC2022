use std::collections::BinaryHeap;
use std::fs;

// Day 1 - Elve calories
pub fn main() {
    let file = fs::read_to_string("./data/d1_1.txt").expect("Could not read file");
    let lines = file.lines();

    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for line in lines {
        if line == "" {
            heap.push(sum);
            sum = 0;
        } else {
            match line.parse::<i32>() {
                Ok(n) => sum += n,
                Err(e) => println!("Parsing error: {}, {:?}", e, line)
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