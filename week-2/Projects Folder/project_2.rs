fn main() {
    let toshiba = 450000.00;
    let mac = 1500000.00;
    let hp = 750000.00;
    let dell = 2850000.00;
    let acer = 250000.00;

    let sum = toshiba + mac + hp + dell + acer;
    let average = sum / 5.0;

    println!("P.M. Okeke and Sons Ltd");
    println!("-------------------------");
    println!("Sales Records");
    println!("Toshiba: ₦{:.2}", toshiba);
    println!("Mac:     ₦{:.2}", mac);
    println!("HP:      ₦{:.2}", hp);
    println!("Dell:    ₦{:.2}", dell);
    println!("Acer:    ₦{:.2}", acer);
    println!("-------------------------");
    println!("Sum = ₦{:.2}", sum);
    println!("Average = ₦{:.2}", average);
}