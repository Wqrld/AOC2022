use std::collections::VecDeque;
use std::fs;

pub fn main(){
    let file = fs::read_to_string("./data/d6.txt").expect("invalid file");
    let line = file.replace("\r\n", ""); //expect windows newlines, doesn't really matter here.


    let mut dq = VecDeque::new();
    let mut i = 0;
    for char in line.chars() {

        dq.push_front(char);
        if dq.len() >= 14 {
            if dq.len() > 14 {
                dq.pop_back();
            }
            // Performance won't be great, but luckily we don't need much of that here.
            let mut vc: Vec<char>  = Vec::from(dq.clone());
            vc.sort();
            vc.dedup();
            let mut compareTo = Vec::from(dq.clone());
            compareTo.sort();
            if vc == Vec::from(compareTo) {
                println!("Match found at {}", i+1);
                break;
            }

        }
        i+=1;
    }
}