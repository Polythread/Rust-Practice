use std::io;

fn main() {
    println!("CALCULATOR");

    println!("Enter Your First Number: ");
    let mut first: String = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Please enter the number");

    println!("Enter Your Second Number: ");
    let mut second: String = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Please enter the number");

    let first: f32 = first.trim().parse().expect("Wrong Input!");
    let second: f32 = second.trim().parse().expect("Wrong Input!");

    println!("Choose an operator: ");
    let mut op: String = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Please enter correct operator!");

    let result = match op.trim() {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => first / second,
        _ => {
            println!("Unknown Operator!");
            return;
        }
    };

    println!("Result: {}", result);
}
