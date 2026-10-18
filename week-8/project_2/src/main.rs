use std::io;

struct Candidate {
    name: String,
    experience: u32,
}

fn main() {
    let mut candidates = Vec::new();

    println!("--- Enter details for 5 users ---");

    for i in 1..=5 {
        println!("\nUser {}:", i);

        // Get name input
        println!("Enter name: ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name");
        let name = name.trim().to_string();

        // Get years of experience input
        println!("Enter years of experience: ");
        let mut exp_input = String::new();
        io::stdin()
            .read_line(&mut exp_input)
            .expect("Failed to read experience");
        
        let experience: u32 = match exp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number entered. Defaulting experience to 0.");
                0
            }
        };

        candidates.push(Candidate { name, experience });
    }

    
    if let Some(most_experienced) = candidates.iter().max_by_key(|c| c.experience) {
        println!("\n==============================");
        println!("       RESULTS SUMMARY        ");
        println!("==============================");
        println!(
            "The most experienced user is {} with {} years of experience!",
            most_experienced.name, most_experienced.experience
        );
    }
}