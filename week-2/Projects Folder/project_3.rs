fn main() {
    let price: f64 = 210000.0;
    let depreciation_rate: f64 = 5.0;
    let years: i32 = 3;

    let value = price * (1.0 - depreciation_rate / 100.0).powi(years);

    println!("The value of the TV after {} years is N{:.2}", years, value);
}