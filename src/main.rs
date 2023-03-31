mod template;

fn addTwo(a: u64, b: u64) -> u64 {
    return a + b;
}

fn main() {
    println!("{}", addTwo(5, 4));
    let name = "hello";
    println!("{}", name);
    println!("Hello, world!");
}
