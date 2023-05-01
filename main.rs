use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();

    let number1 = rng.gen_range(1..100);
    let number2 = rng.gen_range(1..100);
    let addition = number1 + number2;
    let multiplication = number1 * number2;

    if addition < 100 {
        println!("The sum of {number1} and {number2} is less than 100.");
    } else {
        println!("The sum is over 100!");
    }

    if multiplication < 100 {
        println!("The product of {number1} and {number2} is less than 100.");
    } else {
        println!("The product is over 100!");
    }
}