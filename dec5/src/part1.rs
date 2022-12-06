use std::fs;

fn main() {
    // Read in data
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    let andmed = contents.split("\r\n\r\n");
    let read = andmed.collect::<Vec<&str>>();

    // Parse strings
    let mut stacks = read[0].split("\r\n").collect::<Vec<&str>>();
    let commands = read[1].split("\r\n").collect::<Vec<&str>>();
    let nr: Vec<u32> = stacks.pop().unwrap().split(" ").filter_map(|w| w.parse().ok()).collect();
    let stack_count = nr.len();

    // Create stack_count amount of empty stacks
    let mut stacks_vec: Vec<Vec<&str>> = vec![];
    for _i in 0..stack_count {
        stacks_vec.push(vec![]);
    }

    // Populate with crates
    for _i in 0..stacks.len() {
        let vec: Vec<&str> = stacks.pop().unwrap().split("").collect();
        for j in 0..stack_count {
            if vec[4*j+2] != "" && vec[4*j+2] != " "  {
                stacks_vec[j].push(vec[4*j+2]);
            }
        }
    }

    // Go through commands to move crates around
    for command in commands {
        let vec: Vec<&str> = command.split(" ").collect::<Vec<&str>>();
        let how_many = vec[1].parse::<usize>().unwrap();
        let from = vec[3].parse::<usize>().unwrap();
        let to = vec[5].parse::<usize>().unwrap();
        
        for _i in 0..how_many {
            let temp = stacks_vec[from-1].pop().unwrap();
            stacks_vec[to-1].push(temp);
        }
    }

    // Find crates at the top of stacks
    let mut answer = String::new();
    for i in 0..stack_count {
        answer.push_str(stacks_vec[i].last().unwrap());
    }

    println!("{}", answer);
}