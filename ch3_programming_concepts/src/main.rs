fn main() {
    // ###################################################################
    // ch 3.1 variables and mutability
    // ###################################################################

    // mut allows variables to change and increases readability by eluding
    // a variable may change in the future
    let mut x: i32 = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    // ###################################################################
    // ch 3.2 data types
    // ###################################################################

    // signed vs unsigned ints
    //  i8          u8
    //  i16         u16
    //  i32         u32
    //  i64         u64
    //  i128        u128
    // signed stores -(2^(n-1)) through 2^(n-1)-1
    // unsigned stores 0 through 2^n-1
    let xi: i8 = -128;
    let xu: u8 = 254;
    println!("xi = {xi}");
    println!("xu = {xu}");

    // various representations for numbers
    let xdec: i32 = 9_001;
    let xhex: i32 = 0x2329;
    let xoct: i32 = 0o21_451;
    let xbin: i32 = 0b0010_0011_0010_1001;
    println!("xdec = {xdec}");
    println!("xhex = {xhex}");
    println!("xoct = {xoct}");
    println!("xbin = {xbin}");

    // floating point; f32 and f64 (default) only
    // f64 = higher precision
    let xf64: f64 = 3.64;
    let xf32: f32 = 3.32;
    println!("xf64 = {xf64}");
    println!("xf32 = {xf32}");

    // math operations
    let xadd: i32 = 5 + 5;
    let xsub: i32 = 5 - 5;
    let xmult: i32 = 5 * 5;
    let xdiv: i32 = 5 / 5;
    let xflr: i32 = 2 / 3;
    let xrem: i32 = 6 % 5;
    println!("xadd = {xadd}");
    println!("xsub = {xsub}");
    println!("xmult = {xmult}");
    println!("xdiv = {xdiv}");
    println!("xflr = {xflr}");
    println!("xrem = {xrem}");

    // boolean true or false
    let xbool: bool = true;
    println!("xbool = {xbool}");

    // chars
    let xchar: char = 'x';
    let xemoji: char = '🐱';
    println!("xchar = {xchar}");
    println!("xemoji = {xemoji}");

    // compound types
    // tuple is a set size group of various types
    // access individual items with .
    let xtup: (i32, f64, u8, char, bool) = (9_001, 9_000.1, 254, '💋', true);
    println!("xtup.0 = {}", xtup.0);
    println!("xtup.1 = {}", xtup.1);
    println!("xtup.2 = {}", xtup.2);
    println!("xtup.3 = {}", xtup.3);
    println!("xtup.4 = {}", xtup.4);

    // or assign a group of variables to the tuple
    let (tup1, tup2, tup3, tup4, tup5) = xtup;
    println!("tup1 = {tup1}");
    println!("tup2 = {tup2}");
    println!("tup3 = {tup3}");
    println!("tup4 = {tup4}");
    println!("tup5 = {tup5}");

    // arrays are accesseed by elements in []
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {}", arr[0]);

}
