fn main() {
    let a_string = "Hello, world!";
    let a_number = 10;
    let a_boolean = true;

    println!("the string is {}", a_string);
    println!("the number is {}", a_number);
    println!("the boolean is {}", a_boolean);

    // In rust, variables are immutable by default
    // a_number = 15;

    // if declare a variable with mut keyword, the variable is mutable
    let mut _b_number = 15;
    println!("the b number is {}", _b_number);
    _b_number = 10;
    println!("the b number is {}", _b_number);

    // rust allows to bind the same name variable as a previous one.
    // It's called shadowing
    // The old variable still exists but cannot refer.
    let a_number = a_number + 20;
    println!("the number is {}", a_number); // 30

    let cast_number: u32 = "32".parse().expect("Not a number!");
    println!("the cast_number is {}", cast_number); // 32

    // error: type annotations needed
    // let cast_number2 = "32".parse().expect("Not a number!");

    // types
    let _num8: i8 = 0; // unsigned is u8
    let _num16: i16 = 0; // u16
    let _num32: i32 = 0; // u32
    let _num64: i64 = 0; // u64
    let _num128: i128 = 0; // u128
    let _numsize: isize = 0; // usize ... depends on computer running program
    let _float32: f32 = 0.0;
    let _float64: f64 = 0.0;
    let _boolean: bool = 0 > 1;
    let _c: char = 'c'; // utf-8 encoded, 32-bit wide
    let _str = String::from("hello"); // string slice
    let _tuple = ("hello", 123, 'c'); // &str, i32, char
    println!("tuple.0 is {}", _tuple.0);
    assert_eq!(_tuple.1, 123);
}
