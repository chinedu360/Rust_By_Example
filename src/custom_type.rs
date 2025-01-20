//Struts

// Tuple struts
#[allow(dead_code)]
struct Pair(i32, f32);

// classic structs
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//unit struts
// #[derive(Debug)]
#[allow(dead_code)]
struct Unit;

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn struct_main() {
    // Create a new person
    let name = String::from("Mike");
    let age = 27;

    let mike = Person { name, age };

    // Print the person structs
    println!("{:?}", mike);

    // init a new point
    let point = Point { x: 1.9, y: 3.2 };
    let another_point = Point { x: 10.3, ..point };

    // Accessing the fields
    println!("Point cord: ({} {})", point.x, point.y);
    println!("Point cord: ({} {})", another_point.x, point.y);

    let top_left = Point { x: 1.5, y: 2.5 };
    let bottom_right = Point { x: 5.0, y: 20.1 };

    let rectangle = Rectangle {
        top_left,
        bottom_right,
    };

    println!(
        "Rect Corner: top_left ({} {}), bottom_right ({} {})",
        rectangle.top_left.x,
        rectangle.top_left.y,
        rectangle.bottom_right.x,
        rectangle.bottom_right.y
    );

    // Destruturing structs
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("Left edge: {}, Top_edge: {}", left_edge, top_edge);

    // Destruturing structs tuples
    let pair = Pair(10, 2.6);
    let Pair(interger, decimal) = pair;

    println!("Pair Contains {} and {}", interger, decimal);
}

//Enum
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
    Multiply,
}

type Ops = VeryVerboseEnumOfThingsToDoWithNumbers;

// type aliases with enum
// let ops = Ops::Add;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
        }
    }
}

pub fn enum_main() {
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('T');
    let pasted = WebEvent::Paste("Lorem ipsum dolor".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(click);

    let ops = VeryVerboseEnumOfThingsToDoWithNumbers::Multiply;

    println!("Result: {}", ops.run(10, 5))
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("The page loaded"),
        WebEvent::PageUnload => println!("The page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

//use Declaration

//define the stage enum with two variants
enum Stage {
    Beginner,
    Advanced,
}

//define the role enum with two variants
enum Role {
    Student,
    Teacher,
}

// Enum with implicit discriminators (starting at 0).
enum Number {
    Zero, //0
    One,  //1
    Two,  //2
}

// Enum with explicit discriminators.
enum Color {
    Red = 0xff0000,   // Red in hexadecimal
    Green = 0x00ff00, // Green in hexadecimal
    Blue = 0x0000ff,  // Blue in hexadecimal
}

//Use the below main fn if you are working in your own main folder directly
pub fn use_main() {
    //Enums can be cast to intergers using `as` keyword.
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    // Print colors in hexadecimal format.
    println!("roses are #{:06x}", Color::Red as i32);

    //Use explicit imports for specific variants of `Stage`.
    // the below line no longer works because we no longer in the root of the app
    // use crate::Stage::{Beginner, Advanced};
    // Do this instead
    use Stage::{Advanced, Beginner};

    //Automatic import to bring all the variants into scope
    // use crate::Role::*;
    // the below line no longer works because we no longer in the root of the app
    //Do this instead
    use Role::*;

    //Equivalent to `Stage::Beginner`
    let stage = Beginner;
    //Equivalent to `Role::Student
    let role = Student;

    match stage {
        Beginner => println!("He is a beginner"),
        Advanced => println!("He is expreinced"),
    }

    match role {
        Student => println!("This is a student"),
        Teacher => println!("This is a teacher"),
    }
}
