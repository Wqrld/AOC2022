use std::fs;

pub fn to_int(numb: &str) -> i32 {
    return numb.parse().unwrap();
}

pub fn main(){
    let file = fs::read_to_string("./data/d4.txt").expect("thought i could read this file");
    let lines = file.lines();

    let mut total = 0;
    for line in lines {
        let mut splitted = line.split(",");
        let mut l = splitted.next().unwrap().split("-");
        let mut r = splitted.next().unwrap().split("-");
        let l1 = to_int(l.next().unwrap());
        let l2 = to_int(l.next().unwrap());
        let r1 = to_int(r.next().unwrap());
        let r2 = to_int(r.next().unwrap());


        if !(r1  > l2 || r2 < l1) {
            total += 1;
        }
    }
    println!("total: {}", total);
}