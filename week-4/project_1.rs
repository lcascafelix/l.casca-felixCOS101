use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let d: f32 = (b * b) - (4.0 * a * c);

    if d > 0.0 {
        let root1: f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2: f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root: f32 = -b / (2.0 * a);
        println!("Exactly one real root: {}", root);
    } else {
        println!("No real roots");
    }
}