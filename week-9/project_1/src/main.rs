use std::io::Write;

fn main(){
    let lager = "Lager:\n33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n";
    let stout = "Stout:\nLegend\nTurbo King\nWilliams\n\n";
    let non_alcoholic = "Non-Alcoholic:\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n\n";

    let mut file = std::fs::File::create("nb.csv").expect("Failed to create");
    file.write_all("Welcome to Nigerian Breweries\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("Failed to write");
    file.write_all(stout.as_bytes()).expect("Failed to write");
    file.write_all(non_alcoholic.as_bytes()).expect("Failed to write");
    println!("\nData written to file.");
    
}