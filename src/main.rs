fn main() {
    let mut name = "Pranish";
    name = "Anish";
    println!("Hello, !{}", name);
    println!("{}", calculate())
}

fn calculate() -> u128 {
    let a:u128 = 10;
    let b: u128 = 20;
    a + b
}
