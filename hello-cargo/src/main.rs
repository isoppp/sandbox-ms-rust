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
    let mut b_number = 15;
    b_number = 10;
    println!("the b number is {}", b_number);

    // rust allows to bind the same name variable as a previous one.
    // It's called shadowing
    // The old variable still exists but cannot refer.
    let a_number = a_number + 20;
    println!("the number is {}", a_number); // 30
}
