use std::fs;

fn main() {
    // Read in data
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    let andmed = contents.split("\r\n\r\n");
    let read = andmed.collect::<Vec<&str>>();

    let split = read[0].split("");
    let str_vec = split.collect::<Vec<&str>>();
    let mut answer = 0;

    for i in 4..str_vec.len() {
        if str_vec[i-3] != str_vec[i-2] && str_vec[i-3] != str_vec[i-1] &&
        str_vec[i-3] != str_vec[i] && str_vec[i-2] != str_vec[i-1] &&
        str_vec[i-2] != str_vec[i] && str_vec[i-1] != str_vec[i] {
            //println!("{:?}", str_vec[i]);
            answer = i;
            break;
        }
    }
    // 1-2, 1-3, 1-4, 2-3, 2-4, 3-4
    //println!("{:?}", str_vec);
    println!("{}", answer);

}