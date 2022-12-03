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

    let mut a = "";
    let mut b ="";
    let mut c="";

    let mut i = 0;
    let mut total = 0;

    for line in lines {
        match i {
            0 => a = line,
            1 => b = line,
            2 => {
                c = line;
                for char in a.chars() {
                    if b.contains(char) && c.contains(char) {
                        total += getCharVal(char);
                        println!("Found char {} in line {}, val: {}", char, line, getCharVal(char));
                        break;
                    }
                }
                i = -1;
            },
            _ => panic!("Invalid value for i")
        }
        i += 1;


    }

    println!("The total is!: {}", total);


}