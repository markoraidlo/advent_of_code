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
            "Z" => {
                score += 6;
                match vec[0] {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    &_ => println!("False input: {}",  vec[0]),
                }},
            "Y" => {
                score = score + 3;
                match vec[0] {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    &_ => println!("False input: {}",  vec[0]),
                }},
            "X" => {
                score = score + 0;
                match vec[0] {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    &_ => println!("False input: {}",  vec[0]),
                }},
            &_ => println!("False input: {}",  vec[1]),
        }
    }

    println!("Score: {}", score);
}
