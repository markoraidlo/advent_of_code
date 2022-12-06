use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("AAAAA");
    let andmed = contents.split("\r\n");
    let read = andmed.collect::<Vec<&str>>();

    let mut pairs: u32 = 0;

    for rida in read {
        let rida_split = rida.split(",");
        let vec = rida_split.collect::<Vec<&str>>();

        let esimene: Vec<u32> = vec[0].split("-").filter_map(|w| w.parse().ok()).collect();
        let teine: Vec<u32> = vec[1].split("-").filter_map(|w| w.parse().ok()).collect();
         
        if esimene[0] >= teine[0] && esimene[1] <= teine[1] {
            pairs += 1;
        } else if esimene[0] <= teine[0] && esimene[1] >= teine[1] {
            pairs += 1;
        } else if esimene[0] == teine[1] || esimene[1] == teine[0] {
            pairs += 1;
        } else if esimene[0] == teine[0] || esimene[1] == teine[1] {
            pairs += 1;
        } else if esimene[0] <= teine[0] && esimene[1] >= teine[0] {
            pairs += 1;
        } else if esimene[0] >= teine[0] && esimene[0] <= teine[1] {
            pairs += 1;
        }
    }
    println!("{}", pairs);
}
