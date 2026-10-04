use std::io;

fn main() {
    // 1. Display the menu
    println!("===== THE RESTAURANT MENU =====");
    println!("P - Poundo Yam / Edinkaiko Soup  ₦3,200");
    println!("F - Fried Rice & Chicken         ₦3,000");
    println!("A - Amala & Ewedu Soup           ₦2,500");
    println!("E - Eba & Egusi Soup             ₦2,000");
    println!("W - White Rice & Stew            ₦2,500");
    println!("===============================");

    // 2. Read the food type
    println!("Enter the food letter (P, F, A, E or W):");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    // 3. Read the quantity
    println!("Enter the quantity:");
    let mut qty_input = String::new();
    io::stdin()
        .read_line(&mut qty_input)
        .expect("Failed to read input");
    let quantity: u32 = qty_input
        .trim()
        .parse()
        .expect("Please enter a valid whole number");

    // 4. Decide the price based on the letter
    let price: u32 = match choice.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    // 5. Compute the total
    let mut total = (price * quantity) as f64;
    println!("Total before discount: ₦{:.2}", total);

    // 6. Apply 5% discount if total is greater than ₦10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("Discount (5%): -₦{:.2}", discount);
    }

    println!("Total to pay: ₦{:.2}", total);
}

