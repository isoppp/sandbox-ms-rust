fn _basics() {
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

    // structs
    // struct with named fields
    struct Person {
        name: String,
        age: u8,
        likes_oranges: bool
    }
    struct Point2D(u32, u32); // tuple struct
    struct Unit; // unit struct

    let _person: Person = Person {
        name: String::from("Person name!"),
        age: 25,
        likes_oranges: false
    };
    let _origin = Point2D(0, 0);
    let _unit = Unit;

    // enums
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char), // or KeyPress(KeyPress)
        Paste(String),
        Click { x: i64, y: i64} // or Click(Click)
    }
    // struct Click {
    //     x: i64,
    //     y: i64
    // }
    // struct KeyPress(char);
    let _event: WebEvent = WebEvent::PageLoad;
    let _event: WebEvent = WebEvent::PageUnload;
    let _event: WebEvent = WebEvent::KeyPress('a');
    let _event: WebEvent = WebEvent::Paste(String::from("some text"));
    let _event: WebEvent = WebEvent::Click{x: 1, y: 1};
}

//=====================
// Exercise
//=====================
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car {
        color,
        transmission,
        convertible,
        mileage: 0
    };

    // Factory's Quality Control Department says that new cars must always have zero mileage!
    assert_eq!(car.mileage, 0);

    // Display the details of the new car order
    if car.convertible {
        println!("New car = {}, {:?}, Convertible", car.color, car.transmission);
    } else {
        println!("New car = {}, {:?}, Hardtop", car.color, car.transmission);
    }

    return car;
}

fn car_factory_assertion() {
    let client_request_1 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Manual);
    assert_eq!(client_request_1.convertible, false);

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);

    let client_request_2 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_request_2.color, "Yellow");
    assert_eq!(client_request_2.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_2.convertible, false);
}

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }

    dividend % divisor == 0 // In Rust, last expression inside a code block is always returned.
}

fn use_collection_types() {
    // Fixed size array
    let array1 = ["foo", "bar"];
    let array2 = ["baz"; 512];
    println!("{}", array1[1]);
    println!("{}", array2[99]);

    // Vector (like dynamic size Array)
    let vec1 = vec!["foo", "bar"];
    let vec2 = vec!["baz"; 512];
    let mut vec3 = Vec::new();
    vec3.push(1);
    vec3.pop();

    // Hashmap
    use std::collections::HashMap;

    let mut book_reviews: HashMap<String, String> = HashMap::new();
    book_reviews.insert("hoo".to_string(), "bar".to_string());
    assert_eq!(book_reviews.contains_key("hoo"), true);
    assert_eq!(book_reviews["hoo"], "bar".to_string());
    book_reviews.remove("hoo");
    assert_eq!(book_reviews.contains_key("hoo"), false);
}

fn main() {
    use_collection_types();
}
