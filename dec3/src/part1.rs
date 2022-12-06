use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    let andmed = contents.split("\r\n");

    let mut summa: u32 = 0;

    for rida in andmed {
        let vec: Vec<char> = rida.chars().collect();
        let half = vec.len() / 2;

        let mut taht: char = '0';

        for i in 0..half {
            for j in half..vec.len(){
                if vec[i] == vec[j] {
                    taht = vec[i];
                }
            }
        }
        if taht.is_lowercase() {
            summa += taht as u32 - 96;
        } else {
            summa += taht as u32 - 38;
        }
    }
    println!("{}", summa)
}
