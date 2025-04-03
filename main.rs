fn main() {
    let name = "Rustacean";
    let age = 3;

    println!("Hello, {}!", name);
    println!("You are {} years old.", age);

    let sum = add(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}