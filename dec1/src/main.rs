
fn main() {
    let andmed = "2027
    1630
    4699
    3860
    5686
    1178
    4983
    
    45637
    
    5634
    2755
    3537
    5047
    1878
    4820
    
    3013
    1604
    1574
    
    2090
    4120
    
    1099
    4518
    4639
    1637".split("
    ");
    

    let tuhi = "";

    let mut maks = [0, 0, 0];
    let mut kalorid: u32 = 0;

    for rida in andmed {
        if rida.eq(tuhi) {
            // Kontroll top 3
            if kalorid > maks[0] {
                maks[2] = maks[1];
                maks[1] = maks[0];
                maks[0] = kalorid;
            } else if kalorid > maks[1] {
                maks[2] = maks[1];
                maks[1] = kalorid;
            }
            else if kalorid > maks[2] {
                maks[2] = kalorid;
            }
            kalorid = 0;

        } else {
            let rea_nr: u32 = rida.parse().unwrap();
            kalorid = kalorid + rea_nr;
        }
    }
    println!("Nr1 calories = {}", maks[0]);
    println!("Nr2 calories = {}", maks[1]);
    println!("Nr3 calories = {}", maks[2]);
    println!("Sum of three elves: {}", maks[0] + maks[1] + maks[2])
}
