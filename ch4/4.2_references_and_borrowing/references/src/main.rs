fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("Length of '{}' is {}", s1, len);

    let mut s2 = String::from("egypt");

    change(&mut s2);

    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str(", cairo");
}
