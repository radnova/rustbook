fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 2..4");
    }
    
    let condition: bool = false;
    let number = if condition { 5 } else { 6 };

    println!("value: {number}");
}
