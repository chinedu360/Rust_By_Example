fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b; // Explicit return
}

// fizzbuzz
fn is_divisible(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // early return for division by zero
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible(n, 15) {
        println!("fizzbuzz {}", n)
    } else if is_divisible(n, 3) {
        println!("fizz {}", n)
    } else if is_divisible(n, 5) {
        println!("buzz {}", n)
    } else {
        println!("{}", n)
    }
}

//1,2,3,4,5,6,7,8,9,10
fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}

pub fn simple_fn() {
    // println!("Add: {}", add(10, 20));
    // println!("Subtract: {}", subtract(40, 20));

    // fizzbuzz_to(100);
    // Closures
    let x = 4;
    let square = |z| z * x;
    println!("square of 2: {}", square(2));

    let add = |a, b| a + b;
    println!("Sum: {}", add(2, 3));

    // Capturing Variables

    // "Closures can capture variables in three ways:
    // 1. **By reference**: `&T`
    let color = String::from("green");
    let print_color = || println!("Color: {}", color);
    print_color();

    // 2. **By mutable reference**: `&mut T`
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };

    increment();
    increment();
    increment();
    // 3. **By value**: `T`
    let consume = move || println!("Consumed: {}", color);
    consume();

    // print_color()

    // Closures as Input Parameters

    //Closures can also be passed as arguments to functions. Rust provides three traits for this:
    // 1. **Fn**: Captures by reference (`&T`)
    // 2. **FnMut**: Captures by mutable reference (`&mut T`)
    // 3. **FnOnce**: Captures by value (`T`)

    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    let greeting = "Hello,";
    let farewell = "Goodbye".to_owned();

    let dairy = || {
        println!("I said {}", greeting); //capture 'greeting' by reference
        println!("Then i screamed {}", farewell); // capture 'farewell' by value
    };

    apply(dairy);

    //Higher-order functions

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let sum: u32 = (0..1000)
        .map(|n| n * n) // square each numbers
        .take_while(|&n| n < 1000) // Take numbers below 1000
        .filter(|&n| is_odd(n)) // filer odd numbers
        .sum(); // sum up everything

    println!("Summmmmm: {:?}", sum);

    // Diverging Functions
    fn panic() -> ! {
        panic!("This guy never returns..he runs forever!!!");
    }

    panic();
    println!("Summmmmm: {:?}", sum);
}
