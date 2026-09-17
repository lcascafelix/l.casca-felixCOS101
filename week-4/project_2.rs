use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let is_experienced = input1.trim().to_lowercase();

    if is_experienced == "yes" {
        println!("Enter age: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let age: i32 = input2.trim().parse().expect("Not a valid number");

        if age >= 40 {
            println!("Annual Incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual Incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual Incentive: N1,300,000");
        } else {
            println!("No incentive criteria specified for ages 28 to 29.");
        }
    } else {
        println!("Annual Incentive: N100,000");
    }
}