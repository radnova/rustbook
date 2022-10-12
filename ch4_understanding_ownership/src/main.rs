fn main() {
    ch4_1(); // what is ownership
    ch4_2(); // referencing and borrowing
    ch4_3(); // the slice type
}

fn ch4_1() {
    // ###################################################################
    // ch 4.1 flow control
    // ###################################################################

    // every value has an owner and there can only be 1 owner at a time
    // when the owner leaves scope the value is dropped
    let s: &str = "Hello";    // this string literal is valid until the end of the scope
    println!("{}", s);

    // string literals are known in size and can't store user input
    // the String type can be created and ::from() function can store input
    let mut s: String = String::from("Hello");
    s.push_str(", World!"); // push_str appends a string literal
    println!("{}", s);

    // String creates a pointer on the stack with a ptr to the heap with string data
    // a length (amount of mem in bytes) and a capacity (total amount of mem)
    let s1 = String::from("Hello");
    let s2 = s1; // value from s1 moved to s2, s1 is now invalid
    println!("{s2}");

    // to copy the data instead of shimmy shuffling use the .clone() method
    let s1: String = String::from("Hey-o");
    let s2: String = s1.clone();
    println!("{s1}, {s2}");

    // storing data as a tuple to avoid passing back and forth
    fn calc_length(s: String) -> (String, usize) {
        let length = s.len();

        (s, length)
    }
    let s1 = String::from("Howdy");
    let (s2, len) = calc_length(s1);
    println!("{s2} is {len} characters");
}

fn ch4_2() {
    // ###################################################################
    // ch 4.2 references and borrowing
    // ###################################################################

    // calc_length from above requires assigning the string to a new variable, below is the same
    // function to calculate length but when we call the function we pass a reference to s1 so that
    // we no longer require s2 to return the string argument to
    fn calc_length (s: &String) -> usize {
        s.len()
    }
    let s1: String = String::from("hola");
    //let len = calc_length(&s1);
    println!("{} is {} characters long", s1, calc_length(&s1));

    
}

fn ch4_3() {
    // ###################################################################
    // ch 4.3 slice type
    // ###################################################################

}
