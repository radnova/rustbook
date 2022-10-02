fn main() {
    another_function(5);
    print_labeled_measurements(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    
    let x = five();
    println!("x: {x}\ny: {y}");

    let x = plus_one(five());
    println!("x: {x}");
}


fn another_function(x: i32) {
    println!("x: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("Measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
