fn main() {
    println!("Hello, world!");
    println!("{}", calculate())
}

fn calculate() -> u128 {
    let a:u128 = 10;
    let b: u128 = 20;

    return a + b;
}
