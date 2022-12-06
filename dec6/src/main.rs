use std::fs;

fn checker(vec: Vec<&str>) -> bool {
    // Base case
    if vec.len() == 2 {
        return vec[0] != vec[1];
    }

    // Recursive call
    if !checker(vec[1..].to_vec()) {
        return false;
    }

    // Current step check
    for i in 1..vec.len() {
        if vec[0] == vec[i] {
            return false;
        }
    }

    return true;
}

fn main() {
    // Read in data
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    let andmed = contents.split("\r\n\r\n");
    let read = andmed.collect::<Vec<&str>>();

    let split = read[0].split("");
    let str_vec = split.collect::<Vec<&str>>();
    let mut answer = 0;

    for i in 0..str_vec.len() {
        if checker(str_vec[i..i+14].to_vec()) {
            answer = i + 13;
            break
        }
    }
    //println!("{:?}", str_vec);
    println!("{}", answer);

}