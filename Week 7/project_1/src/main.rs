use std::f64::consts::PI;
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== The Shape Calculator ===");
        println!("1. Trapezium Area");
        println!("2. Rhombus Area");
        println!("3. Parallelogram Area");
        println!("4. Cube Surface Area");
        println!("5. Cylinder Volume");
        println!("6. Exit");
        
        let choice = read_number("Enter your choice (1-6): ") as u32;

        match choice {
            1 => calculate_trapezium(),
            2 => calculate_rhombus(),
            3 => calculate_parallelogram(),
            4 => calculate_cube(),
            5 => calculate_cylinder(),
            6 => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a number between 1 and 6."),
        }
    }
}


fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}


fn calculate_trapezium() {
    println!("\n--- Trapezium Area ---");
    let height = read_number("Enter height: ");
    let base1 = read_number("Enter base 1: ");
    let base2 = read_number("Enter base 2: ");

    let area = (height / 2.0) * (base1 + base2);
    println!("Area of the Trapezium: {:.2}", area);
}


fn calculate_rhombus() {
    println!("\n--- Rhombus Area ---");
    let diagonal1 = read_number("Enter diagonal 1: ");
    let diagonal2 = read_number("Enter diagonal 2: ");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of the Rhombus: {:.2}", area);
}


fn calculate_parallelogram() {
    println!("\n--- Parallelogram Area ---");
    let base = read_number("Enter base: ");
    let altitude = read_number("Enter altitude: ");

    let area = base * altitude;
    println!("Area of the Parallelogram: {:.2}", area);
}

// 4. Cube Surface Area: 6 * side * side
fn calculate_cube() {
    println!("\n--- Cube Surface Area ---");
    let side = read_number("Enter side length: ");

    let surface_area = 6.0 * side * side;
    println!("Surface Area of the Cube: {:.2}", surface_area);
}


fn calculate_cylinder() {
    println!("\n--- Cylinder Volume ---");
    let radius = read_number("Enter radius: ");
    let height = read_number("Enter height: ");

    let volume = PI * radius * radius * height;
    println!("Volume of the Cylinder: {:.2}", volume);
}