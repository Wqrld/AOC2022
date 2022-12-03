use std::fs;

fn getCharVal(v: char) -> i32{
    if v >= 'a' && v <= 'z' {
        return v as i32 - 'a' as i32 + 1;
    } else if v >= 'A' && v <= 'Z' {
        return v as i32 - 'A' as i32 + 27;
    }
    panic!("Received invalid character value");
}

pub fn main(){
    let file = fs::read_to_string("./data/d3.txt").expect("Could not read file");
    let lines = file.lines();

    let mut total = 0;
    for line in lines {
        let (left, right): (&str, &str) = line.split_at(line.chars().count() / 2);
        for c in left.chars() {
            if right.contains(c) {
                // the item that appears in both is c;
                total += getCharVal(c);
                break;
            }
        }
    }
    println!("val of p {}", getCharVal('p'));
    println!("val of L {}", getCharVal('L'));
    println!("The total is!: {}", total);


}