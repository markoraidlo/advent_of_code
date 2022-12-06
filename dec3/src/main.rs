use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    let andmed = contents.split("\r\n");
    let read = andmed.collect::<Vec<&str>>();

    let mut summa: u32 = 0;

    for i in 0..read.len()/3 {
        let esimene: Vec<char> = read[i*3].chars().collect();
        let teine: Vec<char> = read[i*3+1].chars().collect();
        let kolmas: Vec<char> = read[i*3+2].chars().collect();

        let mut taht: char = '0';

        for i in 0..esimene.len() {
            for j in 0..teine.len() {
                if esimene[i] == teine[j] {
                    for k in 0..kolmas.len() {
                        if esimene[i] == kolmas[k] {
                            taht = esimene[i];
                        }
                    }
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
