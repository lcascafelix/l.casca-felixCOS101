use std::io::Read;
use std::io::Write;

fn main(){
    let students = vec![
         ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
         ("Adams Aliyu", "ECO10110101", "Economics", 100),
         ("Shania Bolade", "CSC10328828", "Computer Science", 200),
         ("Adekunle Gold", "EEE11020202", "Electrical Engineering", 200),
         ("Bianca Edemoh", "MEE10202001", "Mechanical Engineering", 100),

    ];
    for students in &students{
        println!("   Displaying student Information   ");
        println!("Name: {:<15} | Matric: {} | Dept: {:<10} | Level: {}",
        students.0, students.1, students.2, students.3
         );
    }
    

    let mut file = std::fs::File::create("pau_smis.txt").expect("creation of this file was unsuccesful");

    for student in &students{
        let rows = format!(
        "{} | {} | {} | {}\n", student.0, student.1, student.2, student.3
            );
        file.write_all(rows.as_bytes()).expect("write unsuccessful");

        println!("\nStudent data succesfully written to '{}'","pau_smis.txt");

        let mut file = std::fs::File::open("pau_smis.txt").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        print!("{}",content);
    }
}