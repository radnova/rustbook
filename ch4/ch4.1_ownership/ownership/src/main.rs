fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
