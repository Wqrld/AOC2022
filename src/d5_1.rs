use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn to_int(numb: &str) -> i32 {
    let myint: i32 = numb.parse().unwrap();
    return myint;
}

pub fn main(){
    let mut hash = HashMap::new();
    hash.insert("1", VecDeque::from(["Z", "J", "N", "W", "P", "S"]));
    hash.insert("2", VecDeque::from(["G", "S", "T"]));
    hash.insert("3", VecDeque::from(["V", "Q", "R", "L", "H"]));
    hash.insert("4", VecDeque::from(["V", "S", "T", "D"]));
    hash.insert("5", VecDeque::from(["Q", "Z", "T", "D", "B", "M", "J"]));
    hash.insert("6", VecDeque::from(["M", "W", "T", "J", "D", "C", "Z", "L"]));
    hash.insert("7", VecDeque::from(["L", "P", "M", "W", "G", "T", "J"]));
    hash.insert("8", VecDeque::from(["N", "G", "M", "T", "B", "F", "Q", "H"]));
    hash.insert("9", VecDeque::from(["R", "D", "G", "C", "P", "B", "Q", "E"]));


    let file = fs::read_to_string("./data/d5.txt").expect("Whoops, cant read");
    let lines = file.lines();


    for line in lines {
        let mut l = line.clone().split(" ");
        l.next();
        let amount = to_int(l.next().unwrap());
        l.next();
        let from = l.next().unwrap();
        l.next();
        let to = l.next().unwrap();
        for _ in 0..amount {
            let mut queue = hash.get(from).unwrap().clone();
            let popped = queue.pop_back().unwrap();
            hash.insert(from, queue);
            let mut newqueue = hash.get(to).unwrap().clone();
            newqueue.push_back(popped);
            hash.insert(to, newqueue);
        }



    }
    for (index, queue) in hash {
        let mut q2 = queue.clone();
        println!("{}: {}", index, q2.pop_back().unwrap());
    }
}