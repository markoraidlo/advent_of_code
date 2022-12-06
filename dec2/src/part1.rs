use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    
    let andmed = contents.split("\r\n");

    let mut score: i32 = 0;
    for rida in andmed {
        let split = rida.split(" ");
        let vec: Vec<&str> = split.collect();

        //Punktid kas kivi, paber või käärid
        match vec[1] {
            "X" => {
                score += 1;
                match vec[0] {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    &_ => println!("False input: {}",  vec[0]),
                }},
            "Y" => {
                score = score + 2;
                match vec[0] {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    &_ => println!("False input: {}",  vec[0]),
                }},
            "Z" => {
                score = score + 3;
                match vec[0] {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    &_ => println!("False input: {}",  vec[0]),
                }},
            &_ => println!("False input: {}",  vec[1]),
        }
    }

    println!("Score: {}", score);
}
