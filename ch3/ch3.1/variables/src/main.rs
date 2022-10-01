fn main() {
    let mut x = 5;
    println!("x value: {x}");
    x = 6;
    println!("x value: {x}");
    
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let y = 5;
    let y = y + 1;
    
    {
        let y = y * 2;
        println!("y inner scope: {y}");
    }

    println!("y value: {y}");
}
