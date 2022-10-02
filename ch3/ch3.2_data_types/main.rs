use std::io;

fn main() {

    //#############arithmetic 
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32 
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;
    println!("\nx: {x} \ny: {y} \nsum: {sum} \ndifference: {difference} \nproduct: {product} \nquotient: {quotient} \nfloored: {floored} \nremainder: {remainder}");
    
    //############boolean
    let t = true;   
    let f: bool = false;
    println!("{t} or {f}");

    //############char
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("c: {c} \nz: {z} \ncat: {heart_eyed_cat}");

    //###########compound types
    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the tup: ({x}, {y}, {z})\nx: {x} \ny: {y} \nz: {z}");
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("tup.0: {five_hundred} \ntup.1: {six_point_four} \ntup.2: {one}");

    //arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    
    let afirst = a[0];
    let asecond = a[1];
    let bfirst = b[0];
    let bsecond = b[1];

    println!("afirst: {afirst} \nasecond: {asecond} \nbfirst: {bfirst} \nbsecond: {bsecond}");


    println!("### commence user interface ###");
    println!("enter an array index: ");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("line read fail");

    let index: usize = index
        .trim()
        .parse()
        .expect("index non digit");

    let element = a[index];

    println!("index: {index} = {element}");
}
